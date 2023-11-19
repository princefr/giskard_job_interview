

use serde::{Deserialize, Serialize};
use async_graphql::*;

#[derive(Serialize, Deserialize, Debug, Clone,  InputObject)]
pub struct BountyHunter{
    pub planet: String, // Name of the planet
    pub day: i32, // Day the bounty hunters are on the planet. 0 represents the first day of the mission, i.e. today.
}

impl BountyHunter {
    /*
     * Create a new BountyHunter
     * @param planet: String
     * @param day: i32
     * @return BountyHunter
     */
    pub fn new(planet: String, day: i32) -> BountyHunter {
        BountyHunter {
            planet,
            day,
        }
    }
}