use crate::traits::{ReadAs, Readable};
use std::io::Result;

pub struct Anchor {
    pub version_epoch: u16,
    pub version_major: u16,
    pub version_minor: u16,
    pub version_patch: u16,
    pub seek_header: u64,
    pub nbytes_header: u64,
    pub len_header: u64,
    pub seek_footer: u64,
    pub nbytes_footer: u64,
    pub len_footer: u64,
    pub max_key_size: u64,
}

impl<R: ReadAs> Readable<R> for Anchor {
    fn read_from(reader: &mut R) -> Result<Anchor> {
        // The data in the anchor still is big-endian, so we need to swap the bytes
        let version_epoch = reader.read_as::<u16>()?.swap_bytes();
        let version_major = reader.read_as::<u16>()?.swap_bytes();
        let version_minor = reader.read_as::<u16>()?.swap_bytes();
        let version_patch = reader.read_as::<u16>()?.swap_bytes();
        let seek_header = reader.read_as::<u64>()?.swap_bytes();
        let nbytes_header = reader.read_as::<u64>()?.swap_bytes();
        let len_header = reader.read_as::<u64>()?.swap_bytes();
        let seek_footer = reader.read_as::<u64>()?.swap_bytes();
        let nbytes_footer = reader.read_as::<u64>()?.swap_bytes();
        let len_footer = reader.read_as::<u64>()?.swap_bytes();
        let max_key_size = reader.read_as::<u64>()?.swap_bytes();

        Ok(Anchor {
            version_epoch,
            version_major,
            version_minor,
            version_patch,
            seek_header,
            nbytes_header,
            len_header,
            seek_footer,
            nbytes_footer,
            len_footer,
            max_key_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Anchor;
    use crate::traits::ReadAs;
    use std::io::Cursor;

    #[test]
    fn test_read_anchor() {
        let data: Vec<u8> = vec![
            0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 10, 0, 0, 0, 0, 0, 0, 1, 63, 0, 0, 0, 0,
            0, 0, 3, 229, 0, 0, 0, 0, 0, 0, 95, 184, 0, 0, 0, 0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0,
            148, 0, 0, 0, 0, 64, 0, 0, 0,
        ];
        let mut cursor = Cursor::new(data);
        let anchor = cursor.read_as::<Anchor>().unwrap();
        assert_eq!(anchor.version_epoch, 1);
        assert_eq!(anchor.version_major, 0);
        assert_eq!(anchor.version_minor, 0);
        assert_eq!(anchor.version_patch, 0);
        assert_eq!(anchor.seek_header, 266);
        assert_eq!(anchor.nbytes_header, 319);
        assert_eq!(anchor.len_header, 997);
        assert_eq!(anchor.seek_footer, 24504);
        assert_eq!(anchor.nbytes_footer, 84);
        assert_eq!(anchor.len_footer, 148);
        assert_eq!(anchor.max_key_size, 1073741824);
    }
}
