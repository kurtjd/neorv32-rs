#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    stop_addr: StopAddr,
    delta_src: DeltaSrc,
    delta_dst: DeltaDst,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and status register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Stop tracing when reaching this address, set to all-zero to disable auto-stopping"]
    #[inline(always)]
    pub const fn stop_addr(&self) -> &StopAddr {
        &self.stop_addr
    }
    #[doc = "0x08 - Trace data: delta source address + first-packet bit"]
    #[inline(always)]
    pub const fn delta_src(&self) -> &DeltaSrc {
        &self.delta_src
    }
    #[doc = "0x0c - Trace data: delta destination address + trap-entry bit"]
    #[inline(always)]
    pub const fn delta_dst(&self) -> &DeltaDst {
        &self.delta_dst
    }
}
#[doc = "CTRL (rw) register accessor: Control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control and status register"]
pub mod ctrl;
#[doc = "STOP_ADDR (rw) register accessor: Stop tracing when reaching this address, set to all-zero to disable auto-stopping\n\nYou can [`read`](crate::Reg::read) this register and get [`stop_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop_addr`] module"]
#[doc(alias = "STOP_ADDR")]
pub type StopAddr = crate::Reg<stop_addr::StopAddrSpec>;
#[doc = "Stop tracing when reaching this address, set to all-zero to disable auto-stopping"]
pub mod stop_addr;
#[doc = "DELTA_SRC (r) register accessor: Trace data: delta source address + first-packet bit\n\nYou can [`read`](crate::Reg::read) this register and get [`delta_src::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delta_src`] module"]
#[doc(alias = "DELTA_SRC")]
pub type DeltaSrc = crate::Reg<delta_src::DeltaSrcSpec>;
#[doc = "Trace data: delta source address + first-packet bit"]
pub mod delta_src;
#[doc = "DELTA_DST (r) register accessor: Trace data: delta destination address + trap-entry bit\n\nYou can [`read`](crate::Reg::read) this register and get [`delta_dst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@delta_dst`] module"]
#[doc(alias = "DELTA_DST")]
pub type DeltaDst = crate::Reg<delta_dst::DeltaDstSpec>;
#[doc = "Trace data: delta destination address + trap-entry bit"]
pub mod delta_dst;
