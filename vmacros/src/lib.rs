#[macro_export]
macro_rules! avec {
    () => {{
        Vec::new()
    }};
    ($($element:expr),* $(,)?) => {{
        let mut vs = Vec::new();
        $(
        vs.push($element);
        )+

        vs
    }};
}

#[test]
fn test_empty() {
    let x: Vec<u32> = avec!();
    assert!(x.is_empty());
}

#[test]
fn test_single_element() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn test_double() {
    let x: Vec<u32> = avec![4, 2,];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 4);
    assert_eq!(x[1], 2);
}
