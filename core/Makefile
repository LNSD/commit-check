.DEFAULT_GOAL := build

## Tools
CARGO ?= cargo
CARGO_TARGET_DIR ?= ../target

WASM_TOOLS ?= wasm-tools
WIT_DEPS ?= wit-deps
WASMTIME ?= wasmtime


## Config
PKG_NAME := commit-check-core
PKG_TARGET := wasm32-wasi
PKG_SRC := $(shell find src -type f -name '*.rs')

PKG_OUTPUT_DIR := $(CARGO_TARGET_DIR)/$(PKG_TARGET)/debug

# Wasmtime WASI snapshot preview1 adapters for v16.0.0 (commit 6613acd)
# https://github.com/bytecodealliance/wasmtime/releases/tag/v16.0.0
ADAPTER_WASI_PREVIEW1_COMMAND := adapters/6613acd/wasi_snapshot_preview1.command.wasm
ADAPTER_WASI_PREVIEW1_REACTOR := adapters/6613acd/wasi_snapshot_preview1.reactor.wasm


## Tasks
.PHONY: build run clean

$(PKG_OUTPUT_DIR)/core.wasm: $(PKG_SRC)
	$(CARGO) build --package $(PKG_NAME) --target $(PKG_TARGET) --bin core

$(PKG_OUTPUT_DIR)/core.component.wasm: $(PKG_OUTPUT_DIR)/core.wasm
	$(WASM_TOOLS) component new $< --output $@ \
		--adapt wasi_snapshot_preview1=$(ADAPTER_WASI_PREVIEW1_COMMAND)

build: $(PKG_OUTPUT_DIR)/core.component.wasm

run: $(PKG_OUTPUT_DIR)/core.component.wasm
	$(WASMTIME) --wasm component-model $(PKG_OUTPUT_DIR)/core.component.wasm

clean:
	$(CARGO) clean --package $(PKG_NAME) --target $(PKG_TARGET)
	rm -f $(PKG_OUTPUT_DIR)/$(PKG_BIN).component.wasm
