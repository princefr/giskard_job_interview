

use async_graphql::*;

use crate::structs::{melenium::MileniumFalcon, empire::Empire, route::Route};



pub struct Query;

#[Object]
impl Query {
    async fn get_probability<'ctx>(&self, ctx: &'ctx Context<'_>, empire: Empire) -> Result<f64> {
        if empire.countdown < 0 {
            return Err(Error::new("countdown must be positive"));
        }

        if empire.bounty_hunters.len() == 0 {
            return Err(Error::new("empty bounty_hunters array is not allowed")); 
        }
        
        let milenium_falcon = ctx.data::<MileniumFalcon>().unwrap();
        let routes = Route::load_routes_from_db(&milenium_falcon.routes_db.to_string());
        let chance = milenium_falcon.chance_to_reach_destination(&routes, &empire);
        Ok(chance.unwrap())
    }
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_probability_without_countdown() {
        let melenium_falcon = MileniumFalcon::load_json(String::from("millennium-falcon.json"));
        let melenium_falcon = melenium_falcon.unwrap();
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(melenium_falcon)
        .finish();
        let request = Request::new(
            r#"{
                getProbability(empire: {countdown: 0, bountyHunters: []})
            }"#,
        );

        assert_eq!(schema.execute(request).await.errors.len(), 1);
    }

    #[tokio::test]
    async fn test_get_probability_with_empty_bounty_hunters() {
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

        assert_eq!(schema.execute(request).await.errors.len(), 1);
    }

    #[tokio::test]
    async fn test_get_probability_with_valid_input() {
        let melenium_falcon = MileniumFalcon::load_json(String::from("millennium-falcon.json"));
        let melenium_falcon = melenium_falcon.unwrap();
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(melenium_falcon)
        .finish();
        let request = Request::new(
            r#"{
                getProbability(empire: {countdown: 8, 
                        bountyHunters: [
                            {planet: "Hoth", day: 6 }, 
                            {planet: "Hoth", day: 7 },
                            {planet: "Hoth", day: 8 }
                        ]
                    }
                )
            }"#,
        );

        let response = schema.execute(request).await;
        assert_eq!(response.data, value!({ "getProbability": 81.0 }));
    }
}