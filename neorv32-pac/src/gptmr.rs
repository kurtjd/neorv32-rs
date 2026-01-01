#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr0: Csr0,
    csr1: Csr1,
    _reserved2: [u8; 0x78],
    slice: [Slice; 16],
}
impl RegisterBlock {
    #[doc = "0x00 - Control and status register 0"]
    #[inline(always)]
    pub const fn csr0(&self) -> &Csr0 {
        &self.csr0
    }
    #[doc = "0x04 - Control and status register 1"]
    #[inline(always)]
    pub const fn csr1(&self) -> &Csr1 {
        &self.csr1
    }
    #[doc = "0x80..0x100 - Timer slice"]
    #[inline(always)]
    pub const fn slice(&self, n: usize) -> &Slice {
        &self.slice[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Timer slice"]
    #[inline(always)]
    pub fn slice_iter(&self) -> impl Iterator<Item = &Slice> {
        self.slice.iter()
    }
}
#[doc = "CSR0 (rw) register accessor: Control and status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0`] module"]
#[doc(alias = "CSR0")]
pub type Csr0 = crate::Reg<csr0::Csr0Spec>;
#[doc = "Control and status register 0"]
pub mod csr0;
#[doc = "CSR1 (rw) register accessor: Control and status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`] module"]
#[doc(alias = "CSR1")]
pub type Csr1 = crate::Reg<csr1::Csr1Spec>;
#[doc = "Control and status register 1"]
pub mod csr1;
#[doc = "Timer slice"]
pub use self::slice::Slice;
#[doc = r"Cluster"]
#[doc = "Timer slice"]
pub mod slice;
