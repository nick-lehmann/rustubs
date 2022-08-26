#![allow(dead_code, unused_variables)]
use core::ptr;

pub struct RingBuffer<T, const SIZE: usize> {
    pub buffer: [*mut T; SIZE],
    head: usize,
    tail: usize,
}

impl<T, const SIZE: usize> RingBuffer<T, SIZE> {
    pub const fn new() -> Self {
        RingBuffer {
            buffer: [ptr::null_mut(); SIZE],
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, element: &T) -> bool {
        let mut next_head = self.head + 1;
        if next_head == SIZE {
            next_head = 0;
        }
        if next_head == self.tail {
            return false;
        }
        self.buffer[self.head] = element as *const T as *mut T;
        self.head = next_head;

        true
    }

    pub fn pop(&mut self) -> Option<&mut T> {
        if self.head == self.tail {
            return None;
        }
        let mut next_tail = self.tail + 1;
        if next_tail == SIZE {
            next_tail = 0;
        }
        let element = self.buffer[self.tail];
        self.tail = next_tail;
        Some(unsafe { &mut *element })
    }

    pub fn size(&self) -> usize {
        if self.head >= self.tail {
            return self.head - self.tail;
        }
        return SIZE - self.tail + self.head;
    }
}

// #[cfg(test)]
// #[allow(non_upper_case_globals)]
// mod tests {
//     use super::RingBuffer;

//     #[derive(Clone, Copy, Debug, PartialEq, Eq)]
//     pub struct Element {
//         pub name: &'static str,
//     }

//     impl Element {
//         pub const fn new(name: &'static str) -> Self {
//             Self { name }
//         }
//     }

//     static element1: Element = Element::new("1");
//     static element2: Element = Element::new("2");
//     static element3: Element = Element::new("3");

//     #[test]
//     fn basic() {
//         let mut buffer: RingBuffer<Element, 5> = RingBuffer::new();
//         buffer.push(&element1);
//         assert_eq!(buffer.size(), 1);

//         buffer.push(&element2);
//         assert_eq!(buffer.size(), 2);

//         assert_eq!(buffer.pop().unwrap().name, element1.name);
//         assert_eq!(buffer.pop().unwrap().name, element2.name);
//     }

//     #[test]
//     fn overflow() {
//         let mut buffer: RingBuffer<Element, 3> = RingBuffer::new();

//         assert_eq!(buffer.push(&element1), true);
//         assert_eq!(buffer.push(&element2), true);
//         assert_eq!(buffer.push(&element3), false);
//     }

//     #[test]
//     fn empty() {
//         let mut buffer: RingBuffer<Element, 5> = RingBuffer::new();

//         assert_eq!(buffer.pop(), None);

//         buffer.push(&element1);
//         buffer.push(&element2);

//         assert_eq!(buffer.pop().unwrap(), &element1);
//         assert_eq!(buffer.pop().unwrap(), &element2);
//         assert_eq!(buffer.pop(), None);
//     }
// }
