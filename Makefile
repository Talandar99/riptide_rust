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
	@echo "!!---remember to source env file---!!"
	@if ! grep -qF "$(COMMAND)" $(BASHRC); then \
		echo "Adding shell completion for $(TARGET)..."; \
		echo "$(COMMAND)" >> $(BASHRC); \
		echo "Shell completion added. Restart your shell or run 'source $(BASHRC)' to enable completion."; \
	else \
		echo "Shell completion for $(TARGET) already exists in $(BASHRC). Nothing to do."; \
	fi

uninstall:
	cargo uninstall $(TARGET)
	@sed -i '/$(COMMAND)/d' $(BASHRC)
	@echo "Application uninstalled and shell completion removed."

.PHONY: all install uninstall
