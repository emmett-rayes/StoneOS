// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2022 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2023-2024 Emmett Rayes <emmett.rayes@icloud.com>


//----------------------------------------------------------------------------------------
// Boot Routine
//----------------------------------------------------------------------------------------
.section .text.boot
boot:
    // Only proceed on the boot core. Park it otherwise.
    mrs	x0, MPIDR_EL1
    ldr x1, CORE_ID_MASK        // provided by arch/aarch64/boot.rs
    and	x0, x0, x1
    ldr	x1, BOOT_CORE_ID        // provided by arch/aarch64/boot.rs
    cmp	x0, x1
    b.ne	.L_parking_loop

	// Initialize DRAM.
	adr	x0, __bss_start
	adr x1, __bss_end_exclusive

.L_bss_init_loop:
	cmp	x0, x1
	b.eq	.L_prepare_rust
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop

.L_prepare_rust:
	// Set the stack pointer.
	adr	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

	// Jump to kernel code.
	b	start_kernel

.L_parking_loop:
	wfe
	b	.L_parking_loop

.size	boot, . - boot
.type	boot, function
.global	boot
