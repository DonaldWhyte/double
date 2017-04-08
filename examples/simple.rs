extern crate double;

use double::Mock;

trait Dependency: Clone {
    fn greet<S: AsRef<str>>(&self, name: S) {
        println!("Hello, {}", name.as_ref())
    }
}

struct Consumer<T: Dependency> {
    dependency: T,
}

impl<T: Dependency> Consumer<T> {
    fn new(dependency: &T) -> Self {
        Consumer { dependency: dependency.clone() }
    }

    fn greet_everyone<S: AsRef<str>>(&self, names: Vec<S>) {
        for name in names {
            self.dependency.greet(name)
        }
    }
}

#[derive(Debug, Clone)]
struct MockDependency {
    pub greet: Mock<String, ()>,
}

impl Dependency for MockDependency {
    fn greet<S: AsRef<str>>(&self, name: S) {
        self.greet.call(name.as_ref().to_string())
    }
}

impl Default for MockDependency {
    fn default() -> Self {
        MockDependency { greet: Mock::default() }
    }
}

fn main() {
    let mock = MockDependency::default();
    let consumer = Consumer::new(&mock);

    consumer.greet_everyone(vec!["Fido", "Spot", "Princess"]);

    assert_eq!(mock.greet.num_calls(), 3);

    assert!(mock.greet.called_with("Fido"));
    assert!(mock.greet.called_with("Spot"));
    assert!(mock.greet.called_with("Princess"));
}