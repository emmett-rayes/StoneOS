build:
    cargo build

clean:
    cargo clean
    rm kernel8.img

image $output="kernel8.img":
    cargo objcopy -- -O binary --strip-all $output

size:
    cargo size -- -A

nm $args="":
    cargo nm -- --demangle --print-size $args | rustfilt

objdump $args="":
    cargo objdump -- -T --disassemble --demangle --section .text --section .rodata $args | rustfilt
