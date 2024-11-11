pub trait ExtraIter: Iterator + Sized {
    fn attempt_collect<C>(self) -> Result<C, <C as AttemptFromIterator<Self>>::Error> where
        C: AttemptFromIterator<Self, Item=Self::Item>
    {
        C::attempt_from_iter(self)
    }
}

impl<I: Iterator + Sized> ExtraIter for I {}

pub trait AttemptFromIterator<I>: Sized {
    type Item;
    type Error;

    fn attempt_from_iter(iter: I) -> Result<Self, Self::Error>;
}
