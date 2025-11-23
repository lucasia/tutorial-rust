use log::debug;

mod boxes;
mod derefs;
mod drops;
mod ref_count;

fn main() {
    env_logger::init();

    debug!("====== boxes ======");
    boxes::boxes();

    debug!("====== derefs ======");
    derefs::derefs();

    debug!("====== drops ======");
    drops::drops();

    debug!("====== ref counting ======");
    ref_count::ref_count();

    // TODO - 15.5 RefCell<T> https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
}
