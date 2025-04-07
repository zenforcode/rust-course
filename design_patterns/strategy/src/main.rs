use std::rc::Rc;
// each duck has this two traits
// a duck can display itself on screen
// also can swim
trait DuckInterface {
    fn display(&self);
    fn swim(&self);
}
// there are ducks that they cannot fly
trait FlyBehavior {
    fn fly(&self);
}
// there ducks with different kind of quack
trait QuackBehavior {
    fn quack(&self);
}

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

struct Duck {
    fly_behavior: Rc<dyn FlyBehavior>,
    quack_behavior: Rc<dyn QuackBehavior>,
    name: String,
}

impl DuckInterface for Duck {
    fn display(&self) {
        println!("Hello, I am {}!", self.name);
    }
    fn swim(&self){
        println!("I can swim!");
    }
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
 
    fn set_flybehavior(&mut self, fb: Rc<dyn FlyBehavior>) {
        self.fly_behavior = fb;
    }
 
    fn set_quackbehavior(&mut self, qb: Rc<dyn QuackBehavior>) {
        self.quack_behavior = qb;
    }
}
 
// --- Duck Types ---
fn create_mallardduck() -> Duck {
    Duck::new("Mallard Duck", Rc::new(FlyWithWings), Rc::new(Quack))
}
 
fn create_rubberduck() -> Duck {
    Duck::new("Rubber Duck", Rc::new(FlyNoWay), Rc::new(Squeak))
}
 
fn create_modelduck() -> Duck {
    Duck::new("Model Duck", Rc::new(FlyNoWay), Rc::new(MuteQuack))
}
 
// --- Main Example ---
fn main() {
    let mallard = create_mallardduck();
    mallard.display();
    mallard.perform_fly();
    mallard.perform_quack();
 
    println!("\n--- Rubber Duck ---");
    let rubberduck = create_rubberduck();
    rubberduck.display();
    rubberduck.perform_fly();
    rubberduck.perform_quack();
 
    println!("\n--- Model Duck ---");
    let mut modelduck = create_modelduck();
    
    modelduck.display();
    modelduck.perform_fly();
    println!("Upgrading model duck with rocket power and mute him");
    modelduck.set_flybehavior(Rc::new(FlyRocketPowered));
    modelduck.set_quackbehavior(Rc::new(MuteQuack));
    modelduck.perform_fly();
    modelduck.perform_quack();
    modelduck.swim();
}