use std::marker::PhantomData;

#[derive(Debug)]
pub struct Gc<T: ?Sized> {
    // TODO:
    _phantom: PhantomData<T>,
}
