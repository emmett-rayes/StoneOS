use aarch64_cpu::registers::{CurrentEL, Readable};
use cond::cond;

pub(crate) mod boot;
pub(crate) mod cpu;

enum ExceptionLevel {
    EL0,
    EL1,
    EL2,
    EL3,
}

pub struct Aarch64;

impl Aarch64 {
    fn current_exception_level() -> ExceptionLevel {
        cond! {
            CurrentEL.matches_all(CurrentEL::EL::EL0) => ExceptionLevel::EL0,
            CurrentEL.matches_all(CurrentEL::EL::EL1) => ExceptionLevel::EL1,
            CurrentEL.matches_all(CurrentEL::EL::EL2) => ExceptionLevel::EL2,
            CurrentEL.matches_all(CurrentEL::EL::EL3) => ExceptionLevel::EL3,
            _ => unreachable!(),
        }
    }
}
