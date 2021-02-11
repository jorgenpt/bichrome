use com::ComStrPtr;
use const_format::concatcp;
use std::io;
use std::path::PathBuf;
use windows_bindings;

const SPAD_CANONICAL_NAME: &str = "bichrome.exe";
const CLASS_NAME: &str = "bichromeHTML";

// Configuration for "Set Program Access and Computer Defaults" aka SPAD. StartMenuInternet is the key for browsers
// and they're expected to use the name of the exe as the key.
const SPAD_PATH: &str = concatcp!(r"SOFTWARE\Clients\StartMenuInternet\", SPAD_CANONICAL_NAME);
const APPREG_PATH: &str = concatcp!(
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\",
    SPAD_CANONICAL_NAME
);
const CLSID_PATH: &str = concatcp!(r"SOFTWARE\Classes\", CLASS_NAME);

const DISPLAY_NAME: &str = "biChrome";
const DESCRIPTION: &str = "Pick the right Chrome profile for each URL";

pub fn register_urlhandler(extra_args: Option<&str>) -> Result<(), io::Error> {
    use std::env::current_exe;
    use winreg::enums::*;
    use winreg::RegKey;

    let exe_path = current_exe()?.to_str().unwrap_or_default().to_owned();
    let icon_path = format!("\"{}\",0", exe_path);
    let open_command = if let Some(extra_args) = extra_args {
        format!("\"{}\" {} \"%1\"", exe_path, extra_args)
    } else {
        format!("\"{}\" \"%1\"", exe_path)
    };

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Configure a CLSID for us
    {
        let (clsid, _) = hkcu.create_subkey(CLSID_PATH)?;
        clsid.set_value("", &DISPLAY_NAME)?;

        let (clsid_defaulticon, _) = clsid.create_subkey("DefaultIcon")?;
        clsid_defaulticon.set_value("", &icon_path)?;

        let (clsid_shell_open_command, _) = clsid.create_subkey(r"shell\open\command")?;
        clsid_shell_open_command.set_value("", &open_command)?;
    }

    // Set up the SPAD configuration for the app (https://docs.microsoft.com/en-us/windows/win32/shell/default-programs)
    {
        let (spad, _) = hkcu.create_subkey(SPAD_PATH)?;
        spad.set_value("", &DISPLAY_NAME)?;
        let (spad_capabilities, _) = spad.create_subkey("Capabilities")?;
        spad_capabilities.set_value("ApplicationName", &DISPLAY_NAME)?;
        spad_capabilities.set_value("ApplicationIcon", &icon_path)?;
        spad_capabilities.set_value("ApplicationDescription", &DESCRIPTION)?;

        let (spad_capabilities_startmenu, _) = spad_capabilities.create_subkey("StartMenu")?;
        spad_capabilities_startmenu.set_value("StartMenuInternet", &SPAD_CANONICAL_NAME)?;

        let (spad_capabilities_urlassociations, _) =
            spad_capabilities.create_subkey("URLAssociations")?;
        spad_capabilities_urlassociations.set_value("http", &CLASS_NAME)?;
        spad_capabilities_urlassociations.set_value("https", &CLASS_NAME)?;

        let (spad_defaulticon, _) = spad.create_subkey("DefaultIcon")?;
        spad_defaulticon.set_value("", &icon_path)?;

        let (spad_installinfo, _) = spad.create_subkey("InstallInfo")?;
        spad_installinfo.set_value("ReinstallCommand", &format!("\"{}\" --reinstall", exe_path))?;
        spad_installinfo.set_value("HideIconsCommand", &format!("\"{}\" --hideicons", exe_path))?;
        spad_installinfo.set_value("ShowIconsCommand", &format!("\"{}\" --showicons", exe_path))?;
        spad_installinfo.set_value("IconsVisible", &1u32)?;

        let (spad_shell_open_command, _) = spad.create_subkey(r"shell\open\command")?;
        spad_shell_open_command.set_value("", &open_command)?;
    }

    // Set up a registered application for our SPAD capabilities
    {
        let (registered_applications, _) =
            hkcu.create_subkey(r"SOFTWARE\RegisteredApplications")?;
        let spad_capabilities_path = format!(r"{}\Capabilities", SPAD_PATH);
        registered_applications.set_value(DISPLAY_NAME, &spad_capabilities_path)?;
    }

    // Application Registration (https://docs.microsoft.com/en-us/windows/win32/shell/app-registration)
    {
        let (bichrome_registration, _) = hkcu.create_subkey(APPREG_PATH)?;
        // This is used to resolve "bichrome.exe" -> full path, if needed.
        bichrome_registration.set_value("", &exe_path)?;
        // UseUrl indicates that we don't need the shell to download a file for us -- we can handle direct
        // HTTP URLs.
        bichrome_registration.set_value("UseUrl", &1u32)?;
    }

    // Notify the shell about the updated URL associations.
    unsafe {
        windows_bindings::windows::win32::shell::SHChangeNotify(
            windows_bindings::missing::SHCNE_ASSOCCHANGED,
            windows_bindings::missing::SHCNF_DWORD | windows_bindings::missing::SHCNF_FLUSH,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
    }

    Ok(())
}

fn get_local_app_data_path() -> Option<PathBuf> {
    use winapi::shared::winerror::SUCCEEDED;
    use winapi::um::{knownfolders, shlobj};

    let path_str = unsafe {
        let mut path_ptr = ComStrPtr::null();
        let hr = shlobj::SHGetKnownFolderPath(
            &knownfolders::FOLDERID_LocalAppData,
            0,
            std::ptr::null_mut(),
            path_ptr.mut_ptr(),
        );

        if SUCCEEDED(hr) {
            Some(path_ptr.to_string())
        } else {
            None
        }
    };

    path_str.map(PathBuf::from)
}

pub fn get_chrome_local_state_path() -> Option<PathBuf> {
    let app_data_relative = r"Google\Chrome\User Data\Local State";
    get_local_app_data_path().map(|base| base.join(app_data_relative))
}

mod com {
    use winapi::um::winnt::PWSTR;

    /**
     * A small wrapper around a PWSTR whose memory is owned by COM.
     */
    pub struct ComStrPtr(PWSTR);

    impl ComStrPtr {
        pub fn null() -> ComStrPtr {
            ComStrPtr(std::ptr::null_mut())
        }

        pub fn mut_ptr(&mut self) -> &mut PWSTR {
            &mut self.0
        }

        pub fn ptr(&self) -> PWSTR {
            self.0
        }
    }

    impl ToString for ComStrPtr {
        fn to_string(&self) -> String {
            use std::slice;
            unsafe {
                let len = (0_isize..)
                    .find(|&n| *self.0.offset(n) == 0)
                    .expect("Couldn't find null terminator");
                let array: &[u16] = slice::from_raw_parts(self.ptr(), len as usize);
                String::from_utf16_lossy(array)
            }
        }
    }

    impl Drop for ComStrPtr {
        fn drop(&mut self) {
            use std::ffi::c_void;
            use winapi::um::combaseapi;
            unsafe { combaseapi::CoTaskMemFree(self.ptr() as *mut c_void) };
        }
    }
}
