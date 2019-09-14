use crate::{event::Event, widget::Widget, Kind, Variant};

/// An event handler.
#[derive(derivative::Derivative)]
#[derivative(Copy(bound = ""), Clone(bound = ""))]
enum GenericHandler<W, E, A> {
    A(fn(&W, &E) -> A),
    B(fn(&Widget<A>, &E) -> A),
    C(fn(&W, &Event) -> A),
    D(fn(&Widget<A>, &Event) -> A),
}

impl<W, E, A> GenericHandler<W, E, A> {
    fn decay(&self) -> *const () {
        use GenericHandler::*;
        match *self {
            A(f) => f as *const (),
            B(f) => f as *const (),
            C(f) => f as *const (),
            D(f) => f as *const (),
        }
    }
}

use std::fmt;

impl<W, E, A> fmt::Pointer for GenericHandler<W, E, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.decay().fmt(f)
    }
}

impl<W, E, A> fmt::Debug for GenericHandler<W, E, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.decay().fmt(f)
    }
}

impl<W, E, A> Eq for GenericHandler<W, E, A> {}

impl<W, E, A> PartialEq for GenericHandler<W, E, A> {
    fn eq(&self, other: &Self) -> bool {
        self.decay() == other.decay()
    }
}

use std::hash::{Hash, Hasher};

impl<W, E, A> Hash for GenericHandler<W, E, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.decay().hash(state);
    }
}

/// An event handler.
#[derive(derivative::Derivative)]
#[derivative(
    Debug(bound = ""),
    Copy(bound = ""),
    Clone(bound = ""),
    Eq(bound = ""),
    PartialEq(bound = ""),
    Hash(bound = "")
)]
pub struct Handler<W, E, A>(GenericHandler<W, E, A>);

impl<W, E, A> Handler<W, E, A> {
    pub fn new(f: fn(&W, &E) -> A) -> Self {
        Handler(GenericHandler::A(f))
    }
}

impl<'w, 'e, W, E, A: 'w> Handler<W, E, A>
where
    W: Kind<Widget<'w, A>>,
    E: Kind<Event<'e>>,
    for<'a> Widget<'a, A>: From<&'a W>,
    for<'a> Event<'a>: From<&'a E>,
{
    pub fn handle(&self, widget: &W, event: &E) -> A {
        use GenericHandler::*;
        match *self {
            Handler(A(f)) => f(widget, event),
            Handler(B(f)) => f(&widget.into(), event),
            Handler(C(f)) => f(widget, &event.into()),
            Handler(D(f)) => f(&widget.into(), &event.into()),
        }
    }
}

impl<'w, 'e, W, E, A: 'w> From<fn(&W, &E) -> A> for Handler<W, E, A>
where
    W: Variant<Widget<'w, A>>,
    E: Variant<Event<'e>>,
{
    fn from(f: fn(&W, &E) -> A) -> Self {
        Handler(GenericHandler::A(f))
    }
}

impl<'w, 'e, W, E, A: 'w> From<fn(&Widget<A>, &E) -> A> for Handler<W, E, A>
where
    W: Kind<Widget<'w, A>>,
    E: Variant<Event<'e>>,
{
    fn from(f: fn(&Widget<A>, &E) -> A) -> Self {
        Handler(GenericHandler::B(f))
    }
}

impl<'w, 'e, W, E, A: 'w> From<fn(&W, &Event) -> A> for Handler<W, E, A>
where
    W: Variant<Widget<'w, A>>,
    E: Kind<Event<'e>>,
{
    fn from(f: fn(&W, &Event) -> A) -> Self {
        Handler(GenericHandler::C(f))
    }
}

impl<'w, 'e, W, E, A: 'w> From<fn(&Widget<A>, &Event) -> A> for Handler<W, E, A>
where
    W: Kind<Widget<'w, A>>,
    E: Kind<Event<'e>>,
{
    fn from(f: fn(&Widget<A>, &Event) -> A) -> Self {
        Handler(GenericHandler::D(f))
    }
}

/// An optional event handler.
///
/// Semantically equivalent to `Option<Handler<W, E, A>>`,
/// but more ergonomic to use with the [`widget!`] macro.
///
/// In particular, [`OptionalHandler`] implements [`From<H: Into<Handler>>`](#impl-From<H>).
#[derive(derivative::Derivative)]
#[derivative(
    Debug(bound = ""),
    Default(bound = ""),
    Copy(bound = ""),
    Clone(bound = "")
)]
pub enum OptionalHandler<W, E, A> {
    Some(Handler<W, E, A>),
    #[derivative(Default)]
    None,
}

impl<W, E, A> Eq for OptionalHandler<W, E, A> {}

impl<W, E, A> PartialEq for OptionalHandler<W, E, A> {
    fn eq(&self, other: &Self) -> bool {
        use OptionalHandler::*;
        match (self, other) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        }
    }
}

use std::mem::discriminant;

impl<W, E, A> Hash for OptionalHandler<W, E, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let OptionalHandler::Some(h) = self {
            h.hash(state);
        }

        discriminant(self).hash(state);
    }
}

impl<H, W, E, A> From<H> for OptionalHandler<W, E, A>
where
    Handler<W, E, A>: From<H>,
{
    fn from(h: H) -> Self {
        OptionalHandler::Some(h.into())
    }
}

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, prelude::*};

#[cfg(test)]
impl<W, E, A> Arbitrary for Handler<W, E, A>
where
    W: 'static + Kind<Widget<'static, A>>,
    E: 'static + Kind<Event<'static>>,
    A: 'static + Default,
{
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Handler(GenericHandler::A(|_, _| A::default()))),
            Just(Handler(GenericHandler::B(|_, _| A::default()))),
            Just(Handler(GenericHandler::C(|_, _| A::default()))),
            Just(Handler(GenericHandler::D(|_, _| A::default()))),
        ]
        .boxed()
    }
}

#[cfg(test)]
impl<W, E, A> Arbitrary for OptionalHandler<W, E, A>
where
    W: 'static + Kind<Widget<'static, A>>,
    E: 'static + Kind<Event<'static>>,
    A: 'static + Default,
{
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(OptionalHandler::None),
            any::<Handler<W, E, A>>().prop_map(OptionalHandler::Some),
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
        let h = |_: &Widget<Action>, _: &Event| Action;
        assert_eq!(Handler::new(h), Handler(GenericHandler::A(h)));
    }

    proptest! {
        #[test]
        fn handle(w: Widget<Action>, e: Event, handler: Handler<Widget<_>, Event, Action>) {
            assert_eq!(handler.handle(&w, &e), Action);
        }

        #[test]
        fn debug(handler: Handler<Widget<_>, Event, Action>) {
            assert_eq!(format!("{:?}", handler), format!("Handler({:p})", handler.0));
        }

        #[allow(clippy::clone_on_copy)]
        #[test]
        fn clone(handler: OptionalHandler<Widget<_>, Event, Action>) {
            assert_eq!(handler.clone(), handler);
        }

        #[test]
        fn hash(handler: OptionalHandler<Widget<_>, Event, Action>) {
            let mut hasher = NopHash(0);
            handler.hash(&mut hasher);
            assert_eq!(hasher.finish(), 0);
        }
    }

    #[test]
    fn from() {
        let a: fn(&Button<Action>, &Entered) -> _ = |_, _| Action;
        let b: fn(&Widget<Action>, &Entered) -> _ = |_, _| Action;
        let c: fn(&Button<Action>, &Event) -> _ = |_, _| Action;
        let d: fn(&Widget<Action>, &Event) -> _ = |_, _| Action;

        use GenericHandler::*;

        assert_eq!(Handler::<Button<_>, Entered, _>::from(a), Handler(A(a)));
        assert_eq!(Handler::<Button<_>, Entered, _>::from(b), Handler(B(b)));
        assert_eq!(Handler::<Button<_>, Entered, _>::from(c), Handler(C(c)));
        assert_eq!(Handler::<Button<_>, Entered, _>::from(d), Handler(D(d)));

        assert_eq!(
            OptionalHandler::<Button<_>, Entered, _>::from(a),
            OptionalHandler::Some(Handler(A(a)))
        );

        assert_eq!(
            OptionalHandler::<Button<_>, Entered, _>::from(b),
            OptionalHandler::Some(Handler(B(b)))
        );

        assert_eq!(
            OptionalHandler::<Button<_>, Entered, _>::from(c),
            OptionalHandler::Some(Handler(C(c)))
        );

        assert_eq!(
            OptionalHandler::<Button<_>, Entered, _>::from(d),
            OptionalHandler::Some(Handler(D(d)))
        );
    }
}
