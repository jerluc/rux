use common::*;
use arch::paging::{BASE_PAGE_LENGTH};
use util::{MemoryObject, UniqueReadGuard, UniqueWriteGuard,
           RwLock, RwLockReadGuard, RwLockWriteGuard};
use cap::{UntypedHalf, CapHalf, Capability, CapReadRefObject, CapWriteRefObject};

/// Non-clonable, lock in CapHalf.

#[derive(Debug)]
pub struct PageHalf {
    start_paddr: PAddr,
    lock: RwLock<()>,
}

impl<'a> CapReadRefObject<'a, [u8; BASE_PAGE_LENGTH], UniqueReadGuard<'a, [u8; BASE_PAGE_LENGTH]>> for PageHalf {
    fn read(&'a self) -> UniqueReadGuard<'a, [u8; BASE_PAGE_LENGTH]> {
        unsafe { UniqueReadGuard::new(
            MemoryObject::<[u8; BASE_PAGE_LENGTH]>::new(self.start_paddr),
            self.lock.read()
        ) }
    }
}

impl<'a> CapWriteRefObject<'a, [u8; BASE_PAGE_LENGTH], UniqueWriteGuard<'a, [u8; BASE_PAGE_LENGTH]>> for PageHalf {
    fn write(&'a mut self) -> UniqueWriteGuard<'a, [u8; BASE_PAGE_LENGTH]> {
        unsafe { UniqueWriteGuard::new(
            MemoryObject::<[u8; BASE_PAGE_LENGTH]>::new(self.start_paddr),
            self.lock.write()
        ) }
    }
}

impl PageHalf {
    pub fn start_paddr(&self) -> PAddr {
        self.start_paddr
    }

    pub fn new(untyped: &mut UntypedHalf) -> PageHalf {
        let alignment = BASE_PAGE_LENGTH;
        let paddr = untyped.allocate(BASE_PAGE_LENGTH, alignment);

        let mut half = PageHalf {
            start_paddr: paddr,
            lock: RwLock::new(()),
        };

        for u in half.write().iter_mut() {
            *u = 0x0: u8;
        }

        half
    }

    pub fn length() -> usize {
        BASE_PAGE_LENGTH
    }
}