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
        let mut new_node = Node::new(value);
        match self.head.take() {
            Some(old_head) => new_node.borrow_mut().next = Some(old_head),
            None => self.tail = Some(new_node.clone()),
        }
        self.head = Some(new_node.clone());
        self.length += 1;
    }
    pub fn insert_last(&mut self, value: String) {
        let mut new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => old_tail.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }
        self.tail = Some(new_node);
        self.length += 1;
    }

    pub fn insert_pos(&mut self, value: String, pos: u64) {
        if pos == 0 {
            return self.insert_begin(value);
        }
        if pos >= self.length {
            return self.insert_last(value);
        }
        let mut new_node = Node::new(value);
        let mut index = 0;
        let mut current = self.head.clone();
        let mut prev: Option<Rc<RefCell<Node>>> = None;

        while let Some(curr) = current {
            if index == pos {
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

    fn delete_pos(&mut self, pos: u64) {
        if pos > self.length {
            println!("Invalid postion");
            return;
        }
        let mut current = self.head.clone();
        let mut prev: Option<Rc<RefCell<Node>>> = None;
        let mut index = 0;
        while let Some(curr) = current {
            if index == pos {
                let next = curr.borrow_mut().next.take();
                if let Some(ref p) = prev {
                    p.borrow_mut().next = next.clone();
                }
                if next.is_none() {
                    self.tail = prev.clone()
                }
                self.length -= 1;
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
            print!("{}->", node.borrow().value);
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
        self.head = prev;
    }
    pub fn search(&mut self, value: String) -> bool {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().value == value {
                return true;
            }
            current = node.borrow().next.clone();
        }
        return false;
    }

    fn update(&self, pos: u64, new_value: String) {
        if pos >= self.length {
            println!("Invalid postion");
            return;
        }
        let mut current = self.head.clone();
        let mut index = 0;

        while let Some(node) = current {
            if index == pos {
                node.borrow_mut().value = new_value;
                return;
            }
            current = node.borrow().next.clone();
            index += 1;
        }
    }
    pub fn has_cycle(&self) -> bool {
        let mut slow = self.head.clone();
        let mut fast = self.head.clone();

        loop {
            let Some(s) = slow.clone() else {
                return false;
            };
            let Some(f) = fast.clone() else {
                return false;
            };

            let slow_next = s.borrow().next.clone();
            let fast_next1 = f.borrow().next.clone();
            let Some(f1) = fast_next1 else {
                return false;
            };
            let fast_next2 = f1.borrow().next.clone();
            slow = slow_next;
            fast = fast_next2;
            if let (Some(ref s2), Some(ref f2)) = (&slow, &fast) {
                if Rc::ptr_eq(s2, f2) {
                    return true;
                }
            } else {
                return false;
            }
        }
    }
}
use std::io::{self, BufRead};
fn main() {
    let mut ll = SinglyLinkedList::new_empty();
    loop {
        println!("1. Insert Node at beginning");
        println!("2. Insert Node at lst");
        println!("3. Insert Node at position");
        println!("4. Delete a node");
        println!("5. Reverse the linked list");
        println!("6. Search for an element");
        println!("7. update node value at a position");
        println!("8. Display linked list ");
        println!("9. Detect cycles");
        println!("10. Exit");

        let mut line = String::new();
        let stdin = io::stdin();
        let mut input = stdin.lock();
        input.read_line(&mut line).expect("failed to read choice");
        let choice: u32 = line.trim().parse().expect("Invalid choice ");
        match choice {
            1 => {
                println!("Enter value to insert at the beginning");
                let mut value = String::new();
                input.read_line(&mut value).expect("Failed to read input");
                let val: String = value.trim().parse().expect("Invalid input");
                ll.insert_begin(val);
            }
            2 => {
                println!("Enter value to insert at the last");
                let mut value = String::new();
                input.read_line(&mut value).expect("Failed to read input");
                let val: String = value.trim().parse().expect("Invalid input");
                ll.insert_last(val);
            }
            3 => {
                println!("Enter value and postion to insert ");
                let mut value = String::new();
                input.read_line(&mut value).expect("Failed to read input");
                let val: String = value.trim().parse().expect("Invalid input");
                let mut position = String::new();
                input
                    .read_line(&mut position)
                    .expect("Failed to read the position");
                let pos: u64 = position.trim().parse().unwrap();
                ll.insert_pos(val, pos);
            }
            4 => {
                println!("Enter position of node to delete");
                let mut value = String::new();
                input.read_line(&mut value).expect("Failed to read input");
                let pos: u64 = value.trim().parse().expect("Invalid input");
                ll.delete_pos(pos);
            }
            5 => {
                ll.reverse();
                println!("List reversed");
            }
            6 => {
                println!("Enter value to search");
                let mut value = String::new();
                input.read_line(&mut value).expect("Failed to read input");
                let val: String = value.trim().parse().expect("Invalid input");
                if ll.search(val) {
                    println!("Value found");
                } else {
                    println!("value not found");
                }
            }
            7 => {
                println!("Enter the postion to update :");
                let mut position = String::new();
                input
                    .read_line(&mut position)
                    .expect("Failed to read input");
                let pos: u64 = position.trim().parse().expect("Invalid input");
                println!("Enter the new value:");
                let mut value = String::new();
                input
                    .read_line(&mut value)
                    .expect("Failed to read the value");
                let val: String = value.trim().to_string();
                ll.update(pos, val);
            }
            8 => ll.display(),
            9 => {
                if ll.has_cycle() {
                    println!("Cycle detected!");
                } else {
                    println!("No cycle");
                }
            }
            10 => break,
            _ => {
                println!("Invalid Choice ")
            }
        }
    }
}
