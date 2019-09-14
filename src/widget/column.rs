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
pub struct Column<'w, A> {
    pub children: Vec<Widget<'w, A>>,
}

impl<'w, A> Variant<Widget<'w, A>> for Column<'w, A> {}

#[cfg(test)]
use super::ChildrenStrategy;

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*, strategy::Strategy};

#[cfg(test)]
impl<A: 'static + Default> Arbitrary for Column<'static, A> {
    type Parameters = ChildrenStrategy<A>;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
        params.prop_map(|children| Column { children }).boxed()
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
        assert_eq!(Column::<Action>::default(), Column { children: vec![] });
    }

    proptest! {
        #[test]
        fn clone(column: Column<Action>) {
            assert_eq!(column.clone(), column);
        }

        #[test]
        fn hash(x: Column<Action>, y: Column<Action>) {
            let mut a = DefaultHasher::new();
            x.hash(&mut a);

            let mut b = DefaultHasher::new();
            y.hash(&mut b);

            assert_eq!(x == y, a.finish() == b.finish());
        }
    }
}
