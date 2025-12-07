use log::debug;

mod boxes;
mod derefs;
mod drops;
mod ref_cell;
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

    debug!("====== ref cell ======");
    ref_cell::ref_cell();
}
