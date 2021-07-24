use std::time::Duration;

use tonic::{
    metadata::MetadataValue,
    transport::{self, Endpoint},
    Request,
};

use super::{auth::Token, Database, Interceptor};
use crate::google::firestore::v1::firestore_client::FirestoreClient;
use crate::paths::ProjectPath;

static FIRESTORE_ENDPOINT: &str = "https://firestore.googleapis.com";
static FIRESTORE_TOKEN_AUDIENCE: &str =
    "https://firestore.googleapis.com/google.firestore.v1beta1.Firestore";
static DEFAULT_DATABASE: &str = "(default)";

pub struct DatabaseBuilder {
    endpoint: Endpoint,
    credentials: Credentials,
    project_id: String,
    database_id: String,
}

impl DatabaseBuilder {
    pub fn new(project_id: impl Into<String>) -> DatabaseBuilder {
        DatabaseBuilder {
            endpoint: Endpoint::from_static(FIRESTORE_ENDPOINT),
            credentials: Credentials::Default,
            project_id: project_id.into(),
            database_id: DEFAULT_DATABASE.to_string(),
        }
    }

    pub fn https_endpoint(self, url: &str) -> Self {
        DatabaseBuilder {
            endpoint: Endpoint::from_shared(format!("https://{}", url))
                .expect("Invalid firestore URL"),
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

    pub fn default_credentials(self) -> Self {
        DatabaseBuilder {
            credentials: Credentials::Default,
            ..self
        }
    }

    pub fn emulator_credentials(self) -> Self {
        DatabaseBuilder {
            credentials: Credentials::Emulator,
            ..self
        }
    }

    pub fn emulator_owner_credentials(self) -> Self {
        DatabaseBuilder {
            credentials: Credentials::EmulatorOwner,
            ..self
        }
    }

    #[allow(clippy::redundant_closure)]
    pub async fn connect(self) -> Result<Database, ConnectError> {
        let channel = self.endpoint.connect().await?;
        let interceptor: Interceptor = match self.credentials {
            Credentials::Default => {
                let token = Token::from_default_credentials(FIRESTORE_TOKEN_AUDIENCE)?;
                Box::new(move |mut req: Request<()>| {
                    req.metadata_mut().insert(
                        "authorization",
                        MetadataValue::from_str(&format!("Bearer {}", token.jwt())).unwrap(),
                    );
                    Ok(req)
                })
            }
            Credentials::Emulator => Box::new(move |req: Request<()>| Ok(req)),
            Credentials::EmulatorOwner => Box::new(move |mut req: Request<()>| {
                req.metadata_mut().insert(
                    "authorization",
                    MetadataValue::from_str("Bearer owner").unwrap(),
                );
                Ok(req)
            }),
        };

        let client = FirestoreClient::with_interceptor(channel, interceptor);
        Ok(Database {
            client,
            project_path: ProjectPath::new(self.project_id, self.database_id),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConnectError {
    #[error("Error encoding JWT from credentials: {0}")]
    JwtError(String),
    #[error("gRPC transport error: {0}")]
    TransportError(#[from] transport::Error),
}

impl From<frank_jwt::Error> for ConnectError {
    fn from(e: frank_jwt::Error) -> Self {
        ConnectError::JwtError(e.to_string())
    }
}

enum Credentials {
    Default,
    Emulator,
    EmulatorOwner,
}
