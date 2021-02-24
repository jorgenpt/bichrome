[![Build status](https://github.com/jorgenpt/bichrome/workflows/Build/badge.svg)](https://github.com/jorgenpt/bichrome/actions?query=workflow%3ABuild)

# <img src="assets/bichrome_icon.png?raw=true" width="24"> bichrome

bichrome is a simple utility for Windows that can be configured as your default browser, which will choose between multiple Chrome profiles to open a URL in based on the configuration you specify.

Running `bichrome.exe` without arguments will attempt to register bichrome as a browser at its current path. It's recommended that you put `bichrome.exe` somewhere permanent before this, as its config and log will live next to it. `%localappdata%\Programs\bichrome\bichrome.exe` is a good place to install it for your current user! bichrome does intentionally not support system-wide registration.

Big thanks to Krista A. Leemhuis for the amazing icon!

## `bichrome_config.json`

Configuring bichrome involves setting up a set of `profiles` that define a name and a browser (and for Chrome, optionally a browser profile name or a profile's hosted domain), and setting up a list of profile selectors that pick a profile based on matching patterns against the URL you're opening.

The following snippet shows how profiles are configured. See [the example config][example_config] for a more complete example.

```json
{
  "default_profile": "Personal",
  "profiles": {
    "Work": {
      "browser": "Chrome",
      "hosted_domain": "mycorp.com"
    },
    "Personal": {
      "browser": "Firefox"
    }
  },
  "profile_selection": [ ... ]
}
```

The format for the patterns are documented in detail on [Mozilla.org](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Match_patterns) or in [the documentation of the webextension_pattern crate](https://docs.rs/webextension_pattern/latest/webextension_pattern/index.html) which is used to perform the matching. Some examples can be found in the [the example config][example_config].

Configuring the matching is done under the `profile_selection` key. The browser from the first selector that matches the URL will be used to open the URL. If none of the patterns match, the URL will be opened with the profile named in `default_profile`, and if that doesn't exist, it will default to using Chrome with no profile specified. (Chrome's behavior in this case is to open it in the last activated window.) A profile specifying a browser of `Safari`, `Edge`, or `OsDefault` will use Safari on macOS and Edge on Windows.

The following snippet shows how selectors are configured. See [the example config][example_config] for a more complete example.

```json
{
  "default_profile": "...",
  "profiles": { ... },
  "profile_selection": [
    {
        "profile": "Personal",
        "pattern": "*.twitter.com"
    },
    {
        "profile": "Work",
        "pattern": "*.mycorp.net"
    }
  ]
}
```

`bichrome_config.json` is expected to live next to `bichrome.exe` on Windows, and in `~/Library/Application Support/com.bitspatter.bichrome/bichrome_config.json` on macOS.

You can find an example config in [example_config/bichrome_config.json][example_config].

[example_config]: example_config/bichrome_config.json

## License

[The icon](assets/bichrome_icon.png) is copyright (c) 2021 [Jørgen P. Tjernø](mailto:jorgenpt@gmail.com). All Rights Reserved.

The source code is licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
