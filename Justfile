default:
  @just --list

TARGET := "aarch64-unknown-uefi"

_cargo COMMAND *ARGS:
    cargo {{COMMAND}} --target {{TARGET}} {{ if ARGS == "" { "" } else { "-- " + ARGS } }}

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
