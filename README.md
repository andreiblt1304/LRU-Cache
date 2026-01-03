# LRU Cache (PoC)

This repo is a proof-of-concept implementation of a Least Recently Used (LRU)
cache in Rust, following the interface and behavior of Leetcode problem
`146. LRU Cache`.

## Overview

- Stores key/value pairs with a fixed capacity.
- On access or update, the key becomes most recently used.
- When capacity is exceeded, the least recently used entry is evicted.

## Implementation Details

- `HashMap<i32, usize>` maps keys to indices in a `Vec<Node>`.
- The node vector holds a doubly-linked list via indices.
- Two sentinel nodes live at indices `0` (HEAD) and `1` (TAIL).
- Moving a node to the front is O(1) by pointer rewiring.
- `get` and `put` are both O(1) average time.

## Current Behavior Notes

- `get` increments the stored value by 1 before returning it.
  This differs from the Leetcode spec, which returns the stored value
  unchanged. This is intentional in the PoC and is easy to remove if
  you want strict spec compliance.

## Usage

Build and run the demo in `src/main.rs`:

```bash
cargo run
```

The demo exercises:

- two inserts
- a get that moves the key to the front
- an insert that evicts the LRU entry

## Structure

- `src/main.rs` contains the cache implementation and a small demo.
