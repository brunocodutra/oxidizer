mod handler;
mod mock;
mod sugar;

pub use handler::Handler;

pub mod event;
pub use event::Event;

pub mod widget;
pub use widget::Widget;

/// Marker trait that denotes the variant of `enum T`.
pub trait Kind<T> {}
