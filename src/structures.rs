use core::fmt;

struct Point2D {
    x: f32,
    y: f32
}
struct Rectangle {
    top_left: Point2D,
    bottom_right: Point2D
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left: Point2D { x: tl_nested_x, y: tl_nested_y }, bottom_right: Point2D { x: br_nested_x, y: br_nested_y } } = rect;

    (br_nested_x - tl_nested_x) * (tl_nested_y - br_nested_y)
}

pub fn test() -> () {
    let point = Point2D { x: 2.0, y: 4.2 };

    let rect = Rectangle {
        top_left: point,
        bottom_right: Point2D { x: 4.5, y: 1.3 }
    };

    println!("Rectangle defined by points:      {} and {}", rect.top_left, Point2D { x: 4.5, y: 1.3 });
}