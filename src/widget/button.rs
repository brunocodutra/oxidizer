use crate::event::Clicked;

#[cfg(test)]
use proptest_derive::Arbitrary;

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
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, proptest(no_bound))]
pub struct Button<A: 'static> {
    pub label: String,

    #[cfg_attr(test, proptest(value = "None"))]
    pub handler: Option<fn(Button<A>, Clicked) -> A>,
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
