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
    user_id: Uuid,
) -> Result<FeatureUsage, diesel::result::Error> {
    feature_usages::table
        .filter(feature_usages::user_id.eq(user_id))
        .select(FeatureUsage::as_select())
        .first::<FeatureUsage>(conn)
        .await
}
