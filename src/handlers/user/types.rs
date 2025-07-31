use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct InvitePayload {
    #[validate(email(message = "Please provide a valid email address."))]
    pub email: String,
}
