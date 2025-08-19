use std::rc::Rc;
use std::cell::RefCell;

struct SelfReferential {
    data: Rc<RefCell<String>>,
    reference: String,
}

impl SelfReferential {
    fn new(data: String) -> SelfReferential {
        let rc_data = Rc::new(RefCell::new(data));
        let reference = rc_data.borrow().clone(); // or any other operation on the borrowed data
        SelfReferential { data: rc_data, reference }
    }
}

fn main() {
    let self_ref = SelfReferential::new(String::from("Hello, Rust!"));
    println!("Data: {}", self_ref.data.borrow());
    println!("Reference: {}", self_ref.reference);
}
