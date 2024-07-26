use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::default::Default;

/// The Receiver trait implements a function that is called when a Sender it is connected to uses emit()
pub trait Receiver<T> {
    fn on_received(&mut self, params: T);
}

// Define the Sender struct with a generic tuple
///Create a sender that listeners can register to

pub struct Sender<T> {
    list: Vec<Weak<RefCell<dyn Receiver<T>>>>,
}

impl<T> Default for Sender<T> {
    fn default() -> Self {
        Sender {
            list: Vec::new(), // Initialize with an empty list
        }
    }
}

impl<T> Sender<T> {
    // Emit method with multiple parameters packed into a tuple
    pub fn emit(&mut self, params: T) 
    where
        T: Clone, // Ensure the parameter type implements Clone if needed
    {
        // Collect indices of expired weak references
        let mut to_remove = Vec::new();
        
        for (i, weak_receiver) in self.list.iter().enumerate() {
            if let Some(receiver) = weak_receiver.upgrade() {
                let mut receiver = receiver.borrow_mut();
                receiver.on_received(params.clone()); // Pass a cloned parameter
            } else {
                // Collect indices of expired weak references
                to_remove.push(i);
            }
        }

        // Remove expired references
        for &index in to_remove.iter().rev() {
            self.list.remove(index);
        }
    }

    // Register a new receiver
    pub fn register(&mut self, receiver: Rc<RefCell<dyn Receiver<T>>>) {
        self.list.push(Rc::downgrade(&receiver));
    }

    // Unregister a receiver
    pub fn unregister(&mut self, receiver: Rc<RefCell<dyn Receiver<T>>>) {
        self.list.retain(|r| {
            r.upgrade().map_or(false, |r| !Rc::ptr_eq(&r, &receiver))
        });
    }
}