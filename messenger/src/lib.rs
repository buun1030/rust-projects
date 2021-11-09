pub trait Messenger {
    fn send(&self, msg: &str);
    // sending an email or text message when we call send
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // interior mutability
        sent_messages: RefCell<Vec<String>>,
        // We can’t take the suggestion from the error text 
        // to use &mut self instead, because then the signature of send
        // wouldn’t match the signature in the Messenger trait definition
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    // then implement the Messenger trait for MockMessenger 
    // so we can give a MockMessenger to a LimitTracker
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
            // still an immutable borrow of self, which matches the trait definition
            // call borrow_mut on the RefCell<Vec<String>>
            // in self.sent_messages to get a mutable reference
            // Then we can call push on the mutable reference
            // to the vector to keep track of the messages sent during the test.
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {

        // defines a MockMessenger struct to keep track 
        // of the messages it’s told to send
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        // create a new instance of the mock object, 
        // create a LimitTracker that uses the mock object, 
        // call the set_value method on LimitTracker
        limit_tracker.set_value(80);
        // check that the mock object has the messages we expect

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        // call borrow on the RefCell<Vec<String>> to get an immutable ref to the vector
    }
    // using RefCell<T> makes it possible to write a mock object that can modify itself 
    // to keep track of the messages it has seen while you’re using it in a context 
    // where only immutable values are allowed
}