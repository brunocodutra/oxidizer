mod handler;
mod index;
mod sugar;

pub use handler::{Handler, OptionalHandler};

pub mod event;
pub use event::Event;

pub mod widget;
pub use widget::Widget;

pub use index::TreeIndex;

/// Marker trait used to denote an abstract _kind_.
///
/// A [`Kind`] and its [`Variant`]s form a closed polymorphic group.
pub trait Kind<T> {}

/// Marker trait used to denote the variants of a [`Kind`].
pub trait Variant<T: Kind<T>>: Kind<T> {}

impl<T: Kind<T>, U: Variant<T>> Kind<T> for U {}
