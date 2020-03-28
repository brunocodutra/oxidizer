use std::iter::Copied;

pub trait TreeIndex<I: Into<usize>> {
    type Path: Iterator<Item = I>;
    fn path(&self) -> Self::Path;
}

impl<'a, T, I> TreeIndex<I> for &'a T
where
    T: ?Sized,
    &'a T: IntoIterator<Item = &'a I>,
    I: 'a + Into<usize> + Copy,
{
    type Path = Copied<<&'a T as IntoIterator>::IntoIter>;

    fn path(&self) -> Self::Path {
        self.into_iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn from_row(p: Vec<u8>) {
            assert_eq!((&p[..]).path().collect::<Vec<_>>(), p);
        }
    }
}
