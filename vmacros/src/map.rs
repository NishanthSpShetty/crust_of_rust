use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    () => { HashMap::new() };
    ($($key:expr => $value:expr,)*) => {{
        $crate::hashmap!( $($key => $value),*)
    }};
    ($($key:expr => $value:expr),*) => {{
        let mut m = HashMap::new();
        $(    m.insert($key, $value);)*
        m
    }};
}

///hash macro_rules
///

#[test]
fn test_empty_map() {
    let a: HashMap<i32, &str> = hashmap! {};
    assert!(a.is_empty());
}

#[test]
fn test_single_entry_map() {
    let a: HashMap<i32, &str> = hashmap! {1 => "One"};
    assert!(!a.is_empty());
    assert_eq!(a.get(&1), Some(&"One"));
}

#[test]
fn test_double_entry_map() {
    let a: HashMap<i32, &str> = hashmap! {1 => "One",
    2 => "two",};
    assert!(!a.is_empty());
    assert_eq!(a.get(&1), Some(&"One"));
    assert_eq!(a.get(&2), Some(&"two"));
}
