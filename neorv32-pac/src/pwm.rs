#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    enable: Enable,
    polarity: Polarity,
    clkprsc: Clkprsc,
    mode: Mode,
    _reserved4: [u8; 0x70],
    channel: [Channel; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - Channel enable"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x04 - Channel polarity"]
    #[inline(always)]
    pub const fn polarity(&self) -> &Polarity {
        &self.polarity
    }
    #[doc = "0x08 - Clock prescaler select"]
    #[inline(always)]
    pub const fn clkprsc(&self) -> &Clkprsc {
        &self.clkprsc
    }
    #[doc = "0x0c - Channel operation mode (0 = fast-PWM; 1 = phase-correct PWM)"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x80..0x100 - Channel configuration register"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Channel configuration register"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
}
#[doc = "ENABLE (rw) register accessor: Channel enable\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Channel enable"]
pub mod enable;
#[doc = "POLARITY (rw) register accessor: Channel polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity`] module"]
#[doc(alias = "POLARITY")]
pub type Polarity = crate::Reg<polarity::PolaritySpec>;
#[doc = "Channel polarity"]
pub mod polarity;
#[doc = "CLKPRSC (rw) register accessor: Clock prescaler select\n\nYou can [`read`](crate::Reg::read) this register and get [`clkprsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkprsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkprsc`] module"]
#[doc(alias = "CLKPRSC")]
pub type Clkprsc = crate::Reg<clkprsc::ClkprscSpec>;
#[doc = "Clock prescaler select"]
pub mod clkprsc;
#[doc = "MODE (rw) register accessor: Channel operation mode (0 = fast-PWM; 1 = phase-correct PWM)\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Channel operation mode (0 = fast-PWM; 1 = phase-correct PWM)"]
pub mod mode;
#[doc = "Channel configuration register"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "Channel configuration register"]
pub mod channel;
