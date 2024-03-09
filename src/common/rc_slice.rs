use std::rc::Rc;

macro_rules! check_boundaries {
    ($start:expr, $end:expr, $arr:expr) => {
        if $start > $end {
            panic!("start can not be greater then end: start({}) > end({}).", $start, $end);
        }
        if $arr.len() < $end {
            panic!(
                "end can not be greater then slice len: slice_len({}) < end({}).",
                $arr.len(),
                $end
            )
        }
    };
}

#[derive(Debug, PartialEq)]
pub struct RcSlice {
    data: Rc<Vec<u8>>,
    start: usize,
    end: usize,
}

impl RcSlice {
    pub fn new(data: Vec<u8>, start: usize, end: usize) -> Self {
        check_boundaries!(start, end, data);
        Self {
            data: Rc::new(data),
            start,
            end,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data[self.start..self.end]
    }

    pub fn with_range(&self, start: usize, end: usize) -> Self {
        check_boundaries!(start, end, self.data);
        Self {
            data: Rc::clone(&self.data),
            start,
            end,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Clone for RcSlice {
    fn clone(&self) -> Self {
        Self {
            data: Rc::clone(&self.data),
            start: self.start,
            end: self.end,
        }
    }
}

impl From<&[u8]> for RcSlice {
    fn from(value: &[u8]) -> Self {
        Self {
            data: Rc::new(value.to_vec()),
            start: 0,
            end: value.len(),
        }
    }
}
