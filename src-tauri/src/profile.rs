use crate::config::DiskPreference;
use crate::config::SectionConfig;
use crate::config::KNOWN_SECTION_IDS;
use crate::monitor::MonitorCatalog;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const PROFILE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Profile {
    #[serde(default = "default_profile_schema_version")]
    pub profile_schema_version: u32,
    #[serde(default)]
    pub sections: Vec<SectionConfig>,
    #[serde(default)]
    pub system_fields: Vec<String>,
    #[serde(default = "default_true")]
    pub show_cpu_cores: bool,
    #[serde(default)]
    pub disks: Vec<DiskPreference>,
    #[serde(default = "default_collect_interval")]
    pub collect_interval_ms: u64,
    #[serde(default)]
    pub monitor_catalog: MonitorCatalog,
    #[serde(flatten)]
    pub extra: BTreeMap<String, toml::Value>,
}

fn default_profile_schema_version() -> u32 { PROFILE_SCHEMA_VERSION }
fn default_collect_interval() -> u64 { 1000 }
fn default_true() -> bool { true }

impl Profile {
    pub fn known_sections(&self) -> Vec<&SectionConfig> {
        self.sections.iter().filter(|s| KNOWN_SECTION_IDS.contains(&s.id.as_str())).collect()
    }
    pub fn first_enabled_known_section(&self) -> Option<&SectionConfig> {
        self.known_sections().into_iter().find(|s| s.enabled)
    }
    pub fn section(&self, instance: &str) -> Option<&SectionConfig> {
        self.sections.iter().find(|s| s.instance == instance)
    }
    pub fn disk_preference(&self, id: &str) -> Option<&DiskPreference> {
        self.disks.iter().find(|d| d.id == id)
    }
}
