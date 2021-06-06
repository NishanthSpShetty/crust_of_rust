#[macro_export]
macro_rules! avec {
    ($($element:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::new();
        $( vs.push($element);)*

        vs
    }};

    ($element:expr; $count:expr) => {{
        let _temp = $element;
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        vs.resize(count,_temp.clone());
//        for i in 0..count{
//            //vs.push(_temp.clone());
//            vs[i]= _temp.clone();
//        }
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

#[test]
fn test_repeats() {
    // avec![element; count]
    let x: Vec<u32> = avec![4; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 4);
    assert_eq!(x[1], 4);
}
#[test]

fn test_repeats_non_literal() {
    let mut y = Some(String::from("hello"));
    // avec![element; count]
    let x: Vec<String> = avec![y.take().unwrap(); 8+1];

    assert!(!x.is_empty());
    assert_eq!(x.len(), 9);
    assert_eq!(x[0], String::from("hello"));
    assert_eq!(x[1], String::from("hello"));
}
