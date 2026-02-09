FROM debian:trixie AS libasm_builder

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    nasm make binutils \
    && rm -rf /tmp/* /var/tmp/* /var/lib/apt/lists/*;

WORKDIR /src
COPY ./Makefile ./Filelist.mk 	/src
COPY ./src/ ./src

RUN make all;

FROM rust:trixie AS tester_builder
WORKDIR /src/tester
COPY 						./tester		/src/tester
COPY --from=libasm_builder	/src/libasm.a	/src

CMD [ "cargo", "test", "--release" ]

