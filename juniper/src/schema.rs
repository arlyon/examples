use juniper::FieldResult;
use juniper::RootNode;
use actix_web::client::Client;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    async fn human(id: String) -> FieldResult<Human> {
        let mut client = Client::default();
        let mut response = client.get("https://ipapi.co/8.8.8.8/city").send().await?;
        let home = std::str::from_utf8(&response.body().await?)?;


        Ok(Human {
            id,
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: home.to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    async fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
