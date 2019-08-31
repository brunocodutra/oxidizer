#[doc(hidden)]
#[macro_export]
macro_rules! init {
    ( $t:path $({ $($p:ident: $v:expr $(, $ps:ident: $vs:expr)* $(,)?)? })? ) => {
        {
            #[allow(clippy::needless_update)]
            $t {
                $($($p: $v.into() $(, $ps: $vs.into())*,)*)*
                ..Default::default()
            }
        }
    };

    ( $t:path $({ $($p:ident $(, $ps:ident $(: $vs:expr)?)* $(,)?)? })? ) => {
        $crate::init!($t $($({ $($ps $(: $vs)*,)* $p: $p })*)*)
    };

    ( $t:path $({ $($p:ident: $v:expr $(, $ps:ident $(: $vs:expr)?)* $(,)?)? })? ) => {
        $crate::init!($t $($({ $($ps $(: $vs)*,)* $p: $v })*)*)
    };
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    #[derive(Debug, Default, Eq, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn init_works_with_no_list_of_properties() {
        assert_eq!(init!(Person), Person::default());
    }

    #[test]
    fn init_works_with_an_empty_list_of_properties() {
        assert_eq!(init!(Person {}), Person::default());
    }

    proptest! {
        #[test]
        fn init_works_with_a_partial_list_of_properties(name: String) {
            assert_eq!(
                init!(Person { name: name.clone() }),
                Person { name, age: 0 }
            );
        }

        #[test]
        fn init_works_with_a_complete_list_of_properties(name: String, age: u32) {
            assert_eq!(
                init!(Person {
                    name: name.clone(),
                    age
                }),
                Person { name, age }
            );
        }

        #[test]
        fn init_accepts_a_trailing_comma(name: String, age: u32) {
            assert_eq!(
                init!(Person {
                    name: name.clone(),
                    age,
                }),
                Person { name, age }
            );
        }

        #[test]
        fn init_does_implicit_conversion(name: Box<str>, age: u16) {
            assert_eq!(
                init!(Person {
                    name: name.clone(),
                    age
                }),
                Person {
                    name: name.into(),
                    age: age.into(),
                }
            );
        }
    }
}
