trait Shape {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn total_area<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    // Create a vector of shapes. Note that we use trait objects here.
    let shapes: Vec<&dyn Shape> = vec![&circle, &rectangle];

    let total: f64 = shapes.iter().map(|shape| shape.area()).sum();

    println!("Total area: {}", total);

    // Alternatively, using the generic function for each type separately
    let circles = vec![Circle { radius: 3.0 }, Circle { radius: 4.0 }];
    let rectangles = vec![
        Rectangle {
            width: 2.0,
            height: 3.0,
        },
        Rectangle {
            width: 1.0,
            height: 2.0,
        },
    ];

    println!("Total area of circles: {}", total_area(&circles));
    println!("Total area of rectangles: {}", total_area(&rectangles));
}
