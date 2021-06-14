### pointers 

Smart pointers and interior mutability



### Interior mutability
   
   Pattern which allows rust to mutate data even when there are immutable references to it.  UnsafeCell facilitate such pattern, which is exposed in standard library via safe API's of RefCell.

---

### Exercise 
* Unsafe cell accross thread boundaries [fad0dd](https://github.com/NishanthSpShetty/crust_of_rust/commit/fad0dd00574c32518a63def94a14a06a57181ed3).

    In this commit, We create unsafe Sync implementation for Cell<T> which will allow us to share the cell 
    accross thread boundaries which can violate the rust principles and mutate data unsafely.


* Unsafe cell with mutable reference to data. [770b1e](https://github.com/NishanthSpShetty/crust_of_rust/commit/770b1e45bbd537a8fecb0ea2c812236256d68132).
    
    In this commit we change the get function to return the mutable refernce to data ands we can see that we have 2 mutable references.
    Though the references are supposed to point to 2 different String, allocator might not have reclaimed the location and used the same pointer for new String we craeted.
