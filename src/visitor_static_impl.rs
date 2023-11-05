trait Visitor {
    fn visit_circle(&mut self, circle: &Circle);
    fn visit_square(&mut self, square: &Square);
}

struct ConcreteVisitor;

impl Visitor for ConcreteVisitor {
    fn visit_circle(&mut self, circle: &Circle) {
        println!("Visiting circle with radius {}", circle.radius);
    }

    fn visit_square(&mut self, square: &Square) {
        println!("Visiting square with side {}", square.side);
    }
}

trait Visitable {
    fn accept(&self, visitor: &mut dyn Visitor);
}

struct Circle {
    radius: f64,
}

impl Visitable for Circle {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_circle(self);
    }
}

struct Square {
    side: f64,
}

impl Visitable for Square {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_square(self);
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 10.0 };

    let mut visitor = ConcreteVisitor;

    circle.accept(&mut visitor);
    square.accept(&mut visitor);
}
