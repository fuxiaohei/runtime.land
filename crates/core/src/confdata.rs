use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// RouteConfItem is config item for one project deployment route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfItem {
    pub domain: String,
    pub module: String,
    pub key: String,
    pub time_at: u64,
}

// RoutesConf is config for all project deployment routes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutesConf {
    pub items: Vec<RouteConfItem>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegionIPInfo {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub timezone: String,
    pub readme: String,
}

impl RegionIPInfo {
    pub fn region(&self) -> String {
        format!("{}-{}-{}", self.country, self.region, self.city)
    }
    pub fn region_ip(&self) -> String {
        format!("{}-{}-{}-{}", self.country, self.region, self.city, self.ip)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeData {
    pub hostname: String,
    pub cpu_count: usize,
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionReportData {
    pub localip: RegionIPInfo,
    pub region: String,
    pub runtimes: HashMap<String, RuntimeData>,
    pub conf_value_time_version: u64,
    pub time_at: u64,

    #[serde(skip)]
    pub owner_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionRecvData {
    pub conf_values: RoutesConf,
}
