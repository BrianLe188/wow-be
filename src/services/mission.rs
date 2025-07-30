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

pub async fn create_mission(
    conn: &mut DbConn,
    payload: &NewMission,
) -> Result<Mission, diesel::result::Error> {
    diesel::insert_into(missions::table)
        .values(payload)
        .returning(Mission::as_returning())
        .get_result::<Mission>(conn)
        .await
}

pub async fn get_mission_by_code(
    conn: &mut DbConn,
    code: &str,
) -> Result<Mission, diesel::result::Error> {
    missions::table
        .filter(missions::code.eq(code))
        .select(Mission::as_select())
        .first::<Mission>(conn)
        .await
}

pub async fn get_missions(conn: &mut DbConn) -> Result<Vec<Mission>, diesel::result::Error> {
    missions::table
        .select(Mission::as_select())
        .load(conn)
        .await
}

pub async fn do_mission<'a>(
    conn: &mut DbConn,
    cache_conn: &mut CacheConn<'a>,
    user_id: &str,
    code: &str,
    scale: Option<i32>,
) -> Result<(), String> {
    let today = get_today();

    let cache_key = format!("mission:{}:{}", user_id, today);

    let current_count: Option<i32> = cache_conn
        .hget(&cache_key, code)
        .await
        .map_err(|err| err.to_string())?;

    let mission = get_mission_by_code(conn, code)
        .await
        .map_err(|err| err.to_string())?;

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

    give_exp_to_user(conn, user_id, exp_reward)
        .await
        .map_err(|err| err.to_string())?;

    let is_up = level_up(conn, user_id)
        .await
        .map_err(|err| err.to_string())?;

    if is_up {
        give_usage_count_to_user(conn, user_id, 1)
            .await
            .map_err(|err| err.to_string())?;
    }

    let _: i32 = cache_conn
        .hincr(&cache_key, code, 1)
        .await
        .map_err(|err| err.to_string())?;

    if let Some(count) = current_count {
        if count == 0 {
            let _: i64 = cache_conn
                .expire(&cache_key, get_seconds_to_midnight())
                .await
                .map_err(|err| err.to_string())?;
        }
    }

    Ok(())
}
