use std::ops::RangeBounds;

use crate::error::Result;

pub trait Engine {

    type EngineIterator: EngineIterator;

    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()>;

    fn get(&mut self, key: Vec<u8>) -> Result<Option<Vec<u8>>>;

    fn delete(&mut self, key: Vec<u8>) -> Result<()>;

    fn scan(&mut self, range: impl RangeBounds<Vec<u8>>) -> Self::EngineIterator;

    fn scan_prefix(&mut self, prefix: Vec<u8>) -> Self::EngineIterator {
        todo!()
    }
}

pub trait EngineIterator: DoubleEndedIterator<Item = Result<(Vec<u8>, Vec<u8>)>> {
}