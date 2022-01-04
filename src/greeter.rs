use chrono::{NaiveTime, Timelike};

struct Greeter {
    time: TimeOfDay,
}

enum TimeOfDay {
    Morning,
    Afternoon,
    Evening,
    Night,
}

impl TimeOfDay {
    fn new(time: NaiveTime) -> TimeOfDay {
        if Self::is_night(time) {
            TimeOfDay::Night
        } else if Self::is_evening(time) {
            TimeOfDay::Evening
        } else if Self::is_afternoon(time) {
            TimeOfDay::Afternoon
        } else {
            TimeOfDay::Morning
        }
    }

    fn is_night(time: NaiveTime) -> bool {
        time.hour() >= 22 || time.hour() < 6
    }

    fn is_evening(time: NaiveTime) -> bool {
        time.hour() >= 18 && time.hour() < 22
    }

    fn is_afternoon(time: NaiveTime) -> bool {
        time.hour() >= 12 && time.hour() < 18
    }
}

impl Greeter {
    fn new(time: NaiveTime) -> Greeter {
        Greeter {
            time: TimeOfDay::new(time)
        }
    }

    fn greet(&self, name: &str) -> String {
        let name = capitalize(name.trim().to_string());

        match &self.time {
            TimeOfDay::Night => format!("Good night {}", name),
            TimeOfDay::Evening => format!("Good evening {}", name),
            TimeOfDay::Afternoon => format!("Hello {}", name),
            TimeOfDay::Morning => format!("Good morning {}", name),
        }
    }
}

fn capitalize(string: String) -> String {
    let mut chars = string.chars();

    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn morning() -> NaiveTime {
        NaiveTime::from_hms(6, 0, 0)
    }

    fn afternoon() -> NaiveTime {
        NaiveTime::from_hms(12, 0, 0)
    }

    fn evening() -> NaiveTime {
        NaiveTime::from_hms(18, 0, 0)
    }

    fn night() -> NaiveTime {
        NaiveTime::from_hms(22, 0, 0)
    }

    #[test]
    fn it_returns_hello_name() {
        let greeter = Greeter::new(afternoon());

        assert_eq!("Hello Mark", greeter.greet("Mark"));
    }

    #[test]
    fn it_trims_the_name() {
        let greeter = Greeter::new(afternoon());

        assert_eq!("Hello Mark", greeter.greet("     Mark     "));
    }

    #[test]
    fn it_capitalizes_the_name() {
        let greeter = Greeter::new(afternoon());

        assert_eq!("Hello Mark", greeter.greet("mark"));
    }

    #[test]
    fn it_returns_good_morning_in_the_morning() {
        let greeter = Greeter::new(morning());

        assert_eq!("Good morning Mark", greeter.greet("Mark"));
    }

    #[test]
    fn it_returns_good_evening_in_the_evening() {
        let greeter = Greeter::new(evening());

        assert_eq!("Good evening Mark", greeter.greet("Mark"));
    }

    #[test]
    fn it_returns_good_night_during_the_night() {
        let greeter = Greeter::new(night());

        assert_eq!("Good night Mark", greeter.greet("Mark"));
    }
}