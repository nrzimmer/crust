use std::cmp::min;
use std::io::IoSliceMut;

#[derive(Debug)]
pub struct RingBuffer<T: PartialEq + Clone> {
    buffer: Vec<T>,
    size: usize,
    read_index: usize,
    write_index: usize,
}

impl<T: PartialEq + Clone> RingBuffer<T>
    where T: Default, {
    pub fn new(size: usize) -> Self {
        RingBuffer {
            buffer: vec![T::default(); size],
            size,
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn slices(&mut self) -> Vec<IoSliceMut<'_>> {
        let t_bytes = std::mem::size_of::<T>() / std::mem::size_of::<u8>();
        let raw_data = self.buffer.as_mut_ptr() as *mut u8;
        if self.write_index != 0 {
            unsafe {
                return vec![
                    IoSliceMut::new(std::slice::from_raw_parts_mut(
                        raw_data.offset((self.write_index * t_bytes) as isize),
                        (self.size - self.write_index - 1) * t_bytes,
                    )),
                    IoSliceMut::new(std::slice::from_raw_parts_mut(
                        raw_data,
                        (self.read_index - 1) * t_bytes,
                    )),
                ];
            }
        }
        unsafe {
            return vec![IoSliceMut::new(std::slice::from_raw_parts_mut(
                raw_data,
                self.size * t_bytes,
            ))];
        }
    }

    pub fn wrote(&mut self, size: usize) {
        self.write_index = (self.write_index + size) % self.size;
    }

    pub fn find_first(&self, target: &[T]) -> Option<usize> {
        if self.read_index == self.write_index {
            None
        } else if self.read_index < self.write_index {
            for (index, window) in self.buffer[self.read_index..self.write_index].windows(target.len()).enumerate() {
                if window == target {
                    return Some(index);
                }
            }
            None
        } else {
            for (index, window) in self.buffer[self.read_index..self.size].windows(target.len()).enumerate() {
                if window == target {
                    return Some(index);
                }
            }
            for (index, window) in self.buffer[0..self.write_index].windows(target.len()).enumerate() {
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