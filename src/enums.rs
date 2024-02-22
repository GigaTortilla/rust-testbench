#![allow(dead_code)]

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

    print_end!();
}