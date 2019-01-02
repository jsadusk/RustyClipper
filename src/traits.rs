use crate::error::*;
use crate::cpp;
use std::convert::From;
use std::convert::Into;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct CppPolygons {
    paths: cpp::CppPaths
}

pub trait Polygons: Sized {
    fn to_cpp(&self) -> ClipperResult<cpp::CppPaths>;
    fn from_cpp(other: cpp::CppPaths) -> ClipperResult<Self>;
}

impl Polygons for CppPolygons {
    fn to_cpp(&self) -> ClipperResult<cpp::CppPaths> {
        Ok(self.paths.pseudoclone())
    }

    fn from_cpp(other: cpp::CppPaths) -> ClipperResult<Self> {
        Ok(CppPolygons { paths: other })
    }
}

impl From<CppPolygons> for cpp::CppPaths {
    fn from(polygons: CppPolygons) -> cpp::CppPaths {
        polygons.paths
    }
}

struct PolyWrap<T>(T);

impl<T> TryFrom<CppPolygons> for PolyWrap<T> where T: Polygons {
    type Error = ClipperError;
    
    fn try_from(other: CppPolygons) -> ClipperResult<PolyWrap<T>> {
        Ok(PolyWrap(T::from_cpp(other.paths)?))
    }
}

impl<T> std::ops::Deref for PolyWrap<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/*impl<T> From<PolyWrap<T>> for T {
    fn from(other: PolyWrap<T>) -> T {
        other.0
    }
}*/

pub trait PolygonsOps: Polygons {
    fn union<P: Polygons>(&self, operand : &P)
                          -> ClipperResult<CppPolygons> {
        let mut clip = cpp::Clipper::new()?;
        
        clip.add_paths(self.to_cpp()?,
                       cpp::CppPolyType::PtSubject, true)?;
        clip.add_paths(operand.to_cpp()?,
                       cpp::CppPolyType::PtClip, true)?;

        let solution = clip.execute_closed(
            cpp::CppClipType::CtUnion,
            cpp::CppPolyFillType::PftNonZero,
            cpp::CppPolyFillType::PftNonZero)?;

        Ok(CppPolygons { paths: solution })
    }

    fn union_t<P: Polygons>(&self, operand: &P)
                            -> ClipperResult<Self> {
        Ok(Self::from_cpp(self.union(operand)?.paths)?)
    }
    
    fn difference<P: Polygons>(&self, operand : &P)
         -> ClipperResult<CppPolygons> {
        let mut clip = cpp::Clipper::new()?;
        
        clip.add_paths(self.to_cpp()?,
                       cpp::CppPolyType::PtSubject, true)?;
        clip.add_paths(operand.to_cpp()?,
                       cpp::CppPolyType::PtClip, true)?;

        let solution = clip.execute_closed(
            cpp::CppClipType::CtDifference,
            cpp::CppPolyFillType::PftNonZero,
            cpp::CppPolyFillType::PftNonZero)?;

        Ok(CppPolygons { paths: solution })
    }

    fn difference_t<P: Polygons>(&self, operand: &P)
                            -> ClipperResult<Self> {
        Ok(Self::from_cpp(self.difference(operand)?.paths)?)
    }

    fn intersection<P: Polygons>(&self, operand : &P)
         -> ClipperResult<CppPolygons> {
        let mut clip = cpp::Clipper::new()?;
        
        clip.add_paths(self.to_cpp()?,
                       cpp::CppPolyType::PtSubject, true)?;
        clip.add_paths(operand.to_cpp()?,
                       cpp::CppPolyType::PtClip, true)?;

        let solution = clip.execute_closed(
            cpp::CppClipType::CtIntersection,
            cpp::CppPolyFillType::PftNonZero,
            cpp::CppPolyFillType::PftNonZero)?;

        Ok(CppPolygons { paths: solution })
    }

    fn intersection_t<P: Polygons>(&self, operand: &P)
                            -> ClipperResult<Self> {
        Ok(Self::from_cpp(self.intersection(operand)?.paths)?)
    }
}

impl<T> PolygonsOps for T where T: Polygons { }
    
#[cfg(test)]
mod tests {
    use super::*;
    use crate::simple_polygon::*;

    #[test]
    fn test_to_cpp() {
        let squares : SimplePolygons = vec![
            vec![ SimplePoint {x:0, y:0},
                  SimplePoint {x:5, y:0},
                  SimplePoint {x:5, y:5},
                  SimplePoint {x:0, y:5},
                  SimplePoint {x:0, y:0}
            ],
            vec![ SimplePoint {x:2, y:2},
                  SimplePoint {x:6, y:2},
                  SimplePoint {x:6, y:6},
                  SimplePoint {x:2, y:6},
                  SimplePoint {x:2, y:2}
            ]
        ];

        let cpp_squares : cpp::CppPaths = squares.to_cpp().unwrap();
        let cpp_path = cpp_squares.at(0).unwrap();

        let p = &cpp_path[0];
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);

        let p = &cpp_path[1];
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 0);
        
        let p = &cpp_path[2];
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 5);

        let p = &cpp_path[3];
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 5);

        let p = &cpp_path[4];
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);

        let cpp_path = cpp_squares.at(1).unwrap();

        let p = &cpp_path[0];
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 2);

        let p = &cpp_path[1];
        assert_eq!(p.x, 6);
        assert_eq!(p.y, 2);
        
        let p = &cpp_path[2];
        assert_eq!(p.x, 6);
        assert_eq!(p.y, 6);

        let p = &cpp_path[3];
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 6);

        let p = &cpp_path[4];
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 2);

        let new_squares = SimplePolygons::from_cpp(cpp_squares).unwrap();

        let new_path = &new_squares[0];

        let p = &new_path[0];
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);

        let p = &new_path[1];
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 0);
        
        let p = &new_path[2];
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 5);

        let p = &new_path[3];
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 5);

        let p = &new_path[4];
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);

        let new_path = &new_squares[1];

        let p = &new_path[0];
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 2);

        let p = &new_path[1];
        assert_eq!(p.x, 6);
        assert_eq!(p.y, 2);
        
        let p = &new_path[2];
        assert_eq!(p.x, 6);
        assert_eq!(p.y, 6);

        let p = &new_path[3];
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 6);

        let p = &new_path[4];
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 2);
    }

    #[test]
    fn test_simple_union() {
        let square1 : SimplePolygons = vec![
            vec![ SimplePoint {x:0, y:0},
                  SimplePoint {x:5, y:0},
                  SimplePoint {x:5, y:5},
                  SimplePoint {x:0, y:5},
                  SimplePoint {x:0, y:0}
            ]];
        let square2 : SimplePolygons = vec![
            vec![ SimplePoint {x:2, y:0},
                  SimplePoint {x:6, y:0},
                  SimplePoint {x:6, y:5},
                  SimplePoint {x:2, y:5},
                  SimplePoint {x:2, y:0}
            ]];

        // Something goes wrong with the TryFrom trait
        /*let result : SimplePolygons =
            square1.union(&square2).unwrap().try_into().unwrap();*/

        let result : SimplePolygons =
            square1.union_t(&square2).unwrap();
    }
}
