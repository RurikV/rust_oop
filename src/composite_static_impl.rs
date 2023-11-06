// Define the component trait with common operations for both leaf and composite.
trait Component {
    fn operation(&self) -> String;
}

// Define leaf components that implement the component trait.
struct Leaf {
    name: String,
}

impl Component for Leaf {
    fn operation(&self) -> String {
        format!("Leaf({})", self.name)
    }
}

// Define the composite component that also implements the component trait.
struct Composite {
    name: String,
    children: Vec<Box<dyn Component>>,
}

impl Composite {
    fn add(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
    }
}

impl Component for Composite {
    fn operation(&self) -> String {
        let results = self.children.iter()
            .map(|child| child.operation())
            .collect::<Vec<_>>()
            .join("+");
        format!("Composite({}): [{}]", self.name, results)
    }
}

// Example usage
fn main() {
    let leaf1 = Leaf { name: "Leaf1".to_owned() };
    let leaf2 = Leaf { name: "Leaf2".to_owned() };
    
    let mut composite = Composite { name: "Composite1".to_owned(), children: Vec::new() };
    composite.add(Box::new(leaf1));
    composite.add(Box::new(leaf2));
    
    println!("{}", composite.operation());
}
