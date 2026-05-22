TARGET = avr-none
MCU = atmega328p
BAUD = 115200
PORT = /dev/cu.usbserial-110
BIN = mik

ELF = target/$(TARGET)/release/$(BIN).elf
HEX = $(BIN).hex

.PHONY: build hex flash clean

build:
	cargo build --release

hex: build
	avr-objcopy -O ihex $(ELF) $(HEX)

flash: hex
	avrdude -p $(MCU) -c arduino -P $(PORT) -b $(BAUD) -U flash:w:$(HEX):i

clean:
	cargo clean
	rm -f $(HEX)
