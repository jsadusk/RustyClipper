extern crate libc;
#[macro_use]
extern crate failure;

use libc::{c_char, size_t};
use std::ptr;
use std::ops::{Index, IndexMut};
use std::ffi::CStr;

#[derive(Fail, Debug)]
pub enum ClipperError {
    #[fail(display = "Unexpected C++ exception in call {}", call)]
    CppException { call : &'static str },
    #[fail(display = "Unexpected C++ exception in call {}: {}", call, msg)]
    CppExceptionStr { call : &'static str, msg : String },
    #[fail(display = "Clipper exception in call {}: {}", call, msg)]
    ClipperException { call : &'static str, msg : String }
}

type ClipperResult<T> = Result<T, ClipperError>;

#[repr(C)]
pub struct CppReturnCodeMsg {
    code: i32,
    msg: *const c_char
}

impl CppReturnCodeMsg {
    fn is_err(&self) -> bool {
        self.code != 0
    }
    
    fn to_err(self, call : &'static str) -> ClipperError {
        match self.code {
            1 => ClipperError::CppExceptionStr {
                call: call,
                msg: unsafe {CStr::from_ptr(self.msg)}
                                  .to_str().unwrap().to_owned()
            },
            2 => ClipperError::ClipperException {
                call: call,
                msg: unsafe {CStr::from_ptr(self.msg)}
                                  .to_str().unwrap().to_owned()
            },
            _ => ClipperError::CppException { call: call }
        }
    }
}        

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

pub enum CppClipType {
    CtIntersection = 0,
    CtUnion = 1,
    CtDifference = 2,
    CtXor = 3
}

pub enum CppPolyFillType {
    PftEvenOdd = 0,
    PftNonZero = 1,
    PftPositive = 2,
    PftNegative = 3
}

#[link(name = "ClipperHandle")]
extern {
    fn path_new(obj: *mut *mut CppPathObj, data: *mut *mut CppIntPoint)
                -> CppReturnCodeMsg;
    fn path_new_sized(size : size_t, obj: *mut *mut CppPathObj,
                      data: *mut *mut CppIntPoint) -> CppReturnCodeMsg;
    fn path_data(obj: *const CppPathObj, data: *mut *mut CppIntPoint,
                 size: *mut usize) -> CppReturnCodeMsg;
    fn path_push_back(obj: *mut CppPathObj, elem: *const CppIntPoint,
                      data: *mut *mut CppIntPoint) -> CppReturnCodeMsg;
    fn path_delete(obj: *mut CppPathObj);

    fn paths_new(obj: *mut *mut CppPathsObj, data: *mut *mut CppPathObj)
                 -> CppReturnCodeMsg;
    fn paths_new_sized(size : size_t, obj: *mut *mut CppPathsObj,
                      data: *mut *mut CppPathObj) -> CppReturnCodeMsg;
    fn paths_data(obj: *const CppPathsObj, data: *mut *mut CppPathObj,
                  size: *mut usize) -> CppReturnCodeMsg;
    fn paths_push_back_move(obj: *mut CppPathsObj, elem: *mut CppPathObj,
                            data: *mut *mut CppPathObj)
        -> CppReturnCodeMsg;
    fn paths_delete(obj: *mut CppPathsObj);

    fn clipper_new(obj: *mut *mut CppClipperObj, initOptions : i32)
        -> CppReturnCodeMsg;
    fn clipper_add_path(clipper_obj: *mut CppClipperObj,
                        path_obj : *const CppPathObj,
                        poly_type : i32,
                        closed : i32) -> CppReturnCodeMsg;
    fn clipper_add_paths(clipper_obj: *mut CppClipperObj,
                         paths_obj : *const CppPathsObj,
                         poly_type : i32,
                         closed : i32)
        -> CppReturnCodeMsg;
    fn clipper_execute_open_closed(clipper_obj: *mut CppClipperObj,
                                   solution_open_obj: *mut *mut CppPathsObj,
                                   solution_closed_obj: *mut *mut CppPathsObj,
                                   clip_type: i32,
                                   subj_fill_type: i32,
                                   clip_fill_type: i32)
        -> CppReturnCodeMsg;
    fn clipper_delete(obj: *mut CppClipperObj);

    fn clipper_offset_new(obj: *mut *mut CppClipperOffsetObj)
        -> CppReturnCodeMsg;
    fn clipper_offset_add_path(clipper_obj: *mut CppClipperOffsetObj,
                               path_obj: *const CppPathObj,
                               join_type: i32,
                               end_type: i32)
        -> CppReturnCodeMsg;
    fn clipper_offset_add_paths(clipper_obj: *mut CppClipperOffsetObj,
                                paths_obj: *const CppPathsObj,
                                join_type: i32,
                                end_type: i32)
        -> CppReturnCodeMsg;
    fn clipper_offset_delete(obj: *mut CppClipperOffsetObj);
}

pub struct CppPath {
    obj: *mut CppPathObj,
    data: *mut CppIntPoint,
    size: usize,
    owned: bool
}

pub struct CppPaths {
    obj: *mut CppPathsObj,
    data: *mut CppPathObj,
    size: usize,
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

            if code.is_err() {
                Err(code.to_err("path_new_sized"))
            } else {
                Ok(CppPath { obj: obj, data: data, size: size, owned: true })
            }
        }
    }

    pub fn new() -> ClipperResult<CppPath> {
        unsafe {
            let mut obj: *mut CppPathObj = ptr::null_mut();
            let mut data: *mut CppIntPoint = ptr::null_mut();
            let code = path_new(&mut obj, &mut data);

            if code.is_err() {
                Err(code.to_err("path_new"))
            } else {
                Ok(CppPath { obj: obj, data: data, size: 0, owned: true })
            }
        }
    }

    pub(crate) fn from(obj : *mut CppPathObj) -> ClipperResult<CppPath> {
        unsafe {
            let mut data: *mut CppIntPoint = ptr::null_mut();
            let mut size: usize = 0;
            let code = path_data(obj, &mut data, &mut size);

            if code.is_err() {
                Err(code.to_err("path_data"))
            } else {
                Ok(CppPath { obj: obj, data: data, size: size, owned: false })
            }
        }
    }

    pub fn push(&mut self, elem : CppIntPoint) -> ClipperResult<()> {
        unsafe {
            let code = path_push_back(self.obj, &elem, &mut self.data);

            if code.is_err() {
                Err(code.to_err("path_push_back"))
            } else {
                self.size += 1;
                Ok(())
            }
        }
    }

    pub fn len(&self) -> usize {
        self.size
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

            if code.is_err() {
                Err(code.to_err("paths_new_sized"))
            } else {
                Ok(CppPaths { obj: obj, data: data, size: size, owned: true })
            }
        }
    }

    pub fn new() -> ClipperResult<CppPaths> {
        unsafe {
            let mut obj: *mut CppPathsObj = ptr::null_mut();
            let mut data: *mut CppPathObj = ptr::null_mut();
            let code = paths_new(&mut obj, &mut data);

            if code.is_err() {
                Err(code.to_err("paths_new"))
            } else {
                Ok(CppPaths { obj: obj, data: data, size: 0, owned: true })
            }
        }
    }

    pub(crate) fn from(obj : *mut CppPathsObj) -> ClipperResult<CppPaths> {
        unsafe {
            let mut data: *mut CppPathObj = ptr::null_mut();
            let mut size: usize = 0;
            let code = paths_data(obj, &mut data, &mut size);

            if code.is_err() {
                Err(code.to_err("paths_data"))
            } else {
                Ok(CppPaths { obj: obj, data: data, size: size, owned: false })
            }
        }
    }

    pub fn push(&mut self, elem : CppPath) -> ClipperResult<()> {
        unsafe {
            let code = paths_push_back_move(self.obj, elem.to_handle(),
                                            &mut self.data);

            if code.is_err() {
                Err(code.to_err("paths_push_back_move"))
            } else {
                self.size += 1;
                Ok(())
            }
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
    
    pub(crate) fn to_handle(mut self) -> *mut CppPathsObj {
        self.owned = false;
        self.obj
    }

    pub(crate) fn handle(&self) -> *const CppPathsObj {
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
            if code.is_err() {
                Err(code.to_err("clipper_new"))
            } else {
                Ok(Clipper { obj : obj})
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
            if code.is_err() {
                Err(code.to_err("clipper_add_path"))
            } else {
                Ok(())
            }
        }
    }

    pub fn add_paths(&mut self,
                paths : &CppPaths, poly_type : CppPolyType, closed : bool)
                -> ClipperResult<()> {
        unsafe {
            let code = clipper_add_paths(self.obj, paths.handle(),
                                         poly_type as i32,
                                         match closed { true => 1, false => 0});
            if code.is_err() {
                Err(code.to_err("clipper_add_paths"))
            } else {
                Ok(())
            }
        }
    }

    pub fn execute_open_closed(&mut self,
                               clip_type: CppClipType,
                               subj_fill_type: CppPolyFillType,
                               clip_fill_type: CppPolyFillType)
                               -> ClipperResult<(CppPaths, CppPaths)> {
        unsafe {
            let mut solution_open_obj : *mut CppPathsObj = ptr::null_mut();
            let mut solution_closed_obj : *mut CppPathsObj = ptr::null_mut();
            let code = clipper_execute_open_closed(
                self.obj, &mut solution_open_obj, &mut solution_closed_obj,
                clip_type as i32, subj_fill_type as i32, clip_fill_type as i32);

            if code.is_err() {
                Err(code.to_err("clipper_execute_open_closed"))
            } else {
                Ok((CppPaths::from(solution_open_obj)?,
                    CppPaths::from(solution_closed_obj)?))
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
            if code.is_err() {
                Err(code.to_err("clipper_offset_new"))
            } else {
                Ok(ClipperOffset { obj : obj })
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
            if code.is_err() {
                Err(code.to_err("clipper_offset_add_path"))
            } else {
                Ok(())
            }
        }
    }

    pub fn add_paths(&mut self,
                     paths : &CppPaths, join_type : CppJoinType,
                     end_type: CppEndType)
                     -> ClipperResult<()> {
        unsafe {
            let code = clipper_offset_add_paths(self.obj, paths.handle(),
                                                join_type as i32,
                                                end_type as i32);
            if code.is_err() {
                Err(code.to_err("clipper_offset_add_paths"))
            } else {
                Ok(())
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
        let path1 = CppPath::new().unwrap();
        let mut _path2 = CppPath::from(path1.to_handle()).unwrap();
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
        paths.push(path0).unwrap();

        let path1 = paths.at(0).unwrap();
        let point = &path1[0];
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn test_clipper_open_difference() {
        let mut subj = CppPath::new().unwrap();
        subj.push(CppIntPoint { x: 0, y: 1}).unwrap();
        subj.push(CppIntPoint { x: 5, y: 1}).unwrap();

        let mut clip = CppPath::new().unwrap();
        clip.push(CppIntPoint { x: 1, y: 0 }).unwrap();
        clip.push(CppIntPoint { x: 1, y: 2 }).unwrap();
        clip.push(CppIntPoint { x: 2, y: 2 }).unwrap();
        clip.push(CppIntPoint { x: 2, y: 0 }).unwrap();
        clip.push(CppIntPoint { x: 1, y: 0 }).unwrap();
        
        let mut clipper = Clipper::new().unwrap();
        clipper.add_path(&subj, CppPolyType::PtSubject, false).unwrap();
        clipper.add_path(&clip, CppPolyType::PtClip, true).unwrap();

        let (mut open, closed) = clipper.execute_open_closed(
            CppClipType::CtDifference,
            CppPolyFillType::PftNonZero,
            CppPolyFillType::PftNonZero).unwrap();

        assert_eq!(open.len(), 2);
        assert_eq!(closed.len(), 0);

        let piece1 = open.at(0).unwrap();
        assert_eq!(piece1.len(), 2);
        let pt = &piece1[0];
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 1);

        let pt = &piece1[1];
        assert_eq!(pt.x, 0);
        assert_eq!(pt.y, 1);

        let piece2 = open.at(1).unwrap();
        assert_eq!(piece2.len(), 2);

        let pt = &piece2[0];
        assert_eq!(pt.x, 2);
        assert_eq!(pt.y, 1);

        let pt = &piece2[1];
        assert_eq!(pt.x, 5);
        assert_eq!(pt.y, 1);
    }
}
