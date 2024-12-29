use std::ops::RangeBounds;

use crate::error::Result;

pub trait Engine {
    type EngineIterator<'a>: EngineIterator
    where
        Self: 'a;

    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()>;

    fn get(&mut self, key: Vec<u8>) -> Result<Option<Vec<u8>>>;

    fn delete(&mut self, key: Vec<u8>) -> Result<()>;

    fn scan(&mut self, range: impl RangeBounds<Vec<u8>>) -> Self::EngineIterator<'_>;

    fn scan_prefix(&mut self, prefix: Vec<u8>) -> Self::EngineIterator<'_> {
        todo!()
    }
}

pub trait EngineIterator: DoubleEndedIterator<Item = Result<(Vec<u8>, Vec<u8>)>> {}

#[cfg(test)]
mod tests {
    use super::Engine;
    use crate::error::Result;

    fn test_point_opt(mut eng: impl Engine) -> Result<()> {
        assert_eq!(eng.get(b"not exist".to_vec())?, None);

        eng.set(b"aa".to_vec(), vec![1, 2, 3, 4])?;
        assert_eq!(eng.get(b"aa".to_vec())?, Some(vec![1, 2, 3, 4]));

        eng.set(b"aa".to_vec(), vec![5, 6, 7, 8])?;
        assert_eq!(eng.get(b"aa".to_vec())?, Some(vec![5, 6, 7, 8]));

        eng.delete(b"aa".to_vec())?;
        assert_eq!(eng.get(b"aa".to_vec())?, None);

        assert_eq!(eng.get(b"".to_vec())?, None);
        eng.set(b"".to_vec(), vec![])?;
        assert_eq!(eng.get(b"".to_vec())?, Some(vec![]));

        Ok(())
    }
}
