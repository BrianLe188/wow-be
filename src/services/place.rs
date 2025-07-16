use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;

use crate::{
    config::db::DbConn,
    models::place::{NewPlace, Place},
    schema::places,
};

pub async fn get_place_by_place_id(
    conn: &mut DbConn,
    place_id: &str,
) -> Result<Place, diesel::result::Error> {
    places::table
        .filter(places::place_id.eq(place_id))
        .select(Place::as_select())
        .first::<Place>(conn)
        .await
}

pub async fn create_place(
    conn: &mut DbConn,
    payload: &NewPlace,
) -> Result<Place, diesel::result::Error> {
    diesel::insert_into(places::table)
        .values(payload)
        .returning(Place::as_returning())
        .get_result::<Place>(conn)
        .await
}

pub async fn increase_place_view(
    conn: &mut DbConn,
    place_id: &str,
) -> Result<Place, diesel::result::Error> {
    diesel::update(places::table.filter(places::place_id.eq(place_id)))
        .set(places::range_time_view_count.eq(places::range_time_view_count + 1))
        .returning(Place::as_returning())
        .get_result::<Place>(conn)
        .await
}
