# bichrome

bichrome is a command line tool for Windows that can be configured as your default browser, which will choose between multiple Chrome profiles to open a URL in based on the configuration you specify.

## `bichrome_config.json`

The majority of configuring bichrome comes from setting up a set of profile matching patterns. This is done under the `profile_selection` key, and the first profile selector that matches will be used to open the URL. If none of the patterns match, the URL will be opened without specifying the profile. (Chrome's behavior in this case is to open it in the last activated window.)

`bichrome_config.json` is expected to live next to `bichrome.exe`.

```json
{
    "profile_selection": [
        {
            "profile": "Default",
            "patterns": [
                "[^.]*.facebook.com/",
                "[^.]*.messenger.com/"
            ]
        },
        {
            "profile": "Profile 1",
            "patterns": [
                "[^.]*.mycorp.net/",
                "mycorp.atlassian.net/"
            ]
        },
        {
            "profile": "Default",
            "patterns": [
                ".*"
            ]
        }
    ]
}
```

## Usage

```
    bichrome.exe [FLAGS] [urls]...

FLAGS:
        --debug                      Use debug logging, even more verbose than --verbose
        --dry-run                    Do not launch Chrome, just log what would've been launched
        --force-config-generation    Always generate a config, even if it exists or if we're using --dry-run
    -h, --help                       Prints help information
    -V, --version                    Prints version information
    -v, --verbose                    Use verbose logging

ARGS:
    <urls>...    List of URLs to open
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
