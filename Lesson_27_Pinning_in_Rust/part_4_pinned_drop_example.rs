use std::pin::Pin;

struct MyDropStruct {
    value: String,
}

impl Drop for MyDropStruct {
    fn drop(&mut self) {
        println!("Dropping: {}", self.value);
    }
}

fn main() {
    let my_value = MyDropStruct {
        value: String::from("Drop this value safely!"),
    };

    let pinned_value = Box::pin(my_value);
    // Pinned value will be dropped automatically when it goes out of scope
}
