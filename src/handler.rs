use crate::{event::Event, widget::Widget, Kind, Variant};

/// An event handler.
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""), Copy(bound = ""), Clone(bound = ""))]
pub enum Handler<W: Kind<Widget<A>>, E: Kind<Event>, A> {
    #[doc(hidden)]
    A(fn(W, E) -> A),
    #[doc(hidden)]
    B(fn(Widget<A>, E) -> A),
    #[doc(hidden)]
    C(fn(W, Event) -> A),
    #[doc(hidden)]
    D(fn(Widget<A>, Event) -> A),
}

impl<W: Kind<Widget<A>>, E: Kind<Event>, A> Handler<W, E, A> {
    pub fn new(f: fn(W, E) -> A) -> Self {
        Handler::A(f)
    }

    pub fn handle(&self, widget: W, event: E) -> A {
        use Handler::*;
        match self {
            A(f) => f(widget, event),
            B(f) => f(widget.into(), event),
            C(f) => f(widget, event.into()),
            D(f) => f(widget.into(), event.into()),
        }
    }
}

impl<W: Kind<Widget<A>>, E: Kind<Event>, A> Eq for Handler<W, E, A> {}

impl<W: Kind<Widget<A>>, E: Kind<Event>, A> PartialEq for Handler<W, E, A> {
    fn eq(&self, other: &Self) -> bool {
        use Handler::*;
        match (self, other) {
            (A(f), A(g)) => f == g,
            (B(f), B(g)) => f == g,
            (C(f), C(g)) => f == g,
            (D(f), D(g)) => f == g,
            _ => false,
        }
    }
}

use std::hash::{Hash, Hasher};
use std::mem::discriminant;

impl<W: Kind<Widget<A>>, E: Kind<Event>, A> Hash for Handler<W, E, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Handler::*;
        match self {
            A(f) => f.hash(state),
            B(f) => f.hash(state),
            C(f) => f.hash(state),
            D(f) => f.hash(state),
        }

        discriminant(self).hash(state);
    }
}

impl<W, E, A> From<fn(W, E) -> A> for Handler<W, E, A>
where
    W: Variant<Widget<A>>,
    E: Variant<Event>,
{
    fn from(f: fn(W, E) -> A) -> Self {
        Handler::A(f)
    }
}

impl<W, E, A> From<fn(Widget<A>, E) -> A> for Handler<W, E, A>
where
    W: Kind<Widget<A>>,
    E: Variant<Event>,
{
    fn from(f: fn(Widget<A>, E) -> A) -> Self {
        Handler::B(f)
    }
}

impl<W, E, A> From<fn(W, Event) -> A> for Handler<W, E, A>
where
    W: Variant<Widget<A>>,
    E: Kind<Event>,
{
    fn from(f: fn(W, Event) -> A) -> Self {
        Handler::C(f)
    }
}

impl<W, E, A> From<fn(Widget<A>, Event) -> A> for Handler<W, E, A>
where
    W: Kind<Widget<A>>,
    E: Kind<Event>,
{
    fn from(f: fn(Widget<A>, Event) -> A) -> Self {
        Handler::D(f)
    }
}

impl<W, E, A> From<Handler<Widget<A>, E, A>> for Handler<W, E, A>
where
    W: Variant<Widget<A>>,
    E: Kind<Event>,
{
    fn from(h: Handler<Widget<A>, E, A>) -> Self {
        use Handler::*;
        match h {
            A(f) => Handler::B(f),
            B(f) => Handler::B(f),
            C(f) => Handler::D(f),
            D(f) => Handler::D(f),
        }
    }
}

impl<W, E, A> From<Handler<W, Event, A>> for Handler<W, E, A>
where
    W: Kind<Widget<A>>,
    E: Variant<Event>,
{
    fn from(h: Handler<W, Event, A>) -> Self {
        use Handler::*;
        match h {
            A(f) => Handler::C(f),
            B(f) => Handler::D(f),
            C(f) => Handler::C(f),
            D(f) => Handler::D(f),
        }
    }
}

impl<W, E, A> From<Handler<Widget<A>, Event, A>> for Handler<W, E, A>
where
    W: Variant<Widget<A>>,
    E: Variant<Event>,
{
    fn from(h: Handler<Widget<A>, Event, A>) -> Self {
        use Handler::*;
        match h {
            A(f) => Handler::D(f),
            B(f) => Handler::D(f),
            C(f) => Handler::D(f),
            D(f) => Handler::D(f),
        }
    }
}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl<W, E, A> Arbitrary for Handler<W, E, A>
where
    W: 'static + Kind<Widget<A>>,
    E: 'static + Kind<Event>,
    A: 'static + Default,
{
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Handler::A(|_: W, _: E| A::default())),
            Just(Handler::B(|_: Widget<A>, _: E| A::default())),
            Just(Handler::C(|_: W, _: Event| A::default())),
            Just(Handler::D(|_: Widget<A>, _: Event| A::default())),
        ]
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{event::*, mock::*, widget::*};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Default, Eq, PartialEq)]
    struct Action;

    #[test]
    fn new() {
        let h = |_, _| Action;
        assert_eq!(Handler::<Widget<_>, Event, Action>::new(h), Handler::A(h));
    }

    proptest! {
        #[test]
        fn handle(w: Widget<Action>, e: Event, handler: Handler<Widget<_>, Event, Action>) {
            assert_eq!(handler.handle(w, e), Action);
        }

        #[allow(clippy::clone_on_copy)]
        #[test]
        fn clone(handler: Handler<Widget<_>, Event, Action>) {
            assert_eq!(handler.clone(), handler);
        }

        #[test]
        fn hash(handler: Handler<Widget<_>, Event, Action>) {
            let mut hasher = NopHash(0);
            handler.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }

    #[test]
    fn handler_hash_depends_on_discriminant() {
        let h = |_, _| Action;

        let x = Handler::<Widget<_>, Event, Action>::A(h);
        let y = Handler::<Widget<_>, Event, Action>::D(h);

        let mut a = DefaultHasher::new();
        x.hash(&mut a);

        let mut b = DefaultHasher::new();
        y.hash(&mut b);

        assert_ne!(a.finish(), b.finish())
    }

    #[test]
    fn from() {
        let a: fn(Button<Action>, Clicked) -> _ = |_, _| Action;
        let b: fn(Widget<Action>, Clicked) -> _ = |_, _| Action;
        let c: fn(Button<Action>, Event) -> _ = |_, _| Action;
        let d: fn(Widget<Action>, Event) -> _ = |_, _| Action;

        assert_eq!(Handler::<Button<_>, Clicked, _>::from(a), Handler::A(a));
        assert_eq!(Handler::<Button<_>, Clicked, _>::from(b), Handler::B(b));
        assert_eq!(Handler::<Button<_>, Clicked, _>::from(c), Handler::C(c));
        assert_eq!(Handler::<Button<_>, Clicked, _>::from(d), Handler::D(d));

        {
            let w = Handler::<Widget<_>, Clicked, _>::A(b);
            let x = Handler::<Widget<_>, Clicked, _>::B(b);
            let y = Handler::<Widget<_>, Clicked, _>::C(d);
            let z = Handler::<Widget<_>, Clicked, _>::D(d);

            assert_eq!(Handler::<Button<_>, Clicked, _>::from(w), Handler::B(b));
            assert_eq!(Handler::<Button<_>, Clicked, _>::from(x), Handler::B(b));
            assert_eq!(Handler::<Button<_>, Clicked, _>::from(y), Handler::D(d));
            assert_eq!(Handler::<Button<_>, Clicked, _>::from(z), Handler::D(d));
        }

        {
            let w = Handler::<Button<_>, Event, _>::A(c);
            let x = Handler::<Button<_>, Event, _>::B(d);
            let y = Handler::<Button<_>, Event, _>::C(c);
            let z = Handler::<Button<_>, Event, _>::D(d);

            assert_eq!(Handler::<Button<_>, Clicked, _>::from(w), Handler::C(c));
            assert_eq!(Handler::<Button<_>, Clicked, _>::from(x), Handler::D(d));
            assert_eq!(Handler::<Button<_>, Clicked, _>::from(y), Handler::C(c));
            assert_eq!(Handler::<Button<_>, Clicked, _>::from(z), Handler::D(d));
        }

        {
            let w = Handler::<Widget<_>, Event, _>::A(d);
            let x = Handler::<Widget<_>, Event, _>::B(d);
            let y = Handler::<Widget<_>, Event, _>::C(d);
            let z = Handler::<Widget<_>, Event, _>::D(d);

            assert_eq!(Handler::<Button<_>, Clicked, _>::from(w), Handler::D(d));
            assert_eq!(Handler::<Button<_>, Clicked, _>::from(x), Handler::D(d));
            assert_eq!(Handler::<Button<_>, Clicked, _>::from(y), Handler::D(d));
            assert_eq!(Handler::<Button<_>, Clicked, _>::from(z), Handler::D(d));

            assert_eq!(Handler::<Widget<_>, Clicked, _>::from(w), Handler::C(d));
            assert_eq!(Handler::<Widget<_>, Clicked, _>::from(x), Handler::D(d));
            assert_eq!(Handler::<Widget<_>, Clicked, _>::from(y), Handler::C(d));
            assert_eq!(Handler::<Widget<_>, Clicked, _>::from(z), Handler::D(d));

            assert_eq!(Handler::<Button<_>, Event, _>::from(w), Handler::B(d));
            assert_eq!(Handler::<Button<_>, Event, _>::from(x), Handler::B(d));
            assert_eq!(Handler::<Button<_>, Event, _>::from(y), Handler::D(d));
            assert_eq!(Handler::<Button<_>, Event, _>::from(z), Handler::D(d));
        }
    }
}
