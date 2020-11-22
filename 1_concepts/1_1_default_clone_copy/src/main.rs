use crate::smart_default_point::Point as SmartPoint;
use crate::point_default_impl::Point as ImplPoint;

#[derive(Default, Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Debug)]
struct Polyline {
    pub points: Vec<Point>,
}

impl Polyline {
    fn from_point(p: Point) -> Self {
        Self { points: vec![p] }
    }

    fn from_vec(vec: Vec<Point>) -> Self {
        Self { points: vec}
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }
}

fn main() {
    // instantiating points from the task
    let p1: Point = Default::default();
    let p2 = Point::default();

    // instantiating points with different Default impl
    let p3 = SmartPoint::default();
    let p4 = ImplPoint::default();

    // polyline 1 from the point
    let mut polyline = Polyline::from_point(p1);

    let p5 = p2;
    let p6 = p1;
    let p7 = p6;

    // adding points to the polyline 1
    polyline.add_point(p5);
    polyline.add_point(p7);

    // polyline 2 from polyline (clone, so we don't move)
    let polyline2 = polyline.clone();

    // polyline 3 from the vector of points
    let polyline3 = Polyline::from_vec(vec![p1,p5,p6,p7]);

    println!("Point 1 - {:?}", p1 );
    println!("Point 2 - [{x},{y}]", x = p2.x, y=p2.y);
    println!("Point 3 - [{x},{y}]", x = p3.x, y=p3.y);
    println!("Point 3 - [{x},{y}]", x = p4.x, y=p4.y);
    println!("Polyline - {:#?} ", polyline);
    println!("Polyline2 - {:#?} ", polyline2);
    println!("Polyline2 - {:#?} ", polyline3);

    // if compile - then the clone is passed to the println, so we don't move the variable.
    polyline.add_point(p2);
}

mod smart_default_point {
    use smart_default::SmartDefault;

    #[derive(SmartDefault)]
    pub struct Point {
        #[default = 12.0]
        pub x: f32,
        #[default(Default::default())]
        pub y: f32
    }
}

mod point_default_impl {
    // Eq is derived from partialeq, as i32 has transitive equality.
    // f32 doesn't, as NaN!=NaN
    #[derive(Debug, PartialEq, Eq)]
    pub struct Point {
        pub x: i32,
        pub y: i32
    }

    impl Default for Point {
        fn default() -> Self {
            Self {
                x: 42,
                y: 42
            }
        }
    }

    // implemented derived traits manually

    // impl PartialEq for Point {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.x == other.x && self.y == other.y
    //     }
    // }

    //impl Eq for Point { }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_default() {
        assert_eq!(point_default_impl::Point::default(), point_default_impl::Point {
            x: 42,
            y: 42,
        })
    }
}