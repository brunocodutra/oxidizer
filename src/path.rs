pub trait TreePath<I> {
    type Segments: IntoIterator<Item = I>;
    fn segments(&self) -> Self::Segments;
}

impl<'s, P, S> TreePath<&'s S> for &'s P
where
    P: ?Sized,
    &'s P: IntoIterator<Item = &'s S>,
    S: 's + ?Sized,
{
    type Segments = Self;

    fn segments(&self) -> Self::Segments {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn blanket(path: Vec<u8>) {
            assert_eq!((&path[..]).segments(), &path[..]);
        }
    }
}
