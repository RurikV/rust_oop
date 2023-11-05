use std::any::Any;

trait Visitor {
    fn visit(&mut self, element: &dyn Element);
}

trait Element: Any {
    fn accept(&self, visitor: &mut dyn Visitor);
    fn as_any(&self) -> &dyn Any; // Add this method to support downcasting
}

struct Circle {
    radius: f64,
}

impl Element for Circle {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Square {
    side: f64,
}

impl Element for Square {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct ConcreteVisitor;

impl Visitor for ConcreteVisitor {
    fn visit(&mut self, element: &dyn Element) {
        if let Some(circle) = element.as_any().downcast_ref::<Circle>() {
            println!("Visiting circle with radius {}", circle.radius);
        } else if let Some(square) = element.as_any().downcast_ref::<Square>() {
            println!("Visiting square with side {}", square.side);
        } else {
            println!("Visiting an unknown element type");
        }
    }
}

fn main() {
    let elements: Vec<Box<dyn Element>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 10.0 }),
    ];

    let mut visitor = ConcreteVisitor;

    for element in &elements {
        element.accept(&mut visitor);
    }
}
