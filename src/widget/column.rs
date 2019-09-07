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
pub struct Column<A> {
    pub children: Vec<Widget<A>>,
}

impl<A> Variant<Widget<A>> for Column<A> {}

#[cfg(test)]
use super::ChildrenStrategy;

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*, strategy::Strategy};

#[cfg(test)]
impl<A: 'static> Arbitrary for Column<A> {
    type Parameters = ChildrenStrategy<A>;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
        params.prop_map(|children| Column { children }).boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use std::hash::{Hash, Hasher};

    enum Action {}

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
        fn hash(column: Column<Action>) {
            let mut hasher = NopHash(0);
            column.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
