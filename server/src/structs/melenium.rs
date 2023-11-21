use std::{fs::File, io::Read};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::enums::error::Errors;

use super::{route::Route, empire::Empire};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MileniumFalcon {
    pub autonomy: i32, // autonomy of the Millennium Falcon in days.
    pub departure: String, // Planet where the Millennium Falcon is on day 0.
    pub arrival: String, // Planet where the Millennium Falcon must be at or before countdown.
    pub routes_db: String // Path toward a SQLite database file containing the routes. The path can be either absolute or relative to the location of the millennium-falcon.json file itself
}

impl MileniumFalcon {
    /*!
     * Create a new MileniumFalcon
     * @param autonomy: i32
     * @param departure: String
     * @param arrival: String
     * @param routes_db: String
     * @return MileniumFalcon
     */
    pub fn new(autonomy: i32, departure: String, arrival: String, routes_db: String) -> MileniumFalcon {
        MileniumFalcon {
            autonomy,
            departure,
            arrival,
            routes_db
        }
    }

    /*
     * Check if the file is valid
     * @param file: String
     * @return bool
     */
    pub fn check_file(file: &String) -> Result<bool, Box<dyn Error>> {
        // {
        //     "autonomy": 6,
        //     "departure": "Tatooine",
        //     "arrival": "Endor",
        //     "routes_db": "universe.db"
        //   }
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let value: serde_json::Value = serde_json::from_str(&contents)?;
        let autonomy = value["autonomy"].as_i64().unwrap();
        let departure = value["departure"].as_str().unwrap();
        let arrival = value["arrival"].as_str().unwrap();
        let routes_db = value["routes_db"].as_str().unwrap();
        if autonomy > 0 && departure.len() > 0 && arrival.len() > 0 && routes_db.len() > 0 {
            return Ok(true);
        }
        Err(Errors::BadMeleniumFalconFile.get_error().into())
        
    }

    /*
     * Load the json file
     * @param file: String
     * @return MileniumFalcon
     */
    pub fn load_json(file: String) -> Result<MileniumFalcon, Box<dyn Error>> {
        let _ = MileniumFalcon::check_file(&file)?;
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let milenium_falcon: MileniumFalcon = serde_json::from_str(&contents)?;
        Ok(milenium_falcon)
    }

    /*
     * Chance to reach destination
     * @param routes: Vec<Routes>
     * @param empire: Empire
     * @return Option<f64>
     */
    pub fn chance_to_reach_destination(&self, routes: &Vec<Route>, empire: &Empire) -> Option<f64> {
        let chance: f64 = Route::get_change_to_reach_endor(routes, &self.departure, &self.arrival, empire);

        return Some(chance)
    }


}


// tests
#[cfg(test)]   
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let milenium_falcon = MileniumFalcon::new(10, String::from("Tatooine"), String::from("Naboo"), String::from("universe.db"));
        assert_eq!(milenium_falcon.autonomy, 10);
        assert_eq!(milenium_falcon.departure, String::from("Tatooine"));
        assert_eq!(milenium_falcon.arrival, String::from("Naboo"));
        assert_eq!(milenium_falcon.routes_db, String::from("universe.db"));
    }

    #[test]
    fn test_load_json() {
        let milenium_falcon = MileniumFalcon::load_json(String::from("millennium-falcon.json"));
        let milenium_falcon = milenium_falcon.unwrap();
        assert_eq!(milenium_falcon.autonomy, 6);
        assert_eq!(milenium_falcon.departure, String::from("Tatooine"));
        assert_eq!(milenium_falcon.arrival, String::from("Endor"));
        assert_eq!(milenium_falcon.routes_db, String::from("universe.db"));
    }

    #[test]
    fn test_check_file() {
        let valid = MileniumFalcon::check_file(&String::from("millennium-falcon.json"));
        assert_eq!(valid.unwrap(), true);
    }
 
    #[test]
    fn test_chance_to_reach_destination_0() {
        let milenium_falcon = MileniumFalcon::load_json(String::from("millennium-falcon.json"));
        let milenium_falcon = milenium_falcon.unwrap();
        let empire = Empire::load_json(String::from("empire.json"));
        let empire = empire.unwrap();
        let routes = Route::load_routes_from_db(&milenium_falcon.routes_db.to_string());
        let chance = milenium_falcon.chance_to_reach_destination(&routes, &empire);
        assert_eq!(chance.unwrap(), 0.0);
    }

    #[test]    
    fn test_chance_to_reach_destination_81() {
        let milenium_falcon = MileniumFalcon::load_json(String::from("millennium-falcon.json"));
        let milenium_falcon = milenium_falcon.unwrap();
        let empire = Empire::load_json(String::from("empire.json"));
        let mut empire = empire.unwrap();
        empire.update_countdown(8);
        let routes = Route::load_routes_from_db(&milenium_falcon.routes_db.to_string());
        let chance = milenium_falcon.chance_to_reach_destination(&routes, &empire);
        assert_eq!(chance.unwrap(), 81.0);
    }

    #[test]
    fn test_chance_to_reach_destination_90() {
        let milenium_falcon = MileniumFalcon::load_json(String::from("millennium-falcon.json"));
        let milenium_falcon = milenium_falcon.unwrap();
        let empire = Empire::load_json(String::from("empire.json"));
        let mut empire = empire.unwrap();
        empire.update_countdown(9);
        let routes = Route::load_routes_from_db(&milenium_falcon.routes_db.to_string());
        let chance = milenium_falcon.chance_to_reach_destination(&routes, &empire);
        assert_eq!(chance.unwrap(), 90.0);
    }

    #[test]  
    fn test_chance_to_reach_destination_100() {
        let milenium_falcon = MileniumFalcon::load_json(String::from("millennium-falcon.json"));
        let milenium_falcon = milenium_falcon.unwrap();
        let empire = Empire::load_json(String::from("empire.json"));
        let mut empire = empire.unwrap();
        empire.update_countdown(10);
        let routes = Route::load_routes_from_db(&milenium_falcon.routes_db.to_string());
        let chance = milenium_falcon.chance_to_reach_destination(&routes, &empire);
        assert_eq!(chance.unwrap(), 100.0);
    }

}



