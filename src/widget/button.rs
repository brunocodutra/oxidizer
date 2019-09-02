use crate::{event::Clicked, widget::Widget, Handler, Kind};

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
    pub handler: Option<Handler<Button<A>, Clicked, A>>,
}

impl<A> Kind<Widget<A>> for Button<A> {}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl<A: 'static> Arbitrary for Button<A> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<String>()
            .prop_map(|label| Button {
                label,
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
            Button::<Action>::default(),
            Button {
                label: "".into(),
                handler: None,
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
