FROM debian:buster-slim

ENV TZ="America/Bogota"

RUN apt update

RUN apt install build-essential zlib1g-dev wget -y


RUN apt install curl -y

#Instalación de paquetes base para linux dev
RUN apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev -y
RUN apt install libpq-dev -y #para usar el ORM Diesel
RUN apt install pkg-config -y

#instalacion de RUST
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup install 1.66.0

RUN apt update

#Necesario para compilar SO
RUN rustup override set nightly
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN cargo install bootimage

#Habilita llvm11
RUN rustup component add llvm-tools-preview

RUN apt-get install qemu-system -y
RUN apt-get install qemu -y

RUN rm -rf /var/lib/apt/lists/*

WORKDIR /home/app

ENTRYPOINT ["tail", "-f", "/dev/null"]

