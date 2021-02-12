fn main() {
    windows::build!(windows::win32::shell::*, windows::win32::com::CoTaskMemFree);
}
