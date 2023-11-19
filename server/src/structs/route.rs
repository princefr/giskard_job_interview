use std::collections::HashMap;

use crate::database::sqlite::SQLite;
use pathfinding::prelude::*;
use serde::{Deserialize, Serialize};

use super::hunter::BountyHunter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub fn load_routes_from_db(db: String) -> Vec<Route> {
        let database = SQLite::new(db);
        let routes = database.get_routes();
        routes.unwrap()
    }



    /*
     * Check the spaceship has enough fuel to reach destination
     * 
     * @param Route: Vec<Vec<Route>>
     * @param autonomy: i32
     * @param countdown: i32
     * @return Vec<Vec<Route>> with fuel added to the Route
     */
    pub fn check_fuel(routes: Vec<Vec<Route>>, autonomy: i32) -> Vec<Vec<Route>> {
        let mut _routes = Vec::new();

        for i in 0..routes.len(){
            let mut single_itinary = Vec::new();
            let mut route = Route::new("".to_string(), "".to_string(), 0);
            let mut current_autnomy = autonomy;
            for j in 0..routes[i].len() {
                if routes[i][j] != routes[i][routes[i].len() -1] {
                    if current_autnomy - routes[i][j].travel_time == 0 {
                        route.origin = routes[i][j].origin.clone();
                        route.destination = routes[i][j].destination.clone();
                        route.travel_time = routes[i][j].travel_time;
                        single_itinary.push(route.clone());
                        route.origin = format!("{}_{}", routes[i][j].destination.clone(), "FUEL".to_string());
                        route.destination = format!("{}_{}", routes[i][j].destination.clone(), "FUEL".to_string());
                        route.travel_time = 1;
                        single_itinary.push(route.clone());
                        current_autnomy = 6;
                        if routes[i][j+1] != routes[i][routes[i].len() -1] {
                            route.origin = routes[i][j+1].origin.clone();
                            route.destination = routes[i][j+1].destination.clone();
                            route.travel_time = routes[i][j+1].travel_time;
                            single_itinary.push(route.clone());
                        }
                    }
                    if !routes[i][j].origin.contains("FUEL") {
                        current_autnomy -= routes[i][j].travel_time;
                    }
                }else{
                    route.origin = routes[i][j].origin.clone();
                    route.destination = routes[i][j].destination.clone();
                    route.travel_time = routes[i][j].travel_time;
                    single_itinary.push(route.clone());
                }
            }
            _routes.push(single_itinary.clone());
        }
        return _routes;
    }




    /*
     * Add wait time to routes until destination
     * @param routes: Vec<Vec<Route>>
     * @param countdown: i32
     * @return Vec<Vec<Route>>
     */
    pub fn add_wait_time_to_routes_until_destination(best_routes: Vec<Vec<Route>>, countdown: i32) -> Vec<Vec<Route>> {
        let mut all_routes: Vec<Vec<Route>> = Vec::new();
        for i in 0..best_routes.len() {
            let total_travel_time: i32 = best_routes[i].iter().map(|route| route.travel_time).sum() ;
            if total_travel_time < countdown {
                let mut total_wait_time = countdown - total_travel_time ;
                for _ in 0..total_wait_time {
                    for j in 0..best_routes[i].clone().len() -1 {
                        let mut _itinary = best_routes[i].clone();
                        let remove_fuel_string = best_routes[i][j].clone().destination.replace("_FUEL", "");
                        let destination = format!("{}_{}", remove_fuel_string, "WAIT".to_string());
                        let element = Route::new("WAIT".to_string(), destination, 1);
                        _itinary.insert(j+1, element);
                        let f:i32 =_itinary.iter().map(|route| route.travel_time).sum();
                        if f <= countdown {
                            all_routes.push(_itinary.clone());
                        }
                        
                    }
                    total_wait_time -= 1;

                }
            }else{
                all_routes.push(best_routes[i].clone());
            }
        }
        all_routes
    }   


    /*
     * Find all routes leading to destination
     * @param routes: Vec<Route>
     * @param origin: String
     * @param destination: String
     * @return Vec<Vec<Route>>
     */
    pub fn find_all_routes_leading_to_destination(
        routes: Vec<Route>,
        origin: String,
        destination: String,
    ) -> Vec<Vec<Route>> {
        let mut graph: Vec<(String, Vec<String>)> = Vec::new();
        for route in routes.clone() {
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
            |node| graph.iter().filter(|(n, _)| n == node).flat_map(|(_, neighbors)| neighbors.clone()).map(|n| (n, 0)).collect::<Vec<_>>(),
            |_| 0,
            |node| node == &destination,
        );

        let mut all_routes: Vec<Vec<Route>> = Vec::new();
        let all_nodes = result.clone().unwrap().0;
        
        for i in 0..all_nodes.len() {
            let itinary = &all_nodes[i];
            let mut _routes = Vec::new();
            for j in 0..itinary.len() -1 {
                let route =routes
                    .iter()
                    .find(|&r| r.origin == itinary[j] && r.destination == itinary[j + 1])
                    .unwrap();
                _routes.push(route.clone());
            }
            all_routes.push(_routes);
        }
        all_routes
    }


    /*
     * Find the best itinary for the milenium falcon
     * @param Route: Vec<Route>
     * @param origin: String
     * @param destination: String
     * @return Vec<Route>
     */
    pub fn find_best_routes(
        routes: Vec<Route>,
        origin: String,
        destination: String,
    ) -> Vec<Route> {
        let mut graph = HashMap::new();
        for route in routes.clone() {
            graph.entry(route.origin)
            .or_insert_with(Vec::new)
            .push((route.destination, route.travel_time));
        }

        let result = astar(
            &origin,
            |node| graph[node].iter().cloned(),
            |_| 0,
            |node| node == &destination,
        );

        let mut best_routes = Vec::new();
        let mut route = Route::new("".to_string(), "".to_string(), 0);
        let all_routes = result.clone().unwrap().0;
        let total_travel_time = result.clone().unwrap().1;
        if all_routes.len() > 2 {
            for i in 0..all_routes.len() - 1 {
                route.origin = all_routes[i].to_string();
                route.destination = all_routes[i + 1].to_string();
                route.travel_time = routes.clone()
                    .iter()
                    .find(|&r| r.origin == route.origin && r.destination == route.destination)
                    .unwrap()
                    .travel_time;
                best_routes.push(route.clone());
            }
        }else{
            route.origin = all_routes[0].to_string();
            route.destination = all_routes[1].to_string();
            route.travel_time = total_travel_time;
            best_routes.push(route.clone());
        }
        best_routes

    }


    /*
     * Calculate the total encouters with bounty hunters
     * @param routes: Vec<Route>
     * @param bounty_hunters: Vec<BountyHunter>
     * @return i32
     */
    pub fn calculate_encouters_with_hunters(routes: Vec<Route>, bounty_hunters: Vec<BountyHunter>) -> i32 {
        let mut total_encouters = 0;
        for i in 0..routes.len() {
            for j in 0..bounty_hunters.len() {
                if routes[i].destination == bounty_hunters[j].planet && routes[i].travel_time == bounty_hunters[j].day {
                    total_encouters += 1;
                }

                //TODO: check if the milenium falcon is in the same planet as the bounty hunter with waits and fuels
                if routes[i].origin == routes[i].destination && routes[i-1].travel_time + routes[i].travel_time == bounty_hunters[j].day {
                    total_encouters += 1;
                }
            }
        }
        total_encouters
    }


    /*
     * Remove itinaries with wait time greater than countdown
     * @param routes: Vec<Vec<Route>>
     * @param countdown: i32
     * @return Vec<Vec<Route>>
     */
    pub fn remove_itinaries_with_travel_time_greater_than_countdown(routes: Vec<Vec<Route>>, countdown: i32) -> Vec<Vec<Route>> {
        let back = routes.iter().filter(|_itinary| _itinary.iter().map(|route| route.travel_time).sum::<i32>() < countdown).cloned().collect::<Vec<_>>();
        back
    }


    /*
     * Calculate the reach probability
     * @param encouters_with_hunters: i32
     * @return f64
     */
    pub fn calculate_reach_probability(encouters_with_hunters: i32) -> f64 {
        let mut sum = 0.0;
        for i in 0..= u32::try_from(encouters_with_hunters).unwrap() -1 {
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
        let routes = Route::load_routes_from_db("universe.db".to_string());
        assert_eq!(routes.len(), 5);
    }

    #[test]
    fn test_find_routes() {
        let routes = Route::load_routes_from_db("universe.db".to_string());
        let best_routes =
            Route::find_best_routes(routes, "Tatooine".to_string(), "Endor".to_string());
        assert_eq!(best_routes.len(), 2);
        assert_eq!(best_routes[0].origin, "Tatooine");
        assert_eq!(best_routes[0].destination, "Hoth");
        assert_eq!(best_routes[0].travel_time, 6);
        assert_eq!(best_routes[1].origin, "Hoth");
        assert_eq!(best_routes[1].destination, "Endor");
        assert_eq!(best_routes[1].travel_time, 1);
    }
}
