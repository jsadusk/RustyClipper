use libc::size_t;
use std::ptr;
use std::ops::{Index, IndexMut};

#[repr(C)]
pub struct CppIntPoint {
    pub x : i64,
    pub y : i64
}

enum CppPathObj {}

#[link(name = "ClipperHandle")]
extern {
    fn make_path_with_size(size : size_t, obj: *mut *mut CppPathObj, data: *mut *mut CppIntPoint) -> i32;
}

pub struct CppPath {
    obj: *mut CppPathObj,
    data: *mut CppIntPoint
}

impl CppPath {
    pub fn new_sized(size : usize) -> ClipperResult<CppPath> {
        unsafe {
            let mut obj: *mut CppPathObj = ptr::null_mut();
            let mut data: *mut CppIntPoint = ptr::null_mut();
            let code = make_path_with_size(size, &mut obj, &mut data);

            if code == 0 {
                Ok(CppPath { obj : obj, data : data })
            } else {
                Err(ClipperError::CppException)
            }
        }
    }
}

impl Index<usize> for CppPath {
    type Output = CppIntPoint;

    fn index<'a>(&'a self, idx : usize) -> &'a CppIntPoint {
        unsafe {
            &*self.data.offset(idx as isize)
        }
    }
}

impl IndexMut<usize> for CppPath {
    fn index_mut<'a>(&'a mut self, idx : usize) -> &'a mut CppIntPoint {
        unsafe {
            &mut *self.data.offset(idx as isize)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern {
        fn test_elem(obj : *mut CppPathObj, index : size_t, x : i64, y : i64) -> i32;
    }
    
    #[test]
    fn test_cpp_path_new() {
        let path = CppPath::new_sized(5).unwrap();

        assert!(!path.obj.is_null());
        assert!(!path.data.is_null());
    }

    #[test]
    fn test_cpp_elem() {
        let mut path = CppPath::new_sized(5).unwrap();

        let elem = &mut path[0];
        elem.x = 1;
        elem.y = 2;

        let elem = &mut path[1];
        elem.x = 2;
        elem.y = 3;

        let elem = &mut path[2];
        elem.x = 3;
        elem.y = 4;

        let elem = &mut path[3];
        elem.x = 4;
        elem.y = 5;

        let elem = &mut path[4];
        elem.x = 5;
        elem.y = 6;

        unsafe {
            assert!(test_elem(path.obj, 0, 1, 2) != 0);
            assert!(test_elem(path.obj, 1, 2, 3) != 0);
            assert!(test_elem(path.obj, 2, 3, 4) != 0);
            assert!(test_elem(path.obj, 3, 4, 5) != 0);
            assert!(test_elem(path.obj, 4, 5, 6) != 0);
        }
    }
}
