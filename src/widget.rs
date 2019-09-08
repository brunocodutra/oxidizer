mod button;
mod checkbox;
mod column;
mod entry;
mod row;

pub use button::*;
pub use checkbox::*;
pub use column::*;
pub use entry::*;
pub use row::*;

use crate::Kind;
use maybe_owned::MaybeOwned;

/// The semantic representation of a widget.
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub enum Widget<'w, A> {
    Row(MaybeOwned<'w, Row<'w, A>>),
    Column(MaybeOwned<'w, Column<'w, A>>),
    Button(MaybeOwned<'w, Button<A>>),
    Entry(MaybeOwned<'w, Entry<A>>),
    Checkbox(MaybeOwned<'w, Checkbox<A>>),
}

impl<'w, A> Kind<Widget<'w, A>> for Widget<'w, A> {}

impl<'w, A> Eq for Widget<'w, A> {}

impl<'w, A> PartialEq for Widget<'w, A> {
    fn eq(&self, other: &Self) -> bool {
        use Widget::*;
        match (self, other) {
            (Row(a), Row(b)) => a == b,
            (Column(a), Column(b)) => a == b,
            (Button(a), Button(b)) => a == b,
            (Entry(a), Entry(b)) => a == b,
            (Checkbox(a), Checkbox(b)) => a == b,
            _ => false,
        }
    }
}

use std::hash::{Hash, Hasher};
use std::mem::discriminant;

impl<'w, A> Hash for Widget<'w, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Widget::*;
        match self {
            Row(w) => w.hash(state),
            Column(w) => w.hash(state),
            Button(w) => w.hash(state),
            Entry(w) => w.hash(state),
            Checkbox(w) => w.hash(state),
        }

        discriminant(self).hash(state);
    }
}

impl<'a: 'v, 'w: 'v, 'v, A> From<&'a Widget<'w, A>> for Widget<'v, A> {
    fn from(widget: &'a Widget<'w, A>) -> Self {
        use Widget::*;
        match widget {
            Row(w) => (&**w).into(),
            Column(w) => (&**w).into(),
            Button(w) => (&**w).into(),
            Entry(w) => (&**w).into(),
            Checkbox(w) => (&**w).into(),
        }
    }
}

impl<'r: 'w, 'w, A> From<Row<'r, A>> for Widget<'w, A> {
    fn from(widget: Row<'r, A>) -> Self {
        Widget::Row(widget.into())
    }
}

impl<'a: 'w, 'r: 'w, 'w, A> From<&'a Row<'r, A>> for Widget<'w, A> {
    fn from(widget: &'a Row<'r, A>) -> Self {
        Widget::Row(widget.into())
    }
}

impl<'c: 'w, 'w, A> From<Column<'c, A>> for Widget<'w, A> {
    fn from(widget: Column<'c, A>) -> Self {
        Widget::Column(widget.into())
    }
}

impl<'a: 'w, 'r: 'w, 'w, A> From<&'a Column<'r, A>> for Widget<'w, A> {
    fn from(widget: &'a Column<'r, A>) -> Self {
        Widget::Column(widget.into())
    }
}

impl<'w, A> From<Button<A>> for Widget<'w, A> {
    fn from(widget: Button<A>) -> Self {
        Widget::Button(widget.into())
    }
}

impl<'a: 'w, 'w, A> From<&'a Button<A>> for Widget<'w, A> {
    fn from(widget: &'a Button<A>) -> Self {
        Widget::Button(widget.into())
    }
}

impl<'w, A> From<Entry<A>> for Widget<'w, A> {
    fn from(widget: Entry<A>) -> Self {
        Widget::Entry(widget.into())
    }
}

impl<'a: 'w, 'w, A> From<&'a Entry<A>> for Widget<'w, A> {
    fn from(widget: &'a Entry<A>) -> Self {
        Widget::Entry(widget.into())
    }
}

impl<'w, A> From<Checkbox<A>> for Widget<'w, A> {
    fn from(widget: Checkbox<A>) -> Self {
        Widget::Checkbox(widget.into())
    }
}

impl<'a: 'w, 'w, A> From<&'a Checkbox<A>> for Widget<'w, A> {
    fn from(widget: &'a Checkbox<A>) -> Self {
        Widget::Checkbox(widget.into())
    }
}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, collection::*, prelude::*, strategy::*, test_runner::*};

#[cfg(test)]
const DEPTH: usize = 3;

#[cfg(test)]
const BREADTH: usize = 3;

#[cfg(test)]
#[derive(derivative::Derivative)]
#[derivative(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Cardinality(
    #[derivative(Default(value = "DEPTH"))] pub usize,
    #[derivative(Default(value = "BREADTH"))] pub usize,
);

#[cfg(test)]
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct ChildrenStrategy<A: 'static>(
    #[derivative(Default(
        value = "vec(any_with::<Widget<A>>(Cardinality(DEPTH - 1, BREADTH)), 0..BREADTH)"
    ))]
    VecStrategy<BoxedStrategy<Widget<'static, A>>>,
);

#[cfg(test)]
pub fn children<A, T: Strategy<Value = Widget<'static, A>> + 'static>(
    widgets: T,
    size: impl Into<SizeRange>,
) -> ChildrenStrategy<A> {
    ChildrenStrategy(vec(widgets.boxed(), size))
}

#[cfg(test)]
impl<A> Strategy for ChildrenStrategy<A> {
    type Tree = <VecStrategy<BoxedStrategy<Widget<'static, A>>> as Strategy>::Tree;
    type Value = <VecStrategy<BoxedStrategy<Widget<'static, A>>> as Strategy>::Value;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        self.0.new_tree(runner)
    }
}

#[cfg(test)]
impl<A: 'static> Arbitrary for Widget<'static, A> {
    type Parameters = Cardinality;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(Cardinality(d, b): Self::Parameters) -> Self::Strategy {
        let size = b.pow(d as u32);

        prop_oneof![
            any::<Button<A>>().prop_map_into(),
            any::<Entry<A>>().prop_map_into(),
            any::<Checkbox<A>>().prop_map_into(),
        ]
        .prop_recursive(d as u32, size as u32, b as u32, move |inner| {
            prop_oneof![
                any_with::<Row<A>>(children(inner.clone(), 0..b)).prop_map_into(),
                any_with::<Column<A>>(children(inner.clone(), 0..b)).prop_map_into(),
                inner,
            ]
        })
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use maybe_owned::MaybeOwned::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    enum Action {}

    proptest! {
        #[test]
        fn from_row(w: Row<Action>) {
            assert_eq!(Widget::from(&w), Widget::Row(Borrowed(&w)));
            assert_eq!(Widget::from(w.clone()), Widget::Row(Owned(w.clone())));
            assert_eq!(Widget::from(&Widget::from(w.clone())), Widget::Row(Borrowed(&w)));
        }

        #[test]
        fn from_column(w: Column<Action>) {
            assert_eq!(Widget::from(&w), Widget::Column(Borrowed(&w)));
            assert_eq!(Widget::from(w.clone()), Widget::Column(Owned(w.clone())));
            assert_eq!(Widget::from(&Widget::from(w.clone())), Widget::Column(Borrowed(&w)));
        }

        #[test]
        fn from_button(w: Button<Action>) {
            assert_eq!(Widget::from(&w), Widget::Button(Borrowed(&w)));
            assert_eq!(Widget::from(w.clone()), Widget::Button(Owned(w.clone())));
            assert_eq!(Widget::from(&Widget::from(w.clone())), Widget::Button(Borrowed(&w)));
        }

        #[test]
        fn from_entry(w: Entry<Action>) {
            assert_eq!(Widget::from(&w), Widget::Entry(Borrowed(&w)));
            assert_eq!(Widget::from(w.clone()), Widget::Entry(Owned(w.clone())));
            assert_eq!(Widget::from(&Widget::from(w.clone())), Widget::Entry(Borrowed(&w)));
        }

        #[test]
        fn from_checkbox(w: Checkbox<Action>) {
            assert_eq!(Widget::from(&w), Widget::Checkbox(Borrowed(&w)));
            assert_eq!(Widget::from(w.clone()), Widget::Checkbox(Owned(w.clone())));
            assert_eq!(Widget::from(&Widget::from(w.clone())), Widget::Checkbox(Borrowed(&w)));
        }

        #[test]
        fn clone(w: Widget<Action>) {
            assert_eq!(w.clone(), w);
        }

        #[test]
        fn hash(w: Widget<Action>) {
            let mut hasher = NopHash(0);
            w.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }

    #[test]
    fn widget_hash_depends_on_discriminant() {
        let col: Widget<Action> = Column { children: vec![] }.into();
        let row: Widget<Action> = Row { children: vec![] }.into();

        let mut a = DefaultHasher::new();
        col.hash(&mut a);

        let mut b = DefaultHasher::new();
        row.hash(&mut b);

        assert_ne!(a.finish(), b.finish())
    }
}
