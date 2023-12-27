use std::cmp::min;
use std::io::IoSliceMut;

pub struct FifoBuffer<T: PartialEq + Clone> {
    buffer: Vec<T>,
    size: usize,
    //orig_size: usize,
    read_index: usize,
    write_index: usize,
}

impl<T: PartialEq + Clone> FifoBuffer<T>
where
    T: Default,
{
    pub fn new(size: usize) -> Self {
        let buffer: Vec<T> = vec![T::default(); size];
        FifoBuffer {
            buffer,
            size,
            //orig_size: size,
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn get_vector_for_writing(&mut self) -> Vec<IoSliceMut<'_>> {
        if self.write_index != 0 {
            let raw_data: *mut u8 = self.buffer.as_mut_ptr() as *mut u8;
            let mul = std::mem::size_of::<T>() / std::mem::size_of::<u8>();
            let offset = self.write_index * mul;
            let len = (self.size - self.write_index - 1) * mul;
            let len2 = (self.read_index - 1) * mul;
            unsafe {
                return vec![
                    IoSliceMut::new(std::slice::from_raw_parts_mut(
                        raw_data.offset(offset as isize),
                        len,
                    )),
                    IoSliceMut::new(std::slice::from_raw_parts_mut(raw_data, len2)),
                ];
            }
        }
        let raw_data: *mut u8 = self.buffer.as_mut_ptr() as *mut u8;
        let mul = std::mem::size_of::<T>() / std::mem::size_of::<u8>();
        let len = self.size * mul;
        unsafe {
            return vec![IoSliceMut::new(std::slice::from_raw_parts_mut(
                raw_data, len,
            ))];
        }
    }

    pub fn wrote(&mut self, size: usize) {
        self.write_index = inc_qty(self.write_index, self.size, size);
    }

    /*
    pub fn append(&mut self, buf: &[T]) {
        if buf.len() >= (self.size - 1 - self.write_index + self.read_index) {
            //self.grow();
        }
        // TODO - optimize write
        for u in buf {
            self.buffer[self.write_index] = u.clone();
            inc(self.write_index, self.size);
        }
    }
     */

    /*
    fn grow(&mut self) {
        self.inner_buf.reserve(self.size + self.orig_size);
        if self.read_index > self.write_index {
            self.inner_buf.extend_from_within(0..self.write_index);
            self.write_index += self.size;
        }
        /*
        let mut new_buff = vec![0u8; self.size + self.orig_size];
        let mut new_write = 0usize;
        while self.read_index != self.write_index {
            new_buff[new_write] = self.inner_buf[self.read_index];
            self.inc_read_index();
        }
        self.inner_buf = new_buff;
        self.read_index = 0usize;
        self.write_index = new_write;
        */
        self.size = self.inner_buf.len();
    }
     */

    pub fn find_first(&self, target: &[T]) -> Option<usize> {
        if self.read_index == self.write_index {
            None
        } else if self.read_index < self.write_index {
            for (index, window) in self.buffer[self.read_index..self.write_index]
                .windows(target.len())
                .enumerate()
            {
                if window == target {
                    return Some(index);
                }
            }
            None
        } else {
            for (index, window) in self.buffer[self.read_index..self.size]
                .windows(target.len())
                .enumerate()
            {
                if window == target {
                    return Some(index);
                }
            }
            for (index, window) in self.buffer[0..self.write_index]
                .windows(target.len())
                .enumerate()
            {
                if window == target {
                    return Some(index + self.size - self.read_index - 1);
                }
            }
            None
        }
    }

    pub fn discard(&mut self, up_to: usize) {
        self.read_index = min(self.write_index, self.read_index + up_to);
    }

    pub fn consume(&mut self, up_to: usize) -> Option<Vec<T>> {
        if self.read_index == self.write_index {
            None
        } else if self.read_index < self.write_index {
            let end = min(self.write_index, self.read_index + up_to);
            let mut vec: Vec<T> = Vec::with_capacity(end - self.read_index);
            vec.extend_from_slice(&self.buffer[self.read_index..end]);
            self.read_index = min(self.write_index, self.read_index + up_to);
            return Some(vec);
        } else {
            let remain = self.read_index + up_to % self.size;
            let mut vec: Vec<T> = Vec::with_capacity(up_to);
            vec.extend_from_slice(&self.buffer[self.read_index..self.size]);
            vec.extend_from_slice(&self.buffer[0..remain]);
            self.read_index = remain;
            return Some(vec);
        }
    }
}

/*
impl IntoIterator for RingBuffer {
    type Item = u8;
    type IntoIter = RingBufferIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let size = self.size;
        RingBufferIntoIter {
            buff: self,
            index: 0,
            size
        }
    }
}

struct RingBufferIntoIter {
    buff : RingBuffer,
    index: usize,
    size: usize
}

impl Iterator for RingBufferIntoIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + 1 < self.size {
            self.index += 1;
        } else {
            self.index = 0;
        }
        Some(self.buff.inner_buf[self.index])
    }
}
*/

#[inline]
#[allow(dead_code)]
fn inc(index: usize, max: usize) -> usize {
    inc_qty(index, max, 1)
}

#[inline]
fn inc_qty(index: usize, max: usize, qty: usize) -> usize {
    assert!(qty <= max);
    (index + qty) % max
}
