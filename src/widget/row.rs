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
    use crate::mock::*;
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
        fn hash(row: Row<Action>) {
            let mut hasher = NopHash(0);
            row.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
