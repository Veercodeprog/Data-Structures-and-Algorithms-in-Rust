use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    value: String,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

struct SinglyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: u64,
}

impl SinglyLinkedList {
    pub fn new_empty() -> SinglyLinkedList {
        SinglyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn insert_begin(&mut self, value: String) {
        let new_node = Node::new(value);
        match self.head.take() {
            Some(old_head) => new_node.borrow_mut().next = Some(old_head),
            None => self.tail = Some(new_node.clone()),
        }
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn insert_last(&mut self, value: String) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => old_tail.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }

        self.tail = Some(new_node);
        self.length += 1;
    }
    pub fn insert_pos(&mut self, value: String, position: u64) {
        if position == 0 {
            return self.insert_begin(value);
        }

        if position >= self.length {
            return self.insert_last(value);
        }
        let new_node = Node::new(value);
        let mut current = self.head.clone();
        let mut prev: Option<Rc<RefCell<Node>>> = None;
        let mut index = 0;
        while let Some(curr) = current {
            if index == position {
                new_node.borrow_mut().next = Some(curr.clone());
                if let Some(p) = prev {
                    p.borrow_mut().next = Some(new_node.clone());
                }

                self.length += 1;
                return;
            }
            prev = Some(curr.clone());
            current = curr.borrow().next.clone();
            index += 1;
        }
    }

    pub fn delete_pos(&mut self, postion: u64) {
        if postion >= self.length {
            println!("Invalid postion!");
            return;
        }
        let mut current = self.head.clone();
        let mut prev: Option<Rc<RefCell<Node>>> = None;
        let mut index = 0;
        while let Some(curr) = current {
            if index == postion {
                let next = curr.borrow_mut().next.take();
                if let Some(ref p) = prev {
                    p.borrow_mut().next = next.clone();
                }
                if next.is_none() {
                    self.tail = prev.clone()
                }
                self.length == 1;
                return;
            }
            prev = Some(curr.clone());
            current = curr.borrow().next.clone();
            index += 1;
        }
    }
    pub fn display(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
    pub fn reverse(&mut self) {
        let mut prev: Option<Rc<RefCell<Node>>> = None;
        let mut current = self.head.clone();
        while let Some(curr) = current {
            let next = curr.borrow_mut().next.take();
            curr.borrow_mut().next = prev.clone();
            prev = Some(curr.clone());
            current = next;
        }
        self.tail = self.head.clone();
        self.head = prev
    }
    pub fn search(&self, value: String) -> bool {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().value == value {
                return true;
            }
            current = node.borrow().next.clone();
        }
        //this is the retuned value
        false
    }
    fn update(&mut self, position: u64, new_value: String) {
        if position > self.length {
            println!("Invalid postion");
            return;
        }
        let mut current = self.head.clone();
        let mut index = 0;

        while let Some(node) = current {
            if index == position {
                node.borrow_mut().value = new_value;
                return;
            }
            current = node.borrow().next.clone();
            index += 1;
        }
    }
}
use std::io;
fn main() {
    let mut ll = SinglyLinkedList::new_empty();

    loop {
        println!("1. Insert Node at beginning");
        println!("2. Insert Node at last");
        println!("3. Insert Node at postion");
        println!("4. Delete a particular Node");
        println!("5. Reverse Linked List");
        println!("6. Search Element");
        println!("7.Update Node Value");
        println!("8. Display LL");
        println!("9. Exit");

        println!("Enter your Choice");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice");
        let choice: u32 = choice.trim().parse().expect("Invalid input");
        match choice {
            1 => {
                println!("Enter value to insert at the beginning:");
                let mut value = String::new();

                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read value");
                let val: String = value.trim().parse().expect("Invalid input");
                ll.insert_begin(val);
            }
            2 => {
                println!("Enter value to insert at the end:");
                let mut value = String::new();

                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read value");
                let val: String = value.trim().parse().expect("Invalid input");
                ll.insert_last(val);
            }
            3 => {
                println!("Enter value and  position to insert :");
                let mut value = String::new();

                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read value");
                let val: String = value.trim().parse().expect("Invalid input");

                let mut position = String::new();
                io::stdin()
                    .read_line(&mut position)
                    .expect("Failed to read the position");
                let pos: u64 = position.trim().parse().expect("Invalid input position");
                ll.insert_pos(val, pos);
            }
            4 => {
                println!("Enter postion of node to delete:");
                let mut position = String::new();
                io::stdin()
                    .read_line(&mut position)
                    .expect("Failed to read the position");
                let pos: u64 = position.trim().parse().expect("Invalid input position");
                ll.delete_pos(pos);
            }
            5 => {
                ll.reverse();
                println!("List reversed")
            }
            6 => {
                println!("Enter value to search:");
                let mut value = String::new();

                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read value");
                let val: String = value.trim().parse().expect("Invalid input");

                if ll.search(val) {
                    println!("Value found.");
                } else {
                    println!("value not found");
                }
            }
            7 => {
                println!("Enter the  postion and value to update:");
                let mut value = String::new();

                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read value");
                let val: String = value.trim().parse().expect("Invalid input");

                let mut position = String::new();

                io::stdin()
                    .read_line(&mut position)
                    .expect("Failed to read position");
                let pos: u64 = value.trim().parse().expect("Invalid input");
                ll.update(pos, val);
            }
            8 => ll.display(),
            9 => break,
            _ => {
                println!("Invalid Choice");
            }
        }
    }
}

