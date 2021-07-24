mod service;
mod token;

pub use service::AuthService;
pub use token::Token;

#[derive(Clone)]
pub enum Credentials {
    ServiceAccount(Token),
    AuthToken(String),
    EmulatorOwner,
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Credentials::ServiceAccount(_) => write!(f, "Credentials::ServiceAccount(_)"),
            Credentials::AuthToken(_) => write!(f, "Credentials::AuthToken(_)"),
            Credentials::EmulatorOwner => write!(f, "Credentials::EmulatorOwner"),
        }
    }
}
