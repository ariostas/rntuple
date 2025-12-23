use crate::traits::Readable;
use std::io::{Read, Result};

impl<R: Read> Readable<R> for char {
    // Note that only ASCII is supported here.
    #[inline]
    fn read_from(reader: &mut R) -> Result<char> {
        let mut buf = [0; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] as char)
    }
}

impl<R: Read> Readable<R> for i8 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<i8> {
        let mut buf = [0; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }
}

impl<R: Read> Readable<R> for u8 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<u8> {
        let mut buf = [0; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] as u8)
    }
}

impl<R: Read> Readable<R> for i16 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<i16> {
        let mut buf = [0; 2];
        reader.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }
}

impl<R: Read> Readable<R> for u16 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<u16> {
        let mut buf = [0; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl<R: Read> Readable<R> for i32 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<i32> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl<R: Read> Readable<R> for u32 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<u32> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl<R: Read> Readable<R> for i64 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<i64> {
        let mut buf = [0; 8];
        reader.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
}

impl<R: Read> Readable<R> for u64 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<u64> {
        let mut buf = [0; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

// f16 is ommitted for now since it's only available on nightly.

impl<R: Read> Readable<R> for f32 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<f32> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }
}

impl<R: Read> Readable<R> for f64 {
    #[inline]
    fn read_from(reader: &mut R) -> Result<f64> {
        let mut buf = [0; 8];
        reader.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}

impl<R: Read> Readable<R> for String {
    #[inline]
    fn read_from(reader: &mut R) -> Result<String> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        let len = u32::from_le_bytes(buf) as usize;
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf)?;
        match String::from_utf8(buf) {
            Ok(s) => Ok(s),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::ReadAs;
    use std::io::Cursor;

    #[test]
    fn test_read_basic_types() {
        let data: Vec<u8> = vec![
            0x61, // 'a'
            0xFF, // i8 = -1
            0xFE, // u8 = 254
            0x34, 0x12, // i16 = 0x1234
            0x78, 0x56, // u16 = 0x5678
            0x78, 0x56, 0x34, 0x12, // i32 = 0x12345678
            0xEF, 0xCD, 0xAB, 0x90, // u32 = 0x90ABCDEF
            0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, // i64 = 0x1234567890ABCDEF
            0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, // u64 = 0x1234567890ABCDEF
            0x66, 0xE6, 0x40, 0x46, // f32 = 12345.6
            0xA1, 0xF8, 0x31, 0xE6, 0xD6, 0x1C, 0xC8, 0x40, // f64 = 12345.6789
            0x05, 0x00, 0x00, 0x00, // string length = 5
            b'H', b'e', b'l', b'l', b'o', // "Hello"
        ];
        let mut cursor = Cursor::new(data);

        assert_eq!(cursor.read_as::<char>().unwrap(), 'a');
        assert_eq!(cursor.read_as::<i8>().unwrap(), -1);
        assert_eq!(cursor.read_as::<u8>().unwrap(), 254);
        assert_eq!(cursor.read_as::<i16>().unwrap(), 0x1234);
        assert_eq!(cursor.read_as::<u16>().unwrap(), 0x5678);
        assert_eq!(cursor.read_as::<i32>().unwrap(), 0x12345678);
        assert_eq!(cursor.read_as::<u32>().unwrap(), 0x90ABCDEF);
        assert_eq!(cursor.read_as::<i64>().unwrap(), 0x1234567890ABCDEF);
        assert_eq!(cursor.read_as::<u64>().unwrap(), 0x1234567890ABCDEF);
        assert_eq!(cursor.read_as::<f32>().unwrap(), 12345.6);
        assert_eq!(cursor.read_as::<f64>().unwrap(), 12345.6789);
        assert_eq!(cursor.read_as::<String>().unwrap(), "Hello");
    }
}
