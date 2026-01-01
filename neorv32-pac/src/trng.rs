#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    data: Data,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and data register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Random data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
}
#[doc = "CTRL (rw) register accessor: Control and data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control and data register"]
pub mod ctrl;
#[doc = "DATA (r) register accessor: Random data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Random data"]
pub mod data;
