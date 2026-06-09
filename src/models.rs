use serde::{Serialize, Deserialize};

// Model for creating containers.
#[derive(Serialize, Deserialize)]
pub struct CreateContainer {
    pub name: String,
    pub image: String,
    pub options: String,
}

// Model for creating pods.
#[derive(Serialize, Deserialize)]
pub struct CreatePod {
    pub name: String,
    pub options: String,
}

// Model for creating networks.
#[derive(Serialize, Deserialize)]
pub struct CreateNetwork {
    pub name: String,
    pub options: String,
}

// Model for getting container state.
#[derive(Serialize, Deserialize)]
pub struct StateContainer {
    pub name: String,
    pub state: String,
}

// Model for getting pod state.
#[derive(Serialize, Deserialize)]
pub struct StatePod {
    pub name: String,
    pub state: String,
}

// Model for creating env file.
#[derive(Serialize, Deserialize)]
pub struct CreateEnvFile {
    pub name: String,
    pub content: String,
    pub replace: String,
}

// Model for creating secret.
#[derive(Serialize,Deserialize)]
pub struct CreateSecret {
    pub name: String,
    pub data: String,
    pub labels: String,
    pub replace: String,
}

// Model for deleting env file.
#[derive(Serialize,Deserialize)]
pub struct DeleteEnvFile {
    pub name: String,
    pub key: String,
}

// Model for deleting secrets file.
#[derive(Serialize,Deserialize)]
pub struct DeleteSecFile {
    pub name: String,
    pub key: String,
}

// Model for deleting a container.
#[derive(Serialize,Deserialize)]
pub struct DeleteContainer {
    pub name: String,
    pub key: String,
}

// Model for deleting unused images.
#[derive(Serialize,Deserialize)]
pub struct DeleteImages {
    pub key: String,
}

// Model for deleteing network.
#[derive(Serialize,Deserialize)]
pub struct DeleteNetwork {
    pub name: String,
    pub key: String,
}

// Model for deleteing volume
#[derive(Serialize,Deserialize)]
pub struct DeleteVolume {
    pub name: String,
    pub key: String,
}

#[derive(Serialize,Deserialize)]
pub struct DeletePod {
    pub name: String,
    pub key: String,
}