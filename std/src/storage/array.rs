use core::marker::PhantomData;

use patine_core::{AsNativeType, FromNativeType, U256};

use super::StorageBackend;

pub struct Array<T, B> {
    solt_prefix: U256,

    marker_t: PhantomData<T>,
    marker_b: PhantomData<B>,
}

impl<T, B> Array<T, B>
where
    T: AsNativeType + FromNativeType,
    B: StorageBackend,
{
}
