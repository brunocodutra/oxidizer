pub trait TreePath {
    type Segment;
    type Segments: IntoIterator<Item = Self::Segment>;
    fn segments(&self) -> Self::Segments;
}

impl<'s, P, S> TreePath for &'s P
where
    P: ?Sized,
    &'s P: IntoIterator<Item = &'s S>,
    S: 's + ?Sized,
{
    type Segment = &'s S;
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
