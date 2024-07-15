use std::ops::Deref;
use std::sync::Arc;

use dashmap::{DashMap, DashSet};

use crate::resp::RespFrame;

#[derive(Debug, Clone)]
pub struct Database(Arc<Backend>);

#[derive(Debug)]
pub struct Backend {
    pub(crate) map: DashMap<String, RespFrame>,
    pub(crate) hmap: DashMap<String, DashMap<String, RespFrame>>,
    pub(crate) hset: DashMap<String, DashSet<String>>,
}

impl Deref for Database {
    type Target = Backend;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Database {
    fn default() -> Self {
        Self(Arc::new(Backend::default()))
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
            hset: DashMap::new(),
        }
    }
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        self.hmap
            .get(key)
            .and_then(|v| v.get(field).map(|v| v.value().clone()))
    }

    pub fn hset(&self, key: String, field: String, value: RespFrame) {
        let hmap = self.hmap.entry(key).or_default();
        hmap.insert(field, value);
    }

    pub fn hgetall(&self, key: &str) -> Option<DashMap<String, RespFrame>> {
        self.hmap.get(key).map(|v| v.clone())
    }

    pub fn sadd(&self, key: String, val: String) {
        let hdata = self.hset.entry(key).or_default();
        hdata.insert(val);
    }

    pub fn sall(&self, key: &str) -> Option<DashSet<String>> {
        self.hset.get(key).map(|v| v.clone())
    }
}
