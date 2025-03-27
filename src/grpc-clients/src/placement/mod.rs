pub mod inner;
pub mod kv;
pub mod openraft;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlacementCenterInterface {
  // kv interface
  Set,
  Get,
  Delete,
  Exists,

  // placement inner interface
  ClusterStatus,
  ListNode,
  RegisterNode,
  UnRegisterNOde,
  Heartbeat,
  SendRaftMessage,
  SendRaftConfChange,

  // open raft
  Vote,
  Append,
  Snapshot,
  AddLearner,
  ChangeMembership,
}
