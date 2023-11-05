trait Component {
    fn operation(&self) -> String;
}

struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        "Component".into()
    }
}

struct DecoratorA {
    component: Box<dyn Component>,
}

impl DecoratorA {
    fn new(component: Box<dyn Component>) -> Self {
        Self { component }
    }
}

impl Component for DecoratorA {
    fn operation(&self) -> String {
        let base = self.component.operation();
        format!("DecoratorA({})", base)
    }
}

struct DecoratorB {
    component: Box<dyn Component>,
}

impl DecoratorB {
    fn new(component: Box<dyn Component>) -> Self {
        Self { component }
    }
}

impl Component for DecoratorB {
    fn operation(&self) -> String {
        let base = self.component.operation();
        format!("DecoratorB({})", base)
    }
}

fn main() {
    let component = Box::new(ConcreteComponent);
    let decorator_a = DecoratorA::new(component);
    let decorator_b = DecoratorB::new(Box::new(decorator_a));

    println!("Result: {}", decorator_b.operation());
}
