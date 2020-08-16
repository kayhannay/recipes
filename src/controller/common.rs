use crypto::digest::Digest;
use crypto::sha2::Sha512;
use rocket::http::Cookies;
use rocket::request::FlashMessage;
use std::collections::HashMap;

pub const COOKIE_NAME: &str = "user";

#[derive(Debug, Serialize)]
pub enum MessageType {
    ERROR,
    WARN,
    INFO,
    None,
}

#[derive(Debug, Serialize)]
pub struct User(String);

#[derive(Debug, Serialize)]
pub struct CommonContext {
    pub current_user: Option<User>,
    pub message: Option<String>,
    pub message_type: MessageType,
}

pub fn get_current_user(mut cookies: Cookies) -> Option<User> {
    cookies
        .get_private(COOKIE_NAME)
        .and_then(|cookie| cookie.value().parse().ok())
        .map(User)
}

pub fn create_hash(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.input_str(input);
    hasher.result_str()
}

pub fn create_common_context<'a>(
    flash: Option<FlashMessage>,
    user: Option<User>,
) -> HashMap<&'a str, CommonContext> {
    let mut context = HashMap::new();
    let mut common = CommonContext {
        current_user: user,
        message: None,
        message_type: MessageType::None,
    };
    if let Some(ref msg) = flash {
        let message_type = match msg.name() {
            "error" => MessageType::ERROR,
            "warning" => MessageType::WARN,
            _ => MessageType::INFO,
        };
        common.message = Some(msg.msg().to_string());
        common.message_type = message_type;
        context.insert("common", common);
    }
    context
}
