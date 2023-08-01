# 🌊 Riptide 
Simple cli tool for running scripts 

## How it works / How to use
- Add scripts to `~/my_scripts` directory (or other directory specified in `$HOME/.config/riptide`)
- run by typing `riptide $SCRIPT_NAME`. You can use `<TAB>` for autocompletion
- example:
```bash
~> riptide my_script2.sh
# output
Hello From my_script2
```

## Why not symlink?
- to eliminate risk of running specific scripts by accident. For example imagine that you have script that modify recursivly all files in current directory
- less junk in $PATH 
## Requirements
- Rust
- Bash (not tested on other shells)
- [bash-completion](https://github.com/scop/bash-completion) **(optional)** (install for autocomplete)
## How to install
#### - basic install
```
make install
```
#### - install with alias (typing `rt` insted of `riptide` is faster but check first if it's not in conflict with other command)
```
make install-with-alias
```
#### !!note!! <br/>make will create config file at `$HOME/.config/riptide`, and scripts file at `$HOME/my_scripts` with 2 examples
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
- [ ] support for script arguments
- [ ] remote execution (similar to ansible but simpler)
- [ ] support for multiple paths
- [ ] support for ~ (home directory)
- [ ] support for other shells 
    - [ ] ash
    - [ ] fish
    - [ ] nush
