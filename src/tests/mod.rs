
#[cfg(test)]
mod tests {
    use { crate::cache::LRUCache, crate::cache_trait::CacheTrait };

use std::fs::File;
use std::io::Write;

    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(3); // Taille de 3
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));
        // Premier élément moins récemment utilisé et dernier le plus récent
        // Cache == [B, C, D]

        let my_value = cache.get("A");
        assert_eq!(my_value, None);
        let my_value = cache.get("D");
        assert_eq!(my_value, Some(&String::from("value_d")));
        // Cache == [B, C, D]

        let my_value = cache.get("B");
        assert_eq!(my_value, Some(&String::from("value_b")));
        // Cache == [C, D, B]

        let my_value = cache.get("C");
        assert_eq!(my_value, Some(&String::from("value_c")));
        // Cache == [D, B, C]

        let my_value = cache.get("X");
        assert_eq!(my_value, None);
        // Cache == [D, B, C]

        cache.put("A", String::from("value_a"));
        // Cache == [B, C, A]

        cache.put("X", String::from("value_x"));
        // Cache == [C, A, X]

        let my_value = cache.get("B");
        assert_eq!(my_value, None);
        // Cache == [C, A, X]

        let my_value = cache.get("D");
        // Cache == [C, A, X]
        assert_eq!(my_value, None);
    }


    #[test]
    fn test_lru_cache_save_to_file() {
        let mut cache = LRUCache::new(3); // Taille de 3
        cache.filename = Some(String::from("lru_cache_test.txt"));
        let mut file = File::create("/tmp/lru_cache_test.txt").unwrap();
        writeln!(file, "Test LRU Cache Save to File").unwrap();
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));
        // Premier élément moins récemment utilisé et dernier le plus récent
        // Cache == [B, C, D]

        let my_value = cache.get("A");
        assert_eq!(my_value, None);
        let my_value = cache.get("D");
        assert_eq!(my_value, Some(&String::from("value_d")));
        // Cache == [B, C, D]

        let my_value = cache.get("B");
        assert_eq!(my_value, Some(&String::from("value_b")));
        // Cache == [C, D, B]

        let my_value = cache.get("C");
        assert_eq!(my_value, Some(&String::from("value_c")));
        // Cache == [D, B, C]

        let my_value = cache.get("X");
        assert_eq!(my_value, None);
        // Cache == [D, B, C]

        cache.put("A", String::from("value_a"));
        // Cache == [B, C, A]

        cache.put("X", String::from("value_x"));
        // Cache == [C, A, X]

        let my_value = cache.get("B");
        assert_eq!(my_value, None);
        // Cache == [C, A, X]

        let my_value = cache.get("D");
        // Cache == [C, A, X]
        assert_eq!(my_value, None);

        cache.save_to_file().unwrap();
    }

}