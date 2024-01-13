pub struct PhysicalAddress {
    pub address: usize,
}

pub trait Address {
    unsafe fn new(address: usize) -> Self;
    fn raw_address(&self) -> usize;
}

impl Address for PhysicalAddress {
    unsafe fn new(address: usize) -> Self {
        PhysicalAddress { address }
    }

    fn raw_address(&self) -> usize {
        self.address
    }
}
