use crate::{event::Event, widget::Widget, Kind, Variant};

/// An event handler.
#[derive(derivative::Derivative)]
#[derivative(Copy(bound = ""), Clone(bound = ""))]
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

    fn decay(&self) -> *const () {
        match *self {
            Handler::A(f) => f as *const (),
            Handler::B(f) => f as *const (),
            Handler::C(f) => f as *const (),
            Handler::D(f) => f as *const (),
        }
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

use std::fmt;

impl<W: Kind<Widget<A>>, E: Kind<Event>, A> fmt::Pointer for Handler<W, E, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.decay().fmt(f)
    }
}

impl<W: Kind<Widget<A>>, E: Kind<Event>, A> fmt::Debug for Handler<W, E, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.decay().fmt(f)
    }
}

impl<W: Kind<Widget<A>>, E: Kind<Event>, A> Eq for Handler<W, E, A> {}

impl<W: Kind<Widget<A>>, E: Kind<Event>, A> PartialEq for Handler<W, E, A> {
    fn eq(&self, other: &Self) -> bool {
        self.decay() == other.decay()
    }
}

use std::hash::{Hash, Hasher};

impl<W: Kind<Widget<A>>, E: Kind<Event>, A> Hash for Handler<W, E, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.decay().hash(state);
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

        #[test]
        fn debug(handler: Handler<Widget<_>, Event, Action>) {
            assert_eq!(format!("{:?}", handler), format!("{:p}", handler));
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
    fn from() {
        let a: fn(Button<Action>, Clicked) -> _ = |_, _| Action;
        let b: fn(Widget<Action>, Clicked) -> _ = |_, _| Action;
        let c: fn(Button<Action>, Event) -> _ = |_, _| Action;
        let d: fn(Widget<Action>, Event) -> _ = |_, _| Action;

        assert_eq!(Handler::<Button<_>, Clicked, _>::from(a), Handler::A(a));
        assert_eq!(Handler::<Button<_>, Clicked, _>::from(b), Handler::B(b));
        assert_eq!(Handler::<Button<_>, Clicked, _>::from(c), Handler::C(c));
        assert_eq!(Handler::<Button<_>, Clicked, _>::from(d), Handler::D(d));
    }
}
