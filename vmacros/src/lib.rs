mod map;

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@u;$elelment:expr) => { () };
    ($($elelment:expr),*) => {
        <[()]>::len(&[$($crate::count![@u;$elelment]),*])
    };
}

#[macro_export]
macro_rules! avec {
    ($($element:expr),*) => {{
        let count:usize = $crate::count![$($element),*];
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(count);
        $(vs.push($element);)*
        vs
    }};


    // allow [1,]
    ($($element:expr,)*) => {{
        //call above defn
        $crate::avec![$($element),*]
    }};

    ($element:expr; $count:expr) => {{
        let _temp = $element;
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        vs.resize(count,_temp.clone());
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
