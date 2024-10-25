/// Helps implementing `bytemuck::AnyBitPattern`
///
/// SAFETY: This does the same as `derive(AnyBitPattern)` from `bytemuck_derive`
macro_rules! impl_bytemuck {
    ($($tt:tt)*) => {
        #[cfg(feature = "bytemuck")]
        $crate::bytemuck::_impl_bytemuck![$($tt)*];
    }
}
pub(crate) use impl_bytemuck;

#[allow(unused_macros)]
macro_rules! _impl_bytemuck {
    (AnyBitPattern for $type:ident<$T:ident>) => {
        unsafe impl<$T> bytemuck::Zeroable for $type<$T> where $T: bytemuck::AnyBitPattern {}
        unsafe impl<$T> bytemuck::AnyBitPattern for $type<$T> where $T: bytemuck::AnyBitPattern {}
    };
    (AnyBitPattern for $type:ident) => {
        unsafe impl bytemuck::Zeroable for $type {}
        unsafe impl bytemuck::AnyBitPattern for $type {}
    };

    (NoUninit for $type:ident<$T:ident>) => {
        unsafe impl<$T> bytemuck::NoUninit for $type<$T> where $T: bytemuck::NoUninit {}
    };
    (NoUninit for $type:ident) => {
        unsafe impl bytemuck::NoUninit for $type {}
    };
}
#[allow(unused_imports)]
pub(crate) use _impl_bytemuck;
