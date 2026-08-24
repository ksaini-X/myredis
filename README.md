# Rustis

A Redis-inspired in-memory data store built from scratch in Rust.

This project is a deep dive into how real backend infrastructure is designed and implemented. Rather than simply building a key-value store, the goal is to incrementally develop a production-inspired system while exploring Rust's ownership model, async runtime, networking, concurrency, protocol design, persistence, and efficient data structures.

## Planned Features

- [x] In-memory key-value storage
- [ ] Binary-safe values
- [ ] Command abstraction
- [ ] Custom wire protocol
- [ ] TCP server
- [ ] Async networking with Tokio
- [ ] Concurrent client connections
- [ ] TTL and key expiration
- [ ] Pub/Sub
- [ ] Streams and consumer groups
- [ ] Persistence
- [ ] Rate limiting primitives
- [ ] Background job queues
- [ ] Benchmarking against Redis

## Why?

The goal of this project is not to recreate Redis feature-for-feature.

The goal is to understand the engineering concepts behind systems like Redis by building them from the ground up:

- How does a server receive and parse commands?
- How is data represented internally?
- How does ownership affect server architecture?
- How do multiple clients interact concurrently?
- How do async runtimes work?
- How can data survive process restarts?
- How do Pub/Sub and durable streams differ?
- How can the same data store be used for caching, rate limiting, queues, and event-driven systems?

Each feature is built incrementally, tested, and used in a small real-world example.
