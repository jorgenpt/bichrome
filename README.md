[![Build status](https://github.com/jorgenpt/bichrome/workflows/Build/badge.svg)](https://github.com/jorgenpt/bichrome/actions?query=workflow%3ABuild)

# <img src="assets/bichrome_icon.png?raw=true" width="24"> bichrome

bichrome is a simple utility for Windows and macOS that you configure as your default browser, which then will choose which browser to open a URL in based on the configuration you specify. It also supports picking a particular Chrome profile -- either by specifying a profile name, or by specifying the "hosted domain" of your profile if you're using Google Workspace. (Your hosted domain is the bit after the @ in a non-"gmail dot com" address hosted by GMail.)

It was created to address the problem of clicking links in Slack and other apps and then having to relocate them to the "correct" browser window / Chrome profile where you're logged in to Facebook / JIRA / etc.

Big thanks to Krista A. Leemhuis for the amazing icon!

## Installation

### Windows

1. Download `bichrome-win64.exe` from [the latest release](https://github.com/jorgenpt/bichrome/releases/latest).
2. Move it to its permanent home -- e.g. creating a directory in `%localappdata%\Programs` called bichrome and putting it there.
3. Run `bichrome-win64.exe` once by double clicking it. This will register bichrome as a potential browser.
4. Configure bichrome as your default browser by opening "Default Apps" (You can open your start menu and just type "Default Apps") and clicking the icon under "Web browser", and picking bichrome.

That's it! Now just create a configuration file named `bichrome_config.json` next to `bichrome-win64.exe` (see [the configuration section](#config) for details) -- a good starting place is to download & edit the [example config](https://raw.githubusercontent.com/jorgenpt/bichrome/main/example_config/bichrome_config.json).


### macOS

1. Download `bichrome-macos.zip` from [the latest release](https://github.com/jorgenpt/bichrome/releases/latest).
2. Extract it and copy the `bichrome` app e.g. to `/Applications`
3. Open System Preferences and search for "Default Browser"
4. Pick bichrome as your default browser.

That's it! Now just create a configuration file named `bichrome_config.json` in `~/Library/Application Support/com.bitspatter.bichrome/bichrome_config.json` (see [the configuration section](#config) for details) -- a good starting place is to download & edit the [example config](https://raw.githubusercontent.com/jorgenpt/bichrome/main/example_config/bichrome_config.json).


## `bichrome_config.json`

Configuring bichrome involves setting up a set of `profiles` that define a name and a browser (and for Chrome, optionally a browser profile name or a profile's hosted domain), and setting up a list of profile selectors that pick a profile based on matching patterns against the URL you're opening. Profile names

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

Configuring the matching is done under the `profile_selection` key. The browser from the first selector that matches the URL will be used to open the URL. If none of the patterns match, the URL will be opened with the profile named in `default_profile`, and if that doesn't exist, it will default to using Chrome with no profile specified. (Chrome's behavior in this case is to open it in the last activated window.) A profile specifying a browser of `OsDefault` will use Safari on macOS and Edge on Windows, and `Safari` or `Edge` will open the respective browser iff it's running on a supported OS.

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

`bichrome_config.json` is expected to live next to `bichrome-win64.exe` on Windows, and in `~/Library/Application Support/com.bitspatter.bichrome/bichrome_config.json` on macOS.

You can find an example config in [example_config/bichrome_config.json][example_config].

Profile names for Chrome and Edge can be a little bit opaque -- the standard profile name for both of them (i.e. the first profile created) is `Default`, and then it will create profiles named `Profile 1`, `Profile 2`, and so forth. These will (on Windows) each have a folder in `%localappdata%/Google/Chrome/User Data` or `%localappdata%/Microsoft/Edge/User Data`. The correct profile name for the active profile can be found in the `Profile path` key on `edge://version/` or `chrome://version/` respectively.

For Chrome, `hosted_domain` can be the name of a Google Apps domain that you've signed in to Chrome, in which case bichrome automatically determines which profile that is.

[example_config]: example_config/bichrome_config.json

## License

[The icon](assets/bichrome_icon.png) is copyright (c) 2021 [Jørgen P. Tjernø](mailto:jorgen@tjer.no). All Rights Reserved.

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
