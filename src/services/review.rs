use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    config::db::DbConn,
    models::review::{NewReview, Review},
    schema::reviews,
};

pub async fn get_reviews(
    conn: &mut DbConn,
    place_id: &str,
) -> Result<Vec<Review>, diesel::result::Error> {
    let place_uuid = match Uuid::parse_str(place_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    reviews::table
        .filter(reviews::place_id.eq(place_uuid))
        .select(Review::as_select())
        .load(conn)
        .await
}

pub async fn create_review(
    conn: &mut DbConn,
    payload: &NewReview,
) -> Result<Review, diesel::result::Error> {
    diesel::insert_into(reviews::table)
        .values(payload)
        .returning(Review::as_returning())
        .get_result::<Review>(conn)
        .await
}
