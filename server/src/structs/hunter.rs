

use serde::{Deserialize, Serialize};
use async_graphql::*;

use crate::enums::itinary::Itinary;

use super::route::Route;

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

    pub fn calculate_bounty_hunter_encounters(itinaries: &Vec<Itinary>, bounty_hunters: Vec<BountyHunter>) -> i32 {
        let mut encounters: i32 = 0;
            for j in 0..itinaries.len() {
                for k in 0..bounty_hunters.len() {
                    match &itinaries[j] {
                        Itinary::Fuel => {
                            match &itinaries[j-1] {
                                Itinary::Itinary(_r) => {
                                    let sum = &itinaries[0..j].iter().fold(0, |acc, x| {
                                        match x {
                                            Itinary::Itinary(_r) => acc + _r.travel_time,
                                            Itinary::Fuel => acc+1,
                                            Itinary::Wait => acc+1,
                                        }
                                    });

                                    if *sum == bounty_hunters[k].day && _r.destination == bounty_hunters[k].planet {
                                        encounters += 1;
                                    }

                                    if *sum +1 == bounty_hunters[k].day && _r.destination == bounty_hunters[k].planet {
                                        encounters += 1;
                                    }
                                }
                                _ => {}
                                
                            }
                        },
                        Itinary::Wait => {
                            if itinaries[j] == itinaries[0] {
                                let _itirany = &itinaries[j+1];
                                match _itirany {
                                    Itinary::Itinary(_r) => {
                                        if _r.origin == bounty_hunters[k].planet && 0 == bounty_hunters[k].day {
                                            encounters += 1;
                                        }
                                    }
                                    _ => {}
                                    _ => {}
                                }
                            }else{
                                let sum = &itinaries[0..j].iter().fold(0, |acc, x| {
                                    match x {
                                        Itinary::Itinary(_r) => acc + _r.travel_time,
                                        Itinary::Fuel => acc +1,
                                        Itinary::Wait => acc+1,
                                    }
                                });

                                if *sum == bounty_hunters[k].day {
                                    encounters += 1;
                                }
                            }
                        }

                        Itinary::Itinary(route) => {
                            let sum = &itinaries[0..j].iter().fold(0, |acc, x| {
                                match x {
                                    Itinary::Itinary(_r) => acc + _r.travel_time,
                                    Itinary::Fuel => acc +1,
                                    Itinary::Wait => acc+1,
                                }
                            });

                            
                            if *sum +1  == bounty_hunters[k].day && route.destination == bounty_hunters[k].planet {
                                encounters += 1;
                            }
                        }
                        
                    }
                }
            }
        return encounters
    }
}