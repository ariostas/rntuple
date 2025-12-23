use crate::traits::{ReadAs, Readable};
use std::io::Result;
use strum_macros::FromRepr;
use bitflags::bitflags;

#[derive(FromRepr, Debug, PartialEq)]
#[repr(u16)]
pub enum StructuralRole {
    Plain = 0x00,
    Collection = 0x01,
    Record = 0x02,
    Variant = 0x03,
    Streamer = 0x04,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags: u16 {
        const Repetitive = 0x01;
        const Projected = 0x02;
        const Checksum = 0x04;
    }
}

pub struct FieldDescription {
    pub field_version: u32,
    pub type_version: u32,
    pub parent_field_id: u32,
    pub structural_role: StructuralRole,
    pub flags: Flags,
}

impl<R> Readable<R> for FieldDescription
where
    R: ReadAs,
{
    fn read_from(reader: &mut R) -> Result<FieldDescription> {
        let field_version = reader.read_as::<u32>()?;
        let type_version = reader.read_as::<u32>()?;
        let parent_field_id = reader.read_as::<u32>()?;
        let structural_role_raw = reader.read_as::<u16>()?;
        let structural_role = StructuralRole::from_repr(structural_role_raw).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid structural role: {}", structural_role_raw),
            )
        })?;
        let flags_raw = reader.read_as::<u16>()?;
        let flags = Flags::from_bits_truncate(flags_raw);

        Ok(FieldDescription { field_version, type_version, parent_field_id, structural_role, flags })
    }
}
