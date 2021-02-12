::windows::include_bindings!();

/// A location to keep symbols not exported by the windows-rs bindings.
pub mod missing {
    // These three are filed in https://github.com/microsoft/windows-rs/issues/535
    pub const SHCNE_ASSOCCHANGED: i32 = 0x08000000;
    pub const SHCNF_DWORD: u32 = 0x0003;
    pub const SHCNF_FLUSH: u32 = 0x1000;

    // This one is filed in https://github.com/microsoft/win32metadata/issues/220
    #[allow(non_upper_case_globals)]
    pub const FOLDERID_LocalAppData: ::windows::Guid = ::windows::Guid::from_values(
        0xF1B32785,
        0x6FBA,
        0x4FCF,
        [0x9Du8, 0x55, 0x7B, 0x8E, 0x7F, 0x15, 0x70, 0x91],
    );
}
