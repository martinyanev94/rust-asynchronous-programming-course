struct LinkedStruct {
    data: Rc<RefCell<String>>,
    next: Option<Rc<RefCell<LinkedStruct>>>,
}

impl LinkedStruct {
    fn new(data: String) -> Self {
        LinkedStruct {
            data: Rc::new(RefCell::new(data)),
            next: None,
        }
    }

    fn append(&mut self, data: String) {
        let new_node = Rc::new(RefCell::new(LinkedStruct::new(data)));
        self.next = Some(new_node);
    }

    fn print(&self) {
        println!("Data: {}", self.data.borrow());
        if let Some(ref next_node) = self.next {
            next_node.borrow().print();
        }
    }
}

fn main() {
    let mut head = LinkedStruct::new(String::from("First node"));
    head.append(String::from("Second node"));
    head.append(String::from("Third node"));
    
    head.print();
}
