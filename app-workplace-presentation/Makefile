.PHONY: setup dev build icons audit clean help

help:
	@echo "Workplace Presentation — Makefile targets"
	@echo ""
	@echo "  make setup    Install dependencies, download fonts and Paged.js, embed fonts"
	@echo "  make dev      Launch the development build with hot reload"
	@echo "  make build    Produce a production bundle for the current platform"
	@echo "  make icons    Regenerate platform icons from src-tauri/icons/icon-source.png"
	@echo "  make audit    Run cargo audit and npm audit"
	@echo "  make clean    Remove build artefacts and node_modules"
	@echo ""
	@echo "First-time setup on Linux Mint 22: see DEVELOPMENT.md for system packages"
	@echo "and the webkit 4.0 -> 4.1 pkg-config shim."

setup:
	npm install
	@if [ -f scripts/download-deps.sh ]; then bash scripts/download-deps.sh; else echo "scripts/download-deps.sh not yet present — skipping"; fi
	@if [ -f scripts/embed-fonts.sh ]; then bash scripts/embed-fonts.sh; else echo "scripts/embed-fonts.sh not yet present — skipping"; fi

dev:
	npm run tauri dev

build:
	npm run tauri build

icons:
	@if [ ! -f src-tauri/icons/icon-source.png ]; then \
		echo "Generating placeholder gold icon at src-tauri/icons/icon-source.png"; \
		convert -size 1024x1024 xc:'#c8a96e' src-tauri/icons/icon-source.png; \
	fi
	npx tauri icon src-tauri/icons/icon-source.png

audit:
	cargo audit --manifest-path src-tauri/Cargo.toml || echo "(install cargo-audit with: cargo install cargo-audit)"
	npm audit || true

clean:
	rm -rf src-tauri/target
	rm -rf node_modules
	rm -rf fonts
	rm -f src/js/font-data.js
	rm -f src/js/vendor/paged.polyfill.js
