pub trait TreePath {
    type Segment;
    type Segments: IntoIterator<Item = Self::Segment>;
    fn segments(self) -> Self::Segments;
}

impl<I: IntoIterator> TreePath for I {
    type Segment = I::Item;
    type Segments = I;

    fn segments(self) -> Self::Segments {
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
            assert_eq!((&path).segments(), &path);
        }
    }
}
