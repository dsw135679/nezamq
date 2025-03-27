// use protocol::{
//   AddLearnerReply, AddLearnerRequest, AppendReply, AppendRequest, ChangeMembershipReply,
//   ChangeMembershipRequest, SnapshotReply, SnapshotRequest, VoteReply, VoteRequest,
// };

// 定义一个宏，用于生成 OpenRaft 服务调用的函数
// 为了避免 `generate_openraft_service_call` 宏未使用的警告，我们可以使用 `#[allow(unused_macros)]` 属性来忽略该警告。
#[allow(unused_macros)]
macro_rules! generate_openraft_service_call {
  // 宏的匹配规则，接受四个参数：函数名、请求类型、响应类型和变体
  // ident：专用于标识符;ty: 匹配类型;expr: 匹配表达式；pat：模式（用于匹配）
  // $fn_name:ident 匹配一个标识符，并将其绑定到 $fn_name 变量
  // $req_ty:ty 匹配一个类型，并将其绑定到 $req_ty 变量
  // $rep_ty:ty 匹配一个类型，并将其绑定到 $rep_ty 变量
  // $variant:ident 匹配一个标识符，并将其绑定到 $variant 变量
  ($fn_name:ident, $req_ty:ty, $rep_ty:ty, $variant:ident) => {
    // 生成一个异步的公共函数，函数名为 $fn_name
    /// 为 OpenRaft 服务生成一个异步调用函数。
    ///
    /// 该函数尝试通过重试机制调用指定地址的服务。
    ///
    /// # 参数
    /// - `client_pool`: 客户端连接池的引用。
    /// - `addrs`: 服务地址列表，列表中的每个元素可以转换为字符串引用。
    /// - `request`: 请求对象的引用，类型为 `$req_ty`。
    ///
    /// # 返回值
    /// 如果调用成功，返回一个指向响应对象的引用，类型为 `$rep_ty`。
    /// 如果调用失败，返回一个 `CommonError` 错误。
    pub async fn $fn_name(
      // 客户端连接池的引用
      client_pool: &ClientPool,
      // 服务地址列表，列表中的每个元素可以转换为字符串引用
      addrs: &[impl AsRef<str>],
      // 请求对象的引用
      request: &$req_ty,
    ) -> Result<&$rep_ty, CommonError> {
      // 调用重试调用工具函数，传入客户端连接池、服务地址列表和请求对象
      $crate::utils::retry_call(client_pool, addrs, request).await
    }
  };
}

// generate_openraft_service_call!(placement_openraft_vote, VoteRequest, VoteReply, Vote);
// generate_openraft_service_call!(
//   placement_openraft_append,
//   AppendRequest,
//   AppendReply,
//   Append
// );

// generate_openraft_service_call!(
//   placement_openraft_snapshot,
//   SnapshotRequest,
//   SnapshotReply,
//   Snapshot
// );

// generate_openraft_service_call!(
//   placement_openraft_learner,
//   AddLearnerRequest,
//   AddLearnerReply,
//   AddLearner
// );

// generate_openraft_service_call!(
//   placement_openraft_change_membership,
//   ChangeMembershipRequest,
//   ChangeMembershipReply,
//   ChangeMembership
// );
