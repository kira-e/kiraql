use async_graphql::{EmptyMutation, EmptySubscription, Object, Result, Schema, SimpleObject};

pub type LocationSchema = Schema<LocationQuery, EmptyMutation, EmptySubscription>;

#[derive(SimpleObject)]
pub struct Location {
    name: String,
    longitude: f64,
    latitude: f64,
}

pub struct LocationQuery;
#[Object]
impl LocationQuery {
    async fn location(&self, name: String) -> Result<Option<Location>> {
        Ok(Some(Location {
            name: name.to_owned() + " mock",
            longitude: 69.0,
            latitude: 96.0,
        }))
    }
}
