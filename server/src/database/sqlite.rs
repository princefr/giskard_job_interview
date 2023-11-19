use rusqlite::{Connection, Result};

use crate::structs::route::Route;

// SQLite struct
#[derive(Debug)]
pub struct SQLite {
    pub db: Connection
}

impl SQLite {
    /*!
     * Create a new SQLite
     * @param file_name: String
     * @return SQLite
     */
    pub fn new(file_name: String) -> SQLite {
        let db = Connection::open(file_name).unwrap();
        SQLite { db }
    }


    /*
     * Create table if not exists
     * @return Result<()>
     */
    pub fn create_table_if_not_exists(&self) -> Result<()> {
        let query = "CREATE TABLE IF NOT EXISTS routes (
            origin TEXT NOT NULL,
            destination TEXT NOT NULL,
            travel_time INTEGER NOT NULL,
        )";

        self.db.execute(query, [])?;
        Ok(())
    }

    /*
     * Insert all routes
     * @param routes: Vec<Route>
     * @return Result<()>
     */
    pub fn insert_all(&self, routes: Vec<Route>) -> Result<()> {
        let query = "INSERT INTO routes (origin, destination, travel_time) VALUES (?, ?, ?)";
        for route in routes {
            self.db.execute(query, [route.origin, route.destination, route.travel_time.to_string()])?;
        }
        Ok(())
    }


    /*
     * Check if any route exists
     * @return Result<bool>
     */
    pub fn check_if_any_route_exists(&self) -> Result<bool> {
        let query = "SELECT COUNT(*) FROM routes";
        let mut stmt = self.db.prepare(query)?;
        let mut rows = stmt.query([])?;
        let count: i32 = rows.next().unwrap().unwrap().get(0)?;
        Ok(count > 0)
    }

    /*
     * Get all routes
     * @return Result<Vec<Route>>
     */
    pub fn get_routes(&self) -> Result<Vec<Route>> {
        let query = "SELECT * FROM routes";
        let mut stmt = self.db.prepare(query)?;
        let mut rows = stmt.query([])?;
        let mut routes: Vec<Route> = Vec::new();
        while let Some(row) = rows.next()? {
            let route = Route {
                origin: row.get(0)?,
                destination: row.get(1)?,
                travel_time: row.get(2)?
            };
            routes.push(route);
        }
        Ok(routes)
    }
}