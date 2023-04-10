// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html

enum WebEvent {
    // plain enum
    PageLoad,
    PageUnload,
    // tuple-like structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),

        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),

        // Destructure `Click` into `x` and `y`.
        WebEvent::Click {x: 0, y} => {
            println!("clicked at horizontal level 0!, y={y}")
        }
        WebEvent::Click { x, y } => {
            println!("clicked at x={x}, y={y}.");
        },
    }
}


// https://doc.rust-lang.org/std/keyword.ref.html#-vs-ref
impl WebEvent {
    fn inspect(&self) {
        match *self {
            Self::PageLoad => println!("page loaded - enum method"),
            Self::PageUnload => println!("page unloaded - enum method"),

            // Destructure `c` from inside the `enum` variant.
            Self::KeyPress(c) => println!("pressed '{}' - enum method", c),
            Self::Paste(ref s) => println!("pasted \"{}\" - enum method", s),

            // Destructure `Click` into `x` and `y`.
            Self::Click {x: 0, y} => {
                println!("clicked at horizontal level 0!, y={y}")
            }
            Self::Click { x, y } => {
                println!("clicked at x={x}, y={y} - enum method");
            },
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let click0   = WebEvent::Click { x: 0, y: 90 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(click0);
    inspect(load);
    inspect(unload);

    // pressed.inspect();
    // pasted.inspect();
    // click.inspect();
    // click0.inspect();
    // load.inspect();
    // unload.inspect();
}
