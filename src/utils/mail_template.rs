use tera::{Context, Tera};

pub fn invite_user_mail_body(invite_link: &str) -> Result<String, String> {
    let template = r#"
    <div style="font-family:Arial,sans-serif;background:#f4f6fb;padding:24px;">
        <table style="max-width:480px;margin:auto;background:#fff;border-radius:12px;box-shadow:0 2px 8px #eee;">
          <tr>
            <td style="padding:32px;">
              <h2 style="color:#204080;">You're Invited!</h2>
              <p style="color:#333;font-size:16px;">
                Hello,<br>
                <br>
                You've been invited to join our platform! Click the button below to accept your invitation:
              </p>
              <a href="{{ invite_link }}" style="display:inline-block;margin:24px 0;padding:15px 32px;background:#3479f6;color:#fff;border-radius:6px;text-decoration:none;font-weight:bold;font-size:16px;">
                Accept Invitation
              </a>
              <p style="color:#999;font-size:13px;">
                If you did not expect this invitation, please safely ignore this email.
              </p>
            </td>
          </tr>
        </table>
    </div>
        "#;

    let mut tera = Tera::default();
    tera.add_raw_template("invite.html", template)
        .map_err(|err| err.to_string())?;

    let mut context = Context::new();
    context.insert("invite_link", invite_link);

    tera.render("invite.html", &context)
        .map_err(|err| err.to_string())
}
