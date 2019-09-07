use crate::{event::Entered, widget::Widget, Handler, Variant};

/// The semantic representation of text input.
#[derive(derivative::Derivative)]
#[derivative(
    Debug(bound = ""),
    Default(bound = ""),
    Clone(bound = ""),
    Eq(bound = ""),
    PartialEq(bound = ""),
    Hash(bound = "")
)]
pub struct Entry<A> {
    pub value: String,
    pub handler: Option<Handler<Entry<A>, Entered, A>>,
}

impl<A> Variant<Widget<A>> for Entry<A> {}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl<A: 'static> Arbitrary for Entry<A> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<String>()
            .prop_map(|value| Entry {
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
            Entry::<Action>::default(),
            Entry {
                value: "".into(),
                handler: None,
            }
        );
    }

    proptest! {
        #[test]
        fn clone(entry: Entry<Action>) {
            assert_eq!(entry.clone(), entry);
        }

        #[test]
        fn hash(entry: Entry<Action>) {
            let mut hasher = NopHash(0);
            entry.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
