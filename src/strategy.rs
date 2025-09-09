pub trait Fly {
    fn fly(&self) -> String;
}

trait Quack {
    fn quack(&self) -> String;
}

#[derive(Debug)]
pub enum FlyBehavior {
    WithWings,
    NoWay,
    RocketPowered,
}
impl Fly for FlyBehavior {
    fn fly(&self) -> String {
        match self {
            FlyBehavior::WithWings => fly_with_wings(),
            FlyBehavior::NoWay => fly_no_way(),
            FlyBehavior::RocketPowered => fly_rocket_powered(),
        }
    }
}

pub fn fly_with_wings() -> String {
    String::from("I'm flying!!")
}
pub fn fly_no_way() -> String {
    String::from("I can't fly")
}
pub fn fly_rocket_powered() -> String {
    String::from("I'm flying with a rocket!")
}

#[derive(Debug)]
pub enum QuackBehavior {
    StandardQuack,
    SilentQuack,
}
impl Quack for QuackBehavior {
    fn quack(&self) -> String {
        match self {
            QuackBehavior::StandardQuack => standard_quack(),
            QuackBehavior::SilentQuack => silent_quack(),
        }
    }
}

pub fn standard_quack() -> String {
    String::from("Quack")
}
pub fn silent_quack() -> String {
    String::from("<<Silence>>")
}

#[derive(Debug)]
pub struct Duck {
    pub fly_behavior: FlyBehavior,
    pub quack_behavior: QuackBehavior,
}
impl Duck {
    pub fn set_fly_behavior(&mut self, fb: FlyBehavior) {
        self.fly_behavior = fb;
    }
    pub fn set_quack_behavior(&mut self, qb: QuackBehavior) {
        self.quack_behavior = qb;
    }
    pub fn fly(&self) -> String {
        self.fly_behavior.fly()
    }
    pub fn quack(&self) -> String {
        self.quack_behavior.quack()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fly_with_wings() {
        let fly_behavior = FlyBehavior::WithWings;
        assert_eq!(fly_behavior.fly(), "I'm flying!!");
    }

    #[test]
    fn test_fly_no_way() {
        let fly_behavior = FlyBehavior::NoWay;
        assert_eq!(fly_behavior.fly(), "I can't fly");
    }

    #[test]
    fn test_set_fly_behavior() {
        let mut duck = Duck {
            fly_behavior: FlyBehavior::NoWay,
            quack_behavior: QuackBehavior::StandardQuack,
        };
        assert_eq!(duck.fly(), "I can't fly");
        duck.set_fly_behavior(FlyBehavior::WithWings);
        assert_eq!(duck.fly(), "I'm flying!!");
    }

    #[test]
    fn test_set_quack_behavior() {
        let mut duck = Duck {
            fly_behavior: FlyBehavior::WithWings,
            quack_behavior: QuackBehavior::SilentQuack,
        };
        assert_eq!(duck.quack(), "<<Silence>>");
        duck.set_quack_behavior(QuackBehavior::StandardQuack);
        assert_eq!(duck.quack(), "Quack");
    }
}