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

use crate::{Kind, TreeIndex};
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

impl<'w, A> Widget<'w, A> {
    pub fn get<I: Into<usize>>(&self, index: impl TreeIndex<I>) -> Option<&Widget<'w, A>> {
        index.path().fold(Some(self), |r, i| {
            r.and_then(|w| w.into_iter().nth(i.into()))
        })
    }
}

impl<'a, 'w, A> Kind<Widget<'a, A>> for Widget<'w, A> {}

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

use std::slice::Iter;

impl<'a, 'w: 'a, A> IntoIterator for &'a Widget<'w, A> {
    type Item = &'a Widget<'w, A>;
    type IntoIter = Iter<'a, Widget<'w, A>>;

    fn into_iter(self) -> Self::IntoIter {
        use Widget::*;
        match self {
            Row(w) => w.into_iter(),
            Column(w) => w.into_iter(),
            _ => [].iter(),
        }
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
pub struct ChildrenStrategy<A: 'static + Default>(
    #[derivative(Default(
        value = "vec(any_with::<Widget<A>>(Cardinality(DEPTH - 1, BREADTH)), 0..=BREADTH)"
    ))]
    VecStrategy<BoxedStrategy<Widget<'static, A>>>,
);

#[cfg(test)]
pub fn children<A: Default, T: Strategy<Value = Widget<'static, A>> + 'static>(
    widgets: T,
    size: impl Into<SizeRange>,
) -> ChildrenStrategy<A> {
    ChildrenStrategy(vec(widgets.boxed(), size))
}

#[cfg(test)]
impl<A: Default> Strategy for ChildrenStrategy<A> {
    type Tree = <VecStrategy<BoxedStrategy<Widget<'static, A>>> as Strategy>::Tree;
    type Value = <VecStrategy<BoxedStrategy<Widget<'static, A>>> as Strategy>::Value;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        self.0.new_tree(runner)
    }
}

#[cfg(test)]
impl<A: 'static + Default> Arbitrary for Widget<'static, A> {
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
                any_with::<Row<A>>(children(inner.clone(), 0..=b)).prop_map_into(),
                any_with::<Column<A>>(children(inner.clone(), 0..=b)).prop_map_into(),
            ]
        })
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maybe_owned::MaybeOwned::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::iter::FromIterator;

    #[derive(Default)]
    struct Action;

    proptest! {
        #[test]
        fn get(root: Widget<Action>) {
            let mut indexed = vec![(Vec::<usize>::new(), &root)];

            while let Some((p, w)) = indexed.pop() {
                assert_eq!(root.get(&p), Some(w));

                let out_of_bounds = [&p[..], &[w.into_iter().count()]].concat();
                assert_eq!(root.get(&out_of_bounds), None);

                indexed.extend(
                    w.into_iter()
                        .enumerate()
                        .map(|(i, w)| ([&p[..], &[i]].concat(), w)),
                );
            }
        }

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
        fn hash(x: Widget<Action>, y: Widget<Action>) {
            let mut a = DefaultHasher::new();
            x.hash(&mut a);

            let mut b = DefaultHasher::new();
            y.hash(&mut b);

            assert_eq!(x == y, a.finish() == b.finish());
        }

        #[test]
        fn into_iter(w: Widget<Action>) {
            let items = Vec::from_iter(&w);

            use Widget::*;
            match &w {
                Row(w) => assert_eq!(items, Vec::from_iter(&**w)),
                Column(w) => assert_eq!(items, Vec::from_iter(&**w)),
                _ => assert_eq!(items, Vec::<&Widget<_>>::new())
            }
        }
    }
}
