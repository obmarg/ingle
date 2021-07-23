#[path = "google.api.rs"]
pub mod api;

pub mod firestore {
    #[path = "../google.firestore.v1.rs"]
    pub mod v1;
}

#[path = "google.protobuf.rs"]
pub mod protobuf;

#[path = "google.r#type.rs"]
pub mod r#type;

#[path = "google.rpc.rs"]
pub mod rpc;
