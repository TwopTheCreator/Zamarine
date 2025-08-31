use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::RwLock;

const VIRTUAL_NODES: usize = 160;

#[derive(Debug, Clone)]
pub struct ConsistentHash {
    ring: Arc<RwLock<BTreeMap<u64, String>>>,
    node_hashes: Arc<RwLock<HashMap<String, HashSet<u64>>>>,
}

impl Default for ConsistentHash {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsistentHash {
    pub fn new() -> Self {
        ConsistentHash {
            ring: Arc::new(RwLock::new(BTreeMap::new())),
            node_hashes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, node_id: &str) {
        let mut ring = self.ring.write().await;
        let mut node_hashes = self.node_hashes.write().await;
        
        let hashes = (0..VIRTUAL_NODES).map(|i| {
            let key = format!("{}#{}", node_id, i);
            let hash = Self::hash(&key);
            (hash, node_id.to_string())
        }).collect::<Vec<_>>();
        
        for (hash, node_id_str) in hashes {
            ring.insert(hash, node_id_str);
        }
        
        node_hashes.insert(
            node_id.to_string(),
            hashes.into_iter().map(|(h, _)| h).collect(),
        );
    }
    
    pub async fn remove_node(&self, node_id: &str) -> bool {
        let mut ring = self.ring.write().await;
        let mut node_hashes = self.node_hashes.write().await;
        
        if let Some(hashes) = node_hashes.remove(node_id) {
            for hash in hashes {
                ring.remove(&hash);
            }
            true
        } else {
            false
        }
    }
    
    pub async fn get_node(&self, key: &str) -> Option<String> {
        let ring = self.ring.read().await;
        if ring.is_empty() {
            return None;
        }
        
        let hash = Self::hash(key);
        let range = ring.range(hash..);
        
        if let Some((_, node_id)) = range.next() {
            return Some(node_id.clone());
        }
        
        ring.iter().next().map(|(_, node_id)| node_id.clone())
    }
    
    pub async fn get_replicas(&self, key: &str, n: usize) -> Vec<String> {
        let mut replicas = Vec::with_capacity(n);
        let ring = self.ring.read().await;
        
        if ring.is_empty() {
            return replicas;
        }
        
        let hash = Self::hash(key);
        let mut range = ring.range(hash..);
        
        while replicas.len() < n && !ring.is_empty() {
            if let Some((_, node_id)) = range.next() {
                if !replicas.contains(node_id) {
                    replicas.push(node_id.clone());
                }
            } else {
                range = ring.range(..);
            }
            
            if replicas.len() >= ring.len() {
                break;
            }
        }
        
        replicas
    }
    
    fn hash(key: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_consistent_hashing() {
        let ch = ConsistentHash::new();
        
        ch.add_node("node1").await;
        ch.add_node("node2").await;
        ch.add_node("node3").await;
        
        let node = ch.get_node("test_key").await.unwrap();
        assert!(["node1", "node2", "node3"].contains(&node.as_str()));
        
        let replicas = ch.get_replicas("test_key", 2).await;
        assert_eq!(replicas.len(), 2);
        assert_ne!(replicas[0], replicas[1]);
        
        ch.remove_node("node1").await;
        let node_after_remove = ch.get_node("test_key").await.unwrap();
        assert_ne!(node_after_remove, "node1");
    }
}
