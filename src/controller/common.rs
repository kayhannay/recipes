use crypto::digest::Digest;
use crypto::sha2::Sha512;
use rocket::http::Cookies;
use rocket::request::FlashMessage;

pub const COOKIE_NAME: &str = "user";

#[derive(Debug, Serialize, PartialEq)]
pub enum MessageType {
    ERROR,
    WARN,
    INFO,
    None,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    pub name: String,
    pub uid: i32,
}

#[derive(Debug, Serialize)]
pub struct CommonContext {
    pub current_user: Option<User>,
    pub message: Option<String>,
    pub message_type: MessageType,
}

pub fn get_current_user(mut cookies: Cookies) -> Option<User> {
    cookies
        .get_private(COOKIE_NAME)
        .and_then(|cookie| cookie.value().parse::<String>().ok())
        .map(|value| serde_json::from_str(&value).unwrap())
}

pub fn create_hash(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.input_str(input);
    hasher.result_str()
}

pub fn create_common_context(flash: Option<FlashMessage>, user: Option<User>) -> CommonContext {
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
    }
    common
}

#[cfg(test)]
mod test {
    use super::create_common_context;
    use controller::common::{MessageType, User};

    #[test]
    fn should_create_empty_context() {
        // When
        let result = create_common_context(None, None);

        // Then
        assert!(result.current_user.is_none());
        assert!(result.message.is_none());
        assert_eq!(result.message_type, MessageType::None);
    }

    #[test]
    fn should_create_context_with_user() {
        // Given
        let user = User {
            name: "foo".to_string(),
            uid: 0,
        };

        // When
        let result = create_common_context(None, Some(user.clone()));

        // Then
        assert_eq!(result.current_user, Some(user));
        assert!(result.message.is_none());
        assert_eq!(result.message_type, MessageType::None);
    }
}
