use core::marker::PhantomData;

use patine_core::U256;

pub struct Map<K, V, B> {
    solt_prefix: U256,

    marker_k: PhantomData<K>,
    marker_v: PhantomData<V>,
    marker_b: PhantomData<B>,
}
