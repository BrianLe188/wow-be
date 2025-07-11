use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;

use crate::{
    config::db::DbConn,
    models::user::{NewUser, User},
    schema::users,
};

pub async fn get_user_by_email(
    conn: &mut DbConn,
    email: &str,
) -> Result<User, diesel::result::Error> {
    users::table
        .filter(users::email.eq(email))
        .select(User::as_select())
        .first::<User>(conn)
        .await
}

pub async fn create_user<'a>(
    conn: &mut DbConn,
    payload: &'a NewUser<'a>,
) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users::table)
        .values(payload)
        .returning(User::as_returning())
        .get_result::<User>(conn)
        .await
}
