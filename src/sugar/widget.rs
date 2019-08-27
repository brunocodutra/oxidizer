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
/// let ui: Widget<Action> = widget!(
///     Column [
///         Row [
///             Entry
///             Button { label: "Add Todo".into() }
///         ]
///         Checkbox { label: "buy milk".into() }
///         Checkbox { label: "learn oxidizer".into() }
///     ]
/// );
///
/// assert_eq!(ui, Widget::from(Column {
///         children: vec![
///             Widget::from(Row {
///                 children: vec![
///                     Widget::from(Entry {
///                         value: "".into(),
///                         handler: None,
///                     }),
///                     Widget::from(Button {
///                         label: "Add Todo".into(),
///                         handler: None,
///                     }),
///                 ]
///             }),
///             Widget::from(Checkbox {
///                 value: false,
///                 label: "buy milk".into(),
///                 handler: None,
///             }),
///             Widget::from(Checkbox {
///                 value: false,
///                 label: "learn oxidizer".into(),
///                 handler: None,
///             }),
///         ]
///     })
/// );
/// ```
#[macro_export]
macro_rules! widget {
    ( Row $({ $($ps:tt)* })? $([$($ts:ident $({ $($tps:tt)* })? $([$($tts:tt)*])?)*])? ) => {
        $crate::Widget::Row($crate::init!($crate::widget::Row {
            children: vec![
                $($( $crate::widget!($ts $({ $($tps)* })* $([ $($tts)* ])*), )*)*
            ],
            $($($ps)*)*
        }))
    };

    ( Column $({ $($ps:tt)* })? $([$($ts:ident $({ $($tps:tt)* })? $([$($tts:tt)*])?)*])? ) => {
        $crate::Widget::Column($crate::init!($crate::widget::Column {
            children: vec![
                $($( $crate::widget!($ts $({ $($tps)* })* $([ $($tts)* ])*), )*)*
            ],
            $($($ps)*)*
        }))
    };

    ( Button $({ $($ps:tt)* })? ) => {
        $crate::Widget::Button($crate::init!($crate::widget::Button $({ $($ps)* })*))
    };

    ( Entry $({ $($ps:tt)* })? ) => {
        $crate::Widget::Entry($crate::init!($crate::widget::Entry $({ $($ps)* })*))
    };

    ( Checkbox $({ $($ps:tt)* })? ) => {
        $crate::Widget::Checkbox($crate::init!($crate::widget::Checkbox $({ $($ps)* })*))
    };
}

#[cfg(test)]
mod tests {
    use crate::{event::*, widget::*};
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
                Widget::Row(Row::<()> {
                    children: vec![
                        widget!(Entry),
                        widget!(Button {
                            label: label.clone()
                        }),
                        widget!(Checkbox { label: label.clone(), value }),
                        widget!(Column [
                            Entry
                            Button { label: label.clone() }
                            Checkbox { label: label.clone(), value }
                        ])
                    ]
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
                Widget::Column(Column::<()> {
                    children: vec![
                        widget!(Entry),
                        widget!(Button {
                            label: label.clone()
                        }),
                        widget!(Checkbox { label: label.clone(), value }),
                        widget!(Row [
                            Entry
                            Button { label: label.clone() }
                            Checkbox { label: label.clone(), value }
                        ])
                    ]
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
                Widget::Button::<()>(Button {
                    label,
                    ..Default::default()
                })
            );
        }

        #[test]
        fn button_optionally_takes_a_handler(_: ()) {
            let handler = Some::<fn(Button<()>, Clicked)>(|_, _| {});

            assert_eq!(
                widget!(Button { handler }),
                Widget::Button(Button {
                    handler,
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
                Widget::Entry::<()>(Entry {
                    value,
                    ..Default::default()
                })
            );
        }

        #[test]
        fn entry_optionally_takes_a_handler(_: ()) {
            let handler = Some::<fn(Entry<()>, Entered)>(|_, _| {});

            assert_eq!(
                widget!(Entry { handler }),
                Widget::Entry(Entry {
                    handler,
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
                Widget::Checkbox::<()>(Checkbox {
                    value,
                    ..Default::default()
                })
            );
        }

        #[test]
        fn checkbox_optionally_takes_a_handler(_: ()) {
            let handler = Some::<fn(Checkbox<()>, Toggled)>(|_, _| {});

            assert_eq!(
                widget!(Checkbox { handler }),
                Widget::Checkbox(Checkbox {
                    handler,
                    ..Default::default()
                })
            );
        }
    }
}
