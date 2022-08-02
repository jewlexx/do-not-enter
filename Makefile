DOCKER_IMAGE := jewelexx/do-not-enter-builder:latest

define color_header
    @tput setaf 6 2> /dev/null || true
    @printf '\n%s\n' $(1)
    @tput sgr0 2> /dev/null || true
endef

define color_progress_prefix
    @tput setaf 2 2> /dev/null || true
    @tput bold 2 2> /dev/null || true
    @printf '%12s ' $(1)
    @tput sgr0 2> /dev/null || true
endef


# Default to the RPi3.
BSP ?= rpi3

# Default to a serial device name that is common in Linux.
DEV_SERIAL ?= /dev/ttyUSB0

##--------------------------------------------------------------------------------------------------
## BSP-specific configuration values
##--------------------------------------------------------------------------------------------------
QEMU_MISSING_STRING = "This board is not yet supported for QEMU."

TARGET            = aarch64-unknown-none-softfloat
KERNEL_BIN        = kernel8.img
QEMU_BINARY       = qemu-system-aarch64
QEMU_RELEASE_ARGS = -serial stdio -display none
GEMU_RELEASE_ARGS = -serial stdio
OBJDUMP_BINARY    = aarch64-none-elf-objdump
NM_BINARY         = aarch64-none-elf-nm
READELF_BINARY    = aarch64-none-elf-readelf
LD_SCRIPT_PATH    = $(shell pwd)/src/bsp/raspberrypi
RUSTC_MISC_ARGS   = -C target-cpu=cortex-a53

ifeq ($(BSP),rpi3)
    QEMU_MACHINE_TYPE = raspi3
else ifeq ($(BSP),rpi4)
    QEMU_MACHINE_TYPE =
endif

# Export for build.rs.
export LD_SCRIPT_PATH



##--------------------------------------------------------------------------------------------------
## Targets and Prerequisites
##--------------------------------------------------------------------------------------------------
KERNEL_MANIFEST      = Cargo.toml
KERNEL_LINKER_SCRIPT = kernel.ld
LAST_BUILD_CONFIG    = target/$(BSP).build_config

KERNEL_ELF      = target/$(TARGET)/debug/kernel
KERNEL_ELF_PROD      = target/$(TARGET)/release/kernel
# This parses cargo's dep-info file.
# https://doc.rust-lang.org/cargo/guide/build-cache.html#dep-info-files
KERNEL_ELF_DEPS = $(filter-out %: ,$(file < $(KERNEL_ELF).d)) $(KERNEL_MANIFEST) $(LAST_BUILD_CONFIG)



##--------------------------------------------------------------------------------------------------
## Command building blocks
##--------------------------------------------------------------------------------------------------
RUSTFLAGS = $(RUSTC_MISC_ARGS)                   \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=$(KERNEL_LINKER_SCRIPT)

# Disabled
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) \
    -D warnings                   \
    -D missing_docs

FEATURES      = --features bsp_$(BSP)
COMPILER_ARGS = --target=$(TARGET) \
    $(FEATURES)

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
CHECK_CMD 	= cargo check $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

EXEC_QEMU          = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)
EXEC_TEST_MINIPUSH = ruby tests/chainboot_test.rb
EXEC_MINIPUSH      = ruby chainboot/minipush.rb

##------------------------------------------------------------------------------
## Dockerization
##------------------------------------------------------------------------------
DOCKER_CMD            = docker run -t --rm -v $(shell pwd):/work/tutorial -w /work/tutorial
DOCKER_CMD_INTERACT   = $(DOCKER_CMD) -i
DOCKER_ARG_DIR_COMMON = -v $(shell pwd)/../common:/work/common
DOCKER_ARG_DEV        = --privileged -v /dev:/dev

# DOCKER_IMAGE defined in include file (see top of this file).
DOCKER_QEMU  = $(DOCKER_CMD_INTERACT) $(DOCKER_IMAGE)
DOCKER_TOOLS = $(DOCKER_CMD) $(DOCKER_IMAGE)
DOCKER_TEST  = $(DOCKER_CMD) $(DOCKER_ARG_DIR_COMMON) $(DOCKER_IMAGE)

# Dockerize commands, which require USB device passthrough, only on Linux.
ifeq ($(shell uname -s),Linux)
    DOCKER_CMD_DEV = $(DOCKER_CMD_INTERACT) $(DOCKER_ARG_DEV)

    DOCKER_CHAINBOOT = $(DOCKER_CMD_DEV) $(DOCKER_ARG_DIR_COMMON) $(DOCKER_IMAGE)
endif



##--------------------------------------------------------------------------------------------------
## Targets
##--------------------------------------------------------------------------------------------------
.PHONY: build doc qemu chainboot clippy clean readelf objdump nm check

build: clean $(KERNEL_BIN)

check:
	$(call color_header, "Checking for cargo-clippy warnings")
	$(call color_progress_prefix, "cargo clippy")
	@$(CLIPPY_CMD)
	$(call color_progress_prefix, "cargo check")
	@$(CHECK_CMD)
	$(call color_progress_prefix, "cargo doc")
	@$(DOC_CMD)

##------------------------------------------------------------------------------
## Save the configuration as a file, so make understands if it changed.
##------------------------------------------------------------------------------
$(LAST_BUILD_CONFIG):
	@rm -f target/*.build_config
	@mkdir -p target
	@touch $(LAST_BUILD_CONFIG)

##------------------------------------------------------------------------------
## Compile the kernel ELF
##------------------------------------------------------------------------------
$(KERNEL_ELF): $(KERNEL_ELF_DEPS)
	$(call color_header, "Compiling kernel ELF - $(BSP)")
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(KERNEL_ELF_PROD): $(KERNEL_ELF_DEPS)
	$(call color_header, "Compiling kernel ELF for production - $(BSP)")
	@RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD) --release

bloat: $(KERNEL_ELF_DEPS)
	$(call color_header, "Checking kernel ELF for float - $(BSP)")
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" cargo bloat $(COMPILER_ARGS_PROD)

##------------------------------------------------------------------------------
## Generate the stripped kernel binary
##------------------------------------------------------------------------------
$(KERNEL_BIN): $(KERNEL_ELF)
	$(call color_header, "Generating stripped binary")
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	$(call color_progress_prefix, "Name")
	@echo $(KERNEL_BIN)
	$(call color_progress_prefix, "Size")
	@printf '%s KiB\n' `du -k $(KERNEL_BIN) | cut -f1`

$(KERNEL_BIN)-prod: $(KERNEL_ELF_PROD)
	$(call color_header, "Generating stripped binary")
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	$(call color_progress_prefix, "Name")
	@echo $(KERNEL_BIN)
	$(call color_progress_prefix, "Size")
	@printf '%s KiB\n' `du -k $(KERNEL_BIN) | cut -f1`

##------------------------------------------------------------------------------
## Generate the documentation
##------------------------------------------------------------------------------
doc:
	$(call color_header, "Generating docs")
	@$(DOC_CMD) --document-private-items --open

##------------------------------------------------------------------------------
## Run the kernel in QEMU
##------------------------------------------------------------------------------
ifeq ($(QEMU_MACHINE_TYPE),) # QEMU is not supported for the board.

qemu qemuasm:
	$(call color_header, "$(QEMU_MISSING_STRING)")

else # QEMU is supported.

qemu: $(KERNEL_BIN)
	$(call color_header, "Launching QEMU")
	@$(DOCKER_QEMU) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)

qemu-prod: $(KERNEL_BIN)-prod
	$(call color_header, "Launching QEMU production mode")
	@$(DOCKER_QEMU) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)

gqemu: $(KERNEL_BIN)
	$(call color_header, "Launching QEMU")
	@$(QEMU_BINARY) -M raspi3b $(GEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)

qemuasm: $(KERNEL_BIN)
	$(call color_header, "Launching QEMU with ASM output")
	@$(DOCKER_QEMU) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN) -d in_asm

endif

##------------------------------------------------------------------------------
## Push the kernel to the real HW target
##------------------------------------------------------------------------------
chainboot: $(KERNEL_BIN)
	@$(DOCKER_CHAINBOOT) $(EXEC_MINIPUSH) $(DEV_SERIAL) $(KERNEL_BIN)

##------------------------------------------------------------------------------
## Run clippy
##------------------------------------------------------------------------------
clippy:
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

##------------------------------------------------------------------------------
## Clean
##------------------------------------------------------------------------------
clean:
	rm -rf $(KERNEL_BIN)

##------------------------------------------------------------------------------
## Run readelf
##------------------------------------------------------------------------------
readelf: $(KERNEL_ELF)
	$(call color_header, "Launching readelf")
	@$(DOCKER_TOOLS) $(READELF_BINARY) --headers $(KERNEL_ELF)

##------------------------------------------------------------------------------
## Run objdump
##------------------------------------------------------------------------------
objdump: $(KERNEL_ELF)
	$(call color_header, "Launching objdump")
	@$(DOCKER_TOOLS) $(OBJDUMP_BINARY) --disassemble --demangle \
                --section .text   \
                --section .rodata \
                $(KERNEL_ELF) | rustfilt

##------------------------------------------------------------------------------
## Run nm
##------------------------------------------------------------------------------
nm: $(KERNEL_ELF)
	$(call color_header, "Launching nm")
	@$(DOCKER_TOOLS) $(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort | rustfilt



##--------------------------------------------------------------------------------------------------
## Testing targets
##--------------------------------------------------------------------------------------------------
.PHONY: test test_boot

ifeq ($(QEMU_MACHINE_TYPE),) # QEMU is not supported for the board.

test_boot test:
	$(call color_header, "$(QEMU_MISSING_STRING)")

else # QEMU is supported.

##------------------------------------------------------------------------------
## Run boot test
##------------------------------------------------------------------------------
test_boot: $(KERNEL_BIN)
	$(call color_header, "Boot test - $(BSP)")
	@$(DOCKER_TEST) $(EXEC_TEST_MINIPUSH) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) \
		-kernel $(KERNEL_BIN) $(CHAINBOOT_DEMO_PAYLOAD)

test: test_boot

endif