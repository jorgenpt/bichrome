::windows::include_bindings!();

/// A location to keep symbols not exported by the windows-rs bindings.
pub mod missing {
    // These three are filed in https://github.com/microsoft/windows-rs/issues/535
    pub const SHCNE_ASSOCCHANGED: i32 = 0x08000000;
    pub const SHCNF_DWORD: u32 = 0x0003;
    pub const SHCNF_FLUSH: u32 = 0x1000;
}
