FROM debian:trixie AS libasm_builder

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    nasm make binutils \
    && rm -rf /tmp/* /var/tmp/* /var/lib/apt/lists/*;

WORKDIR /src
COPY ./Makefile ./Filelist.mk 	/src
COPY ./src/ ./src

RUN make bonus;

FROM rust:trixie AS tester_builder
WORKDIR /src/tester
COPY 						./tester			/src/tester

COPY --from=libasm_builder	/src/libasm_bonus.a	/src/libasm.a
COPY --from=libasm_builder	/src/libasm_bonus.a	/src/libasm_bonus.a

CMD [ "cargo", "test", "--offline", "--tests", "--no-fail-fast", "--color=always", "--", "--color=always" ]

