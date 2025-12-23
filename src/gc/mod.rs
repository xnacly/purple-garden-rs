use std::marker::PhantomData;

pub struct Gc<T: ?Sized> {
    // TODO:
    _phantom: PhantomData<T>,
}
