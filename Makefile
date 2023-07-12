# Variables
TARGET = riptide
SRC_DIR = .
BASHRC = ~/.bashrc
COMMAND = complete -C __riptide_shell_completion $(TARGET)

# Main Rule 
all: install


# Compilation and installation rule
install:
	cargo install --force --path $(SRC_DIR)
	@if ! grep -qF "$(COMMAND)" $(BASHRC); then \
		echo "Adding shell completion for $(TARGET)..."; \
		echo "$(COMMAND)" >> $(BASHRC); \
		echo "Shell completion added. Restart your shell or run 'source $(BASHRC)' to enable completion."; \
	else \
		echo "Shell completion for $(TARGET) already exists in $(BASHRC). Nothing to do."; \
	fi
	@echo "!!---remember to source env file---!!"
	@mkdir -p ~/.config/riptide
	@echo "[path]" > ~/.config/riptide/config.toml
	@echo "path=\"$(HOME)/my_scripts/\"" >> ~/.config/riptide/config.toml
	@mkdir -p ~/my_scripts
	@echo "#!/bin/bash" > ~/my_scripts/my_script1.sh
	@echo "echo \"Hello From my_script1\"" >> ~/my_scripts/my_script1.sh
	@chmod +x ~/my_scripts/my_script1.sh
	@echo "#!/bin/bash" > ~/my_scripts/my_script2.sh
	@echo "echo \"Hello From my_script2\"" >> ~/my_scripts/my_script2.sh
	@chmod +x ~/my_scripts/my_script2.sh

uninstall:
	cargo uninstall $(TARGET)
	@sed -i '/$(COMMAND)/d' $(BASHRC)
	@echo "Application uninstalled and shell completion removed."

.PHONY: all install uninstall
