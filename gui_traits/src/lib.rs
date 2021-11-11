pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}


// Each of the types we want to draw on the screen will implement the Draw trait 
// but will use different code in the draw method to define how to draw that particular type
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
    // The Button type, for instance, might have an additional impl block containing methods
    // related to what happens when a user clicks the button.
    // These kinds of methods won’t apply to types like TextField.
}