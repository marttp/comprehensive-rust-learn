#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point(x, y)
    }

    fn magnitude(self) -> f64 {
        f64::from(self.0.pow(2) + self.1.pow(2)).sqrt()
    }

    fn dist(&self, another_point: Point) -> f64 {
        let x = (self.0 - another_point.0).pow(2);
        let y = (self.1 - another_point.1).pow(2);
        f64::from(x + y).sqrt()
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            0: self.0 - other.0,
            1: self.1 - other.1,
        }
    }
}

#[derive(Debug)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    fn new() -> Polygon {
        Polygon { points: Vec::new() }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.0).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    fn perimeter(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }

        let mut result = 0.0;
        let mut last_point = self.points[0];
        for point in &self.points[1..] {
            result += last_point.dist(*point);
            last_point = *point;
        }
        result += last_point.dist(self.points[0]);
        result
    }
}

pub struct Circle {
    pub point: Point,
    pub radius: u8,
}

impl Circle {
    fn new(point: Point, radius: u8) -> Circle {
        Circle { point, radius }
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(polygon) => polygon.perimeter(),
            Shape::Circle(circle) => 2.0 * std::f64::consts::PI * f64::from(circle.radius),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}
