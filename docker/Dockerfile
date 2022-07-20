## SPDX-License-Identifier: MIT OR Apache-2.0
##
## Copyright (c) 2017-2022 Andre Richter <andre.o.richter@gmail.com>
## Copyright (c) 2019-2022 Nao Taco <naotaco@gmail.com>
FROM archlinux:base-devel

ARG VCS_REF
ARG GCC_AARCH64=https://developer.arm.com/-/media/Files/downloads/gnu-a/10.3-2021.07/binrel/gcc-arm-10.3-2021.07-aarch64-aarch64-none-elf.tar.xz
ARG GCC_X86_64=https://developer.arm.com/-/media/Files/downloads/gnu-a/10.3-2021.07/binrel/gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf.tar.xz

LABEL org.label-schema.vcs-ref=$VCS_REF \
    org.label-schema.vcs-url="https://github.com/jewlexx/do-not-enter"

LABEL maintainer="Juliette Cordor"

# Ruby gems
COPY Gemfile .

RUN set -ex;                                      \
    tempPkgs='                                    \
    automake                                  \
    bison                                     \
    flex                                      \
    libtool                                   \
    ninja                          \
    pkg-config                                \
    wget                                      \
    ';
RUN pacman -Syyu --noconfirm \
    $tempPkgs                                 \
    # persistent packages
    ca-certificates                           \
    pixman                           \
    glib2 \
    libusb                          \
    python3                                   \
    ruby                                      \
    ;                                             \
    mkdir /tmp/multi \
    chmod 777 /tmp/multi \
    git clone https://aur.archlinux.org/gdb-multiarch.git /tmp/multi \
    export OLD_WD=$PWD \
    cd /tmp/multi \
    makepkg -si --noconfirm \
    cd $OLD_WD \
    # GCC AArch64 tools
    if [ "$(uname -m)" = "aarch64" ]; then wget ${GCC_AARCH64}; else wget ${GCC_X86_64}; fi; \
    tar -xf gcc-arm-10*;                                                                     \
    cp                                                                                       \
    gcc-arm-10*/bin/aarch64-none-elf-objdump                                             \
    gcc-arm-10*/bin/aarch64-none-elf-readelf                                             \
    gcc-arm-10*/bin/aarch64-none-elf-nm                                                  \
    /usr/local/bin/;                                                                     \
    rm -rf gcc-arm-10*;                                                                      \
    # Ruby dependencies
    gem install bundler;                             \
    bundle config set --local without 'development'; \
    bundle install --retry 3;                        \
    # QEMU
    pacman -S qemu-full --noconfirm                                           \
    # Openocd
    git clone --depth 1 https://git.code.sf.net/p/openocd/code openocd; \
    cd openocd;                                                         \
    ./bootstrap;                                                        \
    ./configure --enable-ftdi;                                          \
    make -j10;                                                          \
    make install;                                                       \
    # GDB
    wget -P ~ git.io/.gdbinit;

# Locales
RUN locale-gen en_US.UTF-8

ENV LANG=en_US.UTF-8   \
    LANGUAGE=en_US:en  \
    LC_ALL=en_US.UTF-8 \
    RUBYOPT=-W0

# Openocd
COPY rpi3.cfg /openocd/
COPY rpi4.cfg /openocd/

# GDB
COPY auto /root/.gdbinit.d/auto
