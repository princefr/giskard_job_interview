use std::io::Read;
use std::error::Error;

use crate::{structs::hunter::BountyHunter, enums::error::Errors};

use serde::{Deserialize, Serialize};

use async_graphql::*;



#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct Empire {
    pub countdown: i32, // number of days before the Death Star annihilates Endor
    pub bounty_hunters: Vec<BountyHunter> // list of all locations where Bounty Hunter are scheduled to be present.
}


impl Empire {
    /*!
     * Create a new Empire
     * @param countdown: i32
     * @param bounty_hunters: Vec<BountyHunter>
     * @return Empire
     */
    pub fn new(countdown: i32, bounty_hunters: Vec<BountyHunter>) -> Empire {
        Empire {
            countdown,
            bounty_hunters
        }
    }


    /*
     * Update the countdown
     * @param countdown: i32
     */
    pub fn update_countdown(&mut self, countdown: i32) {
        self.countdown = countdown;
    }
    
    /*
     * Check if the file is valid
     * @param file: String
     * @return bool
     */
    pub fn check_file(file: String) -> Result<bool, Box<dyn Error>> {
        // {
        //     "countdown": 10,
        //     "bounty_hunters": [
        //       {
        //         "planet": "Tatooine",
        //         "day": 1
        //       },
        //       {
        //         "planet": "Endor",
        //         "day": 3
        //       }
        //     ]
        //   }
        let mut file = std::fs::File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let value: serde_json::Value = serde_json::from_str(&contents)?;
        let countdown = value["countdown"].as_i64().unwrap();
        let bounty_hunters = value["bounty_hunters"].as_array().unwrap();
        if countdown > 0 && bounty_hunters.len() > 0 {
            return Ok(true);
        }
        Err(Errors::BadEmpireFile.get_error().into())
        
        
    }

    /*
     * Load the json file
     * @param file: String
     * @return Empire
     */
    pub fn load_json(file: String) -> Result<Empire, Box<dyn Error>> {
        let _ = Empire::check_file(file.clone())?;
        let mut file = std::fs::File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let empire: Empire = serde_json::from_str(&contents)?;
        Ok(empire)
    }
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let empire = Empire::new(10, vec![]);
        assert_eq!(empire.countdown, 10);
        assert_eq!(empire.bounty_hunters.len(), 0);
    }

    #[test]
    fn test_load_json() {
        let empire = Empire::load_json(String::from("empire.json"));
        let empire = empire.unwrap();
        assert_eq!(empire.countdown, 7);
        assert_eq!(empire.bounty_hunters.len(), 3);
    }
}