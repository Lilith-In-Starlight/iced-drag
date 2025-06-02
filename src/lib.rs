#![warn(clippy::pedantic)]
#![doc = include_str!("../README.md")]
use std::{any::Any, cell::RefCell};

pub mod drag;
pub mod dropzone;

pub use drag::drag;
pub use dropzone::drop_zone;

// The global, internally mutable state of the drag and drop system.
#[derive(Debug, Default)]
pub struct DragAndDrop {
    // The payload being dragged, if any.
    pub dragging: RefCell<Option<Box<dyn Any>>>,
}

impl DragAndDrop {
    // Sets the payload to something.
    pub fn set_to<T: 'static>(&self, to: T) {
        self.dragging.replace(Some(Box::new(to)));
    }

    // Clears the payload, setting it to None.
    pub fn clear(&self) {
        self.dragging.replace(None);
    }

    // Checks if the user is dragging something.
    pub fn has_some(&self) -> bool {
        self.dragging.borrow().is_some()
    }
}
