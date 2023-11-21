use crate::structs::route::Route;
use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Clone,  Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Itinary {
    Itinary(Route),
    Fuel,
    Wait,
}