use std::mem::replace;

struct MyIterator<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    type Item = &'a T;
    fn next<'n>(&'n mut self) -> Option<Self::Item> {
        let (first, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(first)
    }
}

#[warn(dead_code)]
struct MutableIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MutableIterator<'iter, T> {
    type Item = &'iter mut T;
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        //borrow the slice mutably again
        let slice = &mut self.slice;

        //replace the self.slice with empty slice, so above slice gains full access and return the
        //old slice back to us.
        let slice = replace(slice, &mut []);

        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SomeType<'a, T> {
        field: &'a mut [T],
    }

    fn set_some_type<'a, T>(some_type: &'a mut SomeType<'a, T>) -> Option<&'a mut T> {
        let p = some_type.field.as_mut();

        //        let old = replace(p, &mut []);
        let (first, rest) = p.split_first_mut()?;
        some_type.field = rest;

        Some(first)
    }

    #[test]
    fn it_works() {
        let mut collection: Vec<i32> = vec![1, 2, 3, 4];
        let my_iter = MyIterator {
            slice: &collection[..],
        };

        for (i, ele) in my_iter.enumerate() {
            assert_eq!(*ele, collection[i]);
        }
        //mutable borrow
        let mut_iter = MutableIterator {
            slice: &mut collection[..],
        };

        for (_i, ele) in mut_iter.enumerate() {
            *ele = *ele + 1;
        }
        assert_eq!(2, collection[0]);

        let mut r = vec![11];
        let mut st = SomeType { field: &mut r };
        let x = &mut st;
        set_some_type(x);
    }
}
