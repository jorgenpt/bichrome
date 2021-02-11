use std::io;

fn main() -> io::Result<()> {
    #[cfg(windows)]
    {
        winres::WindowsResource::new()
            .set_icon("assets/chrome_split.ico")
            .compile()?;
    }
    Ok(())
}
