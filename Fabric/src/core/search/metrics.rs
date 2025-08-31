use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::collections::VecDeque;
use std::time::SystemTime;

const WINDOW_SIZE: usize = 100;

pub struct SearchMetrics {
    total_searches: AtomicU64,
    total_search_time: AtomicU64,
    recent_searches: parking_lot::Mutex<VecDeque<SearchStats>>,
}

#[derive(Clone, Debug)]
struct SearchStats {
    query: String,
    duration: Duration,
    timestamp: SystemTime,
    result_count: usize,
}

impl Default for SearchMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchMetrics {
    pub fn new() -> Self {
        SearchMetrics {
            total_searches: AtomicU64::new(0),
            total_search_time: AtomicU64::new(0),
            recent_searches: parking_lot::Mutex::new(VecDeque::with_capacity(WINDOW_SIZE)),
        }
    }

    pub fn record_search(&self, duration: Duration) -> u64 {
        let total = self.total_searches.fetch_add(1, Ordering::Relaxed) + 1;
        self.total_search_time
            .fetch_add(duration.as_micros() as u64, Ordering::Relaxed);
        total
    }

    pub fn record_search_with_details(&self, query: &str, duration: Duration, result_count: usize) {
        self.record_search(duration);
        
        let mut recent = self.recent_searches.lock();
        if recent.len() >= WINDOW_SIZE {
            recent.pop_front();
        }
        
        recent.push_back(SearchStats {
            query: query.to_string(),
            duration,
            timestamp: SystemTime::now(),
            result_count,
        });
    }

    pub fn get_average_search_time(&self) -> Option<Duration> {
        let total = self.total_searches.load(Ordering::Relaxed);
        if total == 0 {
            return None;
        }
        
        let total_time = self.total_search_time.load(Ordering::Relaxed);
        Some(Duration::from_micros(total_time / total))
    }

    pub fn get_recent_searches(&self, limit: usize) -> Vec<SearchStats> {
        let recent = self.recent_searches.lock();
        recent.iter().rev().take(limit).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_metrics_recording() {
        let metrics = Arc::new(SearchMetrics::new());
        
        let metrics_clone = metrics.clone();
        let handle = thread::spawn(move || {
            metrics_clone.record_search(Duration::from_millis(100));
            metrics_clone.record_search_with_details("test", Duration::from_millis(200), 5);
        });
        
        handle.join().unwrap();
        
        assert_eq!(metrics.total_searches.load(Ordering::Relaxed), 2);
        
        if let Some(avg) = metrics.get_average_search_time() {
            assert!(avg >= Duration::from_millis(100) && avg <= Duration::from_millis(200));
        } else {
            panic!("Expected average search time");
        }
        
        let recent = metrics.get_recent_searches(1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].query, "test");
        assert_eq!(recent[0].result_count, 5);
    }
}
