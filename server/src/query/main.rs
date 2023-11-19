use async_graphql::*;

use crate::structs::{melenium::MileniumFalcon, empire::Empire, route::Route};



pub struct Query;

#[Object]
impl Query {
    async fn get_probability<'ctx>(&self, ctx: &'ctx Context<'_>, empire: Empire) -> f64 {
        if empire.countdown < 0 {
            return 0.0; // TODO: return error ?
        }

        if empire.bounty_hunters.len() == 0 {
            return 0.0; // TODO: return error ?
        }
        
        let milenium_falcon = ctx.data::<MileniumFalcon>().unwrap();
        let routes = Route::load_routes_from_db(milenium_falcon.clone().routes_db.to_string());
        let chance = milenium_falcon.chance_to_reach_destination(routes, empire.clone());
        chance.unwrap()
    }
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_probability() {
        let melenium_falcon = MileniumFalcon::load_json(String::from("millennium-falcon.json"));
        let melenium_falcon = melenium_falcon.unwrap();
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(melenium_falcon)
        .finish();
        let request = Request::new(
            r#"{
                getProbability(empire: {countdown: 10, bountyHunters: []})
            }"#,
        );

        assert_eq!(schema.execute(request).await.errors.len(), 0);
    }
}