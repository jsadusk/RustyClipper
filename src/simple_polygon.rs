use crate::error::*;
use crate::cpp;
use crate::traits::*;

pub struct SimplePoint {
    pub x: i64,
    pub y: i64
}

pub type SimplePolygon = Vec<SimplePoint>;
pub type SimplePolygons = Vec<SimplePolygon>;

impl Polygons for SimplePolygons {
    fn to_cpp(&self) -> ClipperResult<cpp::CppPaths> {
        let result = cpp::CppPaths::new_sized(self.len())?;

        for (i, path) in self.iter().enumerate() {
            let mut cpp_path = result.at(i)?;

            cpp_path.resize(path.len())?;

            for (i, point) in path.iter().enumerate() {
                let mut cpp_point = &mut cpp_path[i];
                cpp_point.x = point.x;
                cpp_point.y = point.y;
            }
        }

        Ok(result)
    }

    fn from_cpp(cpp_paths: cpp::CppPaths)
                -> ClipperResult<SimplePolygons> {
        let size = cpp_paths.len();
        let mut result = SimplePolygons::with_capacity(size);

        for i in 0..size {
            let cpp_path = cpp_paths.at(i)?;
            let size = cpp_path.len();
            let mut path = SimplePolygon::with_capacity(size);
            for i in 0..size {
                let cpp_point = &cpp_path[i];
                path.push(SimplePoint { x: cpp_point.x,
                                        y: cpp_point.y });
            }
            result.push(path);
        }

        Ok(result)
    }
}

