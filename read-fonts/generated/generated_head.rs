// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The `macStyle` field for the head table.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck :: AnyBitPattern)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct MacStyle {
    bits: u16,
}

impl MacStyle {
    /// Bit 0: Bold (if set to 1)
    pub const BOLD: Self = Self { bits: 0x0001 };

    /// Bit 1: Italic (if set to 1)
    pub const ITALIC: Self = Self { bits: 0x0002 };

    /// Bit 2: Underline (if set to 1)
    pub const UNDERLINE: Self = Self { bits: 0x0004 };

    /// Bit 3: Outline (if set to 1)
    pub const OUTLINE: Self = Self { bits: 0x0008 };

    /// Bit 4: Shadow (if set to 1)
    pub const SHADOW: Self = Self { bits: 0x0010 };

    /// Bit 5: Condensed (if set to 1)
    pub const CONDENSED: Self = Self { bits: 0x0020 };

    /// Bit 6: Extended (if set to 1)
    pub const EXTENDED: Self = Self { bits: 0x0040 };
}

impl MacStyle {
    ///  Returns an empty set of flags.
    #[inline]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Returns the set containing all flags.
    #[inline]
    pub const fn all() -> Self {
        Self {
            bits: Self::BOLD.bits
                | Self::ITALIC.bits
                | Self::UNDERLINE.bits
                | Self::OUTLINE.bits
                | Self::SHADOW.bits
                | Self::CONDENSED.bits
                | Self::EXTENDED.bits,
        }
    }

    /// Returns the raw value of the flags currently stored.
    #[inline]
    pub const fn bits(&self) -> u16 {
        self.bits
    }

    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    #[inline]
    pub const fn from_bits(bits: u16) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self { bits })
        } else {
            None
        }
    }

    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    #[inline]
    pub const fn from_bits_truncate(bits: u16) -> Self {
        Self {
            bits: bits & Self::all().bits,
        }
    }

    /// Returns `true` if no flags are currently stored.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }

    /// Returns `true` if there are flags common to both `self` and `other`.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !(Self {
            bits: self.bits & other.bits,
        })
        .is_empty()
    }

    /// Returns `true` if all of the flags in `other` are contained within `self`.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    /// Inserts the specified flags in-place.
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }

    /// Removes the specified flags in-place.
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.bits &= !other.bits;
    }

    /// Toggles the specified flags in-place.
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.bits ^= other.bits;
    }

    /// Returns the intersection between the flags in `self` and
    /// `other`.
    ///
    /// Specifically, the returned set contains only the flags which are
    /// present in *both* `self` *and* `other`.
    ///
    /// This is equivalent to using the `&` operator (e.g.
    /// [`ops::BitAnd`]), as in `flags & other`.
    ///
    /// [`ops::BitAnd`]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }

    /// Returns the union of between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags which are
    /// present in *either* `self` *or* `other`, including any which are
    /// present in both.
    ///
    /// This is equivalent to using the `|` operator (e.g.
    /// [`ops::BitOr`]), as in `flags | other`.
    ///
    /// [`ops::BitOr`]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Returns the difference between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags present in
    /// `self`, except for the ones present in `other`.
    ///
    /// It is also conceptually equivalent to the "bit-clear" operation:
    /// `flags & !other` (and this syntax is also supported).
    ///
    /// This is equivalent to using the `-` operator (e.g.
    /// [`ops::Sub`]), as in `flags - other`.
    ///
    /// [`ops::Sub`]: https://doc.rust-lang.org/std/ops/trait.Sub.html
    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl std::ops::BitOr for MacStyle {
    type Output = Self;

    /// Returns the union of the two sets of flags.
    #[inline]
    fn bitor(self, other: MacStyle) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
}

impl std::ops::BitOrAssign for MacStyle {
    /// Adds the set of flags.
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.bits |= other.bits;
    }
}

impl std::ops::BitXor for MacStyle {
    type Output = Self;

    /// Returns the left flags, but with all the right flags toggled.
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self {
            bits: self.bits ^ other.bits,
        }
    }
}

impl std::ops::BitXorAssign for MacStyle {
    /// Toggles the set of flags.
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.bits ^= other.bits;
    }
}

impl std::ops::BitAnd for MacStyle {
    type Output = Self;

    /// Returns the intersection between the two sets of flags.
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }
}

impl std::ops::BitAndAssign for MacStyle {
    /// Disables all flags disabled in the set.
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.bits &= other.bits;
    }
}

impl std::ops::Sub for MacStyle {
    type Output = Self;

    /// Returns the set difference of the two sets of flags.
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl std::ops::SubAssign for MacStyle {
    /// Disables all flags enabled in the set.
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.bits &= !other.bits;
    }
}

impl std::ops::Not for MacStyle {
    type Output = Self;

    /// Returns the complement of this set of flags.
    #[inline]
    fn not(self) -> Self {
        Self { bits: !self.bits } & Self::all()
    }
}

impl std::fmt::Debug for MacStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let members: &[(&str, Self)] = &[
            ("BOLD", Self::BOLD),
            ("ITALIC", Self::ITALIC),
            ("UNDERLINE", Self::UNDERLINE),
            ("OUTLINE", Self::OUTLINE),
            ("SHADOW", Self::SHADOW),
            ("CONDENSED", Self::CONDENSED),
            ("EXTENDED", Self::EXTENDED),
        ];
        let mut first = true;
        for (name, value) in members {
            if self.contains(*value) {
                if !first {
                    f.write_str(" | ")?;
                }
                first = false;
                f.write_str(name)?;
            }
        }
        if first {
            f.write_str("(empty)")?;
        }
        Ok(())
    }
}

impl std::fmt::Binary for MacStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.bits, f)
    }
}

impl std::fmt::Octal for MacStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.bits, f)
    }
}

impl std::fmt::LowerHex for MacStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.bits, f)
    }
}

impl std::fmt::UpperHex for MacStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.bits, f)
    }
}

impl font_types::Scalar for MacStyle {
    type Raw = <u16 as font_types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.bits().to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u16>::from_raw(raw);
        Self::from_bits_truncate(t)
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> From<MacStyle> for FieldType<'a> {
    fn from(src: MacStyle) -> FieldType<'a> {
        src.bits().into()
    }
}

/// The [head](https://docs.microsoft.com/en-us/typography/opentype/spec/head)
/// (font header) table.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct HeadMarker {}

impl HeadMarker {
    pub fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }

    pub fn font_revision_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + Fixed::RAW_BYTE_LEN
    }

    pub fn checksum_adjustment_byte_range(&self) -> Range<usize> {
        let start = self.font_revision_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn magic_number_byte_range(&self) -> Range<usize> {
        let start = self.checksum_adjustment_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn flags_byte_range(&self) -> Range<usize> {
        let start = self.magic_number_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn units_per_em_byte_range(&self) -> Range<usize> {
        let start = self.flags_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn created_byte_range(&self) -> Range<usize> {
        let start = self.units_per_em_byte_range().end;
        start..start + LongDateTime::RAW_BYTE_LEN
    }

    pub fn modified_byte_range(&self) -> Range<usize> {
        let start = self.created_byte_range().end;
        start..start + LongDateTime::RAW_BYTE_LEN
    }

    pub fn x_min_byte_range(&self) -> Range<usize> {
        let start = self.modified_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }

    pub fn y_min_byte_range(&self) -> Range<usize> {
        let start = self.x_min_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }

    pub fn x_max_byte_range(&self) -> Range<usize> {
        let start = self.y_min_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }

    pub fn y_max_byte_range(&self) -> Range<usize> {
        let start = self.x_max_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }

    pub fn mac_style_byte_range(&self) -> Range<usize> {
        let start = self.y_max_byte_range().end;
        start..start + MacStyle::RAW_BYTE_LEN
    }

    pub fn lowest_rec_ppem_byte_range(&self) -> Range<usize> {
        let start = self.mac_style_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn font_direction_hint_byte_range(&self) -> Range<usize> {
        let start = self.lowest_rec_ppem_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }

    pub fn index_to_loc_format_byte_range(&self) -> Range<usize> {
        let start = self.font_direction_hint_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }

    pub fn glyph_data_format_byte_range(&self) -> Range<usize> {
        let start = self.index_to_loc_format_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
}

impl MinByteRange for HeadMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.glyph_data_format_byte_range().end
    }
}

impl TopLevelTable for Head<'_> {
    /// `head`
    const TAG: Tag = Tag::new(b"head");
}

impl<'a> FontRead<'a> for Head<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<MajorMinor>();
        cursor.advance::<Fixed>();
        cursor.advance::<u32>();
        cursor.advance::<u32>();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.advance::<LongDateTime>();
        cursor.advance::<LongDateTime>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<MacStyle>();
        cursor.advance::<u16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.finish(HeadMarker {})
    }
}

/// The [head](https://docs.microsoft.com/en-us/typography/opentype/spec/head)
/// (font header) table.
pub type Head<'a> = TableRef<'a, HeadMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Head<'a> {
    /// Version number of the font header table, set to (1, 0)
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Set by font manufacturer.
    pub fn font_revision(&self) -> Fixed {
        let range = self.shape.font_revision_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// To compute: set it to 0, sum the entire font as uint32, then
    /// store 0xB1B0AFBA - sum. If the font is used as a component in a
    /// font collection file, the value of this field will be
    /// invalidated by changes to the file structure and font table
    /// directory, and must be ignored.
    pub fn checksum_adjustment(&self) -> u32 {
        let range = self.shape.checksum_adjustment_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Set to 0x5F0F3CF5.
    pub fn magic_number(&self) -> u32 {
        let range = self.shape.magic_number_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// See the flags enum
    pub fn flags(&self) -> u16 {
        let range = self.shape.flags_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Set to a value from 16 to 16384. Any value in this range is
    /// valid. In fonts that have TrueType outlines, a power of 2 is
    /// recommended as this allows performance optimizations in some
    /// rasterizers.
    pub fn units_per_em(&self) -> u16 {
        let range = self.shape.units_per_em_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of seconds since 12:00 midnight that started January 1st
    /// 1904 in GMT/UTC time zone.
    pub fn created(&self) -> LongDateTime {
        let range = self.shape.created_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of seconds since 12:00 midnight that started January 1st
    /// 1904 in GMT/UTC time zone.
    pub fn modified(&self) -> LongDateTime {
        let range = self.shape.modified_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Minimum x coordinate across all glyph bounding boxes.
    pub fn x_min(&self) -> i16 {
        let range = self.shape.x_min_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Minimum y coordinate across all glyph bounding boxes.
    pub fn y_min(&self) -> i16 {
        let range = self.shape.y_min_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Maximum x coordinate across all glyph bounding boxes.
    pub fn x_max(&self) -> i16 {
        let range = self.shape.x_max_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Maximum y coordinate across all glyph bounding boxes.
    pub fn y_max(&self) -> i16 {
        let range = self.shape.y_max_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Bits identifying the font's style; see [MacStyle]
    pub fn mac_style(&self) -> MacStyle {
        let range = self.shape.mac_style_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Smallest readable size in pixels.
    pub fn lowest_rec_ppem(&self) -> u16 {
        let range = self.shape.lowest_rec_ppem_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Deprecated (Set to 2).
    pub fn font_direction_hint(&self) -> i16 {
        let range = self.shape.font_direction_hint_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// 0 for short offsets (Offset16), 1 for long (Offset32).
    pub fn index_to_loc_format(&self) -> i16 {
        let range = self.shape.index_to_loc_format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// 0 for current format.
    pub fn glyph_data_format(&self) -> i16 {
        let range = self.shape.glyph_data_format_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Head<'a> {
    fn type_name(&self) -> &str {
        "Head"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("font_revision", self.font_revision())),
            2usize => Some(Field::new(
                "checksum_adjustment",
                self.checksum_adjustment(),
            )),
            3usize => Some(Field::new("magic_number", self.magic_number())),
            4usize => Some(Field::new("flags", self.flags())),
            5usize => Some(Field::new("units_per_em", self.units_per_em())),
            6usize => Some(Field::new("created", self.created())),
            7usize => Some(Field::new("modified", self.modified())),
            8usize => Some(Field::new("x_min", self.x_min())),
            9usize => Some(Field::new("y_min", self.y_min())),
            10usize => Some(Field::new("x_max", self.x_max())),
            11usize => Some(Field::new("y_max", self.y_max())),
            12usize => Some(Field::new("mac_style", self.mac_style())),
            13usize => Some(Field::new("lowest_rec_ppem", self.lowest_rec_ppem())),
            14usize => Some(Field::new(
                "font_direction_hint",
                self.font_direction_hint(),
            )),
            15usize => Some(Field::new(
                "index_to_loc_format",
                self.index_to_loc_format(),
            )),
            16usize => Some(Field::new("glyph_data_format", self.glyph_data_format())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Head<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}
