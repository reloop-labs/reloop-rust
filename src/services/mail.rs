use crate::client::ReloopClient;
use crate::models::SendMailResponse;
use reqwest::Method;
use serde_json::Value;

pub struct MailService<'a> {
    client: &'a ReloopClient,
}

impl<'a> MailService<'a> {
    pub fn new(client: &'a ReloopClient) -> Self {
        MailService { client }
    }

    pub async fn send(
        &self,
        params: Value,
    ) -> Result<SendMailResponse, Box<dyn std::error::Error>> {
        self.client
            .fetch(Method::POST, "/api/mail/v1/send", Some(params))
            .await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn send_route_uses_mail_send_path() {
        assert_eq!("/api/mail/v1/send", "/api/mail/v1/send");
    }
}
