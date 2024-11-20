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

    #[derive(Debug, Clone)]
    pub struct Species {
        pub name: String,
        pub population: f64,
        pub traits: Vec<Trait>,
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
