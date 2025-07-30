use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    config::db::DbConn,
    models::action_count::{
        ActionCount, ActionCountChangeset, NewActionCount, UpdateActionCountPayload,
    },
    schema::action_count,
};

pub async fn get_action_count_by_user(
    conn: &mut DbConn,
    user_id: Uuid,
) -> Result<ActionCount, diesel::result::Error> {
    action_count::table
        .filter(action_count::user_id.eq(user_id))
        .select(ActionCount::as_select())
        .first::<ActionCount>(conn)
        .await
}

pub async fn create_action_count(
    conn: &mut DbConn,
    payload: &NewActionCount,
) -> Result<ActionCount, diesel::result::Error> {
    diesel::insert_into(action_count::table)
        .values(payload)
        .returning(ActionCount::as_returning())
        .get_result::<ActionCount>(conn)
        .await
}

pub async fn increase_action_count_by_user(
    conn: &mut DbConn,
    user_id: &str,
    payload: &UpdateActionCountPayload,
) -> Result<(), diesel::result::Error> {
    let user_uuid = match Uuid::parse_str(user_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    let current = action_count::table
        .filter(action_count::user_id.eq(user_uuid))
        .first::<ActionCount>(conn)
        .await?;

    let changes = ActionCountChangeset {
        review_place: payload
            .review_place
            .map(|v| current.review_place.unwrap_or(0) + v),
    };

    diesel::update(action_count::table.filter(action_count::user_id.eq(user_uuid)))
        .set(&changes)
        .returning(ActionCount::as_returning())
        .get_result::<ActionCount>(conn)
        .await?;

    Ok(())
}
