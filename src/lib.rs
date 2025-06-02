#![warn(clippy::pedantic)]

use std::{any::Any, cell::RefCell};

pub mod drag;
pub mod dropzone;

pub use drag::drag;
pub use dropzone::drop_zone;

#[derive(Debug, Default)]
pub struct DragAndDrop {
    pub dragging: RefCell<Option<Box<dyn Any>>>,
}

impl DragAndDrop {
    pub fn set_to<T: 'static>(&self, to: T) {
        self.dragging.replace(Some(Box::new(to)));
    }

    pub fn clear(&self) {
        self.dragging.replace(None);
    }

    pub fn has_some(&self) -> bool {
        self.dragging.borrow().is_some()
    }
}
