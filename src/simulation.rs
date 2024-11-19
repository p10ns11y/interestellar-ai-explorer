// mod schema; // Declares `simulation.rs` as a module

pub mod simulation {
    use crate::schema::schema::{Environment, Planet, Species, Trait};
    use pyo3::prelude::*;
    use rand::Rng;

    fn simulate_ai_evolution(species: &Species) -> PyResult<Trait> {
        Python::with_gil(|py| -> PyResult<Trait> {
            let current_dir = std::env::current_dir()?;
            let sys = py.import("sys")?;
            sys.getattr("path")?
                .call_method1("append", (current_dir.to_str(),))?;

            // TODO: fix module resolution
            let evolution_module = py.import("evolution")?;
            let result = evolution_module.call_method0("evolve_species")?;
            // Process result
            let selected_trait = match result.extract::<String>()? {
                ref s if s == "Intelligent" => Trait::Intelligent,
                ref s if s == "Aggressive" => Trait::Aggressive,
                ref s if s == "Peaceful" => Trait::Peaceful,
                ref s if s == "Adaptive" => Trait::Adaptive,
                _ => Trait::Intelligent,
            };
            // take the return value from result and add it to the species

            Ok(selected_trait)
        })
    }

    // TODO: Insert into the database table
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
                let new_trait = simulate_ai_evolution(&species).unwrap();
                if !species.traits.contains(&new_trait) {
                    species.traits.push(new_trait);
                }
            }
        }
    }
}
