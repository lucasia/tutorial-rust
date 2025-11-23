use log::debug;

mod boxes;
mod derefs;

fn main() {
    env_logger::init();

    debug!("====== boxes ======");
    boxes::boxes();

    debug!("====== derefs ======");
    derefs::derefs();
}