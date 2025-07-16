use serde::Deserialize;

use crate::models::{place::NewPlace, review::NewReview};

#[derive(Deserialize, Debug)]
pub struct UpsertPlacePayload {
    pub place: NewPlace,
    pub reviews: Vec<NewReview>,
}
