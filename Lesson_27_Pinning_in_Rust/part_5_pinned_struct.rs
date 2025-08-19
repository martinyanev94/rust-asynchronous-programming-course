use std::pin::Pin;

struct UnpinnedStruct {
    data: i32,
}

fn main() {
    let unpinned = UnpinnedStruct { data: 5 };

    // Pin the value
    let pinned = Pin::new(Box::new(unpinned));

    // Now we safely use the pinned value
    println!("Pinned data: {}", pinned.data);
}
