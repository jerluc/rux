use core::fmt;
use common::{PAddr, VAddr};

#[macro_use]
mod macros;

/// Representations of page tables.
mod table;

/// Memory objects implementation.
mod with;

/// Basic page length in x86_64 (4 KiB).
pub const BASE_PAGE_LENGTH: usize = 4096; // 4 KiB

/// Large page length in x86_64 (2 MiB).
pub const LARGE_PAGE_LENGTH: usize = 1024 * 1024 * 2; // 2 MiB

/// Huge page length in x86_64 (1 GiB).
pub const HUGE_PAGE_LENGTH: usize = 1024 * 1024 * 1024; // 1 GiB

/// Cache line length in x86_64 (64 Bytes).
pub const CACHE_LINE_LENGTH: usize = 64; // 64 Bytes

/// MAXPHYADDR, which is at most 52; (use CPUID for finding system value).
pub const MAXPHYADDR: u64 = 52;

/// Mask to find the physical address of an entry in a page-table.
const ADDRESS_MASK: u64 = ((1 << MAXPHYADDR) - 1) & !0xfff;

pub use self::table::*;
pub use self::with::{MemoryObject};

/// Invalidate the given address in the TLB using the `invlpg` instruction.
///
/// # Safety
///
/// This function is unsafe as it causes a general protection fault (GP) if the current privilege
/// level is not 0.
pub unsafe fn flush(vaddr: VAddr) {
    asm!("invlpg ($0)" :: "r" (vaddr.into(): usize) : "memory");
}

/// Invalidate the TLB completely by reloading the CR3 register.
///
/// # Safety
///
/// This function is unsafe as it causes a general protection fault (GP) if the current privilege
/// level is not 0.
pub unsafe fn flush_all() {
    use x86::shared::control_regs::{cr3, cr3_write};
    cr3_write(cr3())
}

/// Switch to a PML4 page table.
///
/// # Safety
///
/// The PML4 page table must have kernel mapped in
/// `KERNEL_BASE`. `paddr` must point to a valid PML4 page table.
pub unsafe fn switch_to(paddr: PAddr) {
    use x86::shared::control_regs::{cr3_write};

    cr3_write(paddr.into(): usize);
}
