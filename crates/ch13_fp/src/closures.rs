use std::thread;
use log::{debug, info};

#[derive(Debug)]
struct Square {
    side: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Blue,
    Green,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_blue = 0;
        let mut num_green = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Green => num_green += 1,
            }
        }

        if num_blue > num_green {
            ShirtColor::Blue
        } else {
            ShirtColor::Green
        }
    }
}

pub fn closures() {
    debug!("====== borrow example ======");
    borrow_example();

    debug!("====== mutable example ======");
    mutable_example();

    debug!("====== move example ======");
    move_example();

    debug!("====== FnMut example ======");
    fn_mut_example();

    debug!("====== giveaway example ====== ");
    giveaway_example();
}


pub fn fn_mut_example() {
    let mut list = [
        Square { side: 10 },
        Square { side: 5 },
        Square { side: 1 },
    ];

    list.sort_by_key(|r| r.side);
    debug!("{:?}", list);

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.side
    });
    println!("List sorted in {num_sort_operations} operations");
}

pub fn borrow_example() {
    let list = vec![1,2,3];
    debug!("Before defining closure {:?}", list);
    let only_borrows = || debug!("From closure {list:?}");

    debug!("Before calling closure {:?}", list);
    only_borrows();
    debug!("After calling closure {:?}", list);
}

pub fn mutable_example() {
    let mut list = vec![1,2,3];
    debug!("Before defining closure {:?}", list);

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();

    debug!("After calling closure {:?}", list);
}

pub fn move_example() {
    let list = vec![1, 2, 3];
    debug!("Before defining closure {:?}", list);

    thread::spawn(move || debug!("From thread {list:?}"))
        .join()
        .unwrap()
}

pub fn giveaway_example() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Green, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Green);
    let giveaway1 = store.giveaway(user_pref1);
    info!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);


    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    info!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

}