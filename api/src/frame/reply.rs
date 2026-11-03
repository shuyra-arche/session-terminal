use env::PieToken;

#[derive(Clone, Copy, env::Frame)]
pub struct Reply {
    pub status: u8,
    pub authority: PieToken,
    /// Fresh data grants in the recipient's table; no indexed discovery is needed.
    pub input: PieToken,
    pub output: PieToken,
    pub control: PieToken,
}
