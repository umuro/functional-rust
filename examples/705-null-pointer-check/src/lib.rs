#![allow(clippy::all)]
//! 705 — Null Pointer Handling: NonNull<T>
//!
//! `NonNull<T>` is a `*mut T` guaranteed non-null at the type level.
//! `Option<NonNull<T>>` compresses to pointer-size (null-pointer optimisation),
//! while still forcing explicit null-checking at construction via `NonNull::new`.

use std::mem::size_of;
use std::ptr::NonNull;

// ---------------------------------------------------------------------------
// Linked list node using NonNull for the "next" pointer
// ---------------------------------------------------------------------------

pub struct Node<T> {
    pub value: T,
    pub next: Option<NonNull<Node<T>>>,
}

/// Build a singly-linked list on the heap, returning the head as `NonNull`.
///
/// Each node is allocated with `Box::into_raw` so we own the memory and can
/// free it later.  `NonNull::new_unchecked` is safe here because `Box` never
/// returns a null pointer.
pub fn build_list<T>(values: &[T]) -> Option<NonNull<Node<T>>>
where
    T: Copy,
{
    let mut head: Option<NonNull<Node<T>>> = None;
    for &v in values.iter().rev() {
        let node = Box::new(Node {
            value: v,
            next: head,
        });
        // SAFETY: Box::into_raw is never null.
        head = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
    }
    head
}

/// Traverse the list and collect values.
///
/// # Safety invariant
/// Every `NonNull<Node<T>>` in this list was produced by `Box::into_raw` inside
/// `build_list`, so the pointer is valid, aligned, and not aliased mutably.
pub fn collect_list<T: Copy>(mut cursor: Option<NonNull<Node<T>>>) -> Vec<T> {
    let mut out = Vec::new();
    while let Some(ptr) = cursor {
        // SAFETY: pointer came from Box::into_raw and is still live.
        let node = unsafe { ptr.as_ref() };
        out.push(node.value);
        cursor = node.next;
    }
    out
}

/// Free the heap-allocated nodes in the list.
///
/// # Safety invariant
/// Same as `collect_list` — every pointer came from `Box::into_raw`.
pub fn free_list<T>(mut cursor: Option<NonNull<Node<T>>>) {
    while let Some(ptr) = cursor {
        // SAFETY: pointer came from Box::into_raw; we are the sole owner.
        let node = unsafe { Box::from_raw(ptr.as_ptr()) };
        cursor = node.next;
    }
}

// ---------------------------------------------------------------------------
// Null-pointer optimisation: Option<NonNull<T>> == size_of::<*mut T>()
// ---------------------------------------------------------------------------

/// Returns true when Option<NonNull<T>> is the same size as a raw pointer.
///
/// This is the "null-pointer optimisation": the compiler encodes `None` as the
/// null address, so no extra discriminant word is needed.
pub fn option_nonnull_is_pointer_sized<T>() -> bool {
    size_of::<Option<NonNull<T>>>() == size_of::<*mut T>()
}

// ---------------------------------------------------------------------------
// Simulating a C FFI nullable pointer pattern
// ---------------------------------------------------------------------------

/// Wraps a raw (potentially null) pointer the way a C FFI boundary would.
///
/// `NonNull::new` returns `None` for null, forcing the caller to handle the
/// absent case explicitly rather than dereferencing blindly.
pub fn wrap_nullable<T>(ptr: *mut T) -> Option<NonNull<T>> {
    NonNull::new(ptr)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_pointer_optimisation() {
        // Option<NonNull<T>> must be pointer-sized — this is the whole point.
        assert!(option_nonnull_is_pointer_sized::<i32>());
        assert!(option_nonnull_is_pointer_sized::<u8>());
        assert!(option_nonnull_is_pointer_sized::<[u8; 64]>());

        // Contrast: Option<*mut T> is NOT pointer-sized (needs a discriminant).
        assert!(size_of::<Option<*mut i32>>() > size_of::<*mut i32>());
    }

    #[test]
    fn test_wrap_nullable_null() {
        let null: *mut i32 = std::ptr::null_mut();
        assert!(wrap_nullable(null).is_none());
    }

    #[test]
    fn test_wrap_nullable_nonnull() {
        let mut value: i32 = 42;
        let nn = wrap_nullable(&mut value);
        assert!(nn.is_some());
        // SAFETY: pointer is to a live stack variable, no aliasing.
        let got = unsafe { *nn.unwrap().as_ptr() };
        assert_eq!(got, 42);
    }

    #[test]
    fn test_linked_list_empty() {
        let head = build_list::<i32>(&[]);
        assert!(head.is_none());
        let values = collect_list(head);
        assert!(values.is_empty());
    }

    #[test]
    fn test_linked_list_single() {
        let head = build_list(&[99_i32]);
        let values = collect_list(head);
        assert_eq!(values, [99]);
        free_list(build_list(&[99_i32]));
    }

    #[test]
    fn test_linked_list_multiple() {
        let data = [1_i32, 2, 3, 4, 5];
        let head = build_list(&data);
        let values = collect_list(head);
        assert_eq!(values, data);
        free_list(build_list(&data));
    }

    #[test]
    fn test_nonnull_new_unchecked_never_null() {
        // NonNull::new_unchecked on a live Box pointer must produce Some equivalent.
        let mut x: i32 = 7;
        let raw: *mut i32 = &mut x;
        // SAFETY: raw is non-null (stack variable).
        let nn = unsafe { NonNull::new_unchecked(raw) };
        assert_eq!(unsafe { *nn.as_ptr() }, 7);
    }
}
