use std::time::Duration;

use tonic::{
    codegen::InterceptedService,
    metadata::MetadataValue,
    server::ClientStreamingService,
    transport::{self, Channel, Endpoint},
    Request,
};

use crate::google::firestore::v1::firestore_client::FirestoreClient;

mod auth;

static FIRESTORE_ENDPOINT: &str = "https://firestore.googleapis.com";
static FIRESTORE_TOKEN_AUDIENCE: &str =
    "https://firestore.googleapis.com/google.firestore.v1beta1.Firestore";

// TODO: Connection or client?
// Probably client tbh, but can rename later
struct Connection {
    client: FirestoreClient<InterceptedService<Channel, Interceptor>>,
}

enum Credentials {
    Default,
    Emulator,
    EmulatorOwner,
}

struct ConnectionBuilder {
    endpoint: Endpoint,
    credentials: Credentials,
}

impl ConnectionBuilder {
    pub fn new() -> ConnectionBuilder {
        // TODO: Use env vars to guess emulator?
        ConnectionBuilder {
            endpoint: Endpoint::from_static(FIRESTORE_ENDPOINT),
            credentials: Credentials::Default,
        }
    }

    pub fn https_endpoint(self, url: &str) -> Self {
        ConnectionBuilder {
            endpoint: Endpoint::from_shared(format!("https://{}", url))
                .expect("Invalid firestore URL"),
            ..self
        }
    }

    pub fn http_endpoint(self, url: &str) -> Self {
        ConnectionBuilder {
            endpoint: Endpoint::from_shared(format!("http://{}", url))
                .expect("Invalid firestore URL"),
            ..self
        }
    }

    /// Set whether TCP keepalive messages are enabled on connections.
    ///
    /// If None is specified, keepalive is disabled, otherwise the duration
    /// specified will be the time to remain idle before sending TCP keepalive
    /// probes.
    ///
    /// Default is no keepalive (None)
    pub fn tcp_keepalive(self, duration: impl Into<Option<Duration>>) -> Self {
        ConnectionBuilder {
            endpoint: self.endpoint.tcp_keepalive(duration.into()),
            ..self
        }
    }

    /// Apply a timeout to each request.
    pub fn timeout(self, duration: Duration) -> Self {
        ConnectionBuilder {
            endpoint: self.endpoint.timeout(duration),
            ..self
        }
    }

    pub fn default_credentials(self) -> Self {
        ConnectionBuilder {
            credentials: Credentials::Default,
            ..self
        }
    }

    pub fn emulator_credentials(self) -> Self {
        ConnectionBuilder {
            credentials: Credentials::Emulator,
            ..self
        }
    }

    pub fn emulator_owner_credentials(self) -> Self {
        ConnectionBuilder {
            credentials: Credentials::EmulatorOwner,
            ..self
        }
    }

    #[allow(clippy::redundant_closure)]
    pub async fn connect(self) -> Result<Connection, ConnectionError> {
        // TODO: Probably want retries at some point?
        let channel = self.endpoint.connect().await?;
        let interceptor: Interceptor = match self.credentials {
            Credentials::Default => {
                let token = auth::Token::from_default_credentials(FIRESTORE_TOKEN_AUDIENCE)?;
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
        Ok(Connection { client })
    }
}

type Interceptor = Box<dyn FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>>;

#[derive(thiserror::Error, Debug)]
enum ConnectionError {
    #[error("Error encoding JWT from credentials: {0}")]
    JwtError(String),
    #[error("gRPC transport error: {0}")]
    TransportError(#[from] transport::Error),
}

impl From<frank_jwt::Error> for ConnectionError {
    fn from(e: frank_jwt::Error) -> Self {
        ConnectionError::JwtError(e.to_string())
    }
}
