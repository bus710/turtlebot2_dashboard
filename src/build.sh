#!/bin/bash

set -e

remove_all(){
    rm -rf ./target
}

codegen () {
    term_color_red
    echo "CODE GEN"
    term_color_white

    flutter_rust_bridge_codegen \
        --rust-input lib/turtlebot2/src/api.rs \
        --dart-output dashboard/lib/bridge_generated.dart \
        --llvm-path /usr/lib/llvm-13/lib/libclang.so

    echo
}

cargo_make () {
    term_color_red
    echo "CARGO MAKE"
    term_color_white

    cargo make
}

move () {
    DLIB="dashboard/dlib"
    rm -rf $DLIB
    mkdir -p $DLIB
    mv target/debug/libturtlebot2.so $DLIB

}

term_color_red () {
    echo -e "\e[91m"
    echo
}

term_color_white () {
    echo
    echo -e "\e[39m"
}

byebye () {
    term_color_red
    echo "the library is ready"
    term_color_white
}

trap term_color_white EXIT
remove_all
codegen
cargo_make
move
remove_all
byebye
