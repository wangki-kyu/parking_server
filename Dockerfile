FROM rust:1.86

RUN apt update
RUN apt install -y protobuf-compiler 
RUN apt install -y build-essential 
RUN apt install -y libssl-dev 
RUN apt install -y pkg-config 
RUN apt install -y cmake 

# protoc 경로를 명시적으로 설정합니다.
ENV PROTOC=/usr/bin/protoc
RUN echo $PROTOC

WORKDIR /app

CMD []