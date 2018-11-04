extern crate libc;
#[macro_use]
extern crate failure;

use libc::size_t;
use std::ptr;
use std::ops::{Index, IndexMut};

#[derive(Fail, Debug)]
pub enum ClipperError {
    #[fail(display = "Unexpected C++ exception")]
    CppException
}

type ClipperResult<T> = Result<T, ClipperError>;

#[repr(C)]
pub struct CppIntPoint {
    pub x : i64,
    pub y : i64
}

enum CppPathObj {}

#[link(name = "ClipperHandle")]
extern {
    fn path_new(obj: *mut *mut CppPathObj, data: *mut *mut CppIntPoint) -> i32;
    fn path_new_sized(size : size_t, obj: *mut *mut CppPathObj, data: *mut *mut CppIntPoint) -> i32;
    fn path_push_back(obj: *mut CppPathObj, elem: *const CppIntPoint,
                      data: *mut *mut CppIntPoint) ->i32;
    fn path_delete(obj: *mut CppPathObj);
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
            let code = path_new_sized(size, &mut obj, &mut data);

            if code == 0 {
                Ok(CppPath { obj : obj, data : data })
            } else {
                Err(ClipperError::CppException)
            }
        }
    }

    pub fn new() -> ClipperResult<CppPath> {
        unsafe {
            let mut obj: *mut CppPathObj = ptr::null_mut();
            let mut data: *mut CppIntPoint = ptr::null_mut();
            let code = path_new(&mut obj, &mut data);

            if code == 0 {
                Ok(CppPath { obj : obj, data : data })
            } else {
                Err(ClipperError::CppException)
            }
        }
    }

    pub fn push(&mut self, elem : &CppIntPoint) -> ClipperResult<()> {
        unsafe {
            let code = path_push_back(self.obj, elem, &mut self.data);

            if code == 0 {
                Ok(())
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

impl Drop for CppPath {
    fn drop(& mut self) {
        unsafe {
            path_delete(self.obj);
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
    fn test_cpp_path_elem() {
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

    #[test]
    fn test_cpp_path_push() {
        let mut path = CppPath::new().unwrap();

        path.push(&CppIntPoint { x: 1, y: 2}).unwrap();
        path.push(&CppIntPoint { x: 2, y: 3}).unwrap();
        path.push(&CppIntPoint { x: 3, y: 4}).unwrap();
        path.push(&CppIntPoint { x: 4, y: 5}).unwrap();
        path.push(&CppIntPoint { x: 5, y: 6}).unwrap();

        assert_eq!(path[0].x, 1);
        assert_eq!(path[1].x, 2);
        assert_eq!(path[2].x, 3);
        assert_eq!(path[3].x, 4);
        assert_eq!(path[4].x, 5);
        
        unsafe {
            assert!(test_elem(path.obj, 0, 1, 2) != 0);
            assert!(test_elem(path.obj, 1, 2, 3) != 0);
            assert!(test_elem(path.obj, 2, 3, 4) != 0);
            assert!(test_elem(path.obj, 3, 4, 5) != 0);
            assert!(test_elem(path.obj, 4, 5, 6) != 0);
        }
    }
}
