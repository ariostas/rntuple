use crate::traits::{ReadAs, Readable};
use std::io::Result;

pub struct RecordFrame<T> {
    size: u64,
    pub content: T,
}

impl<R, T> Readable<R> for RecordFrame<T>
where
    R: ReadAs,
    T: ReadAs + Readable<R>,
{
    fn read_from(reader: &mut R) -> Result<RecordFrame<T>> {
        let size = reader.read_as::<i64>()?;
        if size < 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Negative size in RecordFrame",
            ));
        }
        let size = size as u64;
        let content = reader.read_as::<T>()?;

        Ok(RecordFrame { size, content })
    }
}
