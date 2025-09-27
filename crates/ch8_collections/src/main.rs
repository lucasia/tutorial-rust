mod hash_maps;
mod strings;
mod vectors;

fn main() {
    env_logger::init();

    vectors::vectors();
    strings::strings();
    hash_maps::hash_maps();
}
