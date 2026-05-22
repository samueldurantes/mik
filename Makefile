TARGET = avr-none
MCU = atmega328p
BAUD = 115200
PORT = /dev/cu.usbserial-110

.PHONY: build flash clean

build:
	cargo build --release --example $(EXAMPLE)

flash:
	$(eval EXAMPLE := $(basename $(notdir $(filter-out flash,$(MAKECMDGOALS)))))
	cargo build --release --example $(EXAMPLE)
	avrdude -p $(MCU) -c arduino -P $(PORT) -b $(BAUD) -U flash:w:target/$(TARGET)/release/examples/$(EXAMPLE).elf:e

%:
	@:

clean:
	cargo clean
