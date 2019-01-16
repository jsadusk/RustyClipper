use crate::error::*;
use crate::cpp;

pub struct CppPolygons(cpp::CppPaths);
pub struct CppOpenPaths(cpp::CppPaths);

pub trait Polygons: Sized {
    fn to_cpp(&self) -> ClipperResult<cpp::CppPaths>;
    fn from_cpp(other: cpp::CppPaths) -> ClipperResult<Self>;

    fn union_c<P: Polygons>(&self, operand : &P)
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

        Ok(CppPolygons(solution))
    }

    fn union_t<P: Polygons>(&self, operand: &P)
                            -> ClipperResult<Self> {
        Ok(Self::from_cpp(self.union_c(operand)?.0)?)
    }

    fn union<P: Polygons, R: Polygons>(&self, operand: &P)
                                       -> ClipperResult<R> {
        R::from_cpp(self.union_c(operand)?.0)
    }
    
    fn difference_c<P: Polygons>(&self, operand : &P)
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

        Ok(CppPolygons(solution))
    }

    fn difference_t<P: Polygons>(&self, operand: &P)
                            -> ClipperResult<Self> {
        Ok(Self::from_cpp(self.difference_c(operand)?.0)?)
    }

    fn difference<P: Polygons, R: Polygons>(&self, operand: &P)
                                            -> ClipperResult<R> {
        R::from_cpp(self.difference_c(operand)?.0)
    }
    
    fn intersection_c<P: Polygons>(&self, operand : &P)
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

        Ok(CppPolygons(solution))
    }

    fn intersection_t<P: Polygons>(&self, operand: &P)
                            -> ClipperResult<Self> {
        Ok(Self::from_cpp(self.intersection_c(operand)?.0)?)
    }

    fn intersection<P: Polygons, R: Polygons>(&self, operand: &P)
                                              -> ClipperResult<R> {
        R::from_cpp(self.intersection_c(operand)?.0)
    }

    fn offset_c(&self, delta: f64) -> ClipperResult<CppPolygons> {
        let mut clipoff = cpp::ClipperOffset::new()?;
        clipoff.add_paths(self.to_cpp()?,
                          cpp::CppJoinType::JtRound,
                          cpp::CppEndType::EtClosedPolygon)?;
        let solution = clipoff.execute(delta)?;
        Ok(CppPolygons(solution))
    }

    fn offset_t(&self, delta: f64) ->ClipperResult<Self> {
        Self::from_cpp(self.offset_c(delta)?.0)
    }

    fn offset<R: Polygons>(&self, delta: f64)
                                        -> ClipperResult<R> {
        R::from_cpp(self.offset_c(delta)?.0)
    }
}

impl Polygons for CppPolygons {
    fn to_cpp(&self) -> ClipperResult<cpp::CppPaths> {
        Ok(self.0.pseudoclone())
    }

    fn from_cpp(other: cpp::CppPaths) -> ClipperResult<Self> {
        Ok(CppPolygons(other))
    }
}

pub trait OpenPaths : Sized {
    fn to_cpp(&self) -> ClipperResult<cpp::CppPaths>;
    fn from_cpp(other: cpp::CppPaths) -> ClipperResult<Self>;

    fn difference_c<P: Polygons>(&self, operand : &P)
         -> ClipperResult<CppOpenPaths> {
        let mut clip = cpp::Clipper::new()?;
        
        clip.add_paths(self.to_cpp()?,
                       cpp::CppPolyType::PtSubject, false)?;
        clip.add_paths(operand.to_cpp()?,
                       cpp::CppPolyType::PtClip, true)?;

        let solution = clip.execute_closed(
            cpp::CppClipType::CtDifference,
            cpp::CppPolyFillType::PftNonZero,
            cpp::CppPolyFillType::PftNonZero)?;

        Ok(CppOpenPaths(solution))
    }

    fn difference_t<P: Polygons>(&self, operand: &P)
                            -> ClipperResult<Self> {
        Ok(Self::from_cpp(self.difference_c(operand)?.0)?)
    }

    fn difference<P: Polygons, R: OpenPaths>(&self, operand: &P)
                                            -> ClipperResult<R> {
        R::from_cpp(self.difference_c(operand)?.0)
    }
    
    fn intersection_c<P: Polygons>(&self, operand : &P)
         -> ClipperResult<CppOpenPaths> {
        let mut clip = cpp::Clipper::new()?;
        
        clip.add_paths(self.to_cpp()?,
                       cpp::CppPolyType::PtSubject, false)?;
        clip.add_paths(operand.to_cpp()?,
                       cpp::CppPolyType::PtClip, true)?;

        let solution = clip.execute_closed(
            cpp::CppClipType::CtIntersection,
            cpp::CppPolyFillType::PftNonZero,
            cpp::CppPolyFillType::PftNonZero)?;

        Ok(CppOpenPaths(solution))
    }

    fn intersection_t<P: Polygons>(&self, operand: &P)
                            -> ClipperResult<Self> {
        Ok(Self::from_cpp(self.intersection_c(operand)?.0)?)
    }

    fn intersection<P: Polygons, R: OpenPaths>(&self, operand: &P)
                                              -> ClipperResult<R> {
        R::from_cpp(self.intersection_c(operand)?.0)
    }

    fn offset_c(&self, delta: f64) -> ClipperResult<CppOpenPaths> {
        let mut clipoff = cpp::ClipperOffset::new()?;
        clipoff.add_paths(self.to_cpp()?,
                          cpp::CppJoinType::JtRound,
                          cpp::CppEndType::EtOpenRound)?;
        let solution = clipoff.execute(delta)?;
        Ok(CppOpenPaths(solution))
    }

    fn offset_t(&self, delta: f64) ->ClipperResult<Self> {
        Self::from_cpp(self.offset_c(delta)?.0)
    }

    fn offset<R: OpenPaths>(&self, delta: f64)
                                        -> ClipperResult<R> {
        R::from_cpp(self.offset_c(delta)?.0)
    }
}

impl OpenPaths for CppOpenPaths {
    fn to_cpp(&self) -> ClipperResult<cpp::CppPaths> {
        Ok(self.0.pseudoclone())
    }

    fn from_cpp(other: cpp::CppPaths) -> ClipperResult<Self> {
        Ok(CppOpenPaths(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simple_polygon::*;
    use crate::simple_polygon::SimplePolygons;

    #[test]
    fn test_to_cpp() {
        let squares = SimplePolygons(vec![
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
        ]);

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

        let new_path = &new_squares.0[0];

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

        let new_path = &new_squares.0[1];

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
        let square1 = SimplePolygons(vec![
            vec![ SimplePoint {x:0, y:0},
                  SimplePoint {x:5, y:0},
                  SimplePoint {x:5, y:5},
                  SimplePoint {x:0, y:5},
                  SimplePoint {x:0, y:0}
            ]]);
        let square2 = SimplePolygons(vec![
            vec![ SimplePoint {x:2, y:0},
                  SimplePoint {x:6, y:0},
                  SimplePoint {x:6, y:5},
                  SimplePoint {x:2, y:5},
                  SimplePoint {x:2, y:0}
            ]]);

        let square3 = SimplePolygons(vec![
            vec![ SimplePoint {x:2, y:0},
                  SimplePoint {x:6, y:0},
                  SimplePoint {x:6, y:6},
                  SimplePoint {x:2, y:6},
                  SimplePoint {x:2, y:0}
            ]]);

        let result : SimplePolygons =
            square1.union(&square2).unwrap();

        let result =
            square1.union_t(&square2).unwrap();

        let result : SimplePolygons =
            square1.union_c(&square2).unwrap().difference(&square3).unwrap();
    }

    #[test]
    fn test_offset() {
        let square = SimplePolygons(vec![
            vec![ SimplePoint {x:0, y:0},
                  SimplePoint {x:5, y:0},
                  SimplePoint {x:5, y:5},
                  SimplePoint {x:0, y:5},
                  SimplePoint {x:0, y:0}
            ]]);

        let off = square.offset_t(1.0).unwrap();

        let path = &off.0[0];
    }
}
