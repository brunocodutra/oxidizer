mod button;
mod checkbox;
mod entry;

pub use button::*;
pub use checkbox::*;
pub use entry::*;

/// The semantic representation of a widget.
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub enum Widget<A: 'static> {
    Button(Button<A>),
    Entry(Entry<A>),
    Checkbox(Checkbox<A>),
}

impl<A> Eq for Widget<A> {}

impl<A> PartialEq for Widget<A> {
    fn eq(&self, other: &Self) -> bool {
        use Widget::*;
        match (self, other) {
            (Button(a), Button(b)) => a == b,
            (Entry(a), Entry(b)) => a == b,
            (Checkbox(a), Checkbox(b)) => a == b,
            _ => false,
        }
    }
}

use std::hash::{Hash, Hasher};

impl<A> Hash for Widget<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Widget::*;
        match self {
            Button(v) => v.hash(state),
            Entry(v) => v.hash(state),
            Checkbox(v) => v.hash(state),
        }
    }
}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, strategy::BoxedStrategy};

#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
impl<A: 'static + Debug> Arbitrary for Widget<A> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;

        prop_oneof![
            any::<Button<A>>().prop_map(Widget::Button),
            any::<Entry<A>>().prop_map(Widget::Entry),
            any::<Checkbox<A>>().prop_map(Widget::Checkbox),
        ]
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use proptest::prelude::*;
    use std::hash::{Hash, Hasher};

    #[derive(Debug)]
    enum Action {}

    proptest! {
        #[test]
        fn clone(widget: Widget<Action>) {
            assert_eq!(widget.clone(), widget);
        }

        #[test]
        fn hash(widget: Widget<Action>) {
            let mut hasher = NopHash(0);
            widget.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }
}
