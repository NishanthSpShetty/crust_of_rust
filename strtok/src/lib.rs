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
///    //function which takes 'a mutable reference to some 'b as below
///    fn foo(s: &'a mut &'b str, x:&'b str) {
///         *s = x;
///    }
///    
///    //type should match exactly in this case for 'b.
///    // for &'a mut T
///    // covariant over 'a, but invariant overe T
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

    pub fn take_static_str(s: &'static str) {
        println!("{}", s);
    }

    pub fn caller<'a, F: Fn(&'a str) -> ()>(f: F, s: &'a str) {
        f("aaa"); // this will work as we pass something with lifetime static

        // below call wont work as a is bound to 'caller
        //let a:'a String = String::new();
        //f(&*a);

        f(s);
    }

    #[test]
    fn variance() {
        take_static_str("a");

        //this will fail as a is of some lifetime of 'a, but expected something which lives longer
        //than or equal to 'static
        // let a = String::new();
        //take_static_str(&*a);
    }

    #[test]
    fn contravariance() {
        let f = |a| {
            println!("{}", a);
        };
        let s = String::new();
        {
            caller(f, "a");
            caller(|a| println!("{}", a), &*s);
        }
    }

    fn foo<'a, 'b>(s: &'a mut &'b str, x: &'b str) {
        *s = x;
    }

    #[test]
    fn invariance() {
        let mut a = "static str";
        let s = String::new();
        a = &*s;
        {
            foo(&mut a, "some str");
        }
        println!("{}", a);
    }

    #[test]
    fn it_works() {
        let mut x = "hello world boss";
        let hello = strtok(&mut x, ' ');
        assert_eq!(x, "world boss");
        assert_eq!(hello, "hello");
    }
}
