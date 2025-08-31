use crate::distributed::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage, WebSocketStream};
use uuid::Uuid;

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub addr: SocketAddr,
    pub shards: Vec<u64>,
    connections: Arc<RwLock<HashMap<String, WebSocketStream<TcpStream>>>>,
}

impl Node {
    pub fn new(addr: SocketAddr) -> Self {
        Node {
            id: Uuid::new_v4().to_string(),
            addr,
            shards: Vec::new(),
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn connect(&self, addr: SocketAddr) -> Result<(), String> {
        let url = format!("ws://{}/ws", addr);
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;

        let mut connections = self.connections.write().await;
        connections.insert(addr.to_string(), ws_stream);

        Ok(())
    }

    pub async fn send_message(&self, addr: &SocketAddr, message: Message) -> Result<(), String> {
        let connections = self.connections.read().await;
        if let Some(stream) = connections.get(&addr.to_string()) {
            let msg = serde_json::to_string(&message)
                .map_err(|e| format!("Failed to serialize message: {}", e))?;
            
            stream
                .write_message(WsMessage::Text(msg))
                .await
                .map_err(|e| format!("Failed to send message: {}", e))?;
            
            Ok(())
        } else {
            Err("No connection to node".to_string())
        }
    }

    pub async fn join_cluster(&self, coordinator_addr: SocketAddr) -> Result<(), String> {
        let url = format!("ws://{}/ws", coordinator_addr);
        let (mut ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| format!("Failed to connect to coordinator: {}", e))?;

        let join_msg = Message::JoinRequest {
            node_id: self.id.clone(),
            addr: self.addr,
        };

        let msg = serde_json::to_string(&join_msg)
            .map_err(|e| format!("Failed to serialize join message: {}", e))?;

        ws_stream
            .write_message(WsMessage::Text(msg))
            .await
            .map_err(|e| format!("Failed to send join message: {}", e))?;

        if let Some(response) = ws_stream.next().await {
            let msg = response.map_err(|e| format!("Failed to read response: {}", e))?;
            
            if let WsMessage::Text(text) = msg {
                let response: Message = serde_json::from_str(&text)
                    .map_err(|e| format!("Failed to deserialize response: {}", e))?;
                
                if let Message::JoinResponse { success, shard_map } = response {
                    if success {
                        self.process_shard_map(shard_map).await;
                        return Ok(());
                    }
                }
            }
        }

        Err("Failed to join cluster".to_string())
    }

    async fn process_shard_map(&mut self, shard_map: HashMap<u64, String>) {
        self.shards = shard_map
            .into_iter()
            .filter(|(_, node_id)| node_id == &self.id)
            .map(|(shard, _)| shard)
            .collect();
    }

    pub async fn store_data(&self, key: &str, value: Vec<u8>) -> Result<(), String> {
        let shard = self.get_shard_for_key(key);
        
        if self.shards.contains(&shard) {
            self.store_local(key, value).await
        } else {
            self.forward_data(shard, key, value).await
        }
    }

    async fn store_local(&self, key: &str, value: Vec<u8>) -> Result<(), String> {
        Ok(())
    }

    async fn forward_data(&self, shard: u64, key: &str, value: Vec<u8>) -> Result<(), String> {
        Ok(())
    }

    fn get_shard_for_key(&self, key: &str) -> u64 {
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
    async fn test_node_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let node = Node::new(addr);
        
        assert_eq!(node.addr, addr);
        assert!(!node.id.is_empty());
    }
    
    #[tokio::test]
    async fn test_shard_assignment() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut node = Node::new(addr);
        
        let mut shard_map = HashMap::new();
        shard_map.insert(1, node.id.clone());
        shard_map.insert(2, node.id.clone());
        
        node.process_shard_map(shard_map).await;
        
        assert_eq!(node.shards.len(), 2);
        assert!(node.shards.contains(&1));
        assert!(node.shards.contains(&2));
    }
}
