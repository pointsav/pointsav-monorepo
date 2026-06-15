use std::{collections::HashMap, path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub vault: PathBuf,
    pub nav: Arc<HashMap<String, Vec<String>>>,
    pub tenant: String,
}
