FROM debian:bookworm

COPY cmd/.bash /root/.bash
RUN rm -rf ~/.bashrc && mv ~/.bash ~/.bashrc

RUN apt-get update && apt-get install -y build-essential git curl wget \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN BIN="/usr/local/bin" \
    && if [ "$(uname -m)" = "aarch64" ]; then \
        UPX_ARCH="arm64"; \
    else \
        UPX_ARCH="amd64"; \
    fi \
    && UPX_TAR="upx-4.2.4-${UPX_ARCH}_linux.tar.xz" \
    && wget -O ${BIN}/${UPX_TAR} --no-check-certificate https://github.com/upx/upx/releases/download/v4.2.4/${UPX_TAR} \
    && tar xf ${BIN}/${UPX_TAR} -C ${BIN}

RUN apt-get install -y llvm \
    && rustup target add aarch64-unknown-none

ENV FLASK_APP=rust_app
EXPOSE 8000
ENTRYPOINT flask run --host 0.0.0.0 --port 8000