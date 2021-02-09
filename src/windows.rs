use const_format::concatcp;
use std::env::current_exe;
use std::io;
use winreg::enums::*;
use winreg::RegKey;

const SPAD_CANONICAL_NAME: &str = "bichrome.exe";
const CLASS_NAME: &str = "bichromeHTTP";

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

pub fn register_urlhandler() -> Result<(), io::Error> {
    let exe_path = current_exe()?.to_str().unwrap_or_default().to_owned();
    let icon_path = format!("\"{}\",0", exe_path);
    let open_command = format!("\"{}\" \"%1\"", exe_path);

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

        let (spad_capabilities_startmenu_urlassociations, _) =
            spad_capabilities_startmenu.create_subkey("URLAssociations")?;
        spad_capabilities_startmenu_urlassociations.set_value("http", &CLASS_NAME)?;
        spad_capabilities_startmenu_urlassociations.set_value("https", &CLASS_NAME)?;

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

    Ok(())
}
