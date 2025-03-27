use std::{ops::RangeBounds, sync::Arc};

use openraft::{
  AnyError, Entry, ErrorSubject, ErrorVerb, LogId, LogState, NodeId, RaftLogReader, StorageError,
  Vote,
  storage::{IOFlushed, RaftLogStorage},
};
use rocksdb::{ColumnFamily, DB, Direction};

use crate::openraft::typeconfig::TypeConfig;

use super::{StorageResult, bin_to_id, cf_raft_logs, cf_raft_store, id_to_bin};

#[derive(Debug, Clone)]
pub struct LogStore {
  pub db: Arc<DB>,
}

impl LogStore {
  /// 获取 "store" 列族的句柄。
  ///
  /// 该方法从RocksDB实例中获取名为 "store" 的列族句柄。
  /// 如果该列族不存在，该方法会使用 `unwrap` 方法引发恐慌，因为假定该列族是必需的。
  ///
  /// # 返回值
  /// - `&ColumnFamily`: 返回 "store" 列族的不可变引用。
  fn store(&self) -> &ColumnFamily {
    // 从RocksDB实例中获取 "store" 列族的句柄
    self.db.cf_handle(&cf_raft_store()).unwrap()
  }

  /// 获取 "logs" 列族的句柄。
  ///
  /// 该方法从RocksDB实例中获取名为 "logs" 的列族句柄。
  /// 如果该列族不存在，该方法会使用 `unwrap` 方法引发恐慌，因为假定该列族是必需的。
  ///
  /// # 返回值
  /// - `&ColumnFamily`: 返回 "logs" 列族的不可变引用。
  fn logs(&self) -> &ColumnFamily {
    // 从RocksDB实例中获取 "logs" 列族的句柄
    self.db.cf_handle(&cf_raft_logs()).unwrap()
  }

  /// 刷新写入日志到磁盘
  ///
  /// 该方法会将写入日志（WAL）刷新到磁盘，确保数据持久化。
  /// 它接受一个 `ErrorSubject` 和 `ErrorVerb` 作为参数，用于在发生错误时提供上下文信息。
  ///
  /// # 参数
  /// - `subject`: 错误主题，用于标识错误发生的上下文。
  /// - `verb`: 错误动词，用于描述错误的操作。
  ///
  /// # 返回值
  /// - `Result<(), StorageError<TypeConfig>>`: 如果刷新成功，返回 `Ok(())`；如果发生错误，返回 `Err` 并包含相应的 `StorageError`。
  fn flush(
    &self,
    subject: ErrorSubject<TypeConfig>,
    verb: ErrorVerb,
  ) -> Result<(), StorageError<TypeConfig>> {
    // 刷新写入日志到磁盘
    // 如果刷新过程中发生错误，将其转换为 `StorageError` 并返回
    self
      .db
      .flush_wal(true)
      .map_err(|e| StorageError::new(subject, verb, AnyError::new(&e)))?;
    Ok(())
  }

  /// 获取最后一次清除的日志ID。
  ///
  /// 该方法从RocksDB存储中读取最后一次清除的日志ID。
  /// 它首先尝试从指定的列族中获取存储的最后一次清除的日志ID。
  /// 如果获取成功，它会尝试将存储的字节数据反序列化为 `LogId<u64>` 类型。
  ///
  /// # 返回值
  /// - `StorageResult<Option<LogId<u64>>>`: 如果成功，返回包含最后一次清除的日志ID的 `Option`；如果未找到，则返回 `None`。
  /// 如果在读取或反序列化过程中发生错误，返回相应的 `StorageError`。
  fn get_last_purged_(&self) -> StorageResult<Option<LogId<u64>>> {
    // 从指定的列族中获取存储的最后一次清除的日志ID
    // 如果获取过程中发生错误，将其转换为 `StorageError` 并返回
    Ok(
      self
        .db
        .get_cf(&self.store(), b"last_purged_log_id")
        .map_err(|e| StorageError::read(&e))?
        // 如果获取到了数据，尝试将其反序列化为 `LogId<u64>` 类型
        .and_then(|v| serde_json::from_slice(&v).ok()),
    )
  }

  /// 设置最后一次清除的日志ID。
  ///
  /// 该方法将指定的日志ID存储到RocksDB的 "_raft_store" 列族中。
  /// 存储之前，日志ID会被序列化为字节数据。
  /// 存储完成后，会调用 `flush` 方法将写入日志刷新到磁盘，确保数据持久化。
  ///
  /// # 参数
  /// - `log_id`: 要设置的最后一次清除的日志ID。
  ///
  /// # 返回值
  /// - `StorageResult<()>`: 如果设置和刷新成功，返回 `Ok(())`；如果发生错误，返回相应的 `StorageError`。
  fn set_last_purged_(&self, log_id: LogId<u64>) -> StorageResult<()> {
    // 将日志ID序列化为字节数据，并将其存储到 "_raft_store" 列族中
    // 如果存储过程中发生错误，将其转换为 `StorageError` 并返回
    self
      .db
      .put_cf(
        &self.store(),
        b"last_purged_log_id",
        serde_json::to_vec(&log_id).unwrap().as_slice(),
      )
      .map_err(|e| StorageError::write(&e))?;

    // 刷新写入日志到磁盘，确保数据持久化
    // 如果刷新过程中发生错误，将其转换为 `StorageError` 并返回
    self.flush(ErrorSubject::Store, ErrorVerb::Write)?;
    Ok(())
  }

  /// 设置已提交的日志ID。
  ///
  /// 该方法将指定的已提交日志ID存储到RocksDB的 "_raft_store" 列族中。
  /// 存储之前，已提交日志ID会被序列化为字节数据。
  /// 存储完成后，会调用 `flush` 方法将写入日志刷新到磁盘，确保数据持久化。
  ///
  /// # 参数
  /// - `committed`: 要设置的已提交日志ID的可选值。
  ///
  /// # 返回值
  /// - `Result<(), StorageError<TypeConfig>>`: 如果设置和刷新成功，返回 `Ok(())`；如果发生错误，返回相应的 `StorageError`。
  fn set_committed_(
    &self,
    committed: &Option<LogId<NodeId>>,
  ) -> Result<(), StorageError<TypeConfig>> {
    // 将已提交日志ID序列化为字节数据
    let jv = serde_json::to_vec(committed).unwrap();

    // 将序列化后的数据存储到 "_raft_store" 列族中
    // 如果存储过程中发生错误，将其转换为 `StorageError` 并返回
    self
      .db
      .put_cf(self.store(), b"committed", jv)
      .map_err(|e| StorageError::write(&e))?;

    // 刷新写入日志到磁盘，确保数据持久化
    // 如果刷新过程中发生错误，将其转换为 `StorageError` 并返回
    self.flush(ErrorSubject::Store, ErrorVerb::Write)?;
    Ok(())
  }

  /// 获取已提交的日志ID。
  ///
  /// 该方法从RocksDB存储中读取已提交的日志ID。
  /// 它首先尝试从指定的列族中获取存储的已提交日志ID。
  /// 如果获取成功，它会尝试将存储的字节数据反序列化为 `LogId<NodeId>` 类型。
  ///
  /// # 返回值
  /// - `StorageResult<Option<LogId<NodeId>>>`: 如果成功，返回包含已提交日志ID的 `Option`；如果未找到，则返回 `None`。
  /// 如果在读取或反序列化过程中发生错误，返回相应的 `StorageError`。
  fn get_committed_(&self) -> StorageResult<Option<LogId<NodeId>>> {
    // 从指定的列族中获取存储的已提交日志ID
    // 如果获取过程中发生错误，将其转换为 `StorageError` 并返回
    Ok(
      self
        .db
        .get_cf(self.store(), b"committed")
        .map_err(|e| StorageError::read(&e))?
        // 如果获取到了数据，尝试将其反序列化为 `LogId<NodeId>` 类型
        .and_then(|v| serde_json::from_slice(&v).ok()),
    )
  }

  /// 设置投票信息。
  ///
  /// 该方法将指定的投票信息存储到RocksDB的 "_raft_store" 列族中。
  /// 存储之前，投票信息会被序列化为字节数据。
  /// 存储完成后，会调用 `flush` 方法将写入日志刷新到磁盘，确保数据持久化。
  ///
  /// # 参数
  /// - `vote`: 要设置的投票信息。
  ///
  /// # 返回值
  /// - `StorageResult<()>`: 如果设置和刷新成功，返回 `Ok(())`；如果发生错误，返回相应的 `StorageError`。
  fn set_vote_(&self, vote: &Vote<NodeId>) -> StorageResult<()> {
    // 将投票信息序列化为字节数据，并将其存储到 "_raft_store" 列族中
    // 如果存储过程中发生错误，将其转换为 `StorageError` 并返回
    self
      .db
      .put_cf(self.store(), b"vote", serde_json::to_vec(vote).unwrap())
      .map_err(|e| StorageError::write_vote(&e))?;

    // 刷新写入日志到磁盘，确保数据持久化
    // 如果刷新过程中发生错误，将其转换为 `StorageError` 并返回
    self.flush(ErrorSubject::Vote, ErrorVerb::Write)?;
    Ok(())
  }

  /// 获取投票信息。
  ///
  /// 该方法从RocksDB存储中读取投票信息。
  /// 它首先尝试从指定的列族中获取存储的投票信息。
  /// 如果获取成功，它会尝试将存储的字节数据反序列化为 `Vote<NodeId>` 类型。
  ///
  /// # 返回值
  /// - `StorageResult<Option<Vote<NodeId>>>`: 如果成功，返回包含投票信息的 `Option`；如果未找到，则返回 `None`。
  /// 如果在读取或反序列化过程中发生错误，返回相应的 `StorageError`。
  fn get_vote_(&self) -> StorageResult<Option<Vote<NodeId>>> {
    // 从指定的列族中获取存储的投票信息
    // 如果获取过程中发生错误，将其转换为 `StorageError` 并返回
    Ok(
      self
        .db
        .get_cf(self.store(), b"vote")
        .map_err(|e| StorageError::write_vote(&e))?
        // 如果获取到了数据，尝试将其反序列化为 `Vote<NodeId>` 类型
        .and_then(|v| serde_json::from_slice(&v).ok()),
    )
  }
}

impl RaftLogReader<TypeConfig> for LogStore {
  /// 异步尝试获取指定范围内的日志条目。
  ///
  /// 该方法会尝试从RocksDB中读取指定范围内的日志条目。
  /// 范围由 `RangeBounds<u64>` 类型的 `range` 参数指定。
  ///
  /// # 参数
  /// - `range`: 日志条目的范围，实现了 `RangeBounds<u64>` 特征。
  ///
  /// # 返回值
  /// - `impl StorageResult<Vec<Entry<TypeConfig>>>`: 包含日志条目的 `Vec` 的结果。
  /// 如果读取过程中发生错误，返回相应的 `StorageError`。
  async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + OptionalSend>(
    &mut self,
    range: RB,
  ) -> impl StorageResult<Vec<Entry<TypeConfig>>> {
    // 根据范围的起始边界确定起始ID
    let start = match range.start_bound() {
      // 如果范围是包含起始值的，将起始值转换为二进制
      std::ops::Bound::Included(x) => id_to_bin(*x),
      // 如果范围是排除起始值的，将起始值加1后转换为二进制
      std::ops::Bound::Excluded(x) => id_to_bin(*x + 1),
      // 如果范围是无界的，起始ID设为0并转换为二进制
      std::ops::Bound::Unbounded => id_to_bin(0),
    };

    // 从RocksDB的 "_raft_logs" 列族中创建一个迭代器，从起始ID开始向前遍历
    self
      .db
      .iterator_cf(
        self.logs(),
        rocksdb::IteratorMode::From(&start, Direction::Forward),
      )
      .map(|res| {
        // 解包迭代器的结果，获取键值对
        let (id, val) = res.unwrap();
        // 尝试将值反序列化为 `Entry` 类型
        let entry: StorageResult<Entry<_>> =
          serde_json::from_slice(&val).map_err(|e| StorageError::read_logs(&e));
        // 将键转换为日志ID
        let id = bin_to_id(&id);

        // 断言反序列化后的日志ID的索引与键转换后的ID相等
        assert_eq!(Ok(id), entry.as_ref().map(|e| e.log_id.index));
        // 返回日志ID和反序列化后的日志条目
        (id, entry)
      })
      // 过滤出在指定范围内的日志条目
      .take_while(|(id, _)| range.contains(id))
      // 提取日志条目
      .map(|x| x.1)
      // 收集所有符合条件的日志条目到一个 `Vec` 中
      .collect()
  }

  /// 异步读取投票信息。
  ///
  /// 该方法会调用 `get_vote_` 函数从 RocksDB 存储中读取投票信息。
  ///
  /// # 返回值
  /// - `Result<Option<Vote<NodeId>>, StorageError<TypeConfig>>`: 如果成功，返回包含投票信息的 `Option`；如果未找到，则返回 `None`。
  /// 如果在读取或反序列化过程中发生错误，返回相应的 `StorageError`。
  async fn read_vote(&mut self) -> Result<Option<Vote<NodeId>>, StorageError<TypeConfig>> {
    // 调用 get_vote_ 方法获取投票信息
    self.get_vote_()
  }
}

impl RaftLogStorage<TypeConfig> for LogStore {
  type LogReader = Self;

  /// 异步获取日志状态。
  ///
  /// 该方法会从RocksDB中读取最后一条日志的ID和最后一次清除的日志ID，
  /// 并使用这些信息构建并返回一个 `LogState` 结构体。
  ///
  /// # 返回值
  /// - `StorageResult<LogState<TypeConfig>>`: 包含日志状态的结果。
  /// 如果读取过程中发生错误，返回相应的 `StorageError`。
  async fn get_log_state(&mut self) -> StorageResult<LogState<TypeConfig>> {
    // 从RocksDB的 "_raft_logs" 列族中获取最后一条日志条目
    let last = self
      .db
      .iterator_cf(&self.logs(), rocksdb::IteratorMode::End)
      .next()
      .and_then(|res| {
        // 解包迭代器的结果，获取键值对
        let (_, ent) = res.unwrap();
        // 尝试将值反序列化为 `Entry` 类型，并提取其日志ID
        Some(
          serde_json::from_slice::<Entry<TypeConfig>>(&ent)
            .ok()?
            .log_id,
        )
      });

    // 获取最后一次清除的日志ID
    let last_purged_log_id = self.get_last_purged_()?;

    // 确定最后一条日志的ID
    let last_log_id = match last {
      // 如果没有找到最后一条日志，使用最后一次清除的日志ID
      None => last_purged_log_id,
      // 否则，使用找到的最后一条日志的ID
      Some(x) => Some(x),
    };

    // 返回日志状态
    Ok(LogState {
      last_purged_log_id,
      last_log_id,
    })
  }

  /// 异步获取日志读取器。
  ///
  /// 该方法返回一个 `Self::LogReader` 类型的日志读取器实例。
  /// 它通过克隆当前实例来创建一个新的日志读取器，以确保线程安全和独立性。
  ///
  /// # 返回值
  /// - `Self::LogReader`: 返回一个新的日志读取器实例。
  async fn get_log_reader(&mut self) -> Self::LogReader {
    // 克隆当前实例以创建一个新的日志读取器
    self.clone()
  }

  /// 异步保存投票信息到存储中。
  ///
  /// 该方法使用 `tracing` 库进行跟踪，日志级别为 `trace`，并跳过 `self` 参数以避免不必要的日志记录。
  /// 它调用 `set_vote_` 方法将指定的投票信息存储到RocksDB的 "_raft_store" 列族中，并确保数据持久化。
  ///
  /// # 参数
  /// - `vote`: 要保存的投票信息的引用。
  ///
  /// # 返回值
  /// - `Result<(), StorageError<TypeConfig>>`: 如果保存成功，返回 `Ok(())`；如果发生错误，返回相应的 `StorageError`。
  #[tracing::instrument(level = "trace", skip(self))]
  async fn save_vote(&mut self, vote: &Vote<NodeId>) -> Result<(), StorageError<TypeConfig>> {
    // 调用 set_vote_ 方法将投票信息保存到存储中
    self.set_vote_(vote)
  }

  /// 异步追加日志条目到存储中。
  ///
  /// 该方法使用 `tracing` 库进行跟踪，日志级别为 `trace`，并跳过所有参数以避免不必要的日志记录。
  /// 它将指定的日志条目集合追加到RocksDB的 "_raft_logs" 列族中，并确保数据持久化。
  /// 追加完成后，调用 `callback` 函数通知IO操作已完成。
  ///
  /// # 参数
  /// - `entries`: 要追加的日志条目集合，需实现 `IntoIterator` 特征。
  /// - `callback`: 用于通知IO操作完成的回调函数。
  ///
  /// # 返回值
  /// - `StorageResult<()>`: 如果追加成功，返回 `Ok(())`；如果发生错误，返回相应的 `StorageError`。
  #[tracing::instrument(level = "trace", skip_all)]
  async fn append<I>(&mut self, entries: I, callback: IOFlushed<TypeConfig>) -> StorageResult<()>
  where
    // 泛型 I 需实现 IntoIterator 特征，其 Item 类型为 Entry<TypeConfig>，且支持 Send 特征
    I: IntoIterator<Item = Entry<TypeConfig>> + Send,
    // I 的迭代器类型也需支持 Send 特征
    I::IntoIter: Send,
  {
    // 遍历所有要追加的日志条目
    for entry in entries {
      // 将日志条目的索引转换为二进制格式
      let id = id_to_bin(entry.log_id.index);
      // 断言转换后的二进制ID反转换回的索引与原索引相等
      assert_eq!(bin_to_id(&id), entry.log_id.index);
      // 将日志条目序列化为字节数据，并存储到 "_raft_logs" 列族中
      // 如果序列化或存储过程中发生错误，将其转换为 `StorageError` 并返回
      self
        .db
        .put_cf(
          // 调用 logs 方法获取 "_raft_logs" 列族的句柄
          &self.logs(),
          id,
          serde_json::to_vec(&entry).map_err(|e| StorageError::write_logs(&e))?,
        )
        .map_err(|e| StorageError::write_logs(&e))?;
    }

    // 通知回调函数IO操作已完成
    callback.io_completed(Ok(()));

    // 返回成功结果
    Ok(())
  }

  /// 异步截断日志，删除指定日志ID之后的所有日志条目。
  ///
  /// # 参数
  /// - `log_id`: 截断操作的目标日志ID，删除此ID之前的所有日志条目。
  ///
  /// # 返回值
  /// - `StorageResult<()>`: 如果截断操作成功，返回 `Ok(())`；如果发生错误，返回相应的 `StorageError`。
  #[tracing::instrument(level = "debug", skip(self))]
  async fn truncate(&mut self, log_id: LogId<NodeId>) -> StorageResult<()> {
    // 记录调试日志，指示要删除的日志范围
    tracing::debug!("delete_log:[{:?}],+00", log_id);

    // 将指定日志ID的索引转换为二进制格式，作为删除范围的起始点
    let from = id_to_bin(log_id.index);
    // 将一个较大的数值转换为二进制格式，作为删除范围的结束点
    let to = id_to_bin(Oxff_ff_ff_ff_ff_ff_ff_ff);

    // 在 "_raft_logs" 列族中删除指定范围的日志条目
    // 如果删除过程中发生错误，将其转换为 `StorageError` 并返回
    self
      .db
      .delete_range_cf(&self.logs(), &from, &to)
      .map_err(|e| StorageError::write_logs(&e))
  }

  /// 异步清除日志，删除指定日志ID之前（包括该日志ID）的所有日志条目。
  ///
  /// 该方法用于从RocksDB存储中删除指定日志ID之前（包括该日志ID）的所有日志条目。
  /// 它首先更新最后一次清除的日志ID，然后在 "_raft_logs" 列族中删除指定范围的日志条目。
  ///
  /// # 参数
  /// - `log_id`: 清除操作的目标日志ID，删除此ID之前（包括该日志ID）的所有日志条目。
  ///
  /// # 返回值
  /// - `StorageResult<()>`: 如果清除操作成功，返回 `Ok(())`；如果发生错误，返回相应的 `StorageError`。
  #[tracing::instrument(level = "debug", skip(self))]
  async fn purge(&mut self, log_id: LogId<NodeId>) -> StorageResult<()> {
    // 记录调试日志，指示要删除的日志范围
    tracing::debug!("delete_log: [0,{:?]}]", log_id);

    // 设置最后一次清除的日志ID
    self.set_last_purged_(log_id)?;

    // 将起始日志ID（0）转换为二进制格式，作为删除范围的起始点
    let from = id_to_bin(0);
    // 将指定日志ID的索引加1后转换为二进制格式，作为删除范围的结束点
    let to = id_to_bin(log_id.index + 1);

    // 在 "_raft_logs" 列族中删除指定范围的日志条目
    // 如果删除过程中发生错误，将其转换为 `StorageError` 并返回
    self
      .db
      .delete_range_cf(&self.logs(), &from, &to)
      .map_err(|e| StorageError::write_logs(&e))
  }

  /// 异步保存已提交的日志ID到存储中。
  ///
  /// 该方法使用 `set_committed_` 方法将指定的已提交日志ID存储到RocksDB的 "_raft_store" 列族中，并确保数据持久化。
  ///
  /// # 参数
  /// - `_committed`: 要保存的已提交日志ID的可选值。
  ///
  /// # 返回值
  /// - `Result<(), StorageError<TypeConfig>>`: 如果保存成功，返回 `Ok(())`；如果发生错误，返回相应的 `StorageError`。
  async fn save_committed(
    &mut self,
    _committed: Option<LogId<NodeId>>,
  ) -> Result<(), StorageError<TypeConfig>> {
    // 调用 set_committed_ 方法将已提交日志ID保存到存储中
    self.set_committed_(&_committed)?;
    // 返回成功结果
    Ok(())
  }

  /// 异步读取已提交的日志ID。
  ///
  /// 该方法会调用 `get_committed_` 函数从 RocksDB 存储中读取已提交的日志ID。
  ///
  /// # 返回值
  /// - `Result<Option<LogId<NodeId>>, StorageError<TypeConfig>>`: 如果成功，返回包含已提交日志ID的 `Option`；如果未找到，则返回 `None`。
  /// 如果在读取或反序列化过程中发生错误，返回相应的 `StorageError`。
  async fn read_committed(&mut self) -> Result<Option<LogId<NodeId>>, StorageError<TypeConfig>> {
    // 调用 get_committed_ 方法获取已提交的日志ID
    let c = self.get_committed_()?;
    // 返回已提交的日志ID
    Ok(c)
  }
}
