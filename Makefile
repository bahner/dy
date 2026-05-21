BIN       := dy
CARGO     := cargo
CROSS     := cross

# ── local builds ──────────────────────────────────────────────────────────────

.PHONY: build
build:
	$(CARGO) build

.PHONY: release
release:
	$(CARGO) build --release

.PHONY: test
test:
	$(CARGO) test

.PHONY: check
check:
	$(CARGO) check
	$(CARGO) clippy -- -D warnings

.PHONY: clean
clean:
	$(CARGO) clean

# ── cross-compilation targets ─────────────────────────────────────────────────

LINUX_GNU_TARGET  := x86_64-unknown-linux-gnu
LINUX_MUSL_TARGET := x86_64-unknown-linux-musl
LINUX_ARM_TARGET  := aarch64-unknown-linux-musl
MACOS_ARM_TARGET  := aarch64-apple-darwin
WIN_TARGET        := x86_64-pc-windows-gnu

.PHONY: build-linux-gnu
build-linux-gnu:
	$(CROSS) build --release --target $(LINUX_GNU_TARGET)

.PHONY: build-linux-musl
build-linux-musl:
	$(CROSS) build --release --target $(LINUX_MUSL_TARGET)

.PHONY: build-linux-arm-musl
build-linux-arm-musl:
	$(CROSS) build --release --target $(LINUX_ARM_TARGET)

.PHONY: build-macos-arm
build-macos-arm:
	$(CARGO) build --release --target $(MACOS_ARM_TARGET)

.PHONY: build-windows
build-windows:
	$(CROSS) build --release --target $(WIN_TARGET)

.PHONY: build-all
build-all: build-linux-gnu build-linux-musl build-linux-arm-musl build-windows
	@echo "Note: macOS ARM target must be built on a macOS host"

# ── install (local) ───────────────────────────────────────────────────────────

PREFIX    ?= /usr/local
BINDIR     = $(PREFIX)/bin

.PHONY: install
install:
	install -d $(BINDIR)
	install -m 755 target/release/$(BIN) $(BINDIR)/$(BIN)

.PHONY: help
help:
	@echo "Targets:"
	@echo "  build               debug build (current host)"
	@echo "  release             optimised build (current host)"
	@echo "  test                run unit tests"
	@echo "  check               cargo check + clippy"
	@echo "  clean               remove build artefacts"
	@echo "  install             install dy to PREFIX/bin  (default: /usr/local/bin)"
	@echo "  build-linux-gnu     cross-compile for x86_64-unknown-linux-gnu"
	@echo "  build-linux-musl    cross-compile for x86_64-unknown-linux-musl"
	@echo "  build-linux-arm-musl cross-compile for aarch64-unknown-linux-musl"
	@echo "  build-macos-arm     build for aarch64-apple-darwin (macOS host only)"
	@echo "  build-windows       cross-compile for x86_64-pc-windows-gnu"
	@echo "  build-all           all non-macOS cross targets"
