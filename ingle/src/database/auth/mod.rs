mod service;
mod token;

pub use service::AuthService;
pub use token::Token;

#[derive(Clone)]
pub enum Credentials {
    Default(Token),
    EmulatorOwner,
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Credentials::Default(_) => write!(f, "Credentials::Default(_)"),
            Credentials::EmulatorOwner => write!(f, "Credentials::EmulatorOwner"),
        }
    }
}
