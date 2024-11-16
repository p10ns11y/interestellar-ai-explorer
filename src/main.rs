use std::thread::sleep;
use std::time::Duration;

mod simulation; // Declares `simulation.rs` as a module
use simulation::simulation::{simulate_planet_cycle, Environment, Planet, Species, Trait};

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
