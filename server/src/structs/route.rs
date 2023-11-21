

use crate::{
    database::sqlite::SQLite,
    enums::itinary::Itinary,
};
use pathfinding::prelude::*;
use serde::{Deserialize, Serialize};

use super::{empire::Empire, hunter::BountyHunter};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Route {
    pub origin: String,      // Name of the origin planet. Cannot be null or empty.
    pub destination: String, // Name of the destination planet. Cannot be null or empty.
    pub travel_time: i32, // Number days needed to travel from one planet to the other. Must be strictly positive.
}

impl Route {
    /*
     * Create a new Route
     * @param origin: String
     * @param destination: String
     * @param travel_time: i32
     * @return Route
     */
    pub fn new(origin: String, destination: String, travel_time: i32) -> Route {
        Route {
            origin,
            destination,
            travel_time,
        }
    }

    /*
     * Load routes from database
     * @param db: String
     * @return Vec<Route>
     */
    pub fn load_routes_from_db(db: &String) -> Vec<Route> {
        let database = SQLite::new(db.clone());
        let routes = database.get_routes();
        routes.unwrap()
    }

    /*
     * Calculate the full travel time
     * @param itinaries: &[Itinary]
     * @return i32
     */
    pub fn calculate_full_travel_time(itinaries: &Vec<Itinary>) -> i32 {
        let mut total_travel_time = 0;
        for itinary in itinaries {
            match itinary {
                Itinary::Itinary(route) => {
                    total_travel_time += route.travel_time;
                }
                Itinary::Fuel => {
                    total_travel_time += 1;
                }
                Itinary::Wait => {
                    total_travel_time += 1;
                }
            }
        }
        total_travel_time
    }

    /*
     * Get the change to reach endor
     * @param routes: Vec<Route>
     * @param origin: String
     * @param destination: String
     * @param empire: &Empire
     * @return Vec<Vec<Route>>
     */
    pub fn get_change_to_reach_endor(
        routes: &Vec<Route>,
        origin: &String,
        destination: &String,
        empire: &Empire,
    ) -> f64 {
        let mut graph: Vec<(String, Vec<String>)> = Vec::new();
        for route in routes {
            let origin_node = route.origin.to_string();
            let destination_node = route.destination.to_string();

            if !graph.iter().any(|(node, _)| node == &origin_node) {
                graph.push((origin_node.clone(), Vec::new()));
            }

            if !graph.iter().any(|(node, _)| node == &destination_node) {
                graph.push((destination_node.clone(), Vec::new()));
            }

            if let Some((_, neighbors)) = graph.iter_mut().find(|(node, _)| node == &origin_node) {
                neighbors.push(destination_node.clone());
            }
        }

        let result = astar_bag_collect(
            &origin,
            |node| {
                graph
                    .iter()
                    .filter(|(n, _)| n == *node)
                    .flat_map(|(_, neighbors)| neighbors)
                    .map(|n| (n, 0))
                    .collect::<Vec<_>>()
            },
            |_| 0,
            |node| node == &destination,
        );

        let mut all_routes: Vec<Vec<Route>> = Vec::new();
        let all_nodes = result.clone().unwrap().0;

        for i in 0..all_nodes.len() {
            let itinary = &all_nodes[i];
            let mut _routes = Vec::new();
            for j in 0..itinary.len() - 1 {
                let route = routes
                    .iter()
                    .find(|&r| r.origin == *itinary[j] && r.destination == *itinary[j + 1])
                    .unwrap();
                _routes.push(route.clone());
            }
            all_routes.push(_routes);
        }

        let mut graph_2: Vec<(i32, Vec<(f64, Vec<Itinary>)>)> = Vec::new();
        for i in 0..all_routes.clone().len() {

            let mut route: Vec<Itinary> = Vec::new();
            for j in 0..all_routes[i].len() {
                route.push(Itinary::Itinary(all_routes[i][j].clone()));
            }

            let mut current_autonomy = 6;
            for i in 0..route.clone().len() {
                if route.clone()[i] != route.clone().last().unwrap().clone() {
                    match route.clone()[i].clone() {
                        Itinary::Itinary(_rr) => {
                            if current_autonomy - _rr.travel_time == 0 {
                                route.insert(i+1, Itinary::Fuel);
                                current_autonomy = 6;
                            }
                            current_autonomy -= _rr.travel_time;
                        }
                        _ => {}
                    }
                }
                let total_travel = Route::calculate_full_travel_time(&route);
                graph_2.push((total_travel, vec![(0.0, route.clone())]));
            }

            


            


            for x in 0..route.len() - 1 {
                let mut cloned_route = route.clone();
                cloned_route.insert(x, Itinary::Wait);
                let total_travel = Route::calculate_full_travel_time(&cloned_route.clone());
                graph_2.push((total_travel, vec![(0.0, cloned_route.clone())]));
                
            }



            
            let _ = graph_2.iter_mut().filter(|(node, _)| node == &empire.countdown).flat_map(|(c, x)| x).map(|neighbor| {
                let encouter = BountyHunter::calculate_bounty_hunter_encounters(
                    &neighbor.1,
                    empire.bounty_hunters.clone(),
                );

                if encouter > 0 {
                    let prob = Route::calculate_reach_probability(encouter);
                    neighbor.0 = prob;
                }else{
                    neighbor.0 = 100.0;
                } 
            }).collect::<Vec<_>>();
        }


        
        let result = graph_2
            .iter()
            .filter(|(node, _)| node == &empire.countdown)
            .flat_map(|(_, neighbors)| neighbors)
            .map(|n| n.clone())
            .collect::<Vec<_>>();

        if let Some(max_tuple) = result.iter().max_by(|(a,_), (b,_)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)){
            return max_tuple.0;
        }
        return 0.0;
       
    }

    /*
     * Calculate the reach probability
     * @param encouters_with_hunters: i32
     * @return f64
     */
    pub fn calculate_reach_probability(encouters_with_hunters: i32) -> f64 {
        let mut sum = 0.0;
        for i in 0..=u32::try_from(encouters_with_hunters).unwrap() - 1 {
            sum += f64::from(9u32.pow(i)) / f64::from(10u32.pow(i + 1));
        }
        (1.0 - sum) * 100.0
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_routes_from_db() {
        let routes = Route::load_routes_from_db(&"universe.db".to_string());
        assert_eq!(routes.len(), 5);
    }
}
