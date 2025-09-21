// Write a function that takes a list of log lines (strings) and
// return the k most frequently occurring words, sorted by frequency (descending).
// Ignore case (Error and error should be treated the same).
// Words are sequences of alphanumeric characters (a-z, A-Z, 0-9).
// If multiple words have the same frequency, sort them alphabetically.
// The function signature should look like:
// fn top_k_words(logs: &[String], k: usize) -> Vec<(String, usize)>;

// Example Input:
// let logs = vec![
//  "Error: Disk full".to_string(),
//  "Warning: Memory low".to_string(),
//  "error: network down".to_string(),
//  "Error: Disk full".to_string(),
// ];
// let result = top_k_words(&logs, 2);

// Expected Output:
// [("error", 3), ("disk", 2)]
//
// Big-O time complexity: O(n log n)
// n = the total number of characters across all lines.
// u = the number of unique words across all lines.
// Overall: O(n + u log k) => the worst case (k ≈ u): O(n + u log u), u ≤ n =>  O(n log n).
fn top_k_words(lines: &[String], k: usize) -> Vec<(String, usize)> {
    if k == 0 || lines.is_empty() {
        return Vec::new();
    }

    let mut word = String::new();
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for line in lines {
        for &b in line.as_bytes() {
            if b'A' <= b && b <= b'Z' {
                word.push((b | 32) as char); // lowercase
            } else if (b'a' <= b && b <= b'z') || (b'0' <= b && b <= b'9') {
                word.push(b as char);
            } else {
                if !word.is_empty() {
                    let w = std::mem::take(&mut word);
                    *counts.entry(w).or_insert(0) += 1;
                }
            }
        }
        if !word.is_empty() {
            let w = std::mem::take(&mut word);
            *counts.entry(w).or_insert(0) += 1;
        }
    }

    #[derive(Clone)]
    struct HeapItem {
        count: usize,
        word: String,
    }

    impl Eq for HeapItem {}

    impl PartialEq for HeapItem {
        fn eq(&self, other: &Self) -> bool {
            self.count == other.count && self.word == other.word
        }
    }

    impl Ord for HeapItem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if self.count != other.count {
                other.count.cmp(&self.count)
            } else {
                self.word.cmp(&other.word)
            }
        }
    }

    impl PartialOrd for HeapItem {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    use std::collections::BinaryHeap;
    let mut heap: BinaryHeap<HeapItem> = BinaryHeap::new();

    for (word, count) in counts.into_iter() {
        heap.push(HeapItem { count, word });
        if heap.len() > k {
            heap.pop();
        }
    }

    let mut items: Vec<(String, usize)> = heap.into_iter().map(|hi| (hi.word, hi.count)).collect();

    // O(k log k)
    items.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    items
}

fn main() {
    let logs = vec![
        "Error: Disk full".to_string(),
        "Warning: Memory low".to_string(),
        "error: network down".to_string(),
        "Error: Disk full".to_string(),
    ];

    println!("{:?}", top_k_words(&logs, 2));
    // [("error", 3), ("disk", 2)]
}
