trait Component {
    fn operation(&self) -> String;
}

struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        "Component".into()
    }
}

struct DecoratorA<T: Component> {
    component: T,
}

impl<T: Component> Component for DecoratorA<T> {
    fn operation(&self) -> String {
        let base = self.component.operation();
        format!("DecoratorA({})", base)
    }
}

struct DecoratorB<T: Component> {
    component: T,
}

impl<T: Component> Component for DecoratorB<T> {
    fn operation(&self) -> String {
        let base = self.component.operation();
        format!("DecoratorB({})", base)
    }
}

fn main() {
    let component = ConcreteComponent;
    let decorator_a = DecoratorA { component };
    let decorator_b = DecoratorB {
        component: decorator_a,
    };

    println!("Result: {}", decorator_b.operation());
}
