use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum ServerSignal {
    Start,
    Stop,
    Restart,
    Kill,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum ServerStatus {
    Running,
    Stopped,
    Installing,
}
