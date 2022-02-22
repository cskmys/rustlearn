/*
This library tracks a value against a maximum value and sends messages based on how close the current value is to the maximum value.
The user of this library is expected to provide the mechanism for sending the messages(app notification, email, sms etc).
*/

pub trait Messenger { // the library doesn't know the medium of msg, all it needs is something that implements this trait
    fn send(&self, msg: &str); // immutable reference to "self" and "msg" is taken,  which makes sense coz
    // the library doesn't need to modify the object that implements the trait itself and
    // the message string output by the library should not be modified by any other code
} // for unit testing this needs to be mocked, this mock needs to keep track of the messages that it's told to send

pub struct LimitTracker<'a, T: Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker{
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize){ // for unit testing this doesn't return anything for us to make assertions
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Err: You are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    struct MockMessenger {
        sent_messages: Vec<String>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{
                sent_messages: vec![],
            }
        }
    }
    
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.push(String::from(msg)); // throws compilation error
            // by doing "push" on "sent_messages" you are mutating "self" which is borrowed here immutably
            // it is impossible to mutate using immutable borrow
        }
    }
    */
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>> // storing the "Vec<String>" inside a "RefCell"
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]) // "RefCell::new" is used to create initial state of "Vec<String>"
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg)); // "RefCell::borrow_mut" gives a mutable reference which allows us to mutate "self" despite it being borrowed immutably
            /*
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut(); // causes "RefCell" to call "panic!"
            one_borrow.push(String::from(msg));
            two_borrow.push(String::from(msg));
            // at any given point in time, in a given scope there can be only one mutable reference
            // this is violated by creation of 2nd one and hence, "RefCell" panics
            */
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_msg(){
        let mock_msgr = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_msgr, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_msgr.sent_messages.borrow().len(), 1);
    }
}