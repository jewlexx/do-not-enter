FROM alpine:3.15.5

ARG VCS_REF

LABEL org.label-schema.vcs-ref=$VCS_REF \
    org.label-schema.vcs-url="https://github.com/jewlexx/do-not-enter"

LABEL maintainer="Juliette Cordor"

# Ruby gems
COPY Gemfile .

RUN set -ex;                                  \
    apk update;                               \
    apk upgrade;                              \
    apk add                                   \
    automake                                  \
    bison                                     \
    alpine-sdk                                \
    flex                                      \
    git                                       \
    libtool                                   \
    ninja                                     \
    pkgconfig                                 \
    wget                                      \
    qemu-system-aarch64                       \
    ca-certificates                           \
    gdb-multiarch                             \
    pixman                                    \
    glib                                      \
    libusb                                    \
    python3                                   \
    ruby                                      \
    ruby-dev                                  \
    gcc-aarch64-none-elf                      \
    openocd

# Ruby dependencies
RUN gem install bundler;                             \
    bundle config set --local without 'development'; \
    bundle install --retry 3;                        \
    # GDB
    wget -P ~ git.io/.gdbinit

# Locales
ENV MUSL_LOCALE_DEPS cmake make musl-dev gcc gettext-dev libintl
ENV MUSL_LOCPATH /usr/share/i18n/locales/musl

RUN apk add \
    $MUSL_LOCALE_DEPS \
    && wget https://gitlab.com/rilian-la-te/musl-locales/-/archive/master/musl-locales-master.zip \
    && unzip musl-locales-master.zip \
    && cd musl-locales-master \
    && cmake -DLOCALE_PROFILE=OFF -D CMAKE_INSTALL_PREFIX:PATH=/usr . && make && make install \
    && cd .. && rm -r musl-locales-master

# Cleanup
RUN apk del $MUSL_LOCALE_DEPS; \
    rm -rf /var/cache/apk/*


ENV LANG=en_US.UTF-8   \
    LANGUAGE=en_US:en  \
    LC_ALL=en_US.UTF-8 \
    RUBYOPT=-W0

# Openocd
COPY rpi3.cfg /openocd/
COPY rpi4.cfg /openocd/

# GDB
COPY auto /root/.gdbinit.d/auto
