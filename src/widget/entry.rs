use crate::{event::Entered, widget::Widget, OptionalHandler, Variant};

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
    pub handler: OptionalHandler<Entry<A>, Entered, A>,
}

impl<'w, A> Variant<Widget<'w, A>> for Entry<A> {}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl<A: 'static + Default> Arbitrary for Entry<A> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (any::<String>(), any::<OptionalHandler<_, _, _>>())
            .prop_map(|(value, handler)| Entry { value, handler })
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
            Entry::<Action>::default(),
            Entry {
                value: "".into(),
                handler: OptionalHandler::None,
            }
        );
    }

    proptest! {
        #[test]
        fn clone(entry: Entry<Action>) {
            assert_eq!(entry.clone(), entry);
        }

        #[test]
        fn hash(x: Entry<Action>, y: Entry<Action>) {
            let mut a = DefaultHasher::new();
            x.hash(&mut a);

            let mut b = DefaultHasher::new();
            y.hash(&mut b);

            assert_eq!(x == y, a.finish() == b.finish());
        }
    }
}
