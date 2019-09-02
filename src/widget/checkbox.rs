use crate::{event::Toggled, widget::Widget, Handler, Kind};

/// The semantic representation of a checkbox.
#[derive(derivative::Derivative)]
#[derivative(
    Debug(bound = ""),
    Default(bound = ""),
    Clone(bound = ""),
    Eq(bound = ""),
    PartialEq(bound = ""),
    Hash(bound = "")
)]
pub struct Checkbox<A> {
    pub label: String,
    pub value: bool,
    pub handler: Option<Handler<Checkbox<A>, Toggled, A>>,
}

impl<A> Kind<Widget<A>> for Checkbox<A> {}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl<A: 'static> Arbitrary for Checkbox<A> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (any::<String>(), any::<bool>())
            .prop_map(|(label, value)| Checkbox {
                label,
                value,
                handler: None,
            })
            .boxed()
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
        assert_eq!(
            Checkbox::<Action>::default(),
            Checkbox {
                value: false,
                label: "".into(),
                handler: None,
            }
        );
    }

    proptest! {
        #[test]
        fn clone(checkbox: Checkbox<Action>) {
            assert_eq!(checkbox.clone(), checkbox);
        }

        #[test]
        fn hash(checkbox: Checkbox<Action>) {
            let mut hasher = NopHash(0);
            checkbox.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
