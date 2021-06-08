## Lifetime


[WIP] Lifetimes, is what rust compiler uses to track the reference and its usage. All references must have some lifetime ('X) associated with it, which will tell the compiler that the given reference lives as long as 'X.

This could be scoped to a block such as function, or `{}` or even struct member whose lifetime is setup when the struct members are assigned,


```
struct Person <'a>{
    thatPerson: &'a str, 
}
```


here the thatPerson references to some person name, we dont know how long the name lives or even the person:p. So we need to some lifetime associated with it. The referenced data must least for the lifetime of struct Person.


### what is std::mem::replace doing in MutableIterator::next()?

if we do something like this,

```
  let (first, rest) = self.slice.split_first_mut()?;
```

here, we end up creating a `first` reference of type `&'next mut T` from `&'iter mut T`  which is of conflicting type due to difference in lifetime. 

It means we would allow one to get multiple mutable reference to item violating rust rules on mutable reference, so rust catches it and start throwing compilation error.

To play around this restrictions, we need to temporarily disconnect the `self.slice` and tell the compiler that its safe.

so we re borrow here or more like double pointer
```
let slice = &mut self.slice;
```

and set the `self.slice` to some empty slice, while doing so we also need the original slice. thats where `std:mem:replace` comes handy,
```
 let slice = replace(slice, &mut []);
 ```
> note: here slice is shadowed.

Read more
1. [Why cant I take a mutable reference to a struct member safely ?](https://stackoverflow.com/questions/25730586/how-can-i-create-my-own-data-structure-with-an-iterator-that-returns-mutable-ref)


