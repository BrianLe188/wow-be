use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::deadpool::Object};

use crate::{models::user::User, schema::users};

pub async fn get_user_by_email(
    conn: &mut Object<AsyncPgConnection>,
    email: &str,
) -> Result<User, diesel::result::Error> {
    users::table
        .filter(users::email.eq(email))
        .select(User::as_select())
        .first::<User>(conn)
        .await
}
