# Variables
APP_NAME = riptide
APP_ALIAS = rt
SRC_DIR = .
BASHRC = ~/.bashrc
COMMAND = complete -C __riptide_shell_completion $(APP_NAME)
COMMAND_WITH_ALIAS = complete -C __riptide_shell_completion $(APP_ALIAS)


help:
	@echo "install - install riptide"
	@echo "install-with-alias - install riptide with rt alias"
	@echo "uninstall - uninstall riptide"

# Compilation and installation rule
install:
	cargo install --force --path $(SRC_DIR)
	@echo "!!---------------------------------!!"
	@echo "!!---remember to source env file---!!"
	@echo "!!---------------------------------!!"
	@if ! grep -qF "$(COMMAND)" $(BASHRC); then \
		echo "Adding shell completion for $(APP_NAME)..."; \
		echo "$(COMMAND)" >> $(BASHRC); \
		echo "Shell completion added. Restart your shell or run 'source $(BASHRC)' to enable completion."; \
	else \
		echo "Shell completion for $(APP_NAME) already exists in $(BASHRC). Nothing to do."; \
	fi
	@mkdir -p ~/.config/riptide
	@echo "[path]" > ~/.config/riptide/config.toml
	@echo "path=\"$(HOME)/scripts/\"" >> ~/.config/riptide/config.toml
	@echo "[editor]" >> ~/.config/riptide/config.toml
	@echo "editor=\"vim\"" >> ~/.config/riptide/config.toml
	@mkdir -p ~/scripts
	@echo "#!/bin/bash" > ~/scripts/my_script.sh
	@echo "" >> ~/scripts/my_script.sh
	@echo "echo \"Hello From my_script\"" >> ~/scripts/my_script.sh
	@echo "" >> ~/scripts/my_script.sh
	@chmod +x ~/scripts/my_script.sh

	@echo "#!/bin/bash" > ~/scripts/echo_script_args.sh
	@echo "" >> ~/scripts/echo_script_args.sh
	@echo "echo \"Hello From echo_script_args\"" >> ~/scripts/echo_script_args.sh
	@echo "" >> ~/scripts/echo_script_args.sh
	@echo "for arg in \"\$$@\"; do" >> ~/scripts/echo_script_args.sh
	@echo "echo \"\$$arg\"" >> ~/scripts/echo_script_args.sh
	@echo "done" >> ~/scripts/echo_script_args.sh
	@echo "" >> ~/scripts/echo_script_args.sh
	@chmod +x ~/scripts/echo_script_args.sh
	@echo "#!/bin/bash" > ~/scripts/make_dir.sh
	@echo "mkdir $1" >> ~/scripts/make_dir.sh
	@chmod +x ~/scripts/make_dir.sh
	echo "Adding manpage for $(APP_ALIAS)...";
	@sudo cp manpage/riptide.man /usr/share/man/man1/riptide.1
	 
    

install-with-alias: install
	@if ! grep -qF "alias $(APP_ALIAS)=" $(BASHRC); then \
		echo "Adding alias $(APP_ALIAS)=$(APP_NAME)..."; \
		echo "alias $(APP_ALIAS)=$(APP_NAME)" >> $(BASHRC); \
	else \
		echo "Alias $(APP_ALIAS)=$(APP_NAME) already exists in $(BASHRC). Nothing to do."; \
	fi
	@if ! grep -qF "$(COMMAND_WITH_ALIAS)" $(BASHRC); then \
		echo "Adding shell completion for $(APP_ALIAS)..."; \
		echo "$(COMMAND_WITH_ALIAS)" >> $(BASHRC); \
		echo "Shell completion added. Restart your shell or run 'source $(BASHRC)' to enable completion."; \
	else \
		echo "Shell completion for $(APP_ALIAS) already exists in $(BASHRC). Nothing to do."; \
	fi

uninstall:
	cargo uninstall $(APP_NAME)
	@sed -i '/$(COMMAND)/d' $(BASHRC)
	@sed -i '/alias rt=riptide/d' $(BASHRC)
	@sed -i '/$(COMMAND_WITH_ALIAS)/d' $(BASHRC)
	@echo "Application uninstalled and shell completion removed."
	echo "Removing manpage for $(APP_ALIAS)...";
	@sudo rm /usr/share/man/man1/riptide.1

.PHONY: all install uninstall install-with-alias
