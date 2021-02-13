# <img src="assets/bichrome_icon.png?raw=true" width="24"> bichrome

bichrome is a simple utility for Windows that can be configured as your default browser, which will choose between multiple Chrome profiles to open a URL in based on the configuration you specify.

Running `bichrome.exe` without arguments will attempt to register bichrome as a browser at its current path. It's recommended that you put `bichrome.exe` somewhere permanent before this, as its config and log will live next to it. `%localappdata%\Programs\bichrome\bichrome.exe` is a good place to install it for your current user! bichrome does intentionally not support system-wide registration.

Big thanks to Krista A. Leemhuis for the amazing icon!

## `bichrome_config.json`

All of the configuration for bichrome comes from setting up a set of profile matching patterns. This is done under the `profile_selection` key, and the first profile selector that matches will be used to open the URL. If none of the patterns match, the URL will be opened without specifying the profile. (Chrome's behavior in this case is to open it in the last activated window.)

`bichrome_config.json` is expected to live next to `bichrome.exe`. Alternatively, you can distribute `bichrome.exe` with a `bichrome_template.json`, which tells bichrome how to generate a `bichrome_config.json` based on the expected domains of the profiles in the local Chrome install. This is useful if you want to distribute bichrome to coworkers -- you can set up a useful template, and bichrome will on first start attempt to match that template to the users config. See below for details.

You can find an example config in [example_config/bichrome_config.json](example_config/bichrome_config.json).

## `bichrome_template.json`

When `bichrome.exe` is launched, if it can't find `bichrome_config.json`, it will look for `bichrome_template.json` and if it is found, it will use it to generate a configuration.

`bichrome_template.json` has two top-level keys, `"profiles"` and `"configuration"`. `"configuration"` has the full contents of a normal `bichrome_config.json`, but the profile names listed are _profile placeholder names_. When generating the configuration, bichrome will evaluate the entries in `"profiles"` -- the key is a _profile placeholder name_, and the value is the `hosted_domain` from your Chrome's Local State -- this is typically the part after the @ in your e-mail address. If you use `"NO_HOSTED_DOMAIN"` then we'll look for a default gmail.com account.

You can find an example template in [example_config/bichrome_template.json](example_config/bichrome_template.json).

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
