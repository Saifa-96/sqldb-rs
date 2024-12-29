use serde::{Deserialize, Serialize};

use super::{Engine, Transaction};
use crate::{
    error::{Error, Result},
    sql::{
        schema::Table,
        types::{Row, Value},
    },
    storage::{
        engine::Engine as StorageEngine,
        mvcc::{Mvcc, MvccTransaction},
    },
};

pub struct KvEngine<E: StorageEngine> {
    pub kv: Mvcc<E>,
}

impl<E: StorageEngine> Clone for KvEngine<E> {
    fn clone(&self) -> Self {
        KvEngine {
            kv: self.kv.clone(),
        }
    }
}

impl<E: StorageEngine> KvEngine<E> {
    pub fn new(engine: E) -> Self {
        Self {
            kv: Mvcc::new(engine),
        }
    }
}

impl<E: StorageEngine> Engine for KvEngine<E> {
    type Transaction = KVTransaction<E>;

    fn begin(&self) -> Result<Self::Transaction> {
        Ok(Self::Transaction::new(self.kv.begin()?))
    }
}

pub struct KVTransaction<E: StorageEngine> {
    txn: MvccTransaction<E>,
}

impl<E: StorageEngine> KVTransaction<E> {
    pub fn new(txn: MvccTransaction<E>) -> Self {
        Self { txn }
    }
}

impl<E: StorageEngine> Transaction for KVTransaction<E> {
    fn commit(&self) -> Result<()> {
        Ok(())
    }

    fn rollback(&self) -> Result<()> {
        Ok(())
    }

    fn create_row(&mut self, table_name: String, row: Row) -> Result<()> {
        let table = self.must_get_table(table_name.clone())?;
        for (i, col) in table.columns.iter().enumerate() {
            match row[i].datatype() {
                None if col.nullable => {}
                None => {
                    return Err(Error::Internal(format!(
                        "column {} cannot be null",
                        col.name
                    )))
                }
                Some(dt) if dt != col.datatype => {
                    return Err(Error::Internal(format!(
                        "column {} type mismatch",
                        col.name
                    )))
                }
                _ => {}
            }
        }

        let id = Key::Row(table_name.clone(), row[0].clone());
        let value = bincode::serialize(&row)?;
        self.txn.set(bincode::serialize(&id)?, value)?;
        Ok(())
    }

    fn scan_table(&self, table_name: String) -> Result<Vec<Row>> {
        let prefix = KeyPrefix::Row(table_name.clone());
        let results = self.txn.scan_prefix(bincode::serialize(&prefix)?)?;
        let mut rows = Vec::new();
        for result in results {
            let row: Row = bincode::deserialize(&result.value)?;
            rows.push(row);
        }
        Ok(rows)
    }

    fn create_table(&self, table: Table) -> Result<()> {
        if self.get_table(table.name.clone())?.is_some() {
            return Err(Error::Internal(format!(
                "table {} already exists",
                table.name
            )));
        }

        if table.columns.is_empty() {
            return Err(Error::Internal(format!(
                "table {} has no columns",
                table.name
            )));
        }

        let key = Key::Table(table.name.clone());
        let value = bincode::serialize(&table)?;
        self.txn.set(bincode::serialize(&key)?, value)?;
        Ok(())
    }

    fn get_table(&self, table_name: String) -> Result<Option<Table>> {
        let key = Key::Table(table_name);
        let v = self
            .txn
            .get(bincode::serialize(&key)?)?
            .map(|v| bincode::deserialize(&v))
            .transpose()?;
        Ok(v)
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Key {
    Table(String),
    Row(String, Value),
}

#[derive(Debug, Serialize, Deserialize)]
enum KeyPrefix {
    Table,
    Row(String),
}

#[cfg(test)]
mod tests {
    use super::KvEngine;
    use crate::{error::Result, sql::engine::Engine, storage::memory::MemoryEngine};

    #[test]
    fn test_create_table() -> Result<()> {
        let kv_engine = KvEngine::new(MemoryEngine::new());
        let mut s = kv_engine.session()?;

        s.execute("create table t1 (a int, b text, c integer);")?;

        s.execute("insert into t1 values(1, 'a', 1);")?;

        let v1 = s.execute("select * from t1;")?;
        print!("{:?}", v1);
        Ok(())
    }
}
