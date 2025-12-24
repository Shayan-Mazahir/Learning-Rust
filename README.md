# Rust Learning Journey: Building a Redis Clone

## Final Goal
Build a fully functional **Redis clone** from scratch in Rust - a networked, in-memory key-value database that multiple clients can connect to simultaneously.

## Wut Building?

A server application that:
- Listens for TCP connections from multiple clients
- Parses and executes Redis-style commands (GET, SET, DEL, etc.)
- Stores data in-memory using efficient Rust data structures
- Handles concurrent client connections safely
- Supports multiple data types (strings, lists, sets, hashes)
- Persists data to disk (optional but cool!)
- Implements the RESP (Redis Serialization Protocol)

## Learning Path

### Phase 1: Rust Fundamentals (In Progress)
- [x] Hello World & Variables
- [ ] Functions & Control Flow
- [ ] Ownership & Borrowing
- [ ] Structs & Enums
- [ ] Error Handling

**Mini Project:** Simple calculator

---

### Phase 2: CLI Applications
- [ ] Collections (Vec, HashMap, HashSet)
- [ ] File I/O
- [ ] String Handling

**Project:** Todo CLI app with file persistence

---

### Phase 3: Intermediate Concepts
- [ ] Traits & Generics
- [ ] Lifetimes
- [ ] Iterators & Closures
- [ ] Modules & Crates

**Project:** Local key-value store (baby Redis, no networking)

---

### Phase 4: Concurrency & Networking
- [ ] Threads & Channels
- [ ] TCP Networking
- [ ] Async/Await (Tokio)
- [ ] Protocol Parsing (RESP)

**Project:** Multi-client echo server

---

### Phase 5: The Redis Clone
- [ ] RESP Protocol Implementation
- [ ] Core Commands (GET, SET, DEL, EXISTS)
- [ ] Data Structures (Lists, Sets, Hashes)
- [ ] Persistence Layer
- [ ] Advanced Features (TTL, Pub/Sub)

---

## Why This Project?

This hits every major Rust concept:
- **Ownership/Borrowing**: Memory-safe data management
- **Concurrency**: Handling multiple clients safely
- **Networking**: TCP servers and protocol design
- **Collections**: Efficient in-memory storage
- **Error Handling**: Graceful failure modes
- **Performance**: Leveraging Rust's speed

## Tech Stack
- **Language**: Rust
- **Networking**: Tokio (async runtime)
- **Serialization**: Custom RESP parser
- **Testing**: Built-in Rust test framework

---

**Current Status:** Phase 1, Step 2 - Learning Functions & Control Flow

**Started:** December 24, 2025
**Target Completion:** TBD

---

Let's build something awesome! 