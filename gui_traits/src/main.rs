// This code cannot run!!

// If someone using our library decides to implement a SelectBox struct
// that has width, height, and options fields,
// they implement the Draw trait on the SelectBox type as well

use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// SelectBox implements the Draw trait,
// which means it implements the draw method
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        // add a SelectBox and a Button by putting each
        //  in a Box<T> to become a trait object
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// Rust uses the pointers inside the trait object to know which method to call.
// There is a runtime cost when this lookup happens that doesn’t occur with static dispatch.
// Dynamic dispatch also prevents the compiler from choosing to inline a method’s code,
// which in turn prevents some optimizations. However, we did get extra flexibility in the code,
// so it’s a trade-off to consider