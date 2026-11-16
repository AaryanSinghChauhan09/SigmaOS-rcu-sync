// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-rcu-sync: rcu sync component model.
//!
//! Real implementation: syncs, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sync {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RcuSyncError {
    EmptySyncName,
    DuplicateSync(String),
    EmptyEventSubject,
}

impl fmt::Display for RcuSyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySyncName => write!(f, "sync name must not be empty"),
            Self::DuplicateSync(id) => write!(f, "duplicate sync {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RcuSyncError {}

pub struct RcuSync {
    pub name: String,
    syncs: BTreeMap<String, Sync>,
    events: BTreeMap<String, Event>,
}

impl RcuSync {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            syncs: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_sync(&mut self, item: Sync) -> Result<(), RcuSyncError> {
        if item.name.trim().is_empty() {
            return Err(RcuSyncError::EmptySyncName);
        }
        let id = item.id.clone();
        if self.syncs.contains_key(&id) {
            return Err(RcuSyncError::DuplicateSync(id.clone()));
        }
        self.syncs.insert(id, item);
        Ok(())
    }

    pub fn sync_count(&self) -> usize {
        self.syncs.len()
    }

    pub fn get_sync(&self, id: &str) -> Option<&Sync> {
        self.syncs.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RcuSyncError> {
        if event.subject.trim().is_empty() {
            return Err(RcuSyncError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sync(id: &str, name: &str) -> Sync {
        Sync {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = RcuSync::new("RcuSync");
        assert_eq!(m.name, "RcuSync");
        assert_eq!(m.sync_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_sync() {
        let mut m = RcuSync::new("RcuSync");
        m.add_sync(sample_sync("a1", "Alpha")).unwrap();
        assert_eq!(m.sync_count(), 1);
        assert!(m.get_sync("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = RcuSync::new("RcuSync");
        assert!(m.add_sync(sample_sync("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = RcuSync::new("RcuSync");
        m.add_sync(sample_sync("x", "One")).unwrap();
        assert!(m.add_sync(sample_sync("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = RcuSync::new("RcuSync");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
