#[derive(Debug, PartialEq, Eq)]
pub struct Menu {
    description: String,
}

impl Menu {
    pub fn new() -> Self {
        println!("This is menu. You can pick what you want to do here.");
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }

    pub fn to_traffic_interception(&self) -> TrafficInterception {
        println!("Here you will see network intrusions");
        TrafficInterception {
            description: String::from("Here you will see network intrusions"),
        }
    }

    pub fn to_computers_information(&self) -> ComputersInformation {
        println!("Here you will see agents intrusions");
        ComputersInformation {
            description: String::from("Here you will see agents intrusions"),
        }
    }

    pub fn to_network_rules_changing(&self) -> NetworkRulesChanging {
        println!("Here you change rules for nets");
        NetworkRulesChanging {
            description: String::from("Here you change rules for nets"),
        }
    }

    pub fn to_computer_rules_changing(&self) -> ComputerRulesChanging {
        println!("Here you change rules for comps");
        ComputerRulesChanging {
            description: String::from("Here you change rules for comps"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TrafficInterception {
    description: String,
}

impl TrafficInterception {
    pub fn new() -> Self {
        TrafficInterception {
            description: String::from("Here you will see network intrusions"),
        }
    }
    pub fn to_menu(&self) -> Menu {
        println!("This is menu. You can pick what you want to do here.");
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ComputersInformation {
    description: String,
}

impl ComputersInformation {
    pub fn new() -> Self {
        ComputersInformation {
            description: String::from("Here you will see agents intrusions"),
        }
    }
    pub fn to_menu(&self) -> Menu {
        println!("This is menu. You can pick what you want to do here.");
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NetworkRulesChanging {
    description: String,
}

impl NetworkRulesChanging {
    pub fn new() -> Self {
        NetworkRulesChanging {
            description: String::from("Here you change rules for nets"),
        }
    }

    pub fn to_menu(&self) -> Menu {
        println!("This is menu. You can pick what you want to do here.");
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ComputerRulesChanging {
    description: String,
}

impl ComputerRulesChanging {
    pub fn new() -> Self {
        ComputerRulesChanging {
            description: String::from("Here you change rules for comps"),
        }
    }

    pub fn to_menu(&self) -> Menu {
        println!("This is menu. You can pick what you want to do here.");
        Menu {
            description: String::from("This is menu. You can pick what you want to do here."),
        }
    }
}
