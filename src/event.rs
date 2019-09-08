mod changed;

pub use changed::*;

use crate::Kind;
use maybe_owned::MaybeOwned;

/// The semantic representation of an event.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Event<'e> {
    Entered(MaybeOwned<'e, Entered>),
    Toggled(MaybeOwned<'e, Toggled>),
    Clicked(MaybeOwned<'e, Clicked>),
}

impl<'e> Kind<Event<'e>> for Event<'e> {}

impl<'a: 'f, 'e: 'f, 'f> From<&'a Event<'e>> for Event<'f> {
    fn from(event: &'a Event<'e>) -> Self {
        use Event::*;
        match event {
            Entered(e) => (&**e).into(),
            Toggled(e) => (&**e).into(),
            Clicked(e) => (&**e).into(),
        }
    }
}

impl<'e> From<Entered> for Event<'e> {
    fn from(event: Entered) -> Self {
        Event::Entered(event.into())
    }
}

impl<'a: 'e, 'e> From<&'a Entered> for Event<'e> {
    fn from(event: &'a Entered) -> Self {
        Event::Entered(event.into())
    }
}

impl<'e> From<Toggled> for Event<'e> {
    fn from(event: Toggled) -> Self {
        Event::Toggled(event.into())
    }
}

impl<'a: 'e, 'e> From<&'a Toggled> for Event<'e> {
    fn from(event: &'a Toggled) -> Self {
        Event::Toggled(event.into())
    }
}

impl<'e> From<Clicked> for Event<'e> {
    fn from(event: Clicked) -> Self {
        Event::Clicked(event.into())
    }
}

impl<'a: 'e, 'e> From<&'a Clicked> for Event<'e> {
    fn from(event: &'a Clicked) -> Self {
        Event::Clicked(event.into())
    }
}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl Arbitrary for Event<'static> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            any::<Entered>().prop_map_into(),
            any::<Toggled>().prop_map_into(),
            any::<Clicked>().prop_map_into(),
        ]
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use maybe_owned::MaybeOwned::*;
    use std::hash::{Hash, Hasher};

    proptest! {
        #[test]
        fn from_entered(e: Entered) {
            assert_eq!(Event::from(&e), Event::Entered(Borrowed(&e)));
            assert_eq!(Event::from(e.clone()), Event::Entered(Owned(e.clone())));
            assert_eq!(Event::from(&Event::from(e.clone())), Event::Entered(Borrowed(&e)));
        }

        #[test]
        fn from_toggled(e: Toggled) {
            assert_eq!(Event::from(&e), Event::Toggled(Borrowed(&e)));
            assert_eq!(Event::from(e), Event::Toggled(Owned(e)));
            assert_eq!(Event::from(&Event::from(e)), Event::Toggled(Borrowed(&e)));
        }

        #[test]
        fn from_clicked(e: Clicked) {
            assert_eq!(Event::from(&e), Event::Clicked(Borrowed(&e)));
            assert_eq!(Event::from(e), Event::Clicked(Owned(e)));
            assert_eq!(Event::from(&Event::from(e)), Event::Clicked(Borrowed(&e)));
        }

        #[test]
        fn clone(e: Event) {
            assert_eq!(e.clone(), e);
        }

        #[test]
        fn hash(e: Event) {
            let mut hasher = NopHash(0);
            e.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
