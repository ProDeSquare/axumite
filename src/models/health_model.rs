use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    pub status: &'static str,
    pub version: &'static str,
}

impl Health {
    pub fn ok() -> Self {
        Health {
            status: "ok",
            version: "0.1.0",
        }
    }
}
