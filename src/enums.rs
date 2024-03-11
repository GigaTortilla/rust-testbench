#![allow(dead_code)]
use crate::enums::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => tail.len() + 1,
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{} {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

enum WebEvent {
    PageLoad,
    PageUnload,
    // tuple
    KeyPress(char),
    Paste(String),
    // struct
    Click { x: f64, y: f64 },
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// type alias
type Op = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            // Self = type alias for current enum
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed: '{c}'"),
        WebEvent::Paste(str) => println!("Pasted: \"{str}\""),
        WebEvent::Click { x, y } => println!("Clicked at x={x:.2}, y={y:.2}"),
    }
}

pub fn test() {
    println!("enums:");
    let pressed = WebEvent::KeyPress('x');
    // "to_owned()" creates an owned "String" from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 15.0, y: 34.0 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    let op = Op::Add;
    println!("15 + 32 = {}", op.run(15, 32));

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Linked list has length {}", list.len());
    println!("String representation of the linked list:\n{}", list.stringify());

    print_end!();
}