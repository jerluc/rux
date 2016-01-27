use common::*;
use core::mem::{align_of, replace};
use alloc::boxed::Box;

pub enum CapabilityUnion {
    // Memory resources capabilities, all has its start and end address, and a
    // next pointer to the next region (if available).
    //
    // A memory resources capability is essentially a pointer to a memory
    // location.
    UntypedMemory(UntypedMemoryCapability),
    // PageTableLevel4(PageTableCapability<PageTableLevel4>),
    // PageTableLevel3(PageTableCapability<PageTableLevel3>),
    // PageTableLevel2(PageTableCapability<PageTableLevel2>),
    // PageTableLevel1(PageTableCapability<PageTableLevel1>),
    // CapabilityPool(CapabilityPoolCapability),
    // KernelPage(KernelPageCapability),

    // // Hardware primitives.
    // VGABuffer(VGABufferCapability),
    None
}

impl CapabilityUnion {
    pub fn as_untyped_memory(cap: CapabilityUnion) -> Option<UntypedMemoryCapability> {
        if let CapabilityUnion::UntypedMemory(x) = cap
        { Some(cap) } else { None }
    }

    // pub fn as_page_table_level4(&self) -> Option<PageTableCapability<PageTableLevel4>> {
    //     if let PageTableLevel4(x) = *self
    //     { Some(x) } else { None }
    // }

    // pub fn as_page_table_level3(&self) -> Option<PageTableCapability<PageTableLevel3>> {
    //     if let PageTableLevel3(x) = *self
    //     { Some(x) } else { None }
    // }

    // pub fn as_page_table_level2(&self) -> Option<PageTableCapability<PageTableLevel2>> {
    //     if let PageTableLevel2(x) = *self
    //     { Some(x) } else { None }
    // }

    // pub fn as_page_table_level1(&self) -> Option<PageTableCapability<PageTableLevel1>> {
    //     if let PageTableLevel1(x) = *self
    //     { Some(x) } else { None }
    // }

    // pub fn as_capability_pool(&self) -> Option<CapabilityPoolCapability> {
    //     if let CapabilityPool(x) = *self
    //     { Some(x) } else { None }
    // }

    // pub fn as_kernel_page(&self) -> Option<KernelPageCapability> {
    //     if let KernelPage(x) = *self
    //     { Some(x) } else { None }
    // }

    // pub fn as_vga_buffer(&self) -> Option<VGABufferCapability> {
    //     if let VGABuffer(x) = *self
    //     { Some(x) } else { None }
    // }
}

pub struct CapabilityPool([CapabilityUnion; CAPABILITY_POOL_COUNT]);

// The main kernel capability pool is static. Other capability pools are created
// by retype kernel page.

pub struct CapabilityPoolCapability {
    start_addr: PhysicalAddress,
    physical_start_addr: PhysicalAddress,
    object: Box<CapabilityPool>,
}

impl MemoryBlockCapability for CapabilityPoolCapability {
    fn start_addr(&self) -> PhysicalAddress {
        self.start_addr
    }

    fn size(&self) -> usize {
        CAPABILITY_POOL_SIZE
    }

    fn physical_start_addr(&self) -> PhysicalAddress {
        self.physical_start_addr
    }

    fn physical_size(&self) -> usize {
        self.end_addr() - self.physical_start_add()
    }
}

impl CapabilityPoolCapability {
    pub fn from_untyped(cap: UntypedMemoryCapability)
                        -> (Option<CapabilityPoolCapability>, Option<UntypedMemoryCapability>) {
        let size = CAPABILITY_POOL_SIZE;
        let align = align_of::<CapabilityPool>();
        let start_addr = cap.start_addr() + (align - cap.start_addr() % align);
        let end_addr = start_addr + size;

        if end_addr > cap.end_addr() {
            (None, Some(cap))
        } else if end_addr <= cap.end_addr() {
            let pool_box = unsafe {
                let pool_raw = *(start_addr as *mut _);
                replace::<CapabilityPool>(&mut pool_raw,
                                          CapabilityPool([None; CAPABILITY_POOL_COUNT]));
                Box::from_raw(pool_raw)
            };

            let pool_cap = CapabilityPoolCapability {
                start_addr: start_addr,
                physical_start_addr: cap.start_addr(),
                object: pool_box,
            };

            cap.start_addr = end_addr + 1;
            cap.size = cap.size() - (end_addr - physical_start_addr);

            if end_addr = cap.end_addr {
                (Some(pool_cap), None)
            } else {
                (Some(pool_cap), Some(cap))
            }
        }
    }
}