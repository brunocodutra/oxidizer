use crate::{event::Toggled, widget::Widget, OptionalHandler, Variant};

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
    pub handler: OptionalHandler<Checkbox<A>, Toggled, A>,
}

impl<'w, A> Variant<Widget<'w, A>> for Checkbox<A> {}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl<A: 'static + Default> Arbitrary for Checkbox<A> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (
            any::<String>(),
            any::<bool>(),
            any::<OptionalHandler<_, _, _>>(),
        )
            .prop_map(|(label, value, handler)| Checkbox {
                label,
                value,
                handler,
            })
            .boxed()
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
        assert_eq!(
            Checkbox::<Action>::default(),
            Checkbox {
                value: false,
                label: "".into(),
                handler: OptionalHandler::None,
            }
        );
    }

    proptest! {
        #[test]
        fn clone(checkbox: Checkbox<Action>) {
            assert_eq!(checkbox.clone(), checkbox);
        }

        #[test]
        fn hash(x: Checkbox<Action>, y: Checkbox<Action>) {
            let mut a = DefaultHasher::new();
            x.hash(&mut a);

            let mut b = DefaultHasher::new();
            y.hash(&mut b);

            assert_eq!(x == y, a.finish() == b.finish());
        }
    }
}
