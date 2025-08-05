use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use redis::AsyncCommands;

use crate::{
    config::{cache::CacheConn, db::DbConn},
    models::mission::{Mission, NewMission},
    schema::missions,
    services::{
        feature_usage::give_usage_count_to_user,
        user::{give_exp_to_user, level_up},
    },
    utils::time::{get_seconds_to_midnight, get_today},
};

pub async fn create_mission(conn: &mut DbConn, payload: &NewMission) -> Result<Mission, diesel::result::Error> {
    diesel::insert_into(missions::table)
        .values(payload)
        .returning(Mission::as_returning())
        .get_result::<Mission>(conn)
        .await
}

pub async fn get_mission_by_code(conn: &mut DbConn, code: &str) -> Result<Mission, diesel::result::Error> {
    missions::table.filter(missions::code.eq(code)).select(Mission::as_select()).first::<Mission>(conn).await
}

pub async fn get_missions(conn: &mut DbConn) -> Result<Vec<Mission>, diesel::result::Error> {
    missions::table.select(Mission::as_select()).load(conn).await
}

/// Performs an user mission, updates EXP, handles level up, and tracks mission completion in
/// cache
///
/// # Parameters
/// - `conn`: Mutable reference to the database connection.
/// - `cache_conn`: Mutable reference to the cache connection.
/// - `user_id`: The ID of the user performing the mission.
/// - `code`: The mission's unique code string.
/// - `scale`: Optional multiplier for EXP reward.
///
/// # Returns
/// - `Ok(())` if the mission is performed successfully.
/// - `Err(String)` if mission cannot be completed or on error.
///
/// # Behavior
/// - Checks how many times the user has completed this mission today using cache.
/// - If the daily max is reached, returns an error.
/// - Calculates EXP reward, optionally scaled.
/// - Increments user's EXP and checks for level up; if level up, increases usage count as a gift.
/// - Increments mission count in the cache hash.
/// - Sets expiry on the cache has to midnight if this is the first completion today.
///
/// # Caching
/// Uses a hash with key format `mission:{user_id}:{today}`, where each field is mission code, and
/// its value is the completion count for today.
/// The hash expires at midnight.
///
pub async fn do_mission<'a>(conn: &mut DbConn, cache_conn: &mut CacheConn<'a>, user_id: &str, code: &str, scale: Option<i32>) -> Result<(), String> {
    let today = get_today();

    let cache_key = format!("mission:{}:{}", user_id, today);

    let current_count: Option<i32> = cache_conn.hget(&cache_key, code).await.map_err(|err| err.to_string())?;

    let mission = get_mission_by_code(conn, code).await.map_err(|err| err.to_string())?;

    if let Some(count) = current_count {
        if count >= mission.max_per_day.unwrap_or(0) {
            return Err("Mission already completed for today.".into());
        }
    }

    let exp_reward = {
        match scale {
            Some(num) => mission.exp_reward * num,
            None => mission.exp_reward,
        }
    };

    give_exp_to_user(conn, user_id, exp_reward).await.map_err(|err| err.to_string())?;

    let is_up = level_up(conn, user_id).await.map_err(|err| err.to_string())?;

    if is_up {
        give_usage_count_to_user(conn, user_id, 1).await.map_err(|err| err.to_string())?;
    }

    let _: i32 = cache_conn.hincr(&cache_key, code, 1).await.map_err(|err| err.to_string())?;

    if current_count.is_none() {
        let expire_time = get_seconds_to_midnight();

        let _: i64 = cache_conn.expire(&cache_key, expire_time).await.map_err(|err| err.to_string())?;
    }

    Ok(())
}
