use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use crate::{
    config::db::DbConn,
    models::user_place_access::{NewUserPlaceAccess, UserPlaceAccess},
    schema::user_place_access,
};

pub async fn create_user_place_access(conn: &mut DbConn, payload: &NewUserPlaceAccess) -> Result<UserPlaceAccess, diesel::result::Error> {
    diesel::insert_into(user_place_access::table)
        .values(payload)
        .returning(UserPlaceAccess::as_returning())
        .get_result::<UserPlaceAccess>(conn)
        .await
}
