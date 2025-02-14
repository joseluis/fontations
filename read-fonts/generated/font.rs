// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The OpenType [Table Directory](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#table-directory)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct TableDirectoryMarker {
    table_records_byte_len: usize,
}

impl TableDirectoryMarker {
    pub fn sfnt_version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn num_tables_byte_range(&self) -> Range<usize> {
        let start = self.sfnt_version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn search_range_byte_range(&self) -> Range<usize> {
        let start = self.num_tables_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn entry_selector_byte_range(&self) -> Range<usize> {
        let start = self.search_range_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn range_shift_byte_range(&self) -> Range<usize> {
        let start = self.entry_selector_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn table_records_byte_range(&self) -> Range<usize> {
        let start = self.range_shift_byte_range().end;
        start..start + self.table_records_byte_len
    }
}

impl MinByteRange for TableDirectoryMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.table_records_byte_range().end
    }
}

impl<'a> FontRead<'a> for TableDirectory<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u32>();
        let num_tables: u16 = cursor.read()?;
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        let table_records_byte_len = (num_tables as usize)
            .checked_mul(TableRecord::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(table_records_byte_len);
        cursor.finish(TableDirectoryMarker {
            table_records_byte_len,
        })
    }
}

/// The OpenType [Table Directory](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#table-directory)
pub type TableDirectory<'a> = TableRef<'a, TableDirectoryMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> TableDirectory<'a> {
    /// 0x00010000 or 0x4F54544F
    pub fn sfnt_version(&self) -> u32 {
        let range = self.shape.sfnt_version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of tables.
    pub fn num_tables(&self) -> u16 {
        let range = self.shape.num_tables_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    pub fn search_range(&self) -> u16 {
        let range = self.shape.search_range_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    pub fn entry_selector(&self) -> u16 {
        let range = self.shape.entry_selector_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    pub fn range_shift(&self) -> u16 {
        let range = self.shape.range_shift_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Table records array—one for each top-level table in the font
    pub fn table_records(&self) -> &'a [TableRecord] {
        let range = self.shape.table_records_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for TableDirectory<'a> {
    fn type_name(&self) -> &str {
        "TableDirectory"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("sfnt_version", self.sfnt_version())),
            1usize => Some(Field::new("num_tables", self.num_tables())),
            2usize => Some(Field::new("search_range", self.search_range())),
            3usize => Some(Field::new("entry_selector", self.entry_selector())),
            4usize => Some(Field::new("range_shift", self.range_shift())),
            5usize => Some(Field::new(
                "table_records",
                traversal::FieldType::array_of_records(
                    stringify!(TableRecord),
                    self.table_records(),
                    self.offset_data(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for TableDirectory<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Record for a table in a font.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct TableRecord {
    /// Table identifier.
    pub tag: BigEndian<Tag>,
    /// Checksum for the table.
    pub checksum: BigEndian<u32>,
    /// Offset from the beginning of the font data.
    pub offset: BigEndian<u32>,
    /// Length of the table.
    pub length: BigEndian<u32>,
}

impl TableRecord {
    /// Table identifier.
    pub fn tag(&self) -> Tag {
        self.tag.get()
    }

    /// Checksum for the table.
    pub fn checksum(&self) -> u32 {
        self.checksum.get()
    }

    /// Offset from the beginning of the font data.
    pub fn offset(&self) -> u32 {
        self.offset.get()
    }

    /// Length of the table.
    pub fn length(&self) -> u32 {
        self.length.get()
    }
}

impl FixedSize for TableRecord {
    const RAW_BYTE_LEN: usize =
        Tag::RAW_BYTE_LEN + u32::RAW_BYTE_LEN + u32::RAW_BYTE_LEN + u32::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for TableRecord {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "TableRecord",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("tag", self.tag())),
                1usize => Some(Field::new("checksum", self.checksum())),
                2usize => Some(Field::new("offset", self.offset())),
                3usize => Some(Field::new("length", self.length())),
                _ => None,
            }),
            data,
        }
    }
}

/// [TTC Header](https://learn.microsoft.com/en-us/typography/opentype/spec/otff#ttc-header)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct TTCHeaderMarker {
    table_directory_offsets_byte_len: usize,
    dsig_tag_byte_start: Option<usize>,
    dsig_length_byte_start: Option<usize>,
    dsig_offset_byte_start: Option<usize>,
}

impl TTCHeaderMarker {
    pub fn ttc_tag_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + Tag::RAW_BYTE_LEN
    }

    pub fn version_byte_range(&self) -> Range<usize> {
        let start = self.ttc_tag_byte_range().end;
        start..start + MajorMinor::RAW_BYTE_LEN
    }

    pub fn num_fonts_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn table_directory_offsets_byte_range(&self) -> Range<usize> {
        let start = self.num_fonts_byte_range().end;
        start..start + self.table_directory_offsets_byte_len
    }

    pub fn dsig_tag_byte_range(&self) -> Option<Range<usize>> {
        let start = self.dsig_tag_byte_start?;
        Some(start..start + u32::RAW_BYTE_LEN)
    }

    pub fn dsig_length_byte_range(&self) -> Option<Range<usize>> {
        let start = self.dsig_length_byte_start?;
        Some(start..start + u32::RAW_BYTE_LEN)
    }

    pub fn dsig_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.dsig_offset_byte_start?;
        Some(start..start + u32::RAW_BYTE_LEN)
    }
}

impl MinByteRange for TTCHeaderMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.table_directory_offsets_byte_range().end
    }
}

impl<'a> FontRead<'a> for TTCHeader<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<Tag>();
        let version: MajorMinor = cursor.read()?;
        let num_fonts: u32 = cursor.read()?;
        let table_directory_offsets_byte_len = (num_fonts as usize)
            .checked_mul(u32::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(table_directory_offsets_byte_len);
        let dsig_tag_byte_start = version
            .compatible((2u16, 0u16))
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible((2u16, 0u16))
            .then(|| cursor.advance::<u32>());
        let dsig_length_byte_start = version
            .compatible((2u16, 0u16))
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible((2u16, 0u16))
            .then(|| cursor.advance::<u32>());
        let dsig_offset_byte_start = version
            .compatible((2u16, 0u16))
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible((2u16, 0u16))
            .then(|| cursor.advance::<u32>());
        cursor.finish(TTCHeaderMarker {
            table_directory_offsets_byte_len,
            dsig_tag_byte_start,
            dsig_length_byte_start,
            dsig_offset_byte_start,
        })
    }
}

/// [TTC Header](https://learn.microsoft.com/en-us/typography/opentype/spec/otff#ttc-header)
pub type TTCHeader<'a> = TableRef<'a, TTCHeaderMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> TTCHeader<'a> {
    /// Font Collection ID string: \"ttcf\"
    pub fn ttc_tag(&self) -> Tag {
        let range = self.shape.ttc_tag_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Major/minor version of the TTC Header
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of fonts in TTC
    pub fn num_fonts(&self) -> u32 {
        let range = self.shape.num_fonts_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of offsets to the TableDirectory for each font from the beginning of the file
    pub fn table_directory_offsets(&self) -> &'a [BigEndian<u32>] {
        let range = self.shape.table_directory_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Tag indicating that a DSIG table exists, 0x44534947 ('DSIG') (null if no signature)
    pub fn dsig_tag(&self) -> Option<u32> {
        let range = self.shape.dsig_tag_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// The length (in bytes) of the DSIG table (null if no signature)
    pub fn dsig_length(&self) -> Option<u32> {
        let range = self.shape.dsig_length_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// The offset (in bytes) of the DSIG table from the beginning of the TTC file (null if no signature)
    pub fn dsig_offset(&self) -> Option<u32> {
        let range = self.shape.dsig_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for TTCHeader<'a> {
    fn type_name(&self) -> &str {
        "TTCHeader"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        let version = self.version();
        match idx {
            0usize => Some(Field::new("ttc_tag", self.ttc_tag())),
            1usize => Some(Field::new("version", self.version())),
            2usize => Some(Field::new("num_fonts", self.num_fonts())),
            3usize => Some(Field::new(
                "table_directory_offsets",
                self.table_directory_offsets(),
            )),
            4usize if version.compatible((2u16, 0u16)) => {
                Some(Field::new("dsig_tag", self.dsig_tag().unwrap()))
            }
            5usize if version.compatible((2u16, 0u16)) => {
                Some(Field::new("dsig_length", self.dsig_length().unwrap()))
            }
            6usize if version.compatible((2u16, 0u16)) => {
                Some(Field::new("dsig_offset", self.dsig_offset().unwrap()))
            }
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for TTCHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}
