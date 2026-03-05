# Send and Sync Traits

## Definitions
- `Send`: Can be moved to another thread
- `Sync`: Can be shared via `&T` between threads

## Common Types

| Type | Send | Sync | Notes |
|------|------|------|-------|
| `i32` | ✓ | ✓ | Primitive |
| `String` | ✓ | ✓ | Owned |
| `Arc<T>` | ✓* | ✓* | *if T is |
| `Mutex<T>` | ✓* | ✓* | *if T: Send |
| `Rc<T>` | ✗ | ✗ | Non-atomic refcount |
| `Cell<T>` | ✓* | ✗ | Interior mut |
| `RefCell<T>` | ✓* | ✗ | Interior mut |

## Rule
`&T: Send` if and only if `T: Sync`
