use serde::{Deserialize, Serialize};
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Hits(u64);

impl Hits {
    pub fn new(data: u64) -> Self {
        Self(data)
    }

    pub fn into_inner(self) -> Self {
        self.0
    }
}