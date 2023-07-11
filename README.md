# 🌊 Riptide
cli tool for running scripts 

## Why not symlink?
- to eliminate risk of running specific scripts by accident. For example imagine that you have script that modify recursivly all files in current directory
- less junk in $PATH 

## TODO / current state
- [x] running scripts from predefined path
- [x] autocompletion
- [x] makefile or installation script
- [x] config file
- [ ] support for ~ (home directory)
- [ ] support for script arguments

## How to install
```
make install
```
- make will create config file at `$HOME/.config/riptide`, and scripts file at `$HOME/my_scripts` with 2 examples
## How to remove
```
make uninstall
```
## How it works / How to use
- After running for the first time it create configuration file in .config/riptide/conf.toml and .config/riptide/scripts
- You can specify path in toml file
- run by typing `riptide $scriptname`. You can use `<TAB>` for autocompletion

## Default config file
```toml
[path]
path="/home/$HOME/my_scripts/"
[info_header]
show_if_fail=true
show_if_ok=true
fail_msg="Fail: "
ok_msg="Ok: \n"
```
