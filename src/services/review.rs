use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use crate::{
    config::db::DbConn,
    models::review::{NewReview, Review},
    schema::reviews,
};

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
