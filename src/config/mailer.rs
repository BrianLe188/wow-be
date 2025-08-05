use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::{authentication::Credentials, response::Response},
};
use std::env;

pub fn init_mailer(username: &str, password: &str, relay_mail: &str) -> SmtpTransport {
    let creds = Credentials::new(username.to_owned(), password.to_owned());

    SmtpTransport::relay(relay_mail).unwrap().credentials(creds).build()
}

pub fn mailer_send(mailer: &SmtpTransport, mail: &Message) -> Result<Response, String> {
    mailer.send(mail).map_err(|err| err.to_string())
}

pub fn mail_template(to_mail: &str, body: &str) -> Result<Message, String> {
    let from_mail = env::var("MAILER_FROM_MAIL").map_err(|err| err.to_string())?;

    Ok(Message::builder()
        .from(Mailbox::new(Some("Wow".to_owned()), from_mail.as_str().parse().unwrap()))
        .to(Mailbox::new(Some(to_mail.to_owned()), to_mail.parse().unwrap()))
        .subject("You've Been Invited to Wow App")
        .header(ContentType::TEXT_HTML)
        .body(body.to_string())
        .unwrap())
}
