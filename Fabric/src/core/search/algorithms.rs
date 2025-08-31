use std::cmp::min;
use std::collections::HashMap;

const MAX_PREFIX: usize = 100;
const MATCH_BONUS: i32 = 2;
const CAMEL_BONUS: i32 = 2;
const LEADING_LETTER_PENALTY: i32 = -3;
const MAX_LEADING_LETTER_PENALTY: i32 = -9;
const UNMATCHED_LETTER_PENALTY: i32 = -1;

extern "C" {
    fn fast_memcmp(ptr1: *const u8, ptr2: *const u8, len: usize) -> i32;
}

pub fn fuzzy_match(text: &str, pattern: &str) -> Option<f32> {
    if pattern.is_empty() {
        return Some(1.0);
    }
    
    if text.is_empty() || pattern.len() > text.len() {
        return None;
    }
    
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    
    let mut score = 0i32;
    let mut pattern_idx = 0;
    let mut in_gap = false;
    let mut last_char = 0 as char;
    let mut start = 0;
    
    for (i, &b) in text_bytes.iter().enumerate() {
        let current_char = b as char;
        
        if pattern_idx < pattern_bytes.len() && b.eq_ignore_ascii_case(&pattern_bytes[pattern_idx]) {
            let mut char_score = 0;
            
            if pattern_idx == 0 {
                start = i;
                char_score += (text_bytes.len() - i) as i32;
            }
            
            if i > 0 {
                let prev_char = text_bytes[i - 1] as char;
                
                if is_uppercase(current_char) && is_lowercase(prev_char) {
                    char_score += CAMEL_BONUS;
                }
                
                if is_alphanumeric(prev_char) && !is_alphanumeric(current_char) {
                    char_score += MATCH_BONUS;
                } else if !is_alphanumeric(prev_char) && is_alphanumeric(current_char) {
                    char_score += MATCH_BONUS;
                }
                
                if in_gap {
                    char_score += UNMATCHED_LETTER_PENALTY * (i - start - 1) as i32;
                }
            }
            
            score += char_score;
            pattern_idx += 1;
            in_gap = false;
        } else {
            in_gap = true;
        }
        
        last_char = current_char;
    }
    
    if pattern_idx != pattern_bytes.len() {
        return None;
    }
    
    let penalty = if start > 0 {
        let penalty = LEADING_LETTER_PENALTY * start as i32;
        penalty.max(MAX_LEADING_LETTER_PENALTY)
    } else {
        0
    };
    
    score += penalty;
    
    let max_score = (text_bytes.len() * MATCH_BONUS as usize) as i32;
    let normalized = (score as f32 / max_score as f32).max(0.0).min(1.0);
    
    Some(normalized)
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    
    let dot_product: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    dot_product / (norm_a * norm_b)
}

#[inline]
fn is_uppercase(c: char) -> bool {
    c.is_ascii_uppercase()
}

#[inline]
fn is_lowercase(c: char) -> bool {
    c.is_ascii_lowercase()
}

#[inline]
fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fuzzy_match() {
        assert!(fuzzy_match("hello", "hl").is_some());
        assert!(fuzzy_match("hello", "hx").is_none());
        assert!(fuzzy_match("HelloWorld", "HW").is_some());
    }
    
    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim > 0.0 && sim < 1.0);
    }
}
