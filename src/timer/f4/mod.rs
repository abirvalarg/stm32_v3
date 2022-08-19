use crate::sync::SyncCell;

#[cfg(not(feature = "f401"))]
pub mod basic;
pub mod gp34;


pub static TIM3: SyncCell<gp34::Gp34> = SyncCell::new(gp34::Gp34::new(0x4000_0400));
pub static TIM4: SyncCell<gp34::Gp34> = SyncCell::new(gp34::Gp34::new(0x4000_0800));
