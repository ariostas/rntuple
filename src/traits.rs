use std::io::{Read, Result, Write};

pub trait Readable<R>
where
    Self: Sized,
    R: Read + ?Sized,
{
    fn read_from(reader: &mut R) -> Result<Self>;
}

pub trait Writable<W: Write>
where
    Self: Sized,
    W: Write + ?Sized,
{
    fn write_to(&self, writer: &mut W) -> Result<usize>;
}

pub trait Serializable<R, W>: Readable<R> + Writable<W>
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
}

impl<T, R, W> Serializable<R, W> for T
where
    T: Readable<R> + Writable<W>,
    R: Read + ?Sized,
    W: Write + ?Sized,
{
}

pub trait ReadAs: Read {
    fn read_as<T>(&mut self) -> Result<T>
    where
        T: Readable<Self>;
}

impl<R> ReadAs for R
where
    R: Read + ?Sized,
{
    #[inline]
    fn read_as<T>(&mut self) -> Result<T>
    where
        T: Readable<Self>,
    {
        T::read_from(self)
    }
}

pub trait WriteObj: Write {
    fn write_obj<T>(&mut self, obj: &T) -> Result<usize>
    where
        T: Writable<Self>;
}

impl<W> WriteObj for W
where
    W: Write + ?Sized,
{
    #[inline]
    fn write_obj<T>(&mut self, obj: &T) -> Result<usize>
    where
        T: Writable<Self>,
    {
        obj.write_to(self)
    }
}
