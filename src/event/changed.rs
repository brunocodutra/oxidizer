use crate::{event::Event, Variant};

/// An event that may change the value associated with a widget.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Changed<T> {
    /// The new value associated with the widget after an user interaction.
    pub value: T,
}

/// Convenience alias for an event that changes the input associated with the widget.
pub type Entered = Changed<String>;

impl Variant<Event> for Entered {}

/// Convenience alias for an event that toggles the value associated with the widget.
pub type Toggled = Changed<bool>;

impl Variant<Event> for Toggled {}

/// Convenience alias for an event that doesn't change the value associated with the widget.
pub type Clicked = Changed<()>;

impl Variant<Event> for Clicked {}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl<T: 'static + Arbitrary> Arbitrary for Changed<T> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<T>().prop_map(|value| Changed { value }).boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use std::hash::{Hash, Hasher};

    #[test]
    fn default() {
        assert_eq!(Changed::<&str>::default(), Changed { value: "" });
        assert_eq!(Toggled::default(), Changed { value: false });
        assert_eq!(Clicked::default(), Clicked { value: () });
    }

    proptest! {
        #[test]
        fn clone(changed: Changed<String>) {
            assert_eq!(changed.clone(), changed);
        }

        #[test]
        fn hash(changed: Changed<u32>) {
            let mut hasher = NopHash(0);
            changed.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
