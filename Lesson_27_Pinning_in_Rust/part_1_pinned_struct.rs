use std::pin::Pin;

struct MyStruct {
    value: i32,
}

fn main() {
    let my_value = MyStruct { value: 10 };

    let pinned_value = Box::pin(my_value);
    println!("Value: {}", pinned_value.as_ref().value);
}
