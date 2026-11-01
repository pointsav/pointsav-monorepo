// SPDX-License-Identifier: Apache-2.0

use crate::business::Business;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub enum BusinessStoreError {
    NotFound(String),
}

impl std::fmt::Display for BusinessStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusinessStoreError::NotFound(query) => write!(f, "Business not found: {}", query),
        }
    }
}

impl std::error::Error for BusinessStoreError {}

pub struct BusinessStore {
    by_id: RwLock<HashMap<Uuid, Business>>,
}

impl Default for BusinessStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessStore {
    pub fn new() -> Self {
        Self {
            by_id: RwLock::new(HashMap::new()),
        }
    }

    pub fn append(&self, business: Business) -> Result<(), BusinessStoreError> {
        let mut by_id = self.by_id.write().unwrap();
        by_id.insert(business.id, business);
        Ok(())
    }

    pub fn lookup_by_id(&self, id: Uuid) -> Result<Business, BusinessStoreError> {
        let by_id = self.by_id.read().unwrap();
        by_id
            .get(&id)
            .cloned()
            .ok_or_else(|| BusinessStoreError::NotFound(id.to_string()))
    }

    pub fn lookup_by_name(&self, name: &str) -> Result<Business, BusinessStoreError> {
        let by_id = self.by_id.read().unwrap();
        by_id
            .values()
            .find(|b| b.name == name)
            .cloned()
            .ok_or_else(|| BusinessStoreError::NotFound(name.to_string()))
    }

    /// Every business occupying a given `service-places::Place` --
    /// the real POI relationship this crate exists to capture (a
    /// strip mall's several tenants, for instance).
    pub fn list_by_place(&self, place_id: Uuid) -> Vec<Business> {
        let by_id = self.by_id.read().unwrap();
        let mut businesses: Vec<Business> = by_id
            .values()
            .filter(|b| b.place_id == Some(place_id))
            .cloned()
            .collect();
        businesses.sort_by_key(|b| b.created_at);
        businesses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_and_lookup_by_id() {
        let store = BusinessStore::new();
        let business = Business::new("Acme Cafe", "restaurant", None);
        let id = business.id;
        store.append(business).unwrap();
        assert_eq!(store.lookup_by_id(id).unwrap().name, "Acme Cafe");
    }

    #[test]
    fn lookup_by_name() {
        let store = BusinessStore::new();
        let business = Business::new("Acme Cafe", "restaurant", None);
        store.append(business.clone()).unwrap();
        assert_eq!(store.lookup_by_name("Acme Cafe").unwrap().id, business.id);
    }

    #[test]
    fn list_by_place_returns_only_tenants_of_that_place() {
        let store = BusinessStore::new();
        let mall = Uuid::new_v4();
        let other_place = Uuid::new_v4();
        store.append(Business::new("Cafe", "restaurant", Some(mall))).unwrap();
        store.append(Business::new("Bookstore", "retail", Some(mall))).unwrap();
        store.append(Business::new("Elsewhere", "retail", Some(other_place))).unwrap();

        let tenants = store.list_by_place(mall);
        assert_eq!(tenants.len(), 2);
    }

    #[test]
    fn lookup_nonexistent_returns_error() {
        let store = BusinessStore::new();
        assert!(store.lookup_by_id(Uuid::new_v4()).is_err());
    }
}
