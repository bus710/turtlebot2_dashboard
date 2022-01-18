#!/bin/bash

set -e

remove_target(){
    term_color_red
    printf "REMOVE TARGET"
    term_color_white

    cargo clean
    rm -rf ./target
}

remove_generated() {
    term_color_red
    printf "REMOVE GENERATED"
    term_color_white

    rm -rf ./dashboard/lib/bridge_generated.dart
}

codegen () {
    term_color_red
    printf "CODE GEN"
    term_color_white

    flutter_rust_bridge_codegen \
        --rust-input lib/turtlebot2/src/api.rs \
        --dart-output dashboard/lib/bridge_generated.dart \
        --llvm-path /usr/lib/llvm-13/lib/libclang.so

}

cargo_make () {
    term_color_red
    printf "CARGO MAKE"
    term_color_white

    cargo make
}

move () {
    term_color_red
    printf "MOVE to DLIB"
    term_color_white

    DLIB="dashboard/dlib"
    rm -rf $DLIB
    mkdir -p $DLIB
    mv target/debug/libturtlebot2.so $DLIB

}

term_color_red () {
    echo -e "\e[91m"
}

term_color_green () {
    echo -e "\e[92m"
}

term_color_white () {
    echo -e "\e[39m"
}

byebye () {
    term_color_green
    printf "READY under dashboard/dlib"
    term_color_white
}

trap term_color_white EXIT
remove_target
remove_generated
codegen
cargo_make
move
remove_target
byebye
