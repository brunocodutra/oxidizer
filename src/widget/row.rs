use crate::{widget::Widget, Variant};

/// The semantic representation of a container that displays widgets horizontally.
#[derive(derivative::Derivative)]
#[derivative(
    Debug(bound = ""),
    Default(bound = ""),
    Clone(bound = ""),
    Eq(bound = ""),
    PartialEq(bound = ""),
    Hash(bound = "")
)]
pub struct Row<'w, A> {
    pub children: Vec<Widget<'w, A>>,
}

impl<'w, A> Variant<Widget<'w, A>> for Row<'w, A> {}

use std::slice::Iter;

impl<'a, 'w: 'a, A> IntoIterator for &'a Row<'w, A> {
    type Item = &'a Widget<'w, A>;
    type IntoIter = Iter<'a, Widget<'w, A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter()
    }
}

#[cfg(test)]
use super::ChildrenStrategy;

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*, strategy::Strategy};

#[cfg(test)]
impl<A: 'static + Default> Arbitrary for Row<'static, A> {
    type Parameters = ChildrenStrategy<A>;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
        params.prop_map(|children| Row { children }).boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[derive(Default)]
    struct Action;

    #[test]
    fn default() {
        assert_eq!(Row::<Action>::default(), Row { children: vec![] });
    }

    proptest! {
        #[test]
        fn clone(row: Row<Action>) {
            assert_eq!(row.clone(), row);
        }

        #[test]
        fn hash(x: Row<Action>, y: Row<Action>) {
            let mut a = DefaultHasher::new();
            x.hash(&mut a);

            let mut b = DefaultHasher::new();
            y.hash(&mut b);

            assert_eq!(x == y, a.finish() == b.finish());
        }

        #[test]
        fn into_iter(row: Row<Action>) {
            let items = row.into_iter().cloned().collect::<Vec<_>>();
            assert_eq!(items, row.children)
        }
    }
}
