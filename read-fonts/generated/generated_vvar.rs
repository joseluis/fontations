// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [VVAR (Vertical Metrics Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/vvar) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct VvarMarker {}

impl VvarMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn item_variation_store_offset_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn advance_height_mapping_offset_byte_range(&self) -> Range<usize> {
        let start = self.item_variation_store_offset_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn tsb_mapping_offset_byte_range(&self) -> Range<usize> {
        let start = self.advance_height_mapping_offset_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn bsb_mapping_offset_byte_range(&self) -> Range<usize> {
        let start = self.tsb_mapping_offset_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn v_org_mapping_offset_byte_range(&self) -> Range<usize> {
        let start = self.bsb_mapping_offset_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
}

impl<'a> FontRead<'a> for Vvar<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<MajorMinor>();
        cursor.advance::<Offset32>();
        cursor.advance::<Offset32>();
        cursor.advance::<Offset32>();
        cursor.advance::<Offset32>();
        cursor.advance::<Offset32>();
        cursor.finish(VvarMarker {})
    }
}

/// The [VVAR (Vertical Metrics Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/vvar) table
pub type Vvar<'a> = TableRef<'a, VvarMarker>;

impl<'a> Vvar<'a> {
    /// Major version number of the horizontal metrics variations table — set to 1.
    /// Minor version number of the horizontal metrics variations table — set to 0.
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset in bytes from the start of this table to the item variation store table.
    pub fn item_variation_store_offset(&self) -> Offset32 {
        let range = self.shape.item_variation_store_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`item_variation_store_offset`][Self::item_variation_store_offset].
    pub fn item_variation_store(&self) -> Result<ItemVariationStore<'a>, ReadError> {
        let data = self.data;
        self.item_variation_store_offset().resolve(data)
    }

    /// Offset in bytes from the start of this table to the delta-set index mapping for advance heights (may be NULL).
    pub fn advance_height_mapping_offset(&self) -> Nullable<Offset32> {
        let range = self.shape.advance_height_mapping_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`advance_height_mapping_offset`][Self::advance_height_mapping_offset].
    pub fn advance_height_mapping(&self) -> Option<Result<DeltaSetIndexMap<'a>, ReadError>> {
        let data = self.data;
        self.advance_height_mapping_offset().resolve(data)
    }

    /// Offset in bytes from the start of this table to the delta-set index mapping for top side bearings (may be NULL).
    pub fn tsb_mapping_offset(&self) -> Nullable<Offset32> {
        let range = self.shape.tsb_mapping_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`tsb_mapping_offset`][Self::tsb_mapping_offset].
    pub fn tsb_mapping(&self) -> Option<Result<DeltaSetIndexMap<'a>, ReadError>> {
        let data = self.data;
        self.tsb_mapping_offset().resolve(data)
    }

    /// Offset in bytes from the start of this table to the delta-set index mapping for bottom side bearings (may be NULL).
    pub fn bsb_mapping_offset(&self) -> Nullable<Offset32> {
        let range = self.shape.bsb_mapping_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`bsb_mapping_offset`][Self::bsb_mapping_offset].
    pub fn bsb_mapping(&self) -> Option<Result<DeltaSetIndexMap<'a>, ReadError>> {
        let data = self.data;
        self.bsb_mapping_offset().resolve(data)
    }

    /// Offset in bytes from the start of this table to the delta-set index mapping for Y coordinates of vertical origins (may be NULL).
    pub fn v_org_mapping_offset(&self) -> Nullable<Offset32> {
        let range = self.shape.v_org_mapping_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`v_org_mapping_offset`][Self::v_org_mapping_offset].
    pub fn v_org_mapping(&self) -> Option<Result<DeltaSetIndexMap<'a>, ReadError>> {
        let data = self.data;
        self.v_org_mapping_offset().resolve(data)
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Vvar<'a> {
    fn type_name(&self) -> &str {
        "Vvar"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new(
                "item_variation_store_offset",
                FieldType::offset(
                    self.item_variation_store_offset(),
                    self.item_variation_store(),
                ),
            )),
            2usize => Some(Field::new(
                "advance_height_mapping_offset",
                FieldType::offset(
                    self.advance_height_mapping_offset(),
                    self.advance_height_mapping(),
                ),
            )),
            3usize => Some(Field::new(
                "tsb_mapping_offset",
                FieldType::offset(self.tsb_mapping_offset(), self.tsb_mapping()),
            )),
            4usize => Some(Field::new(
                "bsb_mapping_offset",
                FieldType::offset(self.bsb_mapping_offset(), self.bsb_mapping()),
            )),
            5usize => Some(Field::new(
                "v_org_mapping_offset",
                FieldType::offset(self.v_org_mapping_offset(), self.v_org_mapping()),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Vvar<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}