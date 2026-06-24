# UltraMaxx cartridge ROM → PicoROM upload
# Requires: picorom 2.x on PATH (https://github.com/wickerwaka/PicoROM/releases)

CART      ?= ultramaxx
DEVICE    ?= maxx_cart
SIZE      ?= 4kb
REPO_ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../../..)
MAXX      := python3 $(REPO_ROOT)/tools/maxx
ROM       := Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532
BAS       ?= Basic/hello.bas
BAS_OUT   ?= Binary/hello.532

.PHONY: validate compile check test test-rust info upload upload-flash upload-hello simulate list

validate:
	python3 $(REPO_ROOT)/tools/maxx_rom.py validate $(ROM)

BAS_PATH  := Cartridge/Examples/UltraMaxx/Firmware/$(BAS)
BAS_ROM   := Cartridge/Examples/UltraMaxx/Firmware/$(BAS_OUT)

compile:
	$(MAXX) compile $(REPO_ROOT)/$(BAS_PATH) -o $(REPO_ROOT)/$(BAS_ROM)
	$(MAXX) validate $(REPO_ROOT)/$(BAS_ROM)

check:
	$(MAXX) check $(REPO_ROOT)/$(BAS_PATH)

test:
	python3 $(REPO_ROOT)/tools/test_tinybasic_maxx.py
	cargo test --manifest-path $(REPO_ROOT)/tools/maxxbas/Cargo.toml
	$(MAXX) compile $(REPO_ROOT)/$(BAS_PATH) -o $(REPO_ROOT)/$(BAS_ROM)
	$(MAXX) validate $(REPO_ROOT)/$(BAS_ROM)

test-rust:
	cargo test --manifest-path $(REPO_ROOT)/tools/maxxbas/Cargo.toml

list:
	$(MAXX) list $(REPO_ROOT)/$(BAS_ROM)

simulate:
	$(MAXX) simulate $(REPO_ROOT)/$(BAS_ROM)

upload-hello: compile
	$(MAXX) upload $(REPO_ROOT)/$(BAS_PATH) --device $(DEVICE) --size $(SIZE)

info:
	python3 $(REPO_ROOT)/tools/picorom_cart.py info --cart $(CART) --device $(DEVICE)

upload: validate
	python3 $(REPO_ROOT)/tools/picorom_cart.py upload --cart $(CART) --device $(DEVICE) --size $(SIZE)

upload-flash: validate
	python3 $(REPO_ROOT)/tools/picorom_cart.py upload --cart $(CART) --device $(DEVICE) --size $(SIZE) --persist