# <img src="{repo_url}/blob/{this_version}/assets/bichrome_icon.png?raw=true" width="24"> bichrome {this_version}

bichrome is a simple utility for Windows and macOS that you configure as your default browser, which then will choose which browser to open a URL in based on the configuration you specify. It also supports picking a particular Chrome profile -- either by specifying a profile name, or by specifying the "hosted domain" of your profile if you're using Google Workspace. You can read more in the 

## Installation

### Windows

1. Download `bichrome-win64.exe` from [this release][windows_download].
2. Move it to its permanent home -- e.g. creating a directory in `%localappdata%\Programs` called bichrome and putting it there.
3. Run `bichrome-win64.exe` once by double clicking it. This will register bichrome as a potential browser.
4. Configure bichrome as your default browser by opening "Default Apps" (You can open your start menu and just type "Default Apps") and clicking the icon under "Web browser", and picking bichrome.

That's it! Now just create a configuration file named `bichrome_config.json` next to `bichrome-win64.exe` (see [the configuration section][config_readme] for details) -- a good starting place is to download & edit the [example config][example_config]] 


### macOS

1. Download `bichrome-macos.zip` from [this release][macos_download].
2. Extract it and copy the `bichrome` app e.g. to `/Applications`
3. Open System Preferences and search for "Default Browser"
4. Pick bichrome as youre default browser.

That's it! Now just create a configuration file named `bichrome_config.json` in `~/Library/Application Support/com.bitspatter.bichrome/bichrome_config.json` (see [the configuration section][config_readme] for details) -- a good starting place is to download & edit the [example config][example_config].

## Changes in {this_version}
{{ if changelog.scopes }}
{{ for scope in changelog.scopes }}
### {scope.title}

{{ for category in scope.categories -}}
{{ for change in category.changes -}}
 * **{category.title}**: {change}
{{ endfor -}}
{{- endfor -}}
{{ endfor }}

See [the full list of changes][commits_link].
{{ else }}
No significant user changes, see [the full list of changes][commits_link].
{{ endif }}

[commits_link]: {commits_link}
[windows_download]: {repo_url}/releases/download/{this_version}/bichrome-win64.exe
[macos_download]: {repo_url}/releases/download/{this_version}/bichrome-macos.zip
[example_config]: {repo_url}/releases/download/{this_version}/bichrome_example_config.json
[config_readme]: {repo_url}/blob/{this_version}/README.md#bichrome_configjson