//! 717 — Volatile Reads/Writes for Memory-Mapped I/O
//!
//! `read_volatile` / `write_volatile` prevent the compiler from eliding,
//! reordering, or merging accesses to memory-mapped I/O registers.
//! Every access is treated as observable side-effecting I/O.

use std::ptr;

// ── Register offsets ──────────────────────────────────────────────────────────
pub const REG_STATUS: usize = 0;
pub const REG_DATA: usize = 1;
pub const REG_CTRL: usize = 2;

// ── Status-register bit masks ─────────────────────────────────────────────────
pub const TX_READY: u32 = 0x01;
pub const RX_READY: u32 = 0x02;

// ── Control-register bit masks ────────────────────────────────────────────────
pub const CTRL_ENABLE: u32 = 0x01;
pub const CTRL_RESET: u32 = 0x80;

/// Simulated MMIO device with 8 × u32 registers.
///
/// In real embedded code the struct would hold a raw pointer to a fixed
/// hardware address obtained from a linker script or `mmap`. Here we own
/// the backing array so that we can run tests in userspace without special
/// privileges or hardware.
pub struct MmioDevice {
    regs: [u32; 8],
}

impl MmioDevice {
    /// Create a zeroed device.
    pub fn new() -> Self {
        Self { regs: [0u32; 8] }
    }

    /// Volatile write: every call reaches the "hardware", no elision.
    ///
    /// # Safety contract (internal)
    /// `reg < 8` is asserted before the pointer arithmetic, so the pointer
    /// is always valid and properly aligned for `u32`.
    pub fn write(&mut self, reg: usize, val: u32) {
        assert!(reg < self.regs.len(), "register index out of bounds");
        // SAFETY: pointer derived from a live `[u32; 8]` at a checked offset.
        unsafe {
            ptr::write_volatile(self.regs.as_mut_ptr().add(reg), val);
        }
    }

    /// Volatile read: the compiler may not cache the result in a register.
    pub fn read(&self, reg: usize) -> u32 {
        assert!(reg < self.regs.len(), "register index out of bounds");
        // SAFETY: pointer derived from a live `[u32; 8]` at a checked offset.
        unsafe { ptr::read_volatile(self.regs.as_ptr().add(reg)) }
    }

    /// Set individual bits in a register (read-modify-write, both volatile).
    pub fn set_bits(&mut self, reg: usize, mask: u32) {
        let current = self.read(reg);
        self.write(reg, current | mask);
    }

    /// Clear individual bits in a register (read-modify-write, both volatile).
    pub fn clear_bits(&mut self, reg: usize, mask: u32) {
        let current = self.read(reg);
        self.write(reg, current & !mask);
    }

    /// Drain the TX FIFO: poll TX_READY, then send each byte as a u32.
    ///
    /// Returns the number of bytes written.
    /// Demonstrates the canonical volatile-poll pattern used in real drivers.
    pub fn send_bytes(&mut self, bytes: &[u8]) -> usize {
        bytes
            .iter()
            .filter(|&&b| {
                // Every status read goes through read_volatile — the compiler
                // cannot hoist it out of this loop even if it appears invariant.
                let status = self.read(REG_STATUS);
                if status & TX_READY != 0 {
                    self.write(REG_DATA, u32::from(b));
                    true
                } else {
                    false
                }
            })
            .count()
    }
}

impl Default for MmioDevice {
    fn default() -> Self {
        Self::new()
    }
}

// ── Standalone volatile helpers (no wrapper struct) ───────────────────────────

/// Write a u32 to an arbitrary raw pointer using `write_volatile`.
///
/// # Safety
/// The caller must ensure `ptr` is valid, aligned, and exclusively accessible.
pub unsafe fn mmio_write(ptr: *mut u32, val: u32) {
    ptr::write_volatile(ptr, val);
}

/// Read a u32 from an arbitrary raw pointer using `read_volatile`.
///
/// # Safety
/// The caller must ensure `ptr` is valid, aligned, and not concurrently mutated.
pub unsafe fn mmio_read(ptr: *const u32) -> u32 {
    ptr::read_volatile(ptr)
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read_roundtrip() {
        let mut dev = MmioDevice::new();
        dev.write(REG_DATA, 0xDEAD_BEEF);
        assert_eq!(dev.read(REG_DATA), 0xDEAD_BEEF);
    }

    #[test]
    fn test_multiple_registers_are_independent() {
        let mut dev = MmioDevice::new();
        dev.write(REG_STATUS, 0x01);
        dev.write(REG_DATA, 0xFF);
        dev.write(REG_CTRL, 0x80);
        assert_eq!(dev.read(REG_STATUS), 0x01);
        assert_eq!(dev.read(REG_DATA), 0xFF);
        assert_eq!(dev.read(REG_CTRL), 0x80);
    }

    #[test]
    fn test_set_and_clear_bits() {
        let mut dev = MmioDevice::new();
        // Set TX_READY and RX_READY
        dev.set_bits(REG_STATUS, TX_READY | RX_READY);
        assert_eq!(dev.read(REG_STATUS), TX_READY | RX_READY);

        // Clear only TX_READY
        dev.clear_bits(REG_STATUS, TX_READY);
        assert_eq!(dev.read(REG_STATUS), RX_READY);
    }

    #[test]
    fn test_send_bytes_when_tx_ready() {
        let mut dev = MmioDevice::new();
        // Simulate hardware signalling TX ready
        dev.write(REG_STATUS, TX_READY);
        let sent = dev.send_bytes(b"hello");
        assert_eq!(sent, 5);
        // Last byte written should be b'o'
        assert_eq!(dev.read(REG_DATA), u32::from(b'o'));
    }

    #[test]
    fn test_send_bytes_when_tx_not_ready() {
        let mut dev = MmioDevice::new();
        // TX_READY bit is 0 — no bytes should be transmitted
        dev.write(REG_STATUS, 0x00);
        let sent = dev.send_bytes(b"hi");
        assert_eq!(sent, 0);
    }

    #[test]
    fn test_standalone_mmio_helpers() {
        let mut reg: u32 = 0;
        unsafe {
            mmio_write(&mut reg as *mut u32, 0xCAFE_BABE);
            let val = mmio_read(&reg as *const u32);
            assert_eq!(val, 0xCAFE_BABE);
        }
    }

    #[test]
    fn test_ctrl_enable_reset_sequence() {
        let mut dev = MmioDevice::new();
        // Assert reset, then release and enable
        dev.write(REG_CTRL, CTRL_RESET);
        assert!(dev.read(REG_CTRL) & CTRL_RESET != 0);
        dev.clear_bits(REG_CTRL, CTRL_RESET);
        dev.set_bits(REG_CTRL, CTRL_ENABLE);
        assert_eq!(dev.read(REG_CTRL), CTRL_ENABLE);
    }

    #[test]
    fn test_overwrite_same_register_twice() {
        // Both writes must survive — volatile prevents the first from being
        // optimised away as "dead" even though nothing reads between them.
        let mut dev = MmioDevice::new();
        dev.write(REG_DATA, 0x1111_1111);
        dev.write(REG_DATA, 0x2222_2222);
        assert_eq!(dev.read(REG_DATA), 0x2222_2222);
    }
}
