use crate::simulation::{Planet, Species};
use actix_web::{web, Responder};

pub async fn handle_simulation(data: web::Json<Planet>) -> impl Responder {
    // This would be your simulation logic or call to simulation module
    let mut planet = data.into_inner();
    super::simulation::simulate_planet_cycle(&mut planet);
    web::Json(planet)
}
