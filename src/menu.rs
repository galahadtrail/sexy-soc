pub struct Menu {
    description: String,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }

    pub fn to_traffic_interception(self) -> TrafficInterception {
        TrafficInterception {
            description: String::from("Here you will see network intrusions"),
        }
    }

    pub fn to_computers_information(self) -> ComputersInformation {
        ComputersInformation {
            description: String::from("Here you will see agents intrusions"),
        }
    }

    pub fn to_network_rules_changing(self) -> NetworkRulesChanging {
        NetworkRulesChanging {
            description: String::from("Here you change rules for nets"),
        }
    }

    pub fn to_computer_rules_changing(self) -> ComputerRulesChanging {
        ComputerRulesChanging {
            description: String::from("Here you change rules for comps"),
        }
    }
}

pub struct TrafficInterception {
    description: String,
}

impl TrafficInterception {
    pub fn new() -> Self {
        TrafficInterception {
            description: String::from("Here you will see network intrusions"),
        }
    }
    pub fn to_menu(self) -> Menu {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}
pub struct ComputersInformation {
    description: String,
}

impl ComputersInformation {
    pub fn new() -> Self {
        ComputersInformation {
            description: String::from("Here you will see agents intrusions"),
        }
    }
    pub fn to_menu(self) -> Menu {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}

pub struct NetworkRulesChanging {
    description: String,
}

impl NetworkRulesChanging {
    pub fn new() -> Self {
        NetworkRulesChanging {
            description: String::from("Here you change rules for nets"),
        }
    }

    pub fn to_menu(self) -> Menu {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}
pub struct ComputerRulesChanging {
    description: String,
}

impl ComputerRulesChanging {
    pub fn new() -> Self {
        ComputerRulesChanging {
            description: String::from("Here you change rules for comps"),
        }
    }

    pub fn to_menu(self) -> Menu {
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}
