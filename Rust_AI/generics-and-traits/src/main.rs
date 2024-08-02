/*
Generics are the building blocks of abstraction in Rust. They allow us to
define functions, structs, enums, and methods that can adapt to serve
multiple purposes.
*/

/*
Traits in Rust are akin to a contract or an interface in other languages. They
define a set of methods that a type must implement, enabling
polymorphism. Traits are integral to Rust's generics system, as they specify
the behavior that a generic type must have. For example, we can create a
trait that encapsulates the functionality of a data structure that allows for
adding elements and calculating the mean:
*/

// Generics are used for broad abstractions and Traits are used for specifics

fn find_min<T: PartialOrd>(data: &[T]) -> Option<&T> {
    data.iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
}

trait MeanCalculator {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn calculate_mean(&self) -> f64;
}

struct IntegerMeanCalculator {
    data: Vec<i32>,
    total: i32,
}

impl MeanCalculator for IntegerMeanCalculator {
    type Item = i32;
    fn add(&mut self, item: Self::Item) {
        self.data.push(item);
        self.total += item;
    }

    fn calculate_mean(&self) -> f64 {
        self.total as f64 / self.data.len() as f64
    }
}

fn main() {
    // Generics used here
    let integers = vec![3, 1, 4, 1, 5];
    let floats = vec![1.889, 3.14598, 1.16180];

    println!("Minimum integer: {:?}", find_min(&integers));
    println!("Minimum float: {:?}", find_min(&floats));

    // Traits used here
    let mut calculator = IntegerMeanCalculator {
        data: vec![],
        total: 0,
    };
    calculator.add(10);
    calculator.add(20);
    calculator.add(30);

    println!("Mean: {}", calculator.calculate_mean());
}
