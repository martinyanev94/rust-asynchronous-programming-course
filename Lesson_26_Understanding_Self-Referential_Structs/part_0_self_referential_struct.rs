struct SelfReferential<'a> {
    data: String,
    reference: &'a str,
}

fn main() {
    let example = SelfReferential {
        data: String::from("Hello, Rust!"),
        // This will fail because the reference cannot outlive the 'data' field
        reference: &example.data,
    };
}
