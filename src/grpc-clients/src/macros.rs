// 定义一个名为 impl_retriable_request 的宏，用于实现 RetriableRequest trait
#[allow(unused_macros)]
macro_rules! impl_retriable_request {
  // 宏的第一个匹配规则，接收请求类型、客户端类型、响应类型、获取客户端的方法名和操作方法名作为参数
  ($req: ty, $client:ty, $res:ty, $getter:ident, $op:ident) => {
    // 为 $req 类型实现 RetriableRequest trait
    impl $crate::utils::RetriableRequest for $req {
      // 指定客户端类型
      type Client = $client;
      // 指定响应类型
      type Response = $res;
      // 指定错误类型
      type Error = common_base::error::CommonError;

      /// 异步方法，用于从客户端池获取客户端实例
      ///
      /// # 参数
      /// - `pool`: 客户端池的引用
      /// - `addr`: 客户端地址的字符串引用
      ///
      /// # 返回值
      /// - `Result<impl std::ops::DerefMut<Target = Self::Client> + 'a, Self::Error>`: 包含客户端实例的 Result 类型
      async fn get_client<'a>(
        pool: &'a crate::pool::ClientPool,
        addr: &str,
      ) -> Result<impl std::ops::DerefMut<Target = Self::Client> + 'a, Self::Error> {
        // 调用客户端池的 $getter 方法获取客户端实例
        pool.$getter(addr).await
      }

      /// 异步方法，用于调用一次客户端操作
      ///
      /// # 参数
      /// - `client`: 客户端实例的可变引用
      /// - `request`: 请求实例
      ///
      /// # 返回值
      /// - `Result<Self::Response, Self::Error>`: 包含响应结果的 Result 类型
      async fn call_once(
        client: &mut Self::Client,
        request: Self,
      ) -> Result<Self::Response, Self::Error> {
        // 调用客户端的 $op 方法发送请求，并处理响应
        client
          .$op(request)
          .await
          .map(|reply| reply.into_inner())
          .map_err(Into::into)
      }
    }
  };

  // 宏的第二个匹配规则，除了上述参数外，还接收一个布尔表达式，表示是否为写请求
  ($req:ty, $client:ty, $res:ty, $getter:ident, $op:ident, $is_write_request:expr) => {
    // 为 $req 类型实现 RetriableRequest trait
    impl $crate::utils::RetriableRequest for $req {
      // 指定客户端类型
      type Client = $client;
      // 指定响应类型
      type Response = $res;
      // 指定错误类型
      type Error = common_base::error::CommonError;

      // 常量，表示是否为写请求
      const IS_WRITE_REQUEST: bool = $is_write_request;

      /// 异步方法，用于从客户端池获取客户端实例
      ///
      /// # 参数
      /// - `pool`: 客户端池的引用
      /// - `addr`: 客户端地址的字符串引用
      ///
      /// # 返回值
      /// - `Result<impl std::ops::DerefMut<Target = Self::Client> + 'a, Self::Error>`: 包含客户端实例的 Result 类型
      async fn get_client<'a>(
        pool: &'a crate::pool::ClientPool,
        addr: &str,
      ) -> Result<impl std::ops::DerefMut<Target = Self::Client> + 'a, Self::Error> {
        // 调用客户端池的 $getter 方法获取客户端实例
        pool.$getter(addr).await
      }

      /// 异步方法，用于调用一次客户端操作
      ///
      /// # 参数
      /// - `client`: 客户端实例的可变引用
      /// - `request`: 请求实例
      ///
      /// # 返回值
      /// - `Result<Self::Response, Self::Error>`: 包含响应结果的 Result 类型
      async fn call_once(
        client: &mut Self::Client,
        request: Self,
      ) -> Result<Self::Response, Self::Error> {
        client.&op(request).await.map(|reply|reply.into_inner()).map_err(Into::into)
      }
    }
  };
}

// 公开该宏，以便在 crate 内部使用
// pub(crate) use impl_retriable_request;
