use crate::distributed::Message;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

const SHARDS_PER_NODE: usize = 4;

#[derive(Debug)]
pub struct Coordinator {
    pub addr: SocketAddr,
    nodes: Arc<RwLock<HashMap<String, SocketAddr>>>,
    node_shards: Arc<RwLock<HashMap<String, HashSet<u64>>>>,
    shard_map: Arc<RwLock<HashMap<u64, String>>>,
}

impl Coordinator {
    pub fn new(addr: SocketAddr) -> Self {
        Coordinator {
            addr,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            node_shards: Arc::new(RwLock::new(HashMap::new())),
            shard_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, node_id: String, addr: SocketAddr) -> Result<(), String> {
        let mut nodes = self.nodes.write().await;
        let mut node_shards = self.node_shards.write().await;
        let mut shard_map = self.shard_map.write().await;
        
        if nodes.contains_key(&node_id) {
            return Err("Node already exists".to_string());
        }
        
        nodes.insert(node_id.clone(), addr);
        node_shards.insert(node_id.clone(), HashSet::new());
        
        self.rebalance_shards().await;
        
        Ok(())
    }
    
    pub async fn remove_node(&self, node_id: &str) -> Result<(), String> {
        let mut nodes = self.nodes.write().await;
        let mut node_shards = self.node_shards.write().await;
        
        if !nodes.contains_key(node_id) {
            return Err("Node not found".to_string());
        }
        
        nodes.remove(node_id);
        node_shards.remove(node_id);
        
        self.rebalance_shards().await;
        
        Ok(())
    }
    
    async fn rebalance_shards(&self) {
        let nodes = self.nodes.read().await;
        let mut node_shards = self.node_shards.write().await;
        let mut shard_map = self.shard_map.write().await;
        
        if nodes.is_empty() {
            return;
        }
        
        let total_shards = 1024;
        let shards_per_node = total_shards / nodes.len().max(1);
        
        let mut shard = 0;
        let mut node_index = 0;
        let node_ids: Vec<String> = nodes.keys().cloned().collect();
        
        for _ in 0..total_shards {
            let node_id = &node_ids[node_index % node_ids.len()];
            
            shard_map.insert(shard, node_id.clone());
            node_shards
                .get_mut(node_id)
                .unwrap()
                .insert(shard);
            
            shard += 1;
            node_index += 1;
        }
    }
    
    pub async fn get_shard_owners(&self) -> HashMap<u64, String> {
        self.shard_map.read().await.clone()
    }
    
    pub async fn get_node_shards(&self, node_id: &str) -> Option<HashSet<u64>> {
        self.node_shards.read().await.get(node_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    
    #[tokio::test]
    async fn test_add_remove_node() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let coordinator = Coordinator::new(addr);
        
        let node_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081);
        let node_id = Uuid::new_v4().to_string();
        
        coordinator.add_node(node_id.clone(), node_addr).await.unwrap();
        
        let nodes = coordinator.nodes.read().await;
        assert_eq!(nodes.len(), 1);
        drop(nodes);
        
        coordinator.remove_node(&node_id).await.unwrap();
        let nodes = coordinator.nodes.read().await;
        assert_eq!(nodes.len(), 0);
    }
    
    #[tokio::test]
    async fn test_shard_rebalancing() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let coordinator = Coordinator::new(addr);
        
        for i in 0..3 {
            let node_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081 + i as u16);
            coordinator.add_node(Uuid::new_v4().to_string(), node_addr).await.unwrap();
        }
        
        let shard_map = coordinator.shard_map.read().await;
        assert_eq!(shard_map.len(), 1024);
        
        let node_shards = coordinator.node_shards.read().await;
        for shards in node_shards.values() {
            assert!(!shards.is_empty());
        }
    }
}
