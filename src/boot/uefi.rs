use log::info;
use uefi::prelude::*;

use crate::boot::BootInfo;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello UEFI!");
    crate::kernel::main(BootInfo);
}
