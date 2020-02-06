#![allow(dead_code)]

use core::{fmt, ptr};

/// An _instrusive_ linked list of addresses.
///
/// A `LinkedList` maintains a list of `*mut usize`s. The user of the
/// `LinkedList` guarantees that the passed in pointer refers to valid, unique,
/// writeable memory at least `usize` in size.
///
/// # Usage
///
/// A list is created using `LinkedList::new()`. A new address can be prepended
/// using `push()`. The first address in the list, if any, can be removed and
/// returned using `pop()` or returned (but not removed) using `peek()`.
///
/// ```rust
/// # let address_1 = (&mut (1 as usize)) as *mut usize;
/// # let address_2 = (&mut (2 as usize)) as *mut usize;
/// let mut list = LinkedList::new();
/// unsafe {
///     list.push(address_1);
///     list.push(address_2);
/// }
///
/// assert_eq!(list.peek(), Some(address_2));
/// assert_eq!(list.pop(), Some(address_2));
/// assert_eq!(list.pop(), Some(address_1));
/// assert_eq!(list.pop(), None);
/// ```
///
/// `LinkedList` exposes two iterators. The first, obtained via `iter()`,
/// iterates over all of the addresses in the list. The second, returned from
/// `iter_mut()`, returns `Node`s that refer to each address in the list. The
/// `value()` and `pop()` methods of `Node` can be used to read the value or pop
/// the value from the list, respectively.
///
/// ```rust
/// # let address_1 = (&mut (1 as usize)) as *mut usize;
/// # let address_2 = (&mut (2 as usize)) as *mut usize;
/// # let address_3 = (&mut (3 as usize)) as *mut usize;
/// let mut list = LinkedList::new();
/// unsafe {
///     list.push(address_1);
///     list.push(address_2);
///     list.push(address_3);
/// }
///
/// for node in list.iter_mut() {
///     if node.value() == address_2 {
///         node.pop();
///     }
/// }
///
/// assert_eq!(list.pop(), Some(address_3));
/// assert_eq!(list.pop(), Some(address_1));
/// assert_eq!(list.pop(), None);
/// ```
#[derive(Copy, Clone)]
pub struct LinkedList {
    head: *mut usize,
}

unsafe impl Send for LinkedList {}

impl LinkedList {
    /// Returns a new, empty linked list.
    pub const fn new() -> LinkedList {
        LinkedList {
            head: ptr::null_mut(),
        }
    }

    /// Returns `true` if the list is empty and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    /// Pushes the address `item` to the front of the list.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `item` refers to unique, writeable memory at
    /// least `usize` in size that is valid as long as `item` resides in `self`.
    /// Barring the uniqueness constraint, this is equivalent to ensuring that
    /// `*item = some_usize` is a safe operation as long as the pointer resides
    /// in `self`.
    pub unsafe fn push(&mut self, item: *mut usize) {
        *item = self.head as usize;
        self.head = item;
    }

    /// Removes and returns the first item in the list, if any.
    pub fn pop(&mut self) -> Option<*mut usize> {
        let value = self.peek()?;
        self.head = unsafe { *value as *mut usize };
        Some(value)
    }

    /// Returns the first item in the list without removing it, if any.
    pub fn peek(&self) -> Option<*mut usize> {
        match self.is_empty() {
            true => None,
            false => Some(self.head),
        }
    }

    /// Returns an iterator over the items in this list.
    pub fn iter(&self) -> Iter {
        Iter {
            current: self.head,
            _list: self,
        }
    }

    /// Returns an iterator over the items in this list.
    ///
    /// The items returned from the iterator (of type `Node`) allows the given
    /// item to be removed from the linked list via the `Node::pop()` method.
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            prev: &mut self.head as *mut *mut usize as *mut usize,
            current: self.head,
            _list: self,
        }
    }
}

impl fmt::Debug for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// An iterator over the items of the linked list.
pub struct Iter<'a> {
    _list: &'a LinkedList,
    current: *mut usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = *mut usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut list = LinkedList { head: self.current };
        let value = list.pop()?;
        self.current = list.head;
        Some(value)
    }
}

/// An item returned from a mutable iterator of a `LinkedList`.
pub struct Node {
    prev: *mut usize,
    value: *mut usize,
}

impl Node {
    /// Removes and returns the value of this item from the linked list it
    /// belongs to.
    pub fn pop(self) -> *mut usize {
        unsafe {
            *(self.prev) = *(self.value);
        }
        self.value
    }

    /// Returns the value of this element.
    pub fn value(&self) -> *mut usize {
        self.value
    }
}

/// An iterator over the items of the linked list allowing mutability.
pub struct IterMut<'a> {
    _list: &'a mut LinkedList,
    prev: *mut usize,
    current: *mut usize,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let mut list = LinkedList { head: self.current };
        let value = list.pop()?;
        let prev = self.prev;
        self.prev = self.current;
        self.current = list.head;
        Some(Node { prev, value })
    }
}
