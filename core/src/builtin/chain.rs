use crate::U256;

use super::Cnt;

extern "C" {
    fn chainid() -> Cnt;
    fn basefee() -> Cnt;
    fn blobbasefee() -> Cnt;
    fn origin() -> Cnt;
    fn blockhash(b: Cnt) -> Cnt;
    fn blobhash(i: Cnt) -> Cnt;
    fn coinbase() -> Cnt;
    fn timestamp() -> Cnt;
    fn number() -> Cnt;
    fn difficulty() -> Cnt;
    fn prevrandao() -> Cnt;
    fn gaslimit() -> Cnt;
}
