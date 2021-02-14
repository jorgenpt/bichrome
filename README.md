[![Travis CI Build status](https://www.travis-ci.com/jorgenpt/bichrome.svg?branch=main)](https://www.travis-ci.com/github/jorgenpt/bichrome)
[![AppVeyor Build status](https://ci.appveyor.com/api/projects/status/epdwc70kl4sggl4v?svg=true)](https://ci.appveyor.com/project/jorgenpt/bichrome)

# <img src="assets/bichrome_icon.png?raw=true" width="24"> bichrome

bichrome is a simple utility for Windows that can be configured as your default browser, which will choose between multiple Chrome profiles to open a URL in based on the configuration you specify.

Running `bichrome.exe` without arguments will attempt to register bichrome as a browser at its current path. It's recommended that you put `bichrome.exe` somewhere permanent before this, as its config and log will live next to it. `%localappdata%\Programs\bichrome\bichrome.exe` is a good place to install it for your current user! bichrome does intentionally not support system-wide registration.

Big thanks to Krista A. Leemhuis for the amazing icon!

## `bichrome_config.json`

All of the configuration for bichrome comes from setting up a set of profile matching patterns. This is done under the `profile_selection` key, and the first profile selector that matches will be used to open the URL. If none of the patterns match, the URL will be opened without specifying the profile. (Chrome's behavior in this case is to open it in the last activated window.) `bichrome_config.json` is expected to live next to `bichrome.exe` on Windows, and in `~/Library/Application Support/com.bitspatter.bichrome/bichrome_config.json` on macOS.

You can find an example config in [example_config/bichrome_config.json](example_config/bichrome_config.json).

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
