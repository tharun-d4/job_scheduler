use lettre::{
    AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
    transport::smtp::AsyncSmtpTransport,
};

use crate::handlers::models::EmailInfo;

pub fn smtp_sender(server: &str, port: u16) -> AsyncSmtpTransport<Tokio1Executor> {
    AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(server)
        .port(port)
        .build()
}

pub async fn send_email(sender: AsyncSmtpTransport<Tokio1Executor>, info: EmailInfo) {
    let message = Message::builder()
        .from(info.from.parse().unwrap())
        .to(info.to.parse().unwrap())
        .subject(&info.subject)
        .header(ContentType::TEXT_PLAIN)
        .body(info.body)
        .unwrap();

    sender.send(message).await.unwrap();
}
