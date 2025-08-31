use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

mod coordinator;
mod node;
mod sharding;

pub use coordinator::*;
pub use node::*;
pub use sharding::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    JoinRequest { node_id: String, addr: SocketAddr },
    JoinResponse { success: bool, shard_map: HashMap<u64, String> },
    Replicate { key: String, value: Vec<u8> },
    Forward { key: String, value: Vec<u8> },
    Query { key: String },
    QueryResponse { value: Option<Vec<u8>> },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub addr: SocketAddr,
    pub shards: Vec<u64>,
}

#[derive(Debug)]
pub struct Cluster {
    pub nodes: Arc<RwLock<HashMap<String, Node>>>,
    pub shard_map: Arc<RwLock<HashMap<u64, String>>>,
    pub coordinator: Coordinator,
}

impl Cluster {
    pub fn new(coordinator_addr: SocketAddr) -> Self {
        Cluster {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            shard_map: Arc::new(RwLock::new(HashMap::new())),
            coordinator: Coordinator::new(coordinator_addr),
        }
    }
    
    pub async fn join(&self, node_addr: SocketAddr) -> Result<(), String> {
        let node_id = Uuid::new_v4().to_string();
        let node = Node {
            id: node_id.clone(),
            addr: node_addr,
            shards: Vec::new(),
        };
        
        self.nodes.write().await.insert(node_id, node);
        self.coordinator.add_node(node_id, node_addr).await?;
        
        Ok(())
    }
    
    pub async fn leave(&self, node_id: &str) -> Result<(), String> {
        self.coordinator.remove_node(node_id).await?;
        self.nodes.write().await.remove(node_id);
        Ok(())
    }
    
    pub async fn get_node_for_key(&self, key: &str) -> Option<Node> {
        let shard = self.shard_key(key);
        let shard_map = self.shard_map.read().await;
        shard_map.get(&shard)
            .and_then(|node_id| self.nodes.read().blocking_get().get(node_id).cloned())
    }
    
    fn shard_key(&self, key: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() % 1024
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    
    #[tokio::test]
    async fn test_cluster_join_leave() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let cluster = Cluster::new(addr);
        
        let node_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081);
        cluster.join(node_addr).await.unwrap();
        
        let nodes = cluster.nodes.read().await;
        assert_eq!(nodes.len(), 1);
        
        let node_id = nodes.keys().next().unwrap().clone();
        drop(nodes);
        
        cluster.leave(&node_id).await.unwrap();
        let nodes = cluster.nodes.read().await;
        assert_eq!(nodes.len(), 0);
    }
}
