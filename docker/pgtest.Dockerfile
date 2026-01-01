FROM postgres:17

USER root

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        clang \
        curl \
        git \
        llvm-dev \
        libclang-dev \
        build-essential \
        pkg-config \
        bison \
        flex \
        libicu-dev \
        libreadline-dev \
        libssl-dev \
        libxml2-dev \
        libxslt1-dev \
        liblz4-dev \
        libzstd-dev \
        zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
ENV USER=postgres
ENV LOGNAME=postgres
ENV HOME=/var/lib/postgresql

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
RUN cargo install cargo-pgrx --version 0.16.1 --locked

WORKDIR /workspace
COPY . /workspace
RUN chown -R postgres:postgres /workspace /usr/local/rustup /usr/local/cargo

USER postgres
RUN cargo pgrx init --pg17 download

CMD ["bash", "-lc", "env -u PG_VERSION -u PG_MAJOR /usr/local/cargo/bin/cargo pgrx test -p pg_debyte_ext --features pg17"]
