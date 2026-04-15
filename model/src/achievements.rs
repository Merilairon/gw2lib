pub mod categories;
pub mod groups;

use serde::{Deserialize, Serialize};
use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AchievementTier {
    pub count: u32,
    pub points: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Achievement {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub requirement: String,
    pub locked_text: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub flags: Vec<String>,
    pub tiers: Vec<AchievementTier>,
    pub point_cap: Option<i32>,
    pub prerequisites: Option<Vec<u32>>,
    pub bits: Option<Vec<serde_json::Value>>,
    pub rewards: Option<Vec<serde_json::Value>>,
    pub icon: Option<String>,
}

impl Endpoint for Achievement {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/achievements";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Achievement {
    type IdType = u32;
}
impl BulkEndpoint for Achievement {
    const ALL: bool = false;
    fn id(&self) -> &Self::IdType { &self.id }
}
