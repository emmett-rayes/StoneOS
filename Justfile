default:
  @just --list

TARGET := "aarch64-unknown-uefi"
TARGET_DIR := `cargo metadata --format-version 1 --no-deps | jq -r '.target_directory'`
BIN_NAME := `cargo metadata --format-version 1 --no-deps | jq -r '.packages[].targets[] | select( .kind | map(. == "bin") | any ) | .name'`
BINARY_PATH := TARGET_DIR + "/" + TARGET + "/debug/" + BIN_NAME + ".efi"

_cargo COMMAND *ARGS:
    cargo {{COMMAND}} --target {{TARGET}} {{ if ARGS == "" { "" } else { "-- " + ARGS } }}

deps:
    cargo install trebuchet --git https://github.com/Stone-OS/Trebuchet.git

build:
    @just _cargo build

clean:
    @just _cargo clean
    -rm kernel8.img

image OUTPUT="kernel8.img":
    @just _cargo objcopy -O binary --strip-all {{OUTPUT}}

size:
    @just _cargo size -A

nm *ARGS:
    @just _cargo nm --demangle --print-size {{ARGS}} | rustfilt

objdump *ARGS:
    @just _cargo objdump -T --disassemble --demangle --section .text --section .rodata {{ARGS}} | rustfilt

trebuchet SERIAL="/dev/tty.usbserial-0001" BAUD="115200": build
    trebuchet {{SERIAL}} {{BAUD}} {{BINARY_PATH}}
