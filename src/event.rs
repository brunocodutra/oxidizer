mod changed;

pub use changed::*;

#[cfg(test)]
use proptest_derive::Arbitrary;

/// The semantic representation of an event.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum Event {
    Entered(Changed<String>),
    Toggled(Toggled),
    #[cfg_attr(test, allow(clippy::unit_arg))]
    Clicked(Clicked),
}

impl From<Entered> for Event {
    fn from(entered: Entered) -> Self {
        Event::Entered(entered)
    }
}

impl From<Toggled> for Event {
    fn from(toggled: Toggled) -> Self {
        Event::Toggled(toggled)
    }
}

impl From<Clicked> for Event {
    fn from(clicked: Clicked) -> Self {
        Event::Clicked(clicked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use proptest::prelude::*;
    use std::hash::{Hash, Hasher};

    proptest! {
        #[test]
        fn from_entered(event: Entered) {
            assert_eq!(Event::from(event.clone()), Event::Entered(event));
        }

        #[test]
        fn from_toggled(event: Toggled) {
            assert_eq!(Event::from(event), Event::Toggled(event));
        }

        #[test]
        fn from_clicked(event: Clicked) {
            assert_eq!(Event::from(event), Event::Clicked(event));
        }

        #[test]
        fn clone(event: Event) {
            assert_eq!(event.clone(), event);
        }

        #[test]
        fn hash(event: Event) {
            let mut hasher = NopHash(0);
            event.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
