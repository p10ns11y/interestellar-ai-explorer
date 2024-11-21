pub mod models {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Planet {
        pub name: String,
        pub environment: Environment,
        pub species: HashMap<String, Species>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Environment {
        EarthLike,
        IceWorld,
        Desert,
        GasGiant,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Trait {
        Intelligent,
        Aggressive,
        Peaceful,
        Adaptive,
    }

    impl Trait {
        // pub fn from_str(s: &str) -> Self {
        //     match s {
        //         "Intelligent" => Trait::Intelligent,
        //         "Aggressive" => Trait::Aggressive,
        //         "Peaceful" => Trait::Peaceful,
        //         "Adaptive" => Trait::Adaptive,
        //         _ => Trait::Intelligent,
        //     }
        // }

        pub fn to_str(&self) -> &str {
            match self {
                Trait::Intelligent => "Intelligent",
                Trait::Aggressive => "Aggressive",
                Trait::Peaceful => "Peaceful",
                Trait::Adaptive => "Adaptive",
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Species {
        pub name: String,
        pub population: f64,
        pub traits: Vec<Trait>,
        pub universe: String,
    }

    impl Species {
        pub fn traits_for_storage(traits: Vec<Trait>) -> Vec<String> {
            traits.iter().map(|t| t.to_str().to_string()).collect()
        }
    }

    impl Planet {
        pub fn new(name: String, env: Environment) -> Self {
            Planet {
                name,
                environment: env,
                species: HashMap::new(),
            }
        }

        pub fn add_species(&mut self, species: Species) {
            self.species.insert(species.name.clone(), species);
        }
    }
}
