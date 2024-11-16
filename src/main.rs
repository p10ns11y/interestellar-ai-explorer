use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;

#[derive(Debug, Clone)]
struct Planet {
    name: String,
    environment: Environment,
    species: HashMap<String, Species>,
}

#[derive(Debug, Clone, Copy)]
enum Environment {
    EarthLike,
    IceWorld,
    Desert,
    GasGiant,
}

#[derive(Debug, Clone)]
struct Species {
    name: String,
    population: f64,
    traits: Vec<Trait>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Trait {
    Intelligent,
    Aggressive,
    Peaceful,
    Adaptive,
    // More traits can be added
}

impl Planet {
    fn new(name: String, env: Environment) -> Self {
        Planet {
            name,
            environment: env,
            species: HashMap::new(),
        }
    }

    fn add_species(&mut self, species: Species) {
        self.species.insert(species.name.clone(), species);
    }
}

fn simulate_planet_cycle(planet: &mut Planet) {
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

fn main() {
    let mut earth = Planet::new("Earth".to_string(), Environment::EarthLike);
    earth.add_species(Species {
        name: "Human".to_string(),
        population: 8000000000.0,
        traits: vec![Trait::Intelligent, Trait::Adaptive],
    });

    let mut mars = Planet::new("Mars".to_string(), Environment::Desert);
    mars.add_species(Species {
        name: "Martian".to_string(),
        population: 1000000.0,
        traits: vec![Trait::Adaptive, Trait::Aggressive],
    });

    let mut planets = vec![earth, mars];

    for cycle in 0..100 {
        // Simulate 100 cycles
        println!("Simulation Cycle: {}", cycle);
        for planet in planets.iter_mut() {
            simulate_planet_cycle(planet);
            println!("Planet: {:?}", planet.name);
            for species in planet.species.values() {
                println!(
                    "- Species: {}, Population: {}, Traits: {:?}",
                    species.name, species.population, species.traits
                );
            }
        }
        sleep(Duration::from_secs(1)); // Simulate time passing
        println!();
    }
}
