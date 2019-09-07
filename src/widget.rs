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

/// The semantic representation of a widget.
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub enum Widget<A> {
    Row(Row<A>),
    Column(Column<A>),
    Button(Button<A>),
    Entry(Entry<A>),
    Checkbox(Checkbox<A>),
}

impl<A> Kind<Widget<A>> for Widget<A> {}

impl<A> Eq for Widget<A> {}

impl<A> PartialEq for Widget<A> {
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

impl<A> Hash for Widget<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Widget::*;
        match self {
            Row(v) => v.hash(state),
            Column(v) => v.hash(state),
            Button(v) => v.hash(state),
            Entry(v) => v.hash(state),
            Checkbox(v) => v.hash(state),
        }

        discriminant(self).hash(state);
    }
}

impl<A> From<Row<A>> for Widget<A> {
    fn from(row: Row<A>) -> Self {
        Widget::Row(row)
    }
}

impl<A> From<Column<A>> for Widget<A> {
    fn from(column: Column<A>) -> Self {
        Widget::Column(column)
    }
}

impl<A> From<Button<A>> for Widget<A> {
    fn from(button: Button<A>) -> Self {
        Widget::Button(button)
    }
}

impl<A> From<Entry<A>> for Widget<A> {
    fn from(entry: Entry<A>) -> Self {
        Widget::Entry(entry)
    }
}

impl<A> From<Checkbox<A>> for Widget<A> {
    fn from(checkbox: Checkbox<A>) -> Self {
        Widget::Checkbox(checkbox)
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
    VecStrategy<BoxedStrategy<Widget<A>>>,
);

#[cfg(test)]
pub fn children<A, T: Strategy<Value = Widget<A>> + 'static>(
    widgets: T,
    size: impl Into<SizeRange>,
) -> ChildrenStrategy<A> {
    ChildrenStrategy(vec(widgets.boxed(), size))
}

#[cfg(test)]
impl<A> Strategy for ChildrenStrategy<A> {
    type Tree = <VecStrategy<BoxedStrategy<Widget<A>>> as Strategy>::Tree;
    type Value = <VecStrategy<BoxedStrategy<Widget<A>>> as Strategy>::Value;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        self.0.new_tree(runner)
    }
}

#[cfg(test)]
impl<A: 'static> Arbitrary for Widget<A> {
    type Parameters = Cardinality;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(Cardinality(depth, breadth): Self::Parameters) -> Self::Strategy {
        let size = breadth.pow(depth as u32);

        prop_oneof![
            any::<Button<A>>().prop_map(Widget::Button),
            any::<Entry<A>>().prop_map(Widget::Entry),
            any::<Checkbox<A>>().prop_map(Widget::Checkbox),
        ]
        .prop_recursive(depth as u32, size as u32, breadth as u32, move |inner| {
            prop_oneof![
                any_with::<Row<A>>(children(inner.clone(), 0..breadth)).prop_map(Widget::Row),
                any_with::<Column<A>>(children(inner.clone(), 0..breadth)).prop_map(Widget::Column),
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
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    enum Action {}

    proptest! {
        #[test]
        fn from_row(widget: Row<Action>) {
            assert_eq!(Widget::from(widget.clone()), Widget::Row(widget));
        }

        #[test]
        fn from_column(widget: Column<Action>) {
            assert_eq!(Widget::from(widget.clone()), Widget::Column(widget));
        }

        #[test]
        fn from_button(widget: Button<Action>) {
            assert_eq!(Widget::from(widget.clone()), Widget::Button(widget));
        }

        #[test]
        fn from_entry(widget: Entry<Action>) {
            assert_eq!(Widget::from(widget.clone()), Widget::Entry(widget));
        }

        #[test]
        fn from_checkbox(widget: Checkbox<Action>) {
            assert_eq!(Widget::from(widget.clone()), Widget::Checkbox(widget));
        }

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

    #[test]
    fn widget_hash_depends_on_discriminant() {
        let col = Widget::<Action>::Column(Column { children: vec![] });
        let row = Widget::<Action>::Row(Row { children: vec![] });

        let mut a = DefaultHasher::new();
        col.hash(&mut a);

        let mut b = DefaultHasher::new();
        row.hash(&mut b);

        assert_ne!(a.finish(), b.finish())
    }
}
