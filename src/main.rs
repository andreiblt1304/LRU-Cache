use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Node {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}

struct LRUCache {
    cap: i32,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
}

impl LRUCache {
    const HEAD: usize = 0;
    const TAIL: usize = 1;
    fn new(cap: i32) -> Self {
        let mut nodes = Vec::with_capacity(2);
        let sentinel = Node {
            key: 0,
            val: 0,
            prev: Self::HEAD,
            next: Self::TAIL,
        };
        nodes.push(sentinel.clone());
        nodes.push(sentinel);
        nodes[Self::HEAD].next = Self::TAIL;
        nodes[Self::TAIL].prev = Self::HEAD;

        LRUCache {
            cap,
            map: HashMap::new(),
            nodes,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        // check if key exists
        // if yes => val + 1, move to front
        // if no => -1

        if let Some(&id) = self.map.get(&key) {
            self.nodes[id].val += 1;
            self.push_to_front(id);
            self.nodes[id].val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, val: i32) {
        // check if the key already exists => update val + move to front
        // check if len == cap => remove LRU item (pop)
        // else insert (alloc + move to front)
        if let Some(&id) = self.map.get(&key) {
            self.nodes[id].val = val;
            self.move_to_front(id);
            return;
        }
        if self.map.len() == self.cap as usize
            && let Some(id) = self.pop()
        {
            let removed_key = self.nodes[id].key;
            self.map.remove(&removed_key);
        }

        let new_id = self.alloc(key, val);
        self.push_to_front(new_id);
        self.map.insert(key, new_id);
    }

    fn alloc(&mut self, key: i32, val: i32) -> usize {
        let new_id = self.nodes.len();

        self.nodes.push(Node {
            key,
            val,
            next: Self::HEAD,
            prev: Self::TAIL,
        });

        new_id
    }

    fn pop(&mut self) -> Option<usize> {
        if self.nodes[Self::HEAD].next == Self::TAIL {
            return None;
        }
        let id = self.nodes[Self::TAIL].prev;
        self.detach(id);

        Some(id)
    }

    fn detach(&mut self, id: usize) {
        let p = self.nodes[id].prev;
        let n = self.nodes[id].next;

        self.nodes[p].next = n;
        self.nodes[n].prev = p;
    }

    fn move_to_front(&mut self, id: usize) {
        let first = self.nodes[Self::HEAD].next;

        self.nodes[Self::HEAD].next = id;
        self.nodes[id].prev = Self::HEAD;
        self.nodes[id].next = first;
        self.nodes[first].prev = id;
    }

    fn push_to_front(&mut self, id: usize) {
        self.detach(id);
        self.move_to_front(id);
    }
}

fn main() {
    let mut lru = LRUCache::new(2);

    assert!(lru.nodes.len() == 2);

    let (key1, val1) = (1, 1);
    let (key2, val2) = (2, 2);
    let (key3, val3) = (3, 3);

    lru.put(key1, val1);
    lru.put(key2, val2);

    let val2 = lru.get(2);
    assert!(val2 == 3);

    lru.put(key3, val3);

    let valminus = lru.get(1);
    assert!(valminus == -1);
}
