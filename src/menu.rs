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
            description: String::from("Here you will see network intrusions"),
        }
    }

    fn to_computers_information(self) -> ComputersInformation {
        ComputersInformation {
            description: String::from("Here you will see agents intrusions"),
        }
    }

    fn to_network_rules_changing(self) -> NetworkRulesChanging {
        NetworkRulesChanging {
            description: String::from("Here you change rules for nets"),
        }
    }

    fn to_computer_rules_changing(self) -> ComputerRulesChanging {
        ComputerRulesChanging {
            description: String::from("Here you change rules for comps"),
        }
    }
}

struct TrafficInterception {
    description: String,
}

impl TrafficInterception {
    fn new() -> Self {
        TrafficInterception {
            description: String::from("Here you will see network intrusions"),
        }
    }
    fn to_menu(self) -> Menu {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}
struct ComputersInformation {
    description: String,
}

impl ComputersInformation {
    fn new() -> Self {
        ComputersInformation {
            description: String::from("Here you will see agents intrusions"),
        }
    }
    fn to_menu(self) -> Menu {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}

struct NetworkRulesChanging {
    description: String,
}

impl NetworkRulesChanging {
    fn new() -> Self {
        NetworkRulesChanging {
            description: String::from("Here you change rules for nets"),
        }
    }

    fn to_menu(self) -> Menu {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}
struct ComputerRulesChanging {
    description: String,
}

impl ComputerRulesChanging {
    fn new() -> Self {
        ComputerRulesChanging {
            description: String::from("Here you change rules for comps"),
        }
    }

    fn to_menu(self) -> Menu {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}
