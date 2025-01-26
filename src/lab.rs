mod test_lab;

use std::ptr;

/// A node in the list
pub(crate) struct Node<T> {
    value: Option<T>,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

/// Struct to represent a list.
/// The list maintains 2 function pointers to help with the management of the data it is storing.
/// These functions must be provided by the user of this library.
pub struct List<T> {
    compare_to: fn(&T, &T) -> i32,
    destroy_data: fn(&T),
    head: *mut Node<T>,
    len: usize,
}

impl<T> List<T> {
    /// Create a new list with callbacks that know how to deal with the data that list is storing.
    /// # Arguments
    /// * destroy_data: Function that will free the memory for user supplied data
    /// * compare_to: Function that will compare two user data elements
    /// # Returns
    /// The newly allocated list
    pub fn new(compare_to: fn(&T, &T) -> i32, destroy_data: fn(&T)) -> Self {
        let sentinel: *mut Node<T> = Box::into_raw(Box::new(Node {
            value: None,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }));
        unsafe {
            (*sentinel).prev = sentinel;
            (*sentinel).next = sentinel;
        }
        List {
            compare_to,
            destroy_data,
            head: sentinel,
            len: 0,
        }
    }

    /// Adds data to the front of the list
    /// # Arguments
    /// * value: The data to add
    pub fn add(&mut self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: Some(value),
            prev: self.head,
            next: ptr::null_mut(),
        }));
        unsafe {
            (*new_node).next = (*self.head).next;
            (*(*self.head).next).prev = new_node;
            (*self.head).next = new_node;
        }
        self.len += 1;
    }

    /// Removes the data at the specified index.
    /// If the index is invalid, this function does nothing and returns None.
    /// # Arguments
    /// * index: The index of the data to remove
    /// # Returns
    /// The data that was removed, or None if nothing was removed
    pub fn remove_index(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        let mut current = self.head;
        for _ in 0..=index {
            unsafe {
                current = (*current).next;
            }
        }
        let value = unsafe {
            (*(*current).prev).next = (*current).next;
            (*(*current).next).prev = (*current).prev;
            // Free the memory of the node
            let value = (*current).value.take();
            let _ = Box::from_raw(current);
            value
        };
        self.len -= 1;
        value
    }

    /// Search for any occurrence of the specified value in the list.
    /// Internally, this function will call compare_to on each item in the list until a match
    /// is found or the end of the list is reached.
    /// If there are multiple copies of the value in the list, the first one will be returned.
    /// # Arguments
    /// * value: The value to search for
    /// # Returns
    /// The index of the value in the list, or None if the value is not found
    pub fn index_of(&self, value: &T) -> Option<usize> {
        let mut current = self.head;
        for i in 0..self.len {
            unsafe {
                current = (*current).next;
                if (self.compare_to)(value, (*current).value.as_ref().unwrap()) == 0 {
                    return Some(i);
                }
            }
        }
        None
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        unsafe {
            let mut current = (*self.head).next;
            while (*current).value.is_some() {
                let next_node = (*current).next;
                // Call the user provided function to free the memory of the data
                (self.destroy_data)((*current).value.as_ref().unwrap());
                // Box implements Drop,
                // the memory will be freed when the Box is dropped.
                // This will be done automatically when the variable goes out of scope.
                let _ = Box::from_raw(current);
                current = next_node;
            }
            let _ = Box::from_raw(self.head);
        }
    }
}
