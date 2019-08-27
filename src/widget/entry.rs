use crate::event::Entered;

#[cfg(test)]
use proptest_derive::Arbitrary;

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
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, proptest(no_bound))]
pub struct Entry<A: 'static> {
    pub value: String,

    #[cfg_attr(test, proptest(value = "None"))]
    pub handler: Option<fn(Entry<A>, Entered) -> A>,
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
