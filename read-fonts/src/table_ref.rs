//! Typed font tables

use super::read::{FontRead, Format, ReadError};
use crate::{
    font_data::FontData,
    offset::{Offset, ResolveOffset},
};

#[derive(Clone)]
/// Typed access to raw table data.
pub struct TableRef<'a, T> {
    pub(crate) shape: T,
    pub(crate) data: FontData<'a>,
}

impl<'a, T> TableRef<'a, T> {
    /// Resolve the provided offset from the start of this table.
    pub fn resolve_offset<O: Offset, R: FontRead<'a>>(&self, offset: O) -> Result<R, ReadError> {
        offset.resolve(self.data)
    }

    /// Return a reference to this table's raw data.
    ///
    /// We use this in the compile crate to resolve offsets.
    pub fn offset_data(&self) -> FontData<'a> {
        self.data
    }

    /// Return a reference to the table's 'Shape' struct.
    ///
    /// This is a low level implementation detail, but it can be useful in
    /// some cases where you want to know things about a table's layout, such
    /// as the byte offsets of specific fields.
    pub fn shape(&self) -> &T {
        &self.shape
    }
}

// a blanket impl so that the format is available through a TableRef
impl<U, T: Format<U>> Format<U> for TableRef<'_, T> {
    const FORMAT: U = T::FORMAT;
}
