struct CustomSmartPointer {
    data: String,
}

// The body of the drop function is where you would place any logic 
// that you wanted to run when an instance of your type goes out of scope.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
// This example gives you a visual guide to how the drop method works; 
// usually you would specify the cleanup code that your type needs to run 
// rather than a print message.

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}