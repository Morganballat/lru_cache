// doc: https://saurabhnativeblog.medium.com/paytm-frontend-interview-question-implement-a-lru-cache-in-javascript-8e1754ba8b87
// Utiliser une combinaison de hashmap et d'une liste doublement chainée
// => HashMap pour les opérations de recherche en O(1)
// => Liste doublement chainée pour maintenir l'ordre des éléments
// Le dernier élément de la liste doit-être le moins récemment utilisé

// J'ai besoin d'un cache avec une capacité fixe
// Je dois pouvoir ajouter un élément dans le cache
// Je dois pouvoir récupérer un élément du cache
// Si je dépasse la capacité du cache, je dois supprimer le moins récemment utilisé

// doc : https://doc.rust-lang.org/std/collections/struct.HashMap.html
// La hasmap est une collection de paires clé-valeur.
// => Les clefs sont uniques
// => un mécanisme de hshage est utilisé pour stocker les données efficacement. Accès rapide à la donnée avec sa clef

use std::collections::HashMap;

fn main(){

    let mut hash_map = HashMap::new();

    // On peut ajouter ou remplacer une paire clé-valeur avec la méthode insert
    hash_map.insert("key1", "value1");
    hash_map.insert("key2", "value2");

    // On peut accéder à une valeur du hashmap avec la méthode get
    // Vu que la clef peut ne pas exister dans la hasmap
    // Le retour est <Option<&V>> : None si la clé n'existe pas, Some(&v) si la clé existe
    if let Some(value) = hash_map.get("key1") {
        println!("Value for key1: {}", value);
    };

    // itérer sur les paire vlé-valeur de la hashmap
    // J'utilise &hash_map pour emprunter une référence de la hashmap plutôt que de la consommer.
    // Cela permet de lire les données sans les modifier ou les déplacer
    for (key, value) in &hash_map {
        println!("Key: {}, Value: {}", key, value);
    };

    //Supprimer avec la méthode remove
    hash_map.remove("key1");

    // Vérifier si une clé existe dans la hashmap
    if hash_map.contains_key("key1") {
        println!("Key1 exists");
    } else {
        println!("Key1 does not exist");
    };

}
