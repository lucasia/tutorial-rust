#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::ref_cell::List::{Cons, Nil};
use log::debug;
use std::cell::RefCell;
use std::rc::Rc;

/*
 * Borrowing rules:
 *  - At any given time, you can have either one mutable reference or any number of immutable references (but not both).
 *  - References must always be valid.
 *
 * With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
 * With RefCell<T>, these invariants are enforced at runtime.
 * With references, if you break these rules, you’ll get a compiler error.
 * With RefCell<T>, if you break these rules, your program will panic and exit.
 */
pub fn ref_cell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    debug!("a after = {:?}", a);
    debug!("b after = {:?}", b);
    debug!("c after = {:?}", c);

    print_list(&c);
}

fn print_list(c: &List) {
    let mut current_node = c;
    while let Cons(i, next_box) = current_node {
        debug!("RefCell Cons value = {:?}", i.borrow());

        current_node = next_box;
    }

    debug!("finished iterating through list");
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of the quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of the quota!");
            }
        }
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
