use std::time::Duration;
use std::{borrow::BorrowMut, thread::sleep};

mod db;
mod models;
mod simulation;

use db::db::establish_connection;
use models::models::{Environment, Planet, Species, Trait};
use simulation::simulation::simulate_planet_cycle;

fn main() {
    let mut binding = establish_connection();
    let connection = binding.borrow_mut();

    let mut earth = Planet::new("Earth".to_string(), Environment::EarthLike);
    earth.add_species(Species {
        name: "Human".to_string(),
        population: 8000000000.0,
        traits: vec![Trait::Intelligent, Trait::Adaptive],
        universe: "Milky Way".to_string(),
    });

    let mut mars = Planet::new("Mars".to_string(), Environment::Desert);
    mars.add_species(Species {
        name: "Martian".to_string(),
        population: 1000000.0,
        traits: vec![Trait::Adaptive, Trait::Aggressive],
        universe: "Milky Way".to_string(),
    });

    let mut planets = vec![earth, mars];

    for cycle in 0..100 {
        // Simulate 100 cycles
        println!("Simulation Cycle: {}", cycle);
        for planet in planets.iter_mut() {
            simulate_planet_cycle(planet, connection);
            println!("Planet: {:?}", planet.name);
            for species in planet.species.values() {
                println!(
                    "- Species: {}, Population: {}, Traits: {:?}, Universe: {:?}",
                    species.name, species.population, species.traits, species.universe
                );
            }
        }
        sleep(Duration::from_secs(1)); // Simulate time passing
        println!();
    }
}
