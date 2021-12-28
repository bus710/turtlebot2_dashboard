#!/bin/bash

set -e

hide_main(){
    term_color_red
    echo "HIDE MAIN"
    term_color_white

    mv lib/turtlebot2/src/main.rs .
}

recover_main(){
    term_color_red
    echo "RECOVER MAIN"
    term_color_white

    mv ./main.rs lib/turtlebot2/src
}

remove_all(){
    term_color_red
    echo "REMOVE TARGET"
    term_color_white

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
    term_color_red
    echo "MOVE to DLIB"
    term_color_white

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
    echo "The library is ready under DLIB"
    term_color_white
}

trap term_color_white EXIT
hide_main
remove_all
codegen
cargo_make
move
remove_all
recover_main
byebye
