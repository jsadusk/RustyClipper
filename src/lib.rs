extern crate libc;
#[macro_use]
extern crate failure;

use libc::size_t;
use std::ptr;
use std::ops::{Index, IndexMut};

#[derive(Fail, Debug)]
pub enum ClipperError {
    #[fail(display = "Unexpected C++ exception in call {}", call)]
    CppException { call : &'static str }
}

type ClipperResult<T> = Result<T, ClipperError>;

#[repr(C)]
pub struct CppIntPoint {
    pub x : i64,
    pub y : i64
}

pub enum CppPathObj {}
pub enum CppPathsObj {}
enum CppClipperObj {}
enum CppClipperOffsetObj {}

pub enum CppPolyType {
    PtSubject = 0,
    PtClip = 1
}

pub enum CppJoinType {
    JtSquare = 0,
    JtRound = 1,
    JtMitre = 2
}

pub enum CppEndType {
    EtClosedPolygon = 0,
    EtClosedLine = 1,
    EtOpenSquare = 2,
    EtOpenRound = 3,
    EtOpenButt = 4
}

#[link(name = "ClipperHandle")]
extern {
    fn path_new(obj: *mut *mut CppPathObj, data: *mut *mut CppIntPoint) -> i32;
    fn path_new_sized(size : size_t, obj: *mut *mut CppPathObj,
                      data: *mut *mut CppIntPoint) -> i32;
    fn path_data(obj: *const CppPathObj, data: *mut *mut CppIntPoint) ->i32;
    fn path_push_back(obj: *mut CppPathObj, elem: *const CppIntPoint,
                      data: *mut *mut CppIntPoint) ->i32;
    fn path_delete(obj: *mut CppPathObj);

    fn paths_new(obj: *mut *mut CppPathsObj, data: *mut *mut CppPathObj) -> i32;
    fn paths_new_sized(size : size_t, obj: *mut *mut CppPathsObj,
                      data: *mut *mut CppPathObj) -> i32;
    fn paths_data(obj: *const CppPathsObj, data: *mut *mut CppPathObj) ->i32;
    fn paths_push_back_move(obj: *mut CppPathsObj, elem: *mut CppPathObj,
                            data: *mut *mut CppPathObj) ->i32;
    fn paths_delete(obj: *mut CppPathsObj);

    fn clipper_new(obj: *mut *mut CppClipperObj, initOptions : i32) ->i32;
    fn clipper_add_path(clipper_obj: *mut CppClipperObj,
                        path_obj : *const CppPathObj,
                        poly_type : i32,
                        closed : i32) ->i32;
    fn clipper_delete(obj: *mut CppClipperObj);

    fn clipper_offset_new(obj: *mut *mut CppClipperOffsetObj) ->i32;
    fn clipper_offset_add_path(clipper_obj: *mut CppClipperOffsetObj,
                               path_obj: *const CppPathObj,
                               join_type: i32,
                               end_type: i32) ->i32;
    fn clipper_offset_delete(obj: *mut CppClipperOffsetObj);
}

pub struct CppPath {
    obj: *mut CppPathObj,
    data: *mut CppIntPoint,
    owned: bool
}

pub struct CppPaths {
    obj: *mut CppPathsObj,
    data: *mut CppPathObj,
    owned: bool
}

pub struct Clipper {
    obj : *mut CppClipperObj
}

pub struct ClipperOffset {
    obj : *mut CppClipperOffsetObj
}

impl CppPath {
    pub fn new_sized(size : usize) -> ClipperResult<CppPath> {
        unsafe {
            let mut obj: *mut CppPathObj = ptr::null_mut();
            let mut data: *mut CppIntPoint = ptr::null_mut();
            let code = path_new_sized(size, &mut obj, &mut data);

            if code == 0 {
                Ok(CppPath { obj : obj, data : data, owned : true })
            } else {
                Err(ClipperError::CppException { call : "path_new_sized" })
            }
        }
    }

    pub fn new() -> ClipperResult<CppPath> {
        unsafe {
            let mut obj: *mut CppPathObj = ptr::null_mut();
            let mut data: *mut CppIntPoint = ptr::null_mut();
            let code = path_new(&mut obj, &mut data);

            if code == 0 {
                Ok(CppPath { obj : obj, data : data, owned : true })
            } else {
                Err(ClipperError::CppException { call : "path_new" })
            }
        }
    }

    pub(crate) fn from(obj : *mut CppPathObj) -> ClipperResult<CppPath> {
        unsafe {
            let mut data: *mut CppIntPoint = ptr::null_mut();
            let code = path_data(obj, &mut data);

            if code == 0 {
                Ok(CppPath { obj : obj, data : data, owned : false })
            } else {
                Err(ClipperError::CppException { call : "path_data" })
            }
        }
    }

    pub fn push(&mut self, elem : CppIntPoint) -> ClipperResult<()> {
        unsafe {
            let code = path_push_back(self.obj, &elem, &mut self.data);

            if code == 0 {
                Ok(())
            } else {
                Err(ClipperError::CppException { call : "path_push_back" })
            }
        }
    }

    pub(crate) fn to_handle(mut self) -> *mut CppPathObj {
        self.owned = false;
        self.obj
    }

    pub(crate) fn handle(&self) -> *const CppPathObj {
        self.obj as *const CppPathObj
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
        if self.owned {
            unsafe {
                path_delete(self.obj);
            }
        }
    }
}

impl CppPaths {
    pub fn new_sized(size : usize) -> ClipperResult<CppPaths> {
        unsafe {
            let mut obj: *mut CppPathsObj = ptr::null_mut();
            let mut data: *mut CppPathObj = ptr::null_mut();
            let code = paths_new_sized(size, &mut obj, &mut data);

            if code == 0 {
                Ok(CppPaths { obj : obj, data : data, owned : true })
            } else {
                Err(ClipperError::CppException { call : "paths_new_sized" })
            }
        }
    }

    pub fn new() -> ClipperResult<CppPaths> {
        unsafe {
            let mut obj: *mut CppPathsObj = ptr::null_mut();
            let mut data: *mut CppPathObj = ptr::null_mut();
            let code = paths_new(&mut obj, &mut data);

            if code == 0 {
                Ok(CppPaths { obj : obj, data : data, owned : true })
            } else {
                Err(ClipperError::CppException { call : "paths_new" })
            }
        }
    }

    pub(crate) fn from(obj : *mut CppPathsObj) -> ClipperResult<CppPaths> {
        unsafe {
            let mut data: *mut CppPathObj = ptr::null_mut();
            let code = paths_data(obj, &mut data);

            if code == 0 {
                Ok(CppPaths { obj : obj, data : data, owned : false })
            } else {
                Err(ClipperError::CppException { call : "paths_data" })
            }
        }
    }

    pub fn push(&mut self, elem : CppPath) -> ClipperResult<()> {
        unsafe {
            let code = paths_push_back_move(self.obj, elem.to_handle(),
                                            &mut self.data);

            if code == 0 {
                Ok(())
            } else {
                Err(ClipperError::CppException { call : "paths_push_back_move" })
            }
        }
    }

    pub(crate) fn to_handle(mut self) -> *mut CppPathsObj {
        self.owned = false;
        self.obj
    }

    pub(crate) fn handle(self) -> *const CppPathsObj {
        self.obj as *const CppPathsObj
    }

    pub fn at(&mut self, idx : usize) -> ClipperResult<CppPath> {
        CppPath::from(unsafe { self.data.offset(idx as isize) })
    }
}

impl Drop for CppPaths {
    fn drop(& mut self) {
        if self.owned {
            unsafe {
                paths_delete(self.obj);
            }
        }
    }
}

impl Clipper {
    pub fn new() -> ClipperResult<Clipper> {
        unsafe {
            let mut obj: *mut CppClipperObj = ptr::null_mut();
            let code = clipper_new(&mut obj, 0);
            if code == 0 {
                Ok(Clipper { obj : obj})
            } else {
                Err(ClipperError::CppException { call : "clipper_new" })
            }
        }
    }

    pub fn add_path(&mut self,
                path : &CppPath, poly_type : CppPolyType, closed : bool)
                -> ClipperResult<()> {
        unsafe {
            let code = clipper_add_path(self.obj, path.handle(),
                                        poly_type as i32,
                                        match closed { true => 1, false => 0});
            if code == 0 {
                Ok(())
            } else {
                Err(ClipperError::CppException { call : "clipper_add_path" })
            }
        }
    }
}

impl Drop for Clipper {
    fn drop(&mut self) {
        unsafe {
            clipper_delete(self.obj);
        }
    }
}

impl ClipperOffset {
    pub fn new() -> ClipperResult<ClipperOffset> {
        unsafe {
            let mut obj: *mut CppClipperOffsetObj = ptr::null_mut();
            let code = clipper_offset_new(&mut obj);
            if code == 0 {
                Ok(ClipperOffset { obj : obj })
            } else {
                Err(ClipperError::CppException { call : "clipper_offset_new" })
            }
        }
    }

    pub fn add_path(&mut self,
                path : &CppPath, join_type : CppJoinType, end_type: CppEndType)
                -> ClipperResult<()> {
        unsafe {
            let code = clipper_offset_add_path(self.obj, path.handle(),
                                               join_type as i32,
                                               end_type as i32);
            if code == 0 {
                Ok(())
            } else {
                Err(ClipperError::CppException { call : "clipper_offset_add_path" })
            }
        }
    }
}

impl Drop for ClipperOffset {
    fn drop(&mut self) {
        unsafe {
            clipper_offset_delete(self.obj);
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

        path.push(CppIntPoint { x: 1, y: 2}).unwrap();
        path.push(CppIntPoint { x: 2, y: 3}).unwrap();
        path.push(CppIntPoint { x: 3, y: 4}).unwrap();
        path.push(CppIntPoint { x: 4, y: 5}).unwrap();
        path.push(CppIntPoint { x: 5, y: 6}).unwrap();

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

    #[test]
    fn test_clipper_new_drop() {
        let _clipper = Clipper::new().unwrap();
    }        

    #[test]
    fn test_clipper_offset_new_drop() {
        let _clipper_offset = ClipperOffset::new().unwrap();
    }

    #[test]
    fn test_clipper_add_path() {
        let mut clipper = Clipper::new().unwrap();
        let mut path = CppPath::new().unwrap();
        path.push(CppIntPoint { x: 0, y: 0}).unwrap();
        path.push(CppIntPoint { x: 1, y: 1}).unwrap();
        path.push(CppIntPoint { x: 1, y: 0}).unwrap();
        path.push(CppIntPoint { x: 0, y: 0}).unwrap();
        clipper.add_path(&path, CppPolyType::PtClip, true).unwrap();
        let mut path = CppPath::new().unwrap();
        path.push(CppIntPoint { x: 0, y: 0}).unwrap();
        path.push(CppIntPoint { x: -1, y: -1}).unwrap();
        path.push(CppIntPoint { x: -1, y: 0}).unwrap();
        clipper.add_path(&path, CppPolyType::PtSubject, false).unwrap();
    }

    #[test]
    fn test_clipper_offset_add_path() {
        let mut clipper_offset = ClipperOffset::new().unwrap();
        let mut path = CppPath::new().unwrap();
        path.push(CppIntPoint { x: 0, y: 0}).unwrap();
        path.push(CppIntPoint { x: 1, y: 1}).unwrap();
        path.push(CppIntPoint { x: 1, y: 0}).unwrap();
        path.push(CppIntPoint { x: 0, y: 0}).unwrap();
        clipper_offset.add_path(&path, CppJoinType::JtRound,
                         CppEndType::EtClosedPolygon).unwrap();
        let mut path = CppPath::new().unwrap();
        path.push(CppIntPoint { x: 0, y: 0}).unwrap();
        path.push(CppIntPoint { x: -1, y: -1}).unwrap();
        path.push(CppIntPoint { x: -1, y: 0}).unwrap();
        clipper_offset.add_path(&path, CppJoinType::JtMitre,
                                CppEndType::EtOpenRound).unwrap();
    }

    #[test]
    fn test_path_from() {
        let mut path1 = CppPath::new().unwrap();
        let mut path2 = CppPath::from(path1.to_handle()).unwrap();
    }
    
    #[test]
    fn test_paths_new_drop() {
        let _paths = CppPaths::new().unwrap();
    }

    #[test]
    fn test_paths_new_sized() {
        let mut paths = CppPaths::new_sized(3).unwrap();
        let mut path0 = paths.at(0).unwrap();
        path0.push(CppIntPoint { x: 1, y: 2 }).unwrap();
        let mut path1 = paths.at(1).unwrap();
        path1.push(CppIntPoint { x: 1, y: 2 }).unwrap();
        let mut path2 = paths.at(2).unwrap();
        path2.push(CppIntPoint { x: 1, y: 2 }).unwrap();
    }

    #[test]
    fn test_paths_push() {
        let mut path0 = CppPath::new().unwrap();
        path0.push(CppIntPoint { x: 1, y: 2}).unwrap();

        let mut paths = CppPaths::new().unwrap();
        paths.push(path0);

        let mut path1 = paths.at(0).unwrap();
        let mut point = &path1[0];
        assert!(point.x == 1);
        assert!(point.y == 2);
    }
}
