use std::f64::consts::PI;

enum ShapeType {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

// --- プロダクト ---
trait Shape {
    fn get_area(&self) -> f64;
}
// --- プロダクト ---

// --- 具象プロダクト ---
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}
// --- 具象プロダクト ---

// --- クリエイタ ---
struct ShapeFactory;

impl ShapeFactory {
    fn new_shape(s: ShapeType) -> Box<dyn Shape> {
        match s {
            ShapeType::Circle { radius } => Box::new(
                Circle { radius }
            ),
            ShapeType::Rectangle { width, height } => Box::new(
                Rectangle { width, height }
            ),
        }
    }
}
// --- クリエイタ ---

fn main() {
    let circle = ShapeFactory::new_shape(
        ShapeType::Circle { radius: 5.0 }
    );
    println!("Circle Area: {}", circle.get_area());

    let rectangle = ShapeFactory::new_shape(
        ShapeType::Rectangle { width: 10.0, height: 5.0 }
    );
    println!("Rectangle Area: {}", rectangle.get_area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area_circle() {
        let radius = 5.0;
        let circle = Circle{radius: radius};
        let result = radius * radius * PI; 
        assert_eq!(circle.get_area(), result);
    }

    #[test]
    fn test_get_area_rectangle() {
        let width = 7.0;
        let height = 2.0;
        let rectangle = Rectangle{width: width, height: height};
        let result = width * height; 
        assert_eq!(rectangle.get_area(), result);
    }

    #[test]
    fn test_shape_factory_circle() {
        let radius = 5.0;
        let circle = ShapeFactory::new_shape(ShapeType::Circle { radius: radius });
        let result = radius * radius * PI; 
        assert_eq!(circle.get_area(), result);
    }

    #[test]
    fn test_shape_factory_rectangle() {
        let width = 7.0;
        let height = 2.0;
        let rectangle = ShapeFactory::new_shape(
            ShapeType::Rectangle { width: width, height: height }
        );
        let result = width * height;
        assert_eq!(rectangle.get_area(), result);
    }
}
