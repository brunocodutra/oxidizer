mod changed;

pub use changed::*;

use crate::Kind;

/// The semantic representation of an event.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Event {
    Entered(Entered),
    Toggled(Toggled),
    Clicked(Clicked),
}

impl Kind<Event> for Event {}

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
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl Arbitrary for Event {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            any::<Entered>().prop_map(Event::Entered),
            any::<Toggled>().prop_map(Event::Toggled),
            any::<Clicked>().prop_map(Event::Clicked),
        ]
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
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
