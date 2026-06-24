# UltraMaxx cartridge ROM → PicoROM upload
# Requires: picorom 2.x on PATH (https://github.com/wickerwaka/PicoROM/releases)

CART      ?= ultramaxx
DEVICE    ?= maxx_cart
SIZE      ?= 4kb
REPO_ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../../..)
ROM       := Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532
BAS       ?= Basic/hello.bas
BAS_OUT   ?= Binary/hello.532

MAXXBAS   := $(REPO_ROOT)/tools/maxxbas/target/release/maxxbas

.PHONY: validate compile compile-rust check test-rust info upload upload-flash upload-hello

validate:
	python3 $(REPO_ROOT)/tools/maxx_rom.py validate $(ROM)

BAS_PATH  := Cartridge/Examples/UltraMaxx/Firmware/$(BAS)
BAS_ROM   := Cartridge/Examples/UltraMaxx/Firmware/$(BAS_OUT)

compile:
	python3 $(REPO_ROOT)/tools/tinybasic_maxx.py compile $(BAS_PATH) -o $(BAS_ROM)
	python3 $(REPO_ROOT)/tools/maxx_rom.py validate $(BAS_ROM)

compile-rust:
	cargo build --release --manifest-path $(REPO_ROOT)/tools/maxxbas/Cargo.toml
	$(MAXXBAS) compile $(REPO_ROOT)/$(BAS_PATH) -o $(REPO_ROOT)/$(BAS_ROM)
	python3 $(REPO_ROOT)/tools/maxx_rom.py validate $(BAS_ROM)

check:
	python3 $(REPO_ROOT)/tools/tinybasic_maxx.py check $(BAS_PATH)

test-rust:
	cargo test --manifest-path $(REPO_ROOT)/tools/maxxbas/Cargo.toml

upload-hello: compile
	python3 $(REPO_ROOT)/tools/picorom_cart.py upload --device $(DEVICE) --size $(SIZE) --rom $(BAS_ROM)

info:
	python3 $(REPO_ROOT)/tools/picorom_cart.py info --cart $(CART) --device $(DEVICE)

upload: validate
	python3 $(REPO_ROOT)/tools/picorom_cart.py upload --cart $(CART) --device $(DEVICE) --size $(SIZE)

upload-flash: validate
	python3 $(REPO_ROOT)/tools/picorom_cart.py upload --cart $(CART) --device $(DEVICE) --size $(SIZE) --persist