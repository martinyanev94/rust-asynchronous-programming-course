use std::pin::Pin;

struct SelfReferential {
    data: String,
    reference: *const String, // Raw pointer to data, no guarantees here
}

impl SelfReferential {
    fn new(data: String) -> SelfReferential {
        let reference = &data as *const String; // Create initial raw pointer
        SelfReferential { data, reference }
    }

    fn get_reference(&self) -> &String {
        unsafe { &*self.reference } // Dereference the raw pointer
    }
}

fn main() {
    let data = String::from("Pinning in Rust");
    let self_ref = SelfReferential::new(data);

    println!("Data: {}", self_ref.get_reference());
}
