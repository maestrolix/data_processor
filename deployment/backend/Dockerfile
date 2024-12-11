FROM rust:latest

RUN apt-get update
RUN apt install -y libavutil-dev
RUN apt install -y libavformat-dev
RUN apt install -y libavfilter-dev
RUN apt install -y libavdevice-dev
RUN apt-get install -y clang
RUN apt-get install -y cmake

WORKDIR /data_processor
COPY . .


WORKDIR /data_processor/backend

RUN cargo build
