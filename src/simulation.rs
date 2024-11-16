pub mod simulation {
    use rand::Rng;
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

    #[derive(Debug, Clone)]
    pub struct Species {
        pub name: String,
        pub population: f64,
        pub traits: Vec<Trait>,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Trait {
        Intelligent,
        Aggressive,
        Peaceful,
        Adaptive,
        // More traits can be added
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

    pub fn simulate_planet_cycle(planet: &mut Planet) {
        let mut rng = rand::thread_rng();
        for species in planet.species.values_mut() {
            // Simple growth model based on environment and traits
            let growth_factor = match planet.environment {
                Environment::EarthLike => 1.05, // Favorable conditions
                Environment::IceWorld => 0.95,  // Harsh conditions
                Environment::Desert => 1.01,    // Moderate conditions
                Environment::GasGiant => 0.8,   // Very harsh
            };

            // Adjust growth based on species traits
            let species_trait_modifier = species
                .traits
                .iter()
                .map(|species_trait: &Trait| match species_trait {
                    Trait::Adaptive => 1.05,
                    Trait::Aggressive => 1.03, // Might lead to conflicts, but also expansion
                    _ => 1.0,
                })
                .product::<f64>();

            let new_population = species.population as f64 * growth_factor * species_trait_modifier;
            species.population = new_population;

            // Simple evolution: randomly gain or lose traits

            if rng.gen::<f32>() < 0.01 {
                // 1% chance per cycle
                let new_trait = match rng.gen_range(0..3) {
                    0 => Trait::Intelligent,
                    1 => Trait::Aggressive,
                    _ => Trait::Peaceful,
                };
                if !species.traits.contains(&new_trait) {
                    species.traits.push(new_trait);
                }
            }
        }
    }
}
