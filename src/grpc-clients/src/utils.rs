// 引入标准库中的HashSet、DerefMut和Duration类型
use std::{collections::HashSet, ops::DerefMut, time::Duration};
// 引入common_base库中的CommonError类型
use common_base::error::CommonError;
// 引入regex库中的Regex类型
use regex::Regex;
// 引入tokio库中的sleep函数
use tokio::time::sleep;
// 引入当前模块中的ClientPool、retry_sleep_time和retry_times
use crate::{pool::ClientPool, retry_sleep_time, retry_times};

/// 定义一个可重试请求的trait
///
/// 该trait定义了与客户端交互和请求重试相关的方法。
///
/// # 类型参数
/// - `Client`: 客户端类型。
/// - `Response`: 请求的响应类型。
/// - `Error`: 请求可能发生的错误类型。
pub(crate) trait RetriableRequest: Clone {
  /// 客户端类型
  type Client;
  /// 请求的响应类型
  type Response;
  /// 请求可能发生的错误类型
  type Error: std::error::Error;

  /// 表示该请求是否为写请求，默认为false
  const IS_WRITE_REQUEST: bool = false;

  /// 从客户端池中获取一个客户端
  ///
  /// # 参数
  /// - `pool`: 客户端池的引用
  /// - `addr`: 地址的引用
  ///
  /// # 返回值
  /// - 成功时返回一个实现了DerefMut<Target = Self::Client>的对象
  /// - 失败时返回Self::Error类型的错误
  async fn get_client<'a>(
    pool: &'a ClientPool,
    addr: &str,
  ) -> Result<impl DerefMut<Target = Self::Client> + 'a, Self::Error>;

  /// 执行一次请求
  ///
  /// # 参数
  /// - `client`: 客户端的可变引用
  /// - `request`: 请求对象
  ///
  /// # 返回值
  /// - 成功时返回Self::Response类型的响应
  /// - 失败时返回Self::Error类型的错误
  async fn call_once(
    client: &mut Self::Client,
    request: Self,
  ) -> Result<Self::Response, Self::Error>;
}

/// 重试调用请求
///
/// 该函数会重试请求，直到达到最大重试次数或请求成功。
///
/// # 参数
/// - `client_pool`: 客户端池的引用
/// - `addrs`: 地址列表的引用
/// - `request`: 请求对象
///
/// # 返回值
/// - 成功时返回Req::Response类型的响应
/// - 失败时返回CommonError类型的错误
pub(crate) async fn retry_call<Req>(
  client_pool: &ClientPool,
  addrs: &[impl AsRef<str>],
  request: Req,
) -> Result<Req::Response, CommonError>
where
  Req: RetriableRequest,
  Req::Error: Into<CommonError>,
{
  // 如果地址列表为空，返回错误
  if addrs.is_empty() {
    return Err(CommonError::CommonError(
      "Call address list cannot be empty".to_string(),
    ));
  }

  // 初始化重试次数
  let mut times = 1;
  // 初始化已尝试的地址集合
  let mut tried_addrs = HashSet::new();
  loop {
    // 计算当前尝试的地址索引
    let index = times % addrs.len();
    // 获取当前尝试的地址
    let addr = addrs[index].as_ref();
    // 如果是写请求，获取领导者地址，否则使用当前地址
    let target_addr = if Req::IS_WRITE_REQUEST {
      client_pool
        .get_leader_addr(addr)
        .map(|leader| leader.value().to_string())
        .unwrap_or_else(|| addr.to_string())
    } else {
      addr.to_string()
    };
    // // 如果该地址已经尝试过
    // if tried_addrs.contains(&target_addr) {
    //   // 如果超过最大重试次数，返回错误
    //   if times > retry_times() {
    //     return Err(CommonError::CommonError("Not found leader".to_string()));
    //   }
    //   // 增加重试次数
    //   times += 1;
    //   // 继续下一次循环
    //   continue;
    // }

    // 从客户端池获取客户端
    let mut client = Req::get_client(client_pool, &target_addr)
      .await
      .map_err(Into::into)?;

    // 执行一次请求
    match Req::call_once(client.deref_mut(), request.clone()).await {
      // 请求成功，返回响应
      Ok(data) => return Ok(data),
      // 请求失败
      Err(e) => {
        // 将错误转换为CommonError类型
        let err: CommonError = e.into();

        // 如果错误信息包含"forward request to"
        if err.to_string().contains("forward request to") {
          // 将当前地址添加到已尝试地址集合
          tried_addrs.insert(target_addr);

          // 获取转发地址
          if let Some(leader_addr) = get_forward_addr(&err) {
            // 设置领导者地址
            client_pool.set_leader_addr(addr.to_string(), leader_addr.clone());

            // 如果领导者地址未尝试过
            if !tried_addrs.contains(&leader_addr) {
              // 获取领导者客户端
              let mut leader_client = match Req::get_client(client_pool, &leader_addr).await {
                Ok(client) => client,
                Err(_) => {
                  // 将领导者地址添加到已尝试地址集合
                  tried_addrs.insert(leader_addr);
                  // 继续下一次循环
                  continue;
                }
              };
              // 执行一次请求
              match Req::call_once(leader_client.deref_mut(), request.clone()).await {
                // 请求成功，返回响应
                Ok(data) => return Ok(data),
                // 请求失败
                Err(_) => {
                  // 将领导者地址添加到已尝试地址集合
                  tried_addrs.insert(leader_addr);
                }
              }
            } else {
              // 领导者地址已尝试过，返回错误
              return Err(err);
            }

            // 如果超过最大重试次数，返回错误
            if times > retry_times() {
              return Err(CommonError::CommonError("Not found leader".to_string()));
            }
            // 增加重试次数
            times += 1;
            // 休眠一段时间
            sleep(Duration::from_secs(retry_sleep_time(times))).await;
          }
        }
      }
    }
  }
}

/// 从错误信息中提取转发地址
///
/// # 参数
/// - `err`: 错误信息的引用
///
/// # 返回值
/// - 成功时返回转发地址的字符串
/// - 失败时返回None
pub fn get_forward_addr(err: &CommonError) -> Option<String> {
  // 获取错误信息的字符串表示
  let error_info = err.to_string();
  // 编译正则表达式
  let re = Regex::new(r"rpc_addr: ([^}]+)").unwrap();
  // 如果匹配到捕获组
  if let Some(caps) = re.captures(&error_info) {
    // 如果捕获组中有匹配项
    if let Some(rpc_addr) = caps.get(1) {
      // 获取匹配的字符串
      let mut leader_addr = rpc_addr.as_str().to_string();
      // 去除反斜杠
      leader_addr = leader_addr.replace("\\", "");
      // 去除双引号
      leader_addr = leader_addr.replace("\"", "");
      // 去除空格
      leader_addr = leader_addr.replace(" ", "");
      // 返回处理后的地址
      return Some(leader_addr);
    }
  }
  // 未匹配到，返回None
  None
}
