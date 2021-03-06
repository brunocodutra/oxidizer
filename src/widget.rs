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

use crate::{Kind, TreePath};
use maybe_owned::MaybeOwned;

/// The semantic representation of a widget.
#[derive(derivative::Derivative)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    Eq(bound = ""),
    PartialEq(bound = ""),
    Hash(bound = "")
)]
pub enum Widget<'w, A> {
    Row(MaybeOwned<'w, Row<'w, A>>),
    Column(MaybeOwned<'w, Column<'w, A>>),
    Button(MaybeOwned<'w, Button<A>>),
    Entry(MaybeOwned<'w, Entry<A>>),
    Checkbox(MaybeOwned<'w, Checkbox<A>>),
}

impl<'w, A> Widget<'w, A> {
    pub fn get<S: Into<usize>>(&self, path: impl TreePath<Segment = S>) -> Option<&Widget<'w, A>> {
        path.segments().into_iter().fold(Some(self), |r, i| {
            r.and_then(|w| w.into_iter().nth(i.into()))
        })
    }

    pub fn children(&self) -> &[Self] {
        use Widget::*;
        match self {
            Row(w) => w,
            Column(w) => w,
            _ => &[],
        }
    }
}

impl<'a, 'w, A> Kind<Widget<'a, A>> for Widget<'w, A> {}

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

use std::ops::Index;

impl<'w, A, S: Into<usize>, P: TreePath<Segment = S>> Index<P> for Widget<'w, A> {
    type Output = Self;
    fn index(&self, path: P) -> &Self::Output {
        self.get(path).expect("Out of bounds access")
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
        value = "children(any_with::<Widget<A>>(Cardinality(DEPTH - 1, BREADTH)), 0..=BREADTH).0"
    ))]
    BoxedStrategy<Box<[Widget<'static, A>]>>,
);

#[cfg(test)]
pub fn children<A, T, S>(widgets: T, size: S) -> ChildrenStrategy<A>
where
    A: Default,
    T: Strategy<Value = Widget<'static, A>> + 'static,
    S: Into<SizeRange>,
{
    ChildrenStrategy(vec(widgets, size).prop_map_into().boxed())
}

#[cfg(test)]
impl<A: Default> Strategy for ChildrenStrategy<A> {
    type Tree = <BoxedStrategy<Box<[Widget<'static, A>]>> as Strategy>::Tree;
    type Value = Box<[Widget<'static, A>]>;

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
                assert_eq!(root.get(p.iter().copied()), Some(w));

                let out_of_bounds = [&p[..], &[w.into_iter().count()]].concat();
                assert_eq!(root.get(out_of_bounds), None);

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

        #[test]
        fn children(w: Widget<Action>) {
            use Widget::*;
            match &w {
                Row(r) => assert_eq!(w.children(), &***r),
                Column(c) => assert_eq!(w.children(), &***c),
                _ => assert_eq!(w.children(), &[])
            }
        }

        #[test]
        fn index(root: Widget<Action>) {
            let mut indices = vec![Vec::<usize>::new()];

            while let Some(p) = indices.pop() {
                let w = &root[p.iter().copied()];
                indices.extend((0..w.into_iter().count()).map(|i| [&p[..], &[i]].concat()));
                assert_eq!(Some(w), root.get(p));
            }
        }

        #[test]
        #[should_panic]
        fn index_out_of_bounds(w: Widget<Action>) {
            let _ = &w[vec![w.into_iter().count()]];
        }
    }
}
