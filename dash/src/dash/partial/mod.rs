use std::collections::HashMap;

use saasbase::{Database, UserId};
use uuid::Uuid;

use crate::data::{Project, UserData};
use crate::Result;

pub mod footer {
    pub fn year() -> String {
        use chrono::Datelike;
        chrono::Utc::now().year().to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Head {
    pub title: String,
    pub og_site_name: String,
}

impl Default for Head {
    fn default() -> Self {
        Self {
            title: "Dashboard".to_string(),
            og_site_name: "RUDA".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SidebarEntry {
    pub name: String,
    pub url: String,
    pub active: bool,
    pub dropdown: bool,
    pub children: Vec<SidebarEntry>,
}

impl Default for SidebarEntry {
    fn default() -> Self {
        Self {
            name: "entry".to_string(),
            url: "".to_string(),
            active: false,
            dropdown: false,
            children: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sidebar {
    pub entries: Vec<SidebarEntry>,
    pub projects: Vec<Project>,
    pub current_project_id: Uuid,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            projects: vec![],
            entries: vec![
                SidebarEntry {
                    name: "Summary".to_string(),
                    url: "/".to_string(),
                    ..Default::default()
                },
                SidebarEntry {
                    name: "Applications".to_string(),
                    url: "/apps".to_string(),
                    ..Default::default()
                },
                SidebarEntry {
                    name: "Machines".to_string(),
                    url: "/machines".to_string(),
                    ..Default::default()
                },
                SidebarEntry {
                    name: "Sources".to_string(),
                    url: "/sources".to_string(),
                    ..Default::default()
                },
                SidebarEntry {
                    name: "Storages".to_string(),
                    url: "/storages".to_string(),
                    ..Default::default()
                },
                SidebarEntry {
                    name: "Notifications".to_string(),
                    url: "/notifications".to_string(),
                    ..Default::default()
                },
                SidebarEntry {
                    name: "Keys".to_string(),
                    url: "/keys".to_string(),
                    ..Default::default()
                },
                SidebarEntry {
                    name: "Integrations".to_string(),
                    children: vec![
                        SidebarEntry {
                            name: "Cloudflare".to_string(),
                            url: "/cloudflare".to_string(),
                            ..Default::default()
                        },
                        SidebarEntry {
                            name: "Hetzner".to_string(),
                            url: "/hetzner".to_string(),
                            ..Default::default()
                        },
                    ],
                    dropdown: true,
                    ..Default::default()
                },
            ],
            current_project_id: Uuid::new_v4(),
        }
    }
}

impl Sidebar {
    // TODO: this should match on enum
    pub fn at(name: &str, user_id: UserId, db: &Database) -> Result<Self> {
        println!("sidebar::at: user id: {}", user_id);

        let mut s = Self::default();
        for entry in &mut s.entries {
            if &entry.name == name {
                entry.active = true;
            }
        }

        s.projects = db
            .get_collection::<Project>()?
            .into_iter()
            .filter(|p| p.owner == user_id)
            .collect();
        s.current_project_id = db.get::<UserData>(user_id)?.current_project;

        Ok(s)
    }
}