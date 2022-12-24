FROM ubuntu:18.04 as ubuntu18builder


RUN apt-get update
RUN apt-get install -y make git gcc g++ build-essential pkgconf libtool \
   libsystemd-dev libprotobuf-c-dev libcap-dev libseccomp-dev libyajl-dev \
   libgcrypt20-dev go-md2man autoconf python3 automake \
    software-properties-common cmake libboost-all-dev wget libssl-dev

RUN wget https://apt.llvm.org/llvm.sh
RUN chmod +x llvm.sh
RUN ./llvm.sh 12 all


RUN wget https://github.com/Kitware/CMake/releases/download/v3.25.1/cmake-3.25.1.tar.gz
RUN tar xvzf cmake-3.25.1.tar.gz
WORKDIR "/cmake-3.25.1"
RUN   ./bootstrap && make  && make install

WORKDIR "/"
RUN git clone --depth 1 --branch llvmorg-12.0.1 https://github.com/llvm/llvm-project llvm-project
RUN mkdir build
WORKDIR "/build"
RUN ls ../llvm-project
RUN cmake -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_PROJECTS=lld -DCMAKE_INSTALL_PREFIX=/usr/local ../llvm-project/llvm
RUN make install

ENV CC=/usr/bin/clang-12
ENV CXX=/usr/bin/clang++-12
RUN add-apt-repository ppa:ubuntu-toolchain-r/test
RUN apt-get update
RUN apt-get install -y gcc-10 g++-10

WORKDIR "/"
RUN git clone --depth 1 -b 0.11.2 https://github.com/WasmEdge/WasmEdge.git
WORKDIR "/WasmEdge"
RUN mkdir build
WORKDIR "/WasmEdge/build"

RUN cmake -DCMAKE_BUILD_TYPE=Release -DWASMEDGE_BUILD_TESTS=ON .. && make -j && make



COPY . /app-build

WORKDIR "/app-build"

ENV PATH=/root/.cargo/bin:${PATH}

# RUN cargo build --release 