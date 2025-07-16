use diesel::{ExpressionMethods, SelectableHelper, query_dsl::methods::FilterDsl};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    config::db::DbConn,
    models::subscription::{NewSubscription, Subscription},
    schema::subscriptions,
};

pub async fn get_subscription_by_user(
    conn: &mut DbConn,
    user_id: &str,
    app_type: &str,
) -> Result<Subscription, diesel::result::Error> {
    let user_uuid = match Uuid::parse_str(user_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    subscriptions::table
        .filter(subscriptions::user_id.eq(user_uuid))
        .filter(subscriptions::app.eq(app_type))
        .first::<Subscription>(conn)
        .await
}

pub async fn create_subscription<'a>(
    conn: &mut DbConn,
    payload: &'a NewSubscription<'a>,
) -> Result<Subscription, diesel::result::Error> {
    diesel::insert_into(subscriptions::table)
        .values(payload)
        .returning(Subscription::as_returning())
        .get_result::<Subscription>(conn)
        .await
}
