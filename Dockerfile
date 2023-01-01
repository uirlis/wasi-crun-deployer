FROM ubuntu:20.04 as ubuntu18builder

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

RUN apt-get update

RUN apt-get install -y make git gcc g++ build-essential pkgconf libtool \
    libsystemd-dev libprotobuf-c-dev libcap-dev libseccomp-dev libyajl-dev \
    libgcrypt20-dev go-md2man autoconf python3 automake \
    cmake libboost-all-dev wget libssl-dev rapidjson-dev llvm-12-dev \
    liblld-12-dev software-properties-common

WORKDIR "/"
RUN git clone --depth 1 -b 0.11.2 https://github.com/WasmEdge/WasmEdge.git
WORKDIR "/WasmEdge"
RUN mkdir build
WORKDIR "/WasmEdge/build"

RUN cmake -DCMAKE_BUILD_TYPE=Release -DWASMEDGE_BUILD_TESTS=ON .. && make -j && make install

WORKDIR "/"
RUN git clone --depth 1 --recursive https://github.com/containers/crun.git
WORKDIR /crun
RUN ./autogen.sh
RUN ./configure --with-wasmedge
RUN make
RUN ./crun --version

FROM registry.access.redhat.com/ubi8/ubi as rhel8builder

RUN yum install -y gcc openssl-devel && \
    rm -rf /var/cache/dnf && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

COPY manager /app-build/

WORKDIR "/app-build"

ENV PATH=/root/.cargo/bin:${PATH}

RUN cargo build --release

RUN cargo test --release

FROM registry.access.redhat.com/ubi8/ubi-minimal

WORKDIR "/vendor/ubuntu_20_04"
COPY --from=ubuntu18builder /WasmEdge/build/lib/api/libwasmedge.so.0 /crun/crun /usr/lib/x86_64-linux-gnu/libyajl.so.2 \
    /usr/lib/x86_64-linux-gnu/libLLVM-12.so.1 ./

WORKDIR "/app"
COPY --from=rhel8builder /app-build/target/release/manager ./

RUN /app/manager version

CMD ["/app/manager"]
