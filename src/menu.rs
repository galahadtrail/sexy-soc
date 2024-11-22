struct Menu {
    description: String,
}

impl Menu {
    fn new() -> Self {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }

    fn to_traffic_interception(self) -> TrafficInterception {
        TrafficInterception {
            description: String::from("Here you will see network inrusions"),
        }
    }
}

struct TrafficInterception {
    description: String,
}

struct ComputersInformation {
    description: String,
}

struct RulesChanging {
    description: String,
}
