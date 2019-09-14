use crate::{event::Clicked, widget::Widget, OptionalHandler, Variant};

/// The semantic representation of a button.
#[derive(derivative::Derivative)]
#[derivative(
    Debug(bound = ""),
    Default(bound = ""),
    Clone(bound = ""),
    Eq(bound = ""),
    PartialEq(bound = ""),
    Hash(bound = "")
)]
pub struct Button<A> {
    pub label: String,
    pub handler: OptionalHandler<Button<A>, Clicked, A>,
}

impl<'w, A> Variant<Widget<'w, A>> for Button<A> {}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl<A: 'static + Default> Arbitrary for Button<A> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (any::<String>(), any::<OptionalHandler<_, _, _>>())
            .prop_map(|(label, handler)| Button { label, handler })
            .boxed()
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
        assert_eq!(
            Button::<Action>::default(),
            Button {
                label: "".into(),
                handler: OptionalHandler::None,
            }
        );
    }

    proptest! {
        #[test]
        fn clone(button: Button<Action>) {
            assert_eq!(button.clone(), button);
        }

        #[test]
        fn hash(button: Button<Action>) {
            let mut hasher = NopHash(0);
            button.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
