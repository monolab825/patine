use core::marker::PhantomData;

use patine_core::U256;

pub struct Value<V, B> {
    solt: U256,
    marker_v: PhantomData<V>,
    marker_b: PhantomData<B>,
}
