# Effect System Simulation

Algebraic effects separate what from how:
- **Effects** describe operations
- **Handlers** provide implementations

Rust doesn't have native effect systems, but we can simulate with:
- Result types for errors
- Callbacks for effect handling
- State threading for mutable state
