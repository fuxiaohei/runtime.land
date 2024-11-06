use land_utils::version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod user;
pub use user::User;

mod project;
pub use project::Project;

/// Page is the template page vars, for every page
#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub title: String,
    pub nav_active: HashMap<String, String>,
    pub breadcrumb: Vec<BreadCrumb>,
    pub user: Option<User>,
    pub version: String,
    pub is_in_admin: bool,
}

impl Page {
    /// new creates a new page var
    pub fn new(title: &str, breadcrumb: BreadCrumbKey, user: Option<User>) -> Self {
        Page {
            title: title.to_string(),
            nav_active: nav_active(&breadcrumb),
            breadcrumb: BreadCrumb::new(&breadcrumb),
            user,
            version: version::short(),
            is_in_admin: false,
        }
    }
}

/// BreadCrumb is breadcrumb value item
#[derive(Debug, Serialize, Deserialize)]
pub struct BreadCrumb {
    pub title: String,
    pub link: Option<String>,
}

/// BreadCrumb enum
#[derive(strum::Display, Clone, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum BreadCrumbKey {
    Dashboard,
    Projects,
    ProjectSingle,
    ProjectSettings,
    NotFound,
    None,
}

impl BreadCrumb {
    /// title creates a breadcrumb item with title only
    pub fn title(name: &str) -> Self {
        BreadCrumb {
            title: name.to_string(),
            link: None,
        }
    }
    /// new creates breadcrumb items
    pub fn new(key: &BreadCrumbKey) -> Vec<BreadCrumb> {
        match key {
            BreadCrumbKey::Dashboard => vec![Self::title("Dashboard")],
            BreadCrumbKey::Projects
            | BreadCrumbKey::ProjectSingle
            | BreadCrumbKey::ProjectSettings => vec![Self::title("Projects")],
            BreadCrumbKey::None | BreadCrumbKey::NotFound => vec![],
        }
    }
}

/// Vars is the template vars for whole page
#[derive(Serialize, Deserialize, Debug)]
pub struct Vars<T: Serialize> {
    pub page: Page,
    pub data: T,
}

/// Empty is an empty struct with serde support
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Empty {}

/// new_empty creates a new empty template vars
pub fn new_empty(title: &str, breadcrumb: BreadCrumbKey, user: Option<User>) -> Vars<Empty> {
    Vars {
        page: Page::new(title, breadcrumb, user),
        data: Empty::default(),
    }
}

/// new_vars creates a new template vars
pub fn new_vars(
    title: &str,
    breadcrumb: BreadCrumbKey,
    user: Option<User>,
    data: impl Serialize,
) -> Vars<impl Serialize> {
    Vars {
        page: Page::new(title, breadcrumb, user),
        data,
    }
}

/// nav_active sets active navbar items
pub fn nav_active(breadcrumb: &BreadCrumbKey) -> HashMap<String, String> {
    let mut nav_active = HashMap::new();
    // println!("breadcrumb: {:?}", breadcrumb.to_string());
    nav_active.insert(breadcrumb.to_string(), "active".to_string());
    nav_active
}
