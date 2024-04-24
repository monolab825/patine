pub trait Event: Sized {
    fn emit(self);
}
