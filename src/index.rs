pub trait TreeIndex<I> {
    type Path: IntoIterator<Item = I>;
    fn path(&self) -> Self::Path;
}

impl<'i, Idx, I> TreeIndex<&'i I> for &'i Idx
where
    Idx: ?Sized,
    &'i Idx: IntoIterator<Item = &'i I>,
    I: 'i + ?Sized,
{
    type Path = Self;

    fn path(&self) -> Self::Path {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn blanket(p: Vec<u8>) {
            assert_eq!((&p[..]).path(), &p[..]);
        }
    }
}
