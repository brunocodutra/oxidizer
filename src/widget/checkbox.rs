#[cfg(test)]
use proptest_derive::Arbitrary;

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
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, proptest(no_bound))]
pub struct Checkbox<A: 'static> {
    pub label: String,
    pub value: bool,

    #[cfg_attr(test, proptest(value = "None"))]
    pub on_toggle: Option<fn(bool) -> A>,
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
            Checkbox::<Action>::default(),
            Checkbox {
                value: false,
                label: "".into(),
                on_toggle: None,
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
