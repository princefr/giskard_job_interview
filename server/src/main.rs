

mod query;
mod structs;
mod enums;
mod database;
use async_graphql::{http::{playground_source, GraphQLPlaygroundConfig}, Schema, EmptySubscription, EmptyMutation};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use query::main::Query;
use poem::{http::Method, middleware::Cors};


use poem::{
    get, handler,
    listener::TcpListener,
    web::{Data, Html},
    EndpointExt, IntoResponse, Route, Server,
};
use structs::melenium::MileniumFalcon;

// App Schema
pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;


#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[handler]
async fn graphql_handler(
    schema: Data<&AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;
    schema.execute(req).await.into()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let milenium_falcon = MileniumFalcon::load_json(String::from("millennium-falcon.json"));
    let milenium_falcon = milenium_falcon.unwrap();

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(milenium_falcon)
        .finish();

    let cors = Cors::new()
    .allow_method(Method::GET)
    .allow_method(Method::POST)
    .allow_method(Method::OPTIONS)
    .allow_credentials(false);

    let app = Route::new()
    .at("/", get(graphql_playground).post(graphql_handler))
    .with(cors)
    .data(schema);

    Server::new(TcpListener::bind("127.0.0.1:8000"))
    .run(app)
    
    .await
    
    .unwrap();

    println!("Server is running on http://127.0.0.1:8000");

    Ok(())
}
