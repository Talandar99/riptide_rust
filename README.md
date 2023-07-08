# 🌊 Riptide
cli tool for running scripts 

# why not symlink?
- to eliminate risc of running specific scripts by accident. For example imagine that you have script that modify recursivly all files in current directory
- less junk in $PATH 

# TODO / current state
- [x] running scripts from predefined path
- [x] autocompletion
- [ ] config file
- [ ] makefile or installation script
- [ ] support for ~ (home directory)
- [ ] support for script arguments

## installation
```
cargo install --force --path .
```
```
complete -C __riptide_shell_completion riptide
```
## How it works / How to use
- After running for the first time it create configuration file in .config/riptide/conf.toml and .config/riptide/scripts
- You can specify path in toml file
- run by typing `riptide $scriptname`. You can use `<TAB>` for autocompletion

## Example Configuration
```toml
[path]
path="/home/tom/.config/riptide/scripts/"

[info_header]
show_if_fail=true
show_if_ok=true
fail_msg="Fail:"
ok_msg="Ok:"
```
