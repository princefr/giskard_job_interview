use std::error::Error;

use structs::route::Route;
use structs::empire::Empire;
use structs::melenium::MileniumFalcon;
mod structs;
mod database;
mod enums;
mod query;
mod utils;
use utils::util::Utils;



/*
 * Run the program for bin
 * @param milenium_falcon_file: String
 * @param empire_file: String
 * @return Result<(), Box<dyn Error>>
 */
pub fn run_for_bin(milenium_falcon_file: String, empire_file: String) -> Result<f64, Box<dyn Error>> {
    let milenium_falcon_file_absolute_path = Utils::get_absolute_path(milenium_falcon_file.clone());
    let milenium_falcon = structs::melenium::MileniumFalcon::load_json(milenium_falcon_file.clone());
    let milenium_falcon = milenium_falcon?;
    let empire = structs::empire::Empire::load_json(empire_file);
    let empire = empire?;
    let db_path = format!("{}/{}", milenium_falcon_file_absolute_path, milenium_falcon.routes_db);
    let routes = Route::load_routes_from_db(&db_path);
    let chance = milenium_falcon.chance_to_reach_destination(&routes, &empire);
    Ok(chance.unwrap())
}


/*
 * Run the program for server
 * @param milenium_falcon: MileniumFalcon
 * @param empire: Empire
 * @return Result<(), Box<dyn Error>>
 */
pub fn run_for_server(melinium_falcon: MileniumFalcon, empire: Empire) -> Result<f64, Box<dyn Error>> {
    let routes = Route::load_routes_from_db(&melinium_falcon.routes_db.to_string());
    let chance = melinium_falcon.chance_to_reach_destination(&routes, &empire);
    Ok(chance.unwrap())
}