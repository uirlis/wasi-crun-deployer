#! /bin/bash

dnf install -y cmake llvm llvm-devel lld-devel clang git file rpm-build ninja-build boost dpkg-dev
git clone --depth 1 -b 0.11.2 https://github.com/WasmEdge/WasmEdge.git
cd WasmEdge
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DWASMEDGE_BUILD_TESTS=ON .. && make -j && make install

curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -e all

dnf install -y systemd-devel yajl-devel libseccomp-devel pkg-config libgcrypt-devel \
    glibc-static python3-libmount libtool libcap-devel
cd ../../
git clone --depth 1 --recursive https://github.com/containers/crun.git
cd crun
./autogen.sh
./configure --with-wasmedge
make
./crun --version