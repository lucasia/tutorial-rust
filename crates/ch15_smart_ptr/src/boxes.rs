use log::debug;

use List::Cons;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn boxes() {

    debug!("====== box example ======");

    let b = Box::new(42);
    debug!("b = {b}");


    debug!("====== cons list ======");
    cons_list();
}

fn cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(List::Nil))))));

    debug!("Cons list = {:?}", list);
    let mut current_node = &list;

    while let Cons(value, next_box) = current_node {
        debug!("Cons value = {value}");

        current_node = next_box;
    }

    debug!("finished iterating through list");
}
