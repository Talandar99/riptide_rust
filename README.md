# 🌊 Riptide 
Simple cli tool for running scripts 

## How it works / How to use
- Add scripts to `~/my_scripts` directory (or other directory specified in `$HOME/.config/riptide`)
- run by typing `riptide $SCRIPT_NAME`. You can use `<TAB>` for autocompletion
- example usage:
```bash
~> riptide my_script.sh
# output:
Hello From my_script
```
- You can use alias `rt` insted of typing `riptide` (only if installed using `make install-with-alias`):
```bash
~> rt my_script.sh
# output:
Hello From my_script
```
- You can also use script arguments:
```bash
~> riptide echo_script_args.sh foo bar 
# output:
Hello From echo_script_args
foo
bar
```
- You can execute script using ssh with `--remote` or `-r` flag:
```bash
~> riptide make_dir.sh test --remote username@hostname
# output:
-------------------------------
Remote Execution of make_dir.sh
at username@hostname
-------------------------------
make_dir.sh         100%   21     9.2KB/s   00:00
```
## Requirements
- Rust
- Bash (not tested on other shells)
- SSH 
- [bash-completion](https://github.com/scop/bash-completion) **(optional)** (install for autocomplete)
## How to install
##### - basic install
```
make install
```
##### - install with alias (typing `rt` insted of `riptide` is faster but check first if it's not in conflict with other command)
```
make install-with-alias
```
##### !!note!! <br/>make will create config file at `$HOME/.config/riptide`, and scripts file at `$HOME/my_scripts` with 2 examples
## How to remove
```
make uninstall
```
## Default config file
```toml
[path]
path="/home/$USER/my_scripts/"
```
## TODO / current state
- [x] running scripts from predefined path
- [x] autocompletion
- [x] makefile or installation script
- [x] config file
- [x] support for `rt` alias
- [x] support for script arguments
- [ ] remote execution 
- [ ] manual/help command
- [ ] support for multiple paths
- [ ] support for ~ (home directory)
- [ ] support for other shells 
    - [ ] ash
    - [ ] fish
    - [ ] nush
