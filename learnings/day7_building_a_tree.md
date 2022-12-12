# Problems faced building a Tree in Rust

## Lifetimes in Loops

A value created in a loop is dropped if not saved in a collection outside of the loop.



## Stack/Heap Model and Shared Ownership



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