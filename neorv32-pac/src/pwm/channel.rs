#[repr(C)]
#[doc = "Channel configuration register"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    topcmp: Topcmp,
}
impl Channel {
    #[doc = "0x00 - Channel counter full word access alias"]
    #[inline(always)]
    pub const fn topcmp(&self) -> &Topcmp {
        &self.topcmp
    }
}
#[doc = "TOPCMP (rw) register accessor: Channel counter full word access alias\n\nYou can [`read`](crate::Reg::read) this register and get [`topcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@topcmp`] module"]
#[doc(alias = "TOPCMP")]
pub type Topcmp = crate::Reg<topcmp::TopcmpSpec>;
#[doc = "Channel counter full word access alias"]
pub mod topcmp;
