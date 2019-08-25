use super::Widget;

#[cfg(test)]
use super::ChildrenStrategy;

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
#[cfg_attr(test, proptest(params = "ChildrenStrategy<A>"))]
pub struct Row<A: 'static> {
    #[cfg_attr(test, proptest(strategy = "params"))]
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
        assert_eq!(Row::<Action>::default(), Row { children: vec![] });
    }

    proptest! {
        #[test]
        fn clone(row: Row<Action>) {
            assert_eq!(row.clone(), row);
        }

        #[test]
        fn hash(row: Row<Action>) {
            let mut hasher = NopHash(0);
            row.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
