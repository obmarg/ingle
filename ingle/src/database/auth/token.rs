use std::{
    fs,
    io::{self, Write},
    path::Path,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime},
};

use frank_jwt as jwt;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone)]
pub struct Token {
    audience: String,
    credentials: Credentials,
    inner: Arc<Mutex<TokenInner>>,
}

impl Token {
    pub fn jwt(&self) -> String {
        let mut inner = self.inner.lock().unwrap();

        if inner.is_expiring() {
            // Note: calling unwrap here as we've already validated the audience & credentials
            // in Token::new so if this fails then something really bad is happening.
            *inner =
                TokenInner::generate(&self.audience, &self.credentials, Instant::now()).unwrap();
        }

        inner.jwt.clone()
    }

    pub fn new(
        audience: impl ToString,
        credentials: Credentials,
    ) -> Result<Self, frank_jwt::Error> {
        let audience = audience.to_string();
        let inner = TokenInner::generate(&audience, &credentials, Instant::now())?;

        Ok(Token {
            audience,
            credentials,
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub fn from_file(
        path: impl AsRef<Path>,
        audience: impl ToString,
    ) -> Result<Self, frank_jwt::Error> {
        Token::new(audience, credentials_from_file(path)?)
    }
}

static JWT_VALID_TIME: Duration = Duration::from_secs(10 * 60);
static JWT_REFRESH_TIME: Duration = Duration::from_secs(9 * 60);

struct TokenInner {
    jwt: String,
    encoded_at: Instant,
}

impl TokenInner {
    pub fn is_expiring(&self) -> bool {
        self.encoded_at.elapsed() >= JWT_REFRESH_TIME
    }

    pub fn generate(
        audience: &str,
        credentials: &Credentials,
        now: Instant,
    ) -> Result<Self, frank_jwt::Error> {
        let email = &credentials.client_email;

        let now_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Something fucky is happening with system time")
            .as_secs();

        let expires_at_timestamp = (SystemTime::now() + JWT_VALID_TIME)
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Something fucky is happening with system time")
            .as_secs();

        let claims = json!({
            "sub": &email,
            "iss": &email,
            "aud": &audience,
            "iat": now_timestamp,
            "exp": expires_at_timestamp
        });

        let header = json!({});

        Ok(TokenInner {
            jwt: jwt::encode(
                header,
                &credentials.private_key,
                &claims,
                jwt::Algorithm::RS256,
            )?,
            encoded_at: now,
        })
    }
}

/// JSON schema of the GOOGLE_APPLICATION_CREDENTIALS file.
///
/// You can use `credentials_from_file()` as a quick way to read the JSON
/// into a Credentials.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    #[serde(rename = "type")]
    pub key_type: Option<String>,
    pub project_id: Option<String>,
    pub private_key_id: Option<String>,
    pub private_key: String,
    pub client_email: String,
    pub client_id: Option<String>,
    pub auth_uri: Option<String>,
    pub token_uri: Option<String>,
    pub auth_provider_x509_cert_url: Option<String>,
    pub client_x509_cert_url: Option<String>,
}

/// Read a service account key from a JSON file. You can download the JSON keys from the Google
/// Cloud Console or the respective console of your service provider.
pub fn credentials_from_file<S: AsRef<Path>>(path: S) -> io::Result<Credentials> {
    let file = fs::OpenOptions::new().read(true).open(path)?;
    match serde_json::from_reader(file) {
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))),
        Ok(decoded) => Ok(decoded),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_expiring() {
        let creds = serde_json::from_str(include_str!("service-account-test.json")).unwrap();

        let token = TokenInner::generate(
            "test-audience",
            &creds,
            Instant::now()
                .checked_sub(Duration::from_secs(8 * 60))
                .unwrap(),
        )
        .unwrap();

        assert!(!token.is_expiring());

        let token = TokenInner::generate(
            "test-audience",
            &creds,
            Instant::now()
                .checked_sub(Duration::from_secs(9 * 60))
                .unwrap(),
        )
        .unwrap();

        assert!(token.is_expiring());
    }
}
