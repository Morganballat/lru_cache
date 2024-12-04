# Documentation de l'application LRU Cache

## Introduction

Cette application implémente une cache LRU (Least Recently Used) en Rust. Une cache LRU est une structure de données qui stocke un nombre limité d'éléments et élimine les éléments les moins récemment utilisés lorsque la capacité maximale est atteinte.

## Modules

### `cache_trait.rs`

Ce fichier définit le trait `CacheTrait` que doit implémenter toute cache.

### `cache.rs`

Ce fichier implémente la structure `LRUCache` qui utilise un `HashMap` pour stocker les valeurs et un `Vec` pour maintenir l'ordre des clés.

#### Exemple d'utilisation

```rust
use crate::cache::LRUCache;
use crate::cache_trait::CacheTrait;

fn main() {
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
}


Exécution des tests

Pour exécuter les tests, utilisez la commande suivante :
cargo test
Cela exécutera tous les tests définis dans le module tests.
