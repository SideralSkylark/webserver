# Tiny Rust HTTP Server

A minimal HTTP server built from scratch in Rust.

The purpose of this project is to understand:

* TCP sockets
* streams and buffering
* HTTP request/response structure
* request parsing
* blocking IO
* concurrency basics
* filesystem streaming
* basic server architecture

This project is educational and intentionally avoids web frameworks such as:

* Axum
* Actix Web
* Rocket

---

# Functional Requirements

## Networking & Connections

* [ ] RF01: Listen for TCP connections on a configurable port
* [ ] RF02: Accept multiple client connections
* [ ] RF03: Gracefully close client connections

---

## HTTP Parsing

* [ ] RF04: Parse HTTP request line
* [ ] RF05: Parse request method (GET, POST)
* [ ] RF06: Parse request path
* [ ] RF07: Parse HTTP headers
* [ ] RF08: Parse request body
* [ ] RF09: Handle malformed HTTP requests safely

---

## Routing

* [ ] RF10: Route requests based on method + path
* [ ] RF11: Implement `GET /`
* [ ] RF12: Implement `GET /health`
* [ ] RF13: Implement `POST /echo`
* [ ] RF14: Return proper 404 responses for unknown routes

---

## Responses

* [ ] RF15: Generate valid HTTP responses
* [ ] RF16: Return correct status codes
* [ ] RF17: Return response headers
* [ ] RF18: Return plain text responses

---

## Static Files

* [ ] RF19: Serve static files from a configured directory
* [ ] RF20: Detect MIME types
* [ ] RF21: Stream large files without loading entirely into memory
* [ ] RF22: Prevent directory traversal attacks

---

## Concurrency

* [ ] RF23: Handle multiple simultaneous clients
* [ ] RF24: Implement thread-per-connection model
* [ ] RF25: Implement optional thread pool
* [ ] RF26: Avoid blocking unrelated clients

---

## Logging & Observability

* [ ] RF27: Log incoming requests
* [ ] RF28: Log response status codes
* [ ] RF29: Log connection errors
* [ ] RF30: Log server startup and shutdown

---

## Configuration

* [ ] RF31: Configure host and port
* [ ] RF32: Configure static file directory
* [ ] RF33: Configure worker thread count

---

## Testing

* [ ] RF34: Verify responses using `curl`
* [ ] RF35: Verify server behavior using `netcat`
* [ ] RF36: Add unit tests for HTTP parsing
* [ ] RF37: Add malformed request test cases

---

# Non-Functional Requirements

* NF01: **Correctness**: Generate standards-compliant HTTP/1.1 responses
* NF02: **Reliability**: Malformed requests must not crash the server
* NF03: **Performance**: Handle multiple concurrent clients reliably
* NF04: **Resource Usage**: Avoid excessive memory allocations
* NF05: **Security**: Prevent path traversal and unsafe file access
* NF06: **Maintainability**: Keep modules small and responsibilities clear
* NF07: **Observability**: Logs should clearly describe request lifecycle
* NF08: **Educational Value**: Core protocol handling should be implemented manually

---

# Development Roadmap

## Phase 1 — Raw TCP

Goal:

* accept TCP connections
* read bytes from clients
* print raw requests

### Milestones

* [ ] Accept one client
* [ ] Read request bytes
* [ ] Respond with hardcoded bytes

---

## Phase 2 — Minimal HTTP

Goal:

* generate valid HTTP responses

### Milestones

* [ ] Return `HTTP/1.1 200 OK`
* [ ] Add headers
* [ ] Add body support

---

## Phase 3 — Request Parsing

Goal:

* parse requests manually

### Milestones

* [ ] Parse request line
* [ ] Parse headers
* [ ] Parse body
* [ ] Handle malformed input

---

## Phase 4 — Routing

Goal:

* dispatch requests to handlers

### Milestones

* [ ] Method/path matching
* [ ] 404 handling
* [ ] Route parameters

---

## Phase 5 — Static Files

Goal:

* serve files safely

### Milestones

* [ ] HTML responses
* [ ] CSS/JS/image support
* [ ] MIME type detection

---

## Phase 6 — Concurrency

Goal:

* support multiple clients

### Milestones

* [ ] Thread-per-connection
* [ ] Thread pool
* [ ] Graceful shutdown

---

# Stretch Goals

These are optional and should only be attempted after the core server works reliably.

* [ ] Keep-alive connections
* [ ] JSON responses
* [ ] Request timeouts
* [ ] Basic middleware system
* [ ] Access log format
* [ ] Chunked transfer encoding
* [ ] Async runtime experiment
* [ ] Benchmarks using `wrk`
* [ ] Simple reverse proxy
* [ ] Basic caching layer

---

# Constraints

* avoid HTTP frameworks
* avoid HTTP parsing crates initially
* avoid async runtimes initially
* prefer the standard library first
* avoid large abstractions too early
* implement protocol handling manually

---

# Explicit Non-Goals

This project is NOT intended to become:

* a production web framework
* a production reverse proxy
* a fully compliant HTTP implementation
* a replacement for Nginx or Apache HTTP Server

