use super::Widget;

#[cfg(test)]
use proptest::{arbitrary::any, collection::vec, strategy::BoxedStrategy};

#[cfg(test)]
use proptest_derive::Arbitrary;

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
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, proptest(no_bound))]
#[cfg_attr(test, proptest(params = "Option<BoxedStrategy<Widget<A>>>"))]
pub struct Column<A: 'static> {
    #[cfg_attr(
        test,
        proptest(strategy = "vec(params.unwrap_or_else(any::<Widget<A>>), 0..3)")
    )]
    pub children: Vec<Widget<A>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use proptest::prelude::*;
    use std::hash::{Hash, Hasher};

    #[derive(Debug)]
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