📖 **[View on hightechmind.io →](https://hightechmind.io/rust/769-streaming-parser-pattern)**

---

# 769-streaming-parser-pattern — Streaming Parser Pattern

## Problem Statement

Network protocols deliver data in chunks, not as complete messages. A TCP stream may deliver the header in one packet and the body in several others. Streaming parsers process data incrementally, maintaining state between `feed()` calls and yielding complete messages only when enough data has arrived. This pattern is fundamental to every network server: HTTP, WebSocket, gRPC, and custom binary protocols all use streaming parsers. Without it, you must buffer entire messages in memory before parsing.

## Learning Outcomes

- Implement a state machine parser with `ParseState` enum: `Ready`, `InHeader`, `InBody`, `Complete`, `Error`
- Write a `feed(data: &[u8]) -> usize` method that returns bytes consumed
- Handle partial messages: pause and resume when the buffer is exhausted mid-message
- Detect framing errors and transition to an `Error` state
- Test with fragmented input delivered in chunks of 1, 2, or 3 bytes at a time

## Rust Application

`StreamingParser` maintains `state: ParseState`, accumulated `header`, `body`, and a `body_length`. `feed` processes bytes one at a time (for clarity; production would use SIMD scanning). In `Ready` state it reads header bytes until `:`. In `InBody` state it counts bytes and transitions to `Complete` when `remaining == 0`. The `complete_message()` method returns the parsed message only when in `Complete` state. Tests feed the same input in 1-byte chunks to verify streaming correctness.

## OCaml Approach

OCaml's `Angstrom` library is designed exactly for this use case. It provides a streaming interface with `Angstrom.Buffered.parse` that feeds chunks to the parser incrementally. State is maintained implicitly in the parser combinator's continuation. Lwt-based servers use `Angstrom_lwt_unix.parse_reader` to drive streaming parsers from network sockets directly.

## Key Differences

1. **State representation**: Rust uses an explicit `ParseState` enum; Angstrom uses implicit continuation-based state via its parser type.
2. **Buffer management**: Rust's `StreamingParser` owns its buffers; Angstrom manages a circular buffer internally, minimizing copies.
3. **Backpressure**: Rust's `feed() -> usize` reports consumed bytes for flow control; Angstrom integrates with Lwt/Eio for automatic backpressure.
4. **Combinators**: Angstrom's combinator-based parsers (`let* n = take_int 4 in ...`) are more composable than Rust's explicit state machine, but the Rust version is more transparent.

## Exercises

1. Extend the parser to handle multiple messages in a single `feed()` call, accumulating completed messages in a `Vec<(String, Vec<u8>)>`.
2. Add a timeout mechanism: if `body_length > 0` but no bytes arrive for N calls, transition to `Error("timeout")` and reset the parser.
3. Implement a `reset()` method that returns the parser to `Ready` state and clears all accumulated buffers, enabling reuse for the next message.
