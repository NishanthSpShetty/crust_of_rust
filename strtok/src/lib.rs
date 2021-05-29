/// Subtype
///
///     should be able to assign the value with lifetime of some 'X to the vars with lifetime
///     'Y, ('Y ='X)
///     then we can say  'X is subtype of 'Y.
///         'X: 'Y
///     ex:
///         'statisc is subtype 'a
///         'static:'a
//
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
///        // here T= 'static, U= 'a
///        //'static: 'a
///        y = b;
///
/// lifetime variance
/// Covariance
///
/// fn foo(&'a str){} //not generic type, we cant define like this. fn foo(&str)
///
///   the above function takes 'a value, we can call the function with
///   any value which are subtype
///
///   ex:
///     foo(&'a str)
///     foo(&'static)
///   //lets say you are assigning the values of following lieftime to x,
///   //these comply covariance
///   x = &'a str
///   x = &'static str
///  
/// Contravariant
///     fn caller(f: Fn(&'a str)->()){
///        f("" /* passing 'a value here */
///     }
///
///     // this one will fail
///     caller(fn(some'static str){})
///
///     //here we are passing caller a function which expects the value which is subtype of
///     'static, where caller calls wth 'a, which is not subtype of 'static is not
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
