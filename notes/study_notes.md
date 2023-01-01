# Problems faced building a Tree in Rust

## Lifetimes in Loops

A value created in a loop is dropped if not saved in a collection outside of the loop.


## Stack/Heap Model, Shared Ownership and Recursive Structures


Rust has reference containers for shared ownership:
- Box<T> is for single ownership.
- Rc<T> is for multiple ownership.
- Arc<T> is for multiple ownership, but threadsafe.
- Cell<T> is for "interior mutability" for Copy types; that is, when you need to mutate something behind a &T.

One way to look at it is to consider what each one allows you to do that you can't do to other kinds of values.

[Box](https://doc.rust-lang.org/std/boxed/struct.Box.html) is an owning reference. It allows you to have a sized, moveable proxy for a value that can be unsized or either impossible or impractical to move.

[Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) is like Box except that it allows multiple owners, but does not allow any of the owners to obtain unique access to the value (as a &mut T or by moving the value out) unless there are currently no other owners.

[Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html) allows modifying the value inside of it, even through a shared reference, but only by replacing the entire thing at once. That requires the value to implement the Copy trait. However, you cannot make a shared or unique reference (&T or &mut T) to the value contained within the cell.

[RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) is like Cell but instead of requiring the value to be Copy, it instead checks whenever you try to get a reference and only allows you to get a unique reference (&mut T) if there are currently no other ones.

[Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) is like Rc except that it can be safely used on multiple threads. Mutex and RwLock are like RefCell except that they can be safely used on multiple threads.

Rc and RefCell (and Arc and Mutex) are often combined to both allow the value to have multiple owners and allow each of the owners to modify the value.

> **Sources:**
> 
> [Confused between Box, Rc, Cell, Arc](https://users.rust-lang.org/t/confused-between-box-rc-cell-arc/10946)
> [r/learnrust - RefCells, Cell, Rc, and Box? What are these?](https://www.reddit.com/r/learnrust/comments/czu9h4/refcells_cell_rc_and_box_what_are_these/)
> [Wrapper Types in Rust: Choosing Your Guarantees - In Pursuit of Laziness](https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/)
> [The Stack and the Heap - The Rust Programming Language](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html)
> [What are Smart Pointers and their Types in Rust??](https://blog.knoldus.com/smart-pointers-box-rc-ref-and-refmut-in-rust/)


___

# Accessing mutable references in collections

## Why this is hard in a Vec and not in a Struct
One important distinction between struct access and Vec access is that, when you index mutably into a Vec, it's just syntactic sugar for calling the index_mut method from the IndexMut trait:

pub fn index_mut(&mut self, index: I) -> &mut <Vec<T, A> as Index<I>>::Output
Note that self is borrowed mutably, and in this case self is your Vec. So the whole Vec will be mutably borrowed for the lifetime of that borrow, which will be the lifetime of the variable that stores the return value.

This is different from accessing a struct member, which is a language-level feature.


Sometimes a reasonable usage of "unsafe" code. Usually through [Raw Pointers](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/raw-pointers.html) or other Rusts' containers like Cell, RefCell, etc.

> *[r/rust - Comment by u/javajunkie314 on ”When I have a mutable reference of a vector&#x27;s element, I cannot do anything to all other elements of the same vector, is this true?”](https://www.reddit.com/r/rust/comments/p60z3e/comment/h9caby0/?utm_source=share&utm_medium=web2x&context=3)


[**Vec docs**](https://doc.rust-lang.org/std/vec/struct.Vec.html#)

## [Simultaneous mutable access to arbitrary indices of a large vector that are guaranteed to be disjoint](https://stackoverflow.com/questions/55939552/simultaneous-mutable-access-to-arbitrary-indices-of-a-large-vector-that-are-guar)
When the compiler can't enforce that mutable references to a slice elements aren't exclusive, [Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html) is pretty nice.

You can transform a &mut [T] into a &Cell<[T]> using Cell::from_mut, and then a &Cell<[T]> into a &[Cell<T>] using Cell::as_slice_of_cells. All of this is zero-cost: It's just there to guide the type-system.

A &[Cell<T>] is like a &[mut T], if that were possible to write: A shared reference to a slice of mutable elements. What you can do with Cells is limited to read or replace — you can't get a reference, mutable or not, to the wrapped elements themselves. Rust also knows that Cell isn't thread-safe (it does not implement Sync). This guarantees that everything is safe, at no dynamic cost.

```rs
fn main() {
    use std::cell::Cell;

    let slice: &mut [i32] = &mut [1, 2, 3];
    let cell_slice: &Cell<[i32]> = Cell::from_mut(slice);
    let slice_cell: &[Cell<i32>] = cell_slice.as_slice_of_cells();
    
    let two = &slice_cell[1];
    let another_two = &slice_cell[1];

    println!("This is 2: {:?}", two);
    println!("This is also 2: {:?}", another_two);
    
    two.set(42);
    println!("This is now 42!: {:?}", another_two);
}
```

Other options for simultaneous access:
- [split_first_mut()](https://doc.rust-lang.org/std/primitive.slice.html#method.split_first_mut)
- [split_array_mut()](https://doc.rust-lang.org/std/primitive.slice.html#method.split_array_mut)
- [split_at_mut()](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut)
- [iter_mut()](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- ...

## Resources
[r/rust - Comment by u/RobertJacobson on ”Patterns to avoid borrowing mutable self more than once? (beyond RcRefcell and inner objects)”](https://www.reddit.com/r/rust/comments/hv2zqo/comment/fyr6k60/?utm_source=share&utm_medium=web2x&context=3)
[How can I change fields of elements in vectors?](https://stackoverflow.com/questions/43550632/how-can-i-change-fields-of-elements-in-vectors)
[r/rust - How do I modify vector elements through mutable references?](https://www.reddit.com/r/rust/comments/ddd7qm/how_do_i_modify_vector_elements_through_mutable/)
[Unsafe rust and the borrow checker (multiple mutable borrows)](https://users.rust-lang.org/t/unsafe-rust-and-the-borrow-checker-multiple-mutable-borrows/63293/22)
[How to get mutable references to two array elements at the same time?](https://stackoverflow.com/questions/30073684/how-to-get-mutable-references-to-two-array-elements-at-the-same-time)

[Mutable References on Vectors vs. Structs: Some less known techniques.](https://applied-math-coding.medium.com/mutable-references-on-vectors-vs-structs-some-less-known-techniques-87098e2e2ba2)

# Other Readings
- [Dyn async traits, part 8: the soul of Rust](http://smallcultfollowing.com/babysteps/blog/2022/09/18/dyn-async-traits-part-8-the-soul-of-rust/)