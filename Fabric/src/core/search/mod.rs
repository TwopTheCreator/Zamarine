use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::time::Instant;

mod algorithms;
mod metrics;

use algorithms::*;
use metrics::SearchMetrics;

pub struct SearchIndex {
    data: HashMap<String, Vec<u8>>,
    metadata: HashMap<String, HashMap<String, String>>,
    vector_index: Option<VectorIndex>,
    metrics: RwLock<SearchMetrics>,
}

#[derive(Default)]
struct VectorIndex {
    vectors: HashMap<String, Vec<f32>>,
    dimensions: usize,
}

impl SearchIndex {
    pub fn new() -> Self {
        SearchIndex {
            data: HashMap::new(),
            metadata: HashMap::new(),
            vector_index: None,
            metrics: RwLock::new(SearchMetrics::new()),
        }
    }

    pub fn index_data(&mut self, key: &str, data: &[u8], metadata: Option<HashMap<String, String>>) -> bool {
        self.data.insert(key.to_string(), data.to_vec());
        if let Some(meta) = metadata {
            self.metadata.insert(key.to_string(), meta);
        }
        true
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let start = Instant::now();
        let mut results = Vec::new();
        
        for (key, _) in &self.data {
            if let Some(score) = fuzzy_match(key, query) {
                results.push(SearchResult {
                    key: key.clone(),
                    score,
                    metadata: self.metadata.get(key).cloned(),
                });
            }
        }
        
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(limit);
        
        let duration = start.elapsed();
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.record_search(duration);
        }
        
        results
    }
    
    pub fn vector_search(&self, query: &[f32], k: usize) -> Option<Vec<VectorSearchResult>> {
        self.vector_index.as_ref().map(|index| {
            let mut results = Vec::new();
            
            for (key, vector) in &index.vectors {
                let score = cosine_similarity(query, vector);
                results.push(VectorSearchResult {
                    key: key.clone(),
                    score,
                });
            }
            
            results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            results.truncate(k);
            results
        })
    }
}

pub struct SearchResult {
    pub key: String,
    pub score: f32,
    pub metadata: Option<HashMap<String, String>>,
}

pub struct VectorSearchResult {
    pub key: String,
    pub score: f32,
}

static mut GLOBAL_INDEX: Option<Arc<RwLock<SearchIndex>>> = None;

#[no_mangle]
pub extern "C" fn fabric_init() -> bool {
    unsafe {
        if GLOBAL_INDEX.is_none() {
            GLOBAL_INDEX = Some(Arc::new(RwLock::new(SearchIndex::new())));
            true
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn fabric_index_data(
    key: *const c_char,
    data: *const u8,
    length: usize,
) -> bool {
    unsafe {
        if GLOBAL_INDEX.is_none() {
            return false;
        }
        
        let key_cstr = CStr::from_ptr(key);
        let key_str = match key_cstr.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };
        
        let data_slice = std::slice::from_raw_parts(data, length);
        
        if let Ok(mut index) = GLOBAL_INDEX.as_ref().unwrap().write() {
            index.index_data(&key_str, data_slice, None);
            true
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn fabric_search(
    query: *const c_char,
    result: *mut *mut c_char,
) -> bool {
    unsafe {
        if GLOBAL_INDEX.is_none() {
            return false;
        }
        
        let query_cstr = CStr::from_ptr(query);
        let query_str = match query_cstr.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        
        if let Ok(index) = GLOBAL_INDEX.as_ref().unwrap().read() {
            let results = index.search(query_str, 10);
            if !results.is_empty() {
                if let Ok(cstring) = CString::new(results[0].key.clone()) {
                    *result = cstring.into_raw();
                    return true;
                }
            }
        }
        
        false
    }
}

#[no_mangle]
pub extern "C" fn fabric_free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            CString::from_raw(s);
        }
    }
}
