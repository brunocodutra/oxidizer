/// A convenience macro for declaring user interfaces.
///
/// This macro allows declaring a tree of widgets without all the verbosity required to initialize
/// structs in Rust.
///
/// ## Example
/// ```rust
/// use oxidizer::{*, widget::*};
///
/// enum Action { /* ... */ }
///
/// let handler: fn(&Widget<_>, &Event) -> _ = |widget, event| {
///     // ...
///     # unimplemented!()
/// };
///
/// let ui: Widget<Action> = widget!(
///     Column [
///         Row [
///             Entry { handler }
///             Button { label: "Add Todo", handler }
///         ]
///         Checkbox { label: "buy milk", handler }
///         Checkbox { label: "learn oxidizer", handler }
///     ]
/// );
///
/// assert_eq!(ui, Widget::from(Column {
///         children: Box::new([
///             Widget::from(Row {
///                 children: Box::new([
///                     Widget::from(Entry {
///                         value: "".to_string(),
///                         handler: handler.into(),
///                     }),
///                     Widget::from(Button {
///                         label: "Add Todo".to_string(),
///                         handler: handler.into(),
///                     }),
///                 ])
///             }),
///             Widget::from(Checkbox {
///                 value: false,
///                 label: "buy milk".to_string(),
///                 handler: handler.into(),
///             }),
///             Widget::from(Checkbox {
///                 value: false,
///                 label: "learn oxidizer".to_string(),
///                 handler: handler.into(),
///             }),
///         ])
///     })
/// );
/// ```
#[macro_export]
macro_rules! widget {
    ( Row $({ $($ps:tt)* })? $([$($ts:ident $({ $($tps:tt)* })? $([$($tts:tt)*])?)*])? ) => {
        $crate::Widget::from($crate::init!($crate::widget::Row {
            children: vec![
                $($( $crate::widget!($ts $({ $($tps)* })* $([ $($tts)* ])*), )*)*
            ],
            $($($ps)*)*
        }))
    };

    ( Column $({ $($ps:tt)* })? $([$($ts:ident $({ $($tps:tt)* })? $([$($tts:tt)*])?)*])? ) => {
        $crate::Widget::from($crate::init!($crate::widget::Column {
            children: vec![
                $($( $crate::widget!($ts $({ $($tps)* })* $([ $($tts)* ])*), )*)*
            ],
            $($($ps)*)*
        }))
    };

    ( Button $({ $($ps:tt)* })? ) => {
        $crate::Widget::from($crate::init!($crate::widget::Button $({ $($ps)* })*))
    };

    ( Entry $({ $($ps:tt)* })? ) => {
        $crate::Widget::from($crate::init!($crate::widget::Entry $({ $($ps)* })*))
    };

    ( Checkbox $({ $($ps:tt)* })? ) => {
        $crate::Widget::from($crate::init!($crate::widget::Checkbox $({ $($ps)* })*))
    };
}

#[cfg(test)]
mod tests {
    use crate::widget::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn row_can_be_created_with_default_properties(_: ()) {
            assert_eq!(widget!(Row), Widget::Row::<()>(Default::default()));
            assert_eq!(widget!(Row {}), Widget::Row::<()>(Default::default()));
            assert_eq!(widget!(Row []), Widget::Row::<()>(Default::default()));
            assert_eq!(widget!(Row {} []), Widget::Row::<()>(Default::default()));
        }

        #[test]
        fn row_can_have_children(label: String, value: bool) {
            assert_eq!(
                widget!(
                    Row [
                        Entry
                        Button { label: label.clone() }
                        Checkbox { label: label.clone(), value }
                        Column [
                            Entry
                            Button { label: label.clone() }
                            Checkbox { label: label.clone(), value }
                        ]
                    ]
                ),
                Widget::from(Row::<()> {
                    children: Box::new([
                        widget!(Entry),
                        widget!(Button {
                            label: label.clone()
                        }),
                        widget!(Checkbox { label: label.clone(), value }),
                        widget!(Column [
                            Entry
                            Button { label: label.clone() }
                            Checkbox { label: label, value }
                        ])
                    ])
                })
            );
        }

        #[test]
        fn col_can_be_created_with_default_properties(_: ()) {
            assert_eq!(widget!(Column), Widget::Column::<()>(Default::default()));
            assert_eq!(widget!(Column {}), Widget::Column::<()>(Default::default()));
            assert_eq!(widget!(Column []), Widget::Column::<()>(Default::default()));
            assert_eq!(widget!(Column {} []), Widget::Column::<()>(Default::default()));
        }

        #[test]
        fn col_can_have_children(label: String, value: bool) {
            assert_eq!(
                widget!(
                    Column [
                        Entry
                        Button { label: label.clone() }
                        Checkbox { label: label.clone(), value }
                        Row [
                            Entry
                            Button { label: label.clone() }
                            Checkbox { label: label.clone(), value }
                        ]
                    ]
                ),
                Widget::from(Column::<()> {
                    children: Box::new([
                        widget!(Entry),
                        widget!(Button {
                            label: label.clone()
                        }),
                        widget!(Checkbox { label: label.clone(), value }),
                        widget!(Row [
                            Entry
                            Button { label: label.clone() }
                            Checkbox { label: label, value }
                        ])
                    ])
                })
            );
        }

        #[test]
        fn button_can_be_created_with_default_properties(_: ()) {
            assert_eq!(widget!(Button), Widget::Button::<()>(Default::default()));
            assert_eq!(widget!(Button {}), Widget::Button::<()>(Default::default()));
        }

        #[test]
        fn button_optionally_takes_a_label(label: String) {
            assert_eq!(
                widget!(Button {
                    label: label.clone()
                }),
                Widget::<()>::from(Button {
                    label,
                    ..Default::default()
                })
            );
        }

        #[test]
        fn button_optionally_takes_a_handler(_: ()) {
            let handler: fn(&_, &_) = |_, _| {};

            assert_eq!(
                widget!(Button { handler }),
                Widget::from(Button {
                    handler: handler.into(),
                    ..Default::default()
                })
            );
        }

        #[test]
        fn entry_can_be_created_with_default_properties(_: ()) {
            assert_eq!(widget!(Entry), Widget::Entry::<()>(Default::default()));
            assert_eq!(widget!(Entry {}), Widget::Entry::<()>(Default::default()));
        }

        #[test]
        fn entry_optionally_takes_a_value(value: String) {
            assert_eq!(
                widget!(Entry {
                    value: value.clone()
                }),
                Widget::<()>::from(Entry {
                    value,
                    ..Default::default()
                })
            );
        }

        #[test]
        fn entry_optionally_takes_a_handler(_: ()) {
            let handler: fn(&_, &_) = |_, _| {};

            assert_eq!(
                widget!(Entry { handler }),
                Widget::from(Entry {
                    handler: handler.into(),
                    ..Default::default()
                })
            );
        }

        #[test]
        fn checkbox_can_be_created_with_default_properties(_: ()) {
            assert_eq!(widget!(Checkbox), Widget::Checkbox::<()>(Default::default()));
            assert_eq!(widget!(Checkbox {}), Widget::Checkbox::<()>(Default::default()));
        }

        #[test]
        fn checkbox_optionally_takes_a_value(value: bool) {
            assert_eq!(
                widget!(Checkbox { value }),
                Widget::<()>::from(Checkbox {
                    value,
                    ..Default::default()
                })
            );
        }

        #[test]
        fn checkbox_optionally_takes_a_handler(_: ()) {
            let handler: fn(&_, &_) = |_, _| {};

            assert_eq!(
                widget!(Checkbox { handler }),
                Widget::from(Checkbox {
                    handler: handler.into(),
                    ..Default::default()
                })
            );
        }
    }
}
