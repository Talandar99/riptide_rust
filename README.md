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
- You can execute script using ssh with `--remote` or `-r` flag. Script will be coppied via scp, executed, and removed from remote host:
```bash
~> riptide make_dir.sh test --remote username@hostname
# output:
-------------------------------
Remote Execution of make_dir.sh
at username@hostname
-------------------------------
make_dir.sh         100%   21     9.2KB/s   00:00
```
- You can edit script using `--edit` or `-e` flag. default editor is VIM but you can change it in config file
```bash
~> riptide --edit make_dir.sh
# output: "it will open vim"
```
- You can cat script into your terminal using `--cat` or `-c` flag.
```bash
~> riptide --cat make_dir.sh
# output:
#!/bin/bash
mkdir 
```
## Requirements
- Rust
- Bash (not tested on other shells)
- SSH 
- [bash-completion](https://github.com/scop/bash-completion) **(optional)** (install for autocomplete)
- make sure you have `~/.cargo/bin` in your path 

## How to install
##### - basic install
```
make install
```
##### - install with alias (typing `rt` insted of `riptide` is faster but check first if it's not in conflict with other command)
```
make install-with-alias
```
##### !!note!! <br/>make will create config file at `$HOME/.config/riptide`, and scripts file at `$HOME/my_scripts` with 3 examples
## How to remove
```
make uninstall
```
## Default config file
```toml
[path]
path="/home/talandar/my_scripts/"
[editor]
editor="vim"
```
## FAQ
#### command not found. Why?
Make sure you added .cargo/bin to your path. Just add this into `.bashrc`:
```
export PATH=$HOME/.cargo/bin:$PATH
```
and restart your terminal emulator . I will add global installation in future
## TODO / current state
- [x] running scripts from predefined path
- [x] autocompletion
- [x] makefile or installation script
- [x] config file
- [x] support for `rt` alias
- [x] support for script arguments
- [x] remote execution 
- [x] cat - you can see script with a flag
- [x] edit - you can script with a flag
- [ ] manual/help command
- [ ] support for multiple paths
- [ ] support for ~ (home directory)
- [ ] support for other shells 
    - [ ] ash
    - [ ] fish
    - [ ] nush
