pub trait TruncateTo<T> {
    fn truncate(&self) -> T;
}

trait WidenTo<T> {
    fn widen(&self) -> T;
}

macro_rules! impl_trunc_widen {
    ( $big:ty > $small:ty ) => {
        impl TruncateTo<$small> for $big {
            fn truncate(&self) -> $small {
                *self as $small
            }
        }
        
        impl WidenTo<$big> for $small {
            fn widen(&self) -> $big {
                *self as $big
            }
        }
    };
}

impl_trunc_widen!(u16 > u8);
impl_trunc_widen!(u32 > u8);
impl_trunc_widen!(u32 > u16);
impl_trunc_widen!(u64 > u8);
impl_trunc_widen!(u64 > u16);
impl_trunc_widen!(u64 > u32);


impl_trunc_widen!(usize > u32);
impl_trunc_widen!(usize > u16);
impl_trunc_widen!(usize > u8);


impl_trunc_widen!(i16 > i8);
impl_trunc_widen!(i32 > i8);
impl_trunc_widen!(i32 > i16);
impl_trunc_widen!(i64 > i8);
impl_trunc_widen!(i64 > i16);
impl_trunc_widen!(i64 > i32);