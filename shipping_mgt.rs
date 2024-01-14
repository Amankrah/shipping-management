/// An enumeration representing the possible colors of a shipping box.
#[derive(PartialEq, Copy, Clone)]
enum Color {
    Brown,
    Red,
    Blue,
}

impl Color {
    /// Prints the color.
    fn print(&self) {
        match self {
            Color::Brown => println!("Color: Brown"),
            Color::Red => println!("Color: Red"),
            Color::Blue => println!("Color: Blue"),
        }
    }
}

/// A struct to represent the dimensions of a shipping box.
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    /// Prints the dimensions of the box.
    fn print(&self) {
        println!("Dimensions: width: {:?}m, height: {:?}m, depth: {:?}m", self.width, self.height, self.depth);
    }

    /// Calculates and returns the volume of the box.
    fn volume(&self) -> f64 {
        self.width * self.height * self.depth
    }
}

/// A struct representing a shipping box.
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    /// Constructs a new `ShippingBox`.
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    /// Prints details of the shipping box.
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {}kg", self.weight);
    }
}

/// A struct to manage a collection of shipping boxes.
struct ShippingManager {
    boxes: Vec<ShippingBox>,
}

impl ShippingManager {
    /// Constructs a new `ShippingManager`.
    fn new() -> Self {
        Self { boxes: vec![] }
    }

    /// Adds a shipping box to the manager.
    fn add_box(&mut self, my_box: ShippingBox) {
        self.boxes.push(my_box);
    }

    /// Displays details of all the boxes managed.
    fn display_all_boxes(&self) {
        for (index, my_box) in self.boxes.iter().enumerate() {
            println!("Box {}:", index + 1);
            my_box.print();
            println!();
        }
    }

    /// Calculates and returns the total weight of all boxes.
    fn total_weight(&self) -> f64 {
        self.boxes.iter().map(|b| b.weight).sum()
    }

    /// Filters and returns a vector of boxes matching the specified color.
    fn filter_by_color(&self, color: Color) -> Vec<&ShippingBox> {
        self.boxes.iter().filter(|&b| b.color == color).collect()
    }
}

fn main() {
    let mut manager = ShippingManager::new();

    let small_dimensions = Dimensions { width: 1.0, height: 2.0, depth: 3.0 };
    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);

    let large_dimensions = Dimensions { width: 2.0, height: 3.0, depth: 4.0 };
    let large_box = ShippingBox::new(10.0, Color::Brown, large_dimensions);

    manager.add_box(small_box);
    manager.add_box(large_box);

    manager.display_all_boxes();

    println!("Total weight of all boxes: {}kg", manager.total_weight());

    // Display all red boxes
    let red_boxes = manager.filter_by_color(Color::Red);
    println!("Red Boxes:");
    for my_box in red_boxes {
        my_box.print();
        println!();
    }
}
