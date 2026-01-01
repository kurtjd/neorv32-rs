#[repr(C)]
#[doc = "Timer slice"]
#[doc(alias = "SLICE")]
pub struct Slice {
    cnt: Cnt,
    thr: Thr,
}
impl Slice {
    #[doc = "0x00 - Counter register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x04 - Threshold register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        &self.thr
    }
}
#[doc = "CNT (rw) register accessor: Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter register"]
pub mod cnt;
#[doc = "THR (rw) register accessor: Threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`] module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "Threshold register"]
pub mod thr;
