use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    config::db::DbConn,
    models::user::{NewUser, User, UserPhotoChangeset},
    schema::users,
};

pub async fn update_user_photo(conn: &mut DbConn, id: &str, field: &str, url: &str) -> Result<User, diesel::result::Error> {
    let user_uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    let mut changes = UserPhotoChangeset { avatar_url: None, cover_url: None };

    if field == "avatar_url" {
        changes.avatar_url = Some(url.to_string());
    }

    if field == "cover_url" {
        changes.cover_url = Some(url.to_string());
    }

    diesel::update(users::table.filter(users::id.eq(user_uuid)))
        .set(&changes)
        .returning(User::as_returning())
        .get_result::<User>(conn)
        .await
}

pub async fn get_user_by_id(conn: &mut DbConn, id: &str) -> Result<User, diesel::result::Error> {
    let user_uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    users::table.filter(users::id.eq(user_uuid)).select(User::as_select()).first::<User>(conn).await
}

pub async fn get_user_by_email(conn: &mut DbConn, email: &str) -> Result<User, diesel::result::Error> {
    users::table.filter(users::email.eq(email)).select(User::as_select()).first::<User>(conn).await
}

pub async fn create_user(conn: &mut DbConn, payload: &NewUser) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users::table).values(payload).returning(User::as_returning()).get_result::<User>(conn).await
}

pub async fn give_exp_to_user(conn: &mut DbConn, id: &str, exp: i32) -> Result<(), diesel::result::Error> {
    let user_uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    diesel::update(users::table.filter(users::id.eq(user_uuid)))
        .set(users::exp.eq(users::exp + exp))
        .returning(User::as_returning())
        .get_result::<User>(conn)
        .await?;

    Ok(())
}

pub async fn level_up(conn: &mut DbConn, id: &str) -> Result<bool, diesel::result::Error> {
    let user = get_user_by_id(conn, id).await?;

    let exp = user.exp.unwrap_or(0);
    let level = user.level.unwrap_or(0);

    let level_up_exp = 200;

    if exp >= level_up_exp {
        let add_level = exp / level_up_exp;
        let remain_exp = exp % level_up_exp;

        let user_uuid = Uuid::parse_str(id).map_err(|_| diesel::result::Error::NotFound)?;

        diesel::update(users::table.filter(users::id.eq(user_uuid)))
            .set((users::level.eq(level + add_level), users::exp.eq(remain_exp)))
            .execute(conn)
            .await?;

        return Ok(true);
    }

    Ok(false)
}
