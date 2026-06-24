# UltraMaxx cartridge ROM → PicoROM upload
# Requires: picorom 2.x on PATH (https://github.com/wickerwaka/PicoROM/releases)

CART      ?= ultramaxx
DEVICE    ?= maxx_cart
SIZE      ?= 4kb
REPO_ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../../..)
ROM       := Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532

.PHONY: validate info upload upload-flash

validate:
	python3 $(REPO_ROOT)/tools/maxx_rom.py validate $(ROM)

info:
	python3 $(REPO_ROOT)/tools/picorom_cart.py info --cart $(CART) --device $(DEVICE)

upload: validate
	python3 $(REPO_ROOT)/tools/picorom_cart.py upload --cart $(CART) --device $(DEVICE) --size $(SIZE)

upload-flash: validate
	python3 $(REPO_ROOT)/tools/picorom_cart.py upload --cart $(CART) --device $(DEVICE) --size $(SIZE) --persist