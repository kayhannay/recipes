use rocket::http::Cookies;

pub const COOKIE_NAME: &str = "user";

#[derive(Debug, Serialize)]
pub enum MessageType {
    ERROR,
    WARN,
    INFO,
    None
}

#[derive(Debug, Serialize)]
pub struct User(String);

#[derive(Debug, Serialize)]
pub struct CommonContext {
    pub current_user: Option<User>,
    pub message: Option<String>,
    pub message_type: MessageType
}

pub fn get_current_user(mut cookies: Cookies) -> Option<User> {
    cookies
        .get_private(COOKIE_NAME)
        .and_then(|cookie| cookie.value().parse().ok())
        .map(User)
}
