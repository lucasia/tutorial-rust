mod vectors;
mod string;
mod hashmap;

// see: https://doc.rust-lang.org/std/collections/index.html
// vector - allows you to store a variable number of values next to each other.
// string - collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
// hash map - allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.
fn main() {
    hashmap::hashmap_main();

    // vectors::vector_main();
    // string::string_main();
}

