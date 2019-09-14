use crate::{event::Event, Variant};

/// An event that may change the value associated with a widget.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Changed<T> {
    /// The new value associated with the widget after an user interaction.
    pub value: T,
}

/// Convenience alias for an event that changes the input associated with the widget.
pub type Entered = Changed<String>;

impl<'e> Variant<Event<'e>> for Entered {}

/// Convenience alias for an event that toggles the value associated with the widget.
pub type Toggled = Changed<bool>;

impl<'e> Variant<Event<'e>> for Toggled {}

/// Convenience alias for an event that doesn't change the value associated with the widget.
pub type Clicked = Changed<()>;

impl<'e> Variant<Event<'e>> for Clicked {}

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
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn default() {
        assert_eq!(Entered::default(), Changed { value: "".into() });
        assert_eq!(Toggled::default(), Changed { value: false });
        assert_eq!(Clicked::default(), Clicked { value: () });
    }

    proptest! {
        #[allow(clippy::clone_on_copy)]
        #[test]
        fn clone(changed: Changed<u32>) {
            assert_eq!(changed.clone(), changed);
        }

        #[test]
        fn hash(x: Changed<u32>, y: Changed<u32>) {
            let mut a = DefaultHasher::new();
            x.hash(&mut a);

            let mut b = DefaultHasher::new();
            y.hash(&mut b);

            assert_eq!(x == y, a.finish() == b.finish());
        }
    }
}
