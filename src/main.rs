mod strategy;
use std::{ fmt };

use strategy::{ Duck, FlyBehavior, QuackBehavior };

fn main() {
    println!("Strategy Pattern Demo:");
    strategy_pattern_demo();

    println!("Observer Pattern Demo:");
    observer_pattern_demo();
}

/// https://github.com/bethrobson/Head-First-Design-Patterns/tree/master/src/headfirst/designpatterns/observer/weather
fn strategy_pattern_demo() {
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

fn observer_pattern_demo() {}

trait Subject<P> {
    fn register_observer(&self, o: &impl Observer<P>) -> ();
    fn remove_observer(&self, o: &impl Observer<P>) -> ();
    fn notify_observers(&self) -> ();
    fn get_value(&self) -> P;
}

trait Observer<P> {
    fn update(&mut self, payload: P) -> ();
}

#[derive(Debug, Clone, Copy)]
struct WeatherData {
    temperature: f32,
    humidity: f32,
    pressure: f32,
}

enum WeatherDisplayKind {
    CurrentConditions(WeatherData),
    Statistics(WeatherData),
    Forecast(WeatherData),
}
struct WeatherDisplay {
    data: WeatherData,
    kind: WeatherDisplayKind,
}
impl WeatherDisplay {
    fn new(&self, kind: WeatherDisplayKind, station: &WeatherStation) -> Self {
        let display = WeatherDisplay { data: station.get_value(), kind };
        &station.register_observer(&display);
        display
    }
    fn display(&self) {
        match self.kind {
            Self::CurrentConditions => println!("Current conditions: {}F degrees and {}% humidity", self.data.temperature, self.data.humidity),
            Self::Statistics => println!("")
        }
    }
}
impl Observer<WeatherData> for WeatherDisplay {
    fn update(&mut self, data: WeatherData) -> () {
        self.data = data;
    }
}

struct WeatherStation {
    subscribers: Vec<WeatherDisplay>,
}
impl Default for WeatherStation {
    fn default() -> Self {
        WeatherStation { subscribers: vec![] }
    }
}
impl Subject<WeatherData> for WeatherStation {
    fn register_observer(&self, o: &impl Observer<WeatherData>) -> () {
        self.subscribers.push(o);
    }

    fn remove_observer(&self, o: &impl Observer<WeatherData>) -> () {
        todo!()
    }

    fn notify_observers(&self) -> () {
        todo!()
    }
    fn get_value(&self) -> WeatherData {
        todo!()
    }
}
