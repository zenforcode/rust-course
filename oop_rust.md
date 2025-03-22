# 🦆 Strategy Pattern in Rust: Duck Behavior Example

Ducks can have different flying and quacking behaviors that can be swapped at runtime.

## 📁 Structure

- Traits for `FlyBehavior` and `QuackBehavior`
- Implementations like `FlyWithWings`, `FlyNoWay`, `Quack`, `MuteQuack`
- `Duck` struct with dynamic behaviors
- Specific duck types like `MallardDuck`, `RubberDuck`, `ModelDuck`

---

## 🦀 Rust Code

```rust
use std::rc::Rc;

// --- Behavior Traits ---
trait FlyBehavior {
    fn fly(&self);
}

trait QuackBehavior {
    fn quack(&self);
}

// --- Fly Behavior Implementations ---
struct FlyWithWings;
impl FlyBehavior for FlyWithWings {
    fn fly(&self) {
        println!("I'm flying with wings!");
    }
}

struct FlyNoWay;
impl FlyBehavior for FlyNoWay {
    fn fly(&self) {
        println!("I can't fly.");
    }
}

struct FlyRocketPowered;
impl FlyBehavior for FlyRocketPowered {
    fn fly(&self) {
        println!("I'm flying with a rocket!");
    }
}

// --- Quack Behavior Implementations ---
struct Quack;
impl QuackBehavior for Quack {
    fn quack(&self) {
        println!("Quack!");
    }
}

struct MuteQuack;
impl QuackBehavior for MuteQuack {
    fn quack(&self) {
        println!("...");
    }
}

struct Squeak;
impl QuackBehavior for Squeak {
    fn quack(&self) {
        println!("Squeak!");
    }
}

// --- Duck Struct ---
struct Duck {
    fly_behavior: Rc<dyn FlyBehavior>,
    quack_behavior: Rc<dyn QuackBehavior>,
    name: String,
}

impl Duck {
    fn new(name: &str, fly: Rc<dyn FlyBehavior>, quack: Rc<dyn QuackBehavior>) -> Self {
        Duck {
            name: name.to_string(),
            fly_behavior: fly,
            quack_behavior: quack,
        }
    }

    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }

    fn perform_quack(&self) {
        self.quack_behavior.quack();
    }

    fn set_fly_behavior(&mut self, fb: Rc<dyn FlyBehavior>) {
        self.fly_behavior = fb;
    }

    fn set_quack_behavior(&mut self, qb: Rc<dyn QuackBehavior>) {
        self.quack_behavior = qb;
    }

    fn display(&self) {
        println!("I'm a {}!", self.name);
    }
}

// --- Duck Types ---
fn create_mallard_duck() -> Duck {
    Duck::new("Mallard Duck", Rc::new(FlyWithWings), Rc::new(Quack))
}

fn create_rubber_duck() -> Duck {
    Duck::new("Rubber Duck", Rc::new(FlyNoWay), Rc::new(Squeak))
}

fn create_model_duck() -> Duck {
    Duck::new("Model Duck", Rc::new(FlyNoWay), Rc::new(MuteQuack))
}

// --- Main Example ---
fn main() {
    let mut mallard = create_mallard_duck();
    mallard.display();
    mallard.perform_fly();
    mallard.perform_quack();

    println!("\n--- Rubber Duck ---");
    let rubber_duck = create_rubber_duck();
    rubber_duck.display();
    rubber_duck.perform_fly();
    rubber_duck.perform_quack();

    println!("\n--- Model Duck ---");
    let mut model_duck = create_model_duck();
    model_duck.display();
    model_duck.perform_fly();
    println!("Upgrading model duck with rocket power...");
    model_duck.set_fly_behavior(Rc::new(FlyRocketPowered));
    model_duck.perform_fly();
}
