use List::Cons;
use log::debug;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// create two lists `b` and `c` that share ownership of a third list, `a`
// b --> |3|=-----\
//          a --> |5|=-> |10|=-> |Nil|
// c --> |4|=-----/
pub fn ref_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(List::Nil)))));
    debug!("Count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    debug!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        debug!("Count after creating c = {}", Rc::strong_count(&a));
    }

    debug!("Count after c goes out of scope = {}", Rc::strong_count(&a));

    print_list(a);
}

fn print_list(list: Rc<List>) {
    // Start with the initial Rc<List>
    let mut current_rc = list;

    // Use a loop that matches the List content *inside* the Rc
    while let Cons(value, next_rc) = &*current_rc {
        // 1. &*current_rc dereferences the Rc<List> to get a reference to List (&List)
        // 2. The pattern match extracts value (i32) and next_rc (&Rc<List>)

        debug!("Cons value = {value}");

        // 3. Clone the inner Rc to get the next owned Rc<List> for the loop
        //    This increments the reference count for the next node's list element.
        current_rc = Rc::clone(next_rc);
    }
}
