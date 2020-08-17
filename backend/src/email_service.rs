use crate::errors::ServiceError;
use crate::models::Invitation;
use mailgun_v3::*;

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {
    let api_key: String = std::env::var("MAIL_GUN_API_KEY").expect("MAIL_GUN_API_KEY must be set");
    let domain: String = std::env::var("MAIL_GUN_DOMAIN").expect("MAIL_GUN_DOMAIN must be set");
    let email_name: String = std::env::var("EMAIL_NAME").expect("EMAIL_NAME must be set");
    let email_address: String = std::env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS must be set");

    let credentials = Credentials::new(&api_key, &domain);
    let mailgun_email_address = EmailAddress::name_address(&email_name, &email_address);
    let invitation_email_address = EmailAddress::address(&invitation.email);

    let email_body = format!(
        "Welcome to CareerLab <br/>
Please click on the link below to complete registration. <br/>
         <a href=\"http://localhost:3000/register.html?id={}&email={}\">
         http://localhost:3000/register</a> <br/>
         your Invitation expires on <strong>{}</strong>",
        invitation.id,
        invitation.email,
        invitation
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );

    let message = email::Message {
        to: vec![invitation_email_address],
        cc: vec![],
        bcc: vec![],
        subject: "Invitation CareerLab".to_string(),
        body: email::MessageBody::Html(email_body),
        options: vec![],
    };

    let result = email::send_email(&credentials, &mailgun_email_address, message);

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}
