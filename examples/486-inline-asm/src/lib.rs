//! # Inline Assembly — Low-Level Control
//!
//! Using inline assembly in Rust (requires nightly for some features).

// Note: asm! is stable as of Rust 1.59, but some features need nightly

/// Simple example without actual asm (portable version)
pub fn add_one(x: u32) -> u32 {
    x + 1
}

/// CPUID example (x86_64 only)
#[cfg(target_arch = "x86_64")]
pub fn get_cpu_vendor() -> [u8; 12] {
    let mut vendor = [0u8; 12];

    #[cfg(target_arch = "x86_64")]
    unsafe {
        use std::arch::asm;

        let mut ebx: u32;
        let mut ecx: u32;
        let mut edx: u32;

        asm!(
            "cpuid",
            inout("eax") 0u32 => _,
            out("ebx") ebx,
            out("ecx") ecx,
            out("edx") edx,
        );

        vendor[0..4].copy_from_slice(&ebx.to_le_bytes());
        vendor[4..8].copy_from_slice(&edx.to_le_bytes());
        vendor[8..12].copy_from_slice(&ecx.to_le_bytes());
    }

    vendor
}

/// Read timestamp counter (x86_64)
#[cfg(target_arch = "x86_64")]
pub fn rdtsc() -> u64 {
    unsafe {
        use std::arch::asm;

        let lo: u32;
        let hi: u32;

        asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
            options(nostack, nomem),
        );

        ((hi as u64) << 32) | (lo as u64)
    }
}

/// Pause instruction for spin loops
#[cfg(target_arch = "x86_64")]
pub fn spin_pause() {
    unsafe {
        use std::arch::asm;
        asm!("pause", options(nostack, nomem));
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn spin_pause() {
    std::hint::spin_loop();
}

/// Memory fence
pub fn memory_fence() {
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(41), 42);
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_cpu_vendor() {
        let vendor = get_cpu_vendor();
        let vendor_str = std::str::from_utf8(&vendor).unwrap_or("unknown");
        println!("CPU Vendor: {}", vendor_str);
        // Should be "GenuineIntel" or "AuthenticAMD" or similar
        assert!(vendor.iter().any(|&b| b != 0));
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_rdtsc() {
        let t1 = rdtsc();
        let t2 = rdtsc();
        assert!(t2 >= t1);
    }

    #[test]
    fn test_spin_pause() {
        spin_pause(); // Should not panic
    }
}
