/// lifetime variance
///
/// T: U
///     can be read as : T is as useful as U.
///
///        let a = String::new();
///
///        let b: &'static str = "hello";
///
///        //create y with lifetime of some 'a
///        let mut y = &*a;
///
///        //now assign b with 'static to 'a
///        // here T= static, U= 'a
///        //'static: 'a
///        y = b;
///
/// Covariance
///
/// fn foo(&'a str){}
///
///   the above function takes 'a value, we can call the function with
///   any value which are subtype
///
///   ex:
///     foo(&'str)
///     foo(&'static)
///
///  
/// Contravariant
///
///
/// invariance
///
///
pub fn strtok<'a, 'b>(s: &'b mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[i + (delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use crate::strtok;

    #[test]
    fn it_works() {
        let mut x = "hello world boss";
        let hello = strtok(&mut x, ' ');
        assert_eq!(x, "world boss");
        assert_eq!(hello, "hello");
    }
}
