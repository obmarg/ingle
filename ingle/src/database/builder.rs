use std::time::Duration;

use tonic::transport::{self, ClientTlsConfig, Endpoint};

use super::{
    auth::{AuthService, Credentials, Token},
    Database,
};
use crate::google::firestore::v1::firestore_client::FirestoreClient;
use crate::paths::ProjectPath;

static FIRESTORE_ENDPOINT: &str = "https://firestore.googleapis.com";
static FIRESTORE_TOKEN_AUDIENCE: &str =
    "https://firestore.googleapis.com/google.firestore.v1beta1.Firestore";
static DEFAULT_DATABASE: &str = "(default)";

pub struct DatabaseBuilder {
    endpoint: Endpoint,
    credentials: Option<Credentials>,
    project_id: String,
    database_id: String,
}

impl DatabaseBuilder {
    pub fn new(project_id: impl Into<String>) -> DatabaseBuilder {
        DatabaseBuilder {
            endpoint: Endpoint::from_static(FIRESTORE_ENDPOINT)
                .tls_config(ClientTlsConfig::default())
                .expect("Couldn't configure TLS"),
            credentials: None,
            project_id: project_id.into(),
            database_id: DEFAULT_DATABASE.to_string(),
        }
    }

    pub fn https_endpoint(self, url: &str) -> Self {
        DatabaseBuilder {
            endpoint: Endpoint::from_shared(format!("https://{}", url))
                .expect("Invalid firestore URL")
                .tls_config(ClientTlsConfig::default())
                .expect("Couldn't initialise TLS"),
            ..self
        }
    }

    pub fn http_endpoint(self, url: &str) -> Self {
        DatabaseBuilder {
            endpoint: Endpoint::from_shared(format!("http://{}", url))
                .expect("Invalid firestore URL"),
            ..self
        }
    }

    /// Set whether TCP keepalive messages are enabled on Databases.
    ///
    /// If None is specified, keepalive is disabled, otherwise the duration
    /// specified will be the time to remain idle before sending TCP keepalive
    /// probes.
    ///
    /// Default is no keepalive (None)
    pub fn tcp_keepalive(self, duration: impl Into<Option<Duration>>) -> Self {
        DatabaseBuilder {
            endpoint: self.endpoint.tcp_keepalive(duration.into()),
            ..self
        }
    }

    /// Apply a timeout to each request.
    pub fn timeout(self, duration: Duration) -> Self {
        DatabaseBuilder {
            endpoint: self.endpoint.timeout(duration),
            ..self
        }
    }

    pub fn default_credentials(self) -> Result<Self, DefaultCredentialsError> {
        let filename = match std::env::var("GOOGLE_APPLICATION_CREDENTIALS") {
            Ok(filename) => filename,
            Err(std::env::VarError::NotPresent) => {
                return Err(DefaultCredentialsError::MissingEnvVar)
            }
            Err(std::env::VarError::NotUnicode(_)) => {
                return Err(DefaultCredentialsError::MalformedEnvVar)
            }
        };

        Ok(DatabaseBuilder {
            credentials: Some(Credentials::ServiceAccount(Token::from_file(
                filename,
                FIRESTORE_TOKEN_AUDIENCE,
            )?)),
            ..self
        })
    }

    pub fn emulator_owner_credentials(self) -> Self {
        DatabaseBuilder {
            credentials: Some(Credentials::EmulatorOwner),
            ..self
        }
    }

    pub fn auth_token(self, token: impl Into<String>) -> Self {
        DatabaseBuilder {
            credentials: Some(Credentials::AuthToken(token.into())),
            ..self
        }
    }

    #[allow(clippy::redundant_closure)]
    pub async fn connect(self) -> Result<Database, ConnectError> {
        // TODO: Probably want retries at some point?
        let channel = self.endpoint.connect().await?;
        let credentials = self.credentials;

        let service = AuthService::new(channel, credentials);

        Ok(Database {
            client: FirestoreClient::new(service),
            project_path: ProjectPath::new(self.project_id, self.database_id),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DefaultCredentialsError {
    #[error("The GOOGLE_APPLICATION_CREDENTIALS environment variable could not be loaded")]
    MissingEnvVar,
    #[error("The GOOGLE_APPLICATION_CREDENTIALS environment variable was not valid unicode")]
    MalformedEnvVar,
    #[error("Error encoding a JWT from credentials: {0}")]
    JwtError(String),
}

impl From<frank_jwt::Error> for DefaultCredentialsError {
    fn from(e: frank_jwt::Error) -> Self {
        DefaultCredentialsError::JwtError(e.to_string())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConnectError {
    #[error("Error encoding JWT from credentials: {0}")]
    JwtError(String),
    #[error("gRPC transport error: {0}")]
    TransportError(#[from] transport::Error),
}
