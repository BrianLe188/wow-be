use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;

use crate::{
    config::db::DbConn,
    models::feature_usage::{FeatureUsage, NewFeatureUsage},
    schema::feature_usages,
};
use uuid::Uuid;

pub async fn create_feature_usage(
    conn: &mut DbConn,
    payload: &NewFeatureUsage,
) -> Result<FeatureUsage, diesel::result::Error> {
    diesel::insert_into(feature_usages::table)
        .values(payload)
        .returning(FeatureUsage::as_returning())
        .get_result::<FeatureUsage>(conn)
        .await
}

pub async fn get_feature_usage_by_user(
    conn: &mut DbConn,
    user_id: &str,
) -> Result<FeatureUsage, diesel::result::Error> {
    let user_uuid = match Uuid::parse_str(user_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    feature_usages::table
        .filter(feature_usages::user_id.eq(user_uuid))
        .select(FeatureUsage::as_select())
        .first::<FeatureUsage>(conn)
        .await
}

pub async fn give_usage_count_to_user(
    conn: &mut DbConn,
    user_id: &str,
    count: i32,
) -> Result<(), diesel::result::Error> {
    let user_uuid = match Uuid::parse_str(user_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    diesel::update(feature_usages::table.filter(feature_usages::user_id.eq(user_uuid)))
        .set(
            feature_usages::route_calculation_count
                .eq(feature_usages::route_calculation_count + count),
        )
        .returning(FeatureUsage::as_returning())
        .get_result::<FeatureUsage>(conn)
        .await?;

    Ok(())
}
