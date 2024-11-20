// @generated automatically by Diesel CLI.

diesel::table! {
    species (id) {
        id -> Int4,
        planet -> Varchar,
        name -> Varchar,
        population -> Float8,
        // How to match rust specific enum to database enum?
        traits -> Array<Nullable<Text>>,
    }
}
// Automatically generated content ends here

pub mod schema {
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use dotenv::dotenv;
    use std::collections::HashMap;
    use std::env;

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

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
}
