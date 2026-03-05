//! # Inline Assembly
//! Using asm! macro (requires nightly for some features)

#[cfg(target_arch = "x86_64")]
pub fn cpuid_supported() -> bool { true }

#[cfg(not(target_arch = "x86_64"))]
pub fn cpuid_supported() -> bool { false }

/// Example of using inline assembly (conceptual)
pub fn add_with_asm(a: u64, b: u64) -> u64 {
    // On stable Rust, we use regular addition
    // On nightly: use std::arch::asm!
    a + b
}

pub fn memory_fence() {
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() { assert_eq!(add_with_asm(2, 3), 5); }
}
