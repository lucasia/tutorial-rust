use log::debug;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        debug!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

pub fn drops() {
    debug!("drop it like it's hot...");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    CustomSmartPointer {
        data: String::from("other stuff"),
    };

    debug!("CustomSmartPointer created.");
    drop(c); // use std::mem::drop.  c.drop() would lead to double drop
    debug!("CustomSmartPointer dropped before the end of the function.");
}
