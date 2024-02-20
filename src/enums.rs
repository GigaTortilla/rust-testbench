enum WebEvent {
    PageLoad,
    PageUnload,
    // tuple
    KeyPress(char),
    Paste(String),
    // struct
    Click { x: f64, y: f64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed: '{c}'"),
        WebEvent::Paste(str) => println!("Pasted: \"{str}\""),
        WebEvent::Click { x, y } => println!("Clicked at x={x}, y={y}"),
    }
}

pub fn test() {
    let pressed = WebEvent::KeyPress('x');
    // "to_owned()" creates an owned "String" from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 15.0, y: 34.0 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    
    println!("{:-<1$}", "", 40);
}