mod strategy;
use strategy::{Duck, FlyBehavior, QuackBehavior};

fn main() {
    let mallard = Duck {
        fly_behavior: FlyBehavior::WithWings,
        quack_behavior: QuackBehavior::StandardQuack,
    };

    println!("{}", mallard.fly());
    println!("{}", mallard.quack());

    let mut decoy = Duck {
        fly_behavior: FlyBehavior::NoWay,
        quack_behavior: QuackBehavior::SilentQuack,
    };

    println!("{}", decoy.fly());
    println!("{}", decoy.quack());
    decoy.set_fly_behavior(FlyBehavior::RocketPowered);
    decoy.set_quack_behavior(QuackBehavior::StandardQuack);

    println!("{}", decoy.fly());
    println!("{}", decoy.quack());
}