use crate::traits::{ReadAs, Readable};
use std::io::Result;

pub struct ListFrame<T> {
    size: u64,
    num_items: u32,
    pub contents: Vec<T>,
}

impl<R, T> Readable<R> for ListFrame<T>
where
    R: ReadAs,
    T: ReadAs + Readable<R>,
{
    fn read_from(reader: &mut R) -> Result<ListFrame<T>> {
        let size = reader.read_as::<i64>()?;
        if size > 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Positive size in RecordFrame",
            ));
        }
        let size = size.abs() as u64;
        let num_items = reader.read_as::<u32>()?;
        let mut contents = Vec::with_capacity(num_items as usize);
        for _ in 0..num_items {
            let item = reader.read_as::<T>()?;
            contents.push(item);
        }

        Ok(ListFrame {
            size,
            num_items,
            contents,
        })
    }
}
