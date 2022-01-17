#!/bin/bash

set -e

remove_target(){
    term_color_red
    echo "REMOVE TARGET"
    term_color_white

    rm -rf ./target
}

remove_generated() {
    term_color_red
    echo "REMOVE GENERATED"
    term_color_white

    rm -rf ./dashboard/lib/bridge_generated.dart
}

codegen () {
    term_color_red
    echo "CODE GEN"
    term_color_white

    flutter_rust_bridge_codegen \
        --rust-input lib/adapter/src/api.rs \
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
    term_color_red
    echo "MOVE to DLIB"
    term_color_white

    DLIB="dashboard/dlib"
    rm -rf $DLIB
    mkdir -p $DLIB
    mv target/debug/libadapter.so $DLIB

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
    echo "The library is ready under DLIB"
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
