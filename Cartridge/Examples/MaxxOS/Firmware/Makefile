# MaxxOS cartridge ROM
CART      ?= maxxos
DEVICE    ?= maxx_cart
SIZE      ?= 4kb
REPO_ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../../..)
MAXX      := $(REPO_ROOT)/tools/bin/maxx
ROM       := Cartridge/Examples/MaxxOS/Firmware/Binary/MaxxOS.532

.PHONY: build validate simulate upload info

build:
	python3 $(REPO_ROOT)/Cartridge/Examples/MaxxOS/Firmware/build_maxxos.py

validate: build
	python3 $(REPO_ROOT)/tools/maxx_rom.py validate $(REPO_ROOT)/$(ROM)

simulate: validate
	$(MAXX) simulate $(REPO_ROOT)/$(ROM) --cycles 50000

upload: validate
	python3 $(REPO_ROOT)/tools/picorom_cart.py upload --cart $(CART) --device $(DEVICE) --size $(SIZE)

info:
	python3 $(REPO_ROOT)/tools/picorom_cart.py info --cart $(CART) --device $(DEVICE)