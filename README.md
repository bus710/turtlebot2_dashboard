# turtlebot2_dashboard

A GUI demo to drive Turtle Bot 2. 

The demo consists of Flutter GUI and Rust library. (Thanks to [Flutter-Rust-Bridge](https://github.com/fzyzcjy/flutter_rust_bridge) to make this happen!) The Flutter desktop app shows the state of the robot and the Rust library translates the commands and data to connect the app and the robot. 


<br/>
<br/>

## Prerequisites

```sh
$ dart pub global activate ffigen

$ cargo install cbindgen
$ cargo install cargo-make
$ cargo install flutter_rust_bridge_codegen

$ sudo apt-get install -y 
    libclang-dev \
    libgtk-3-dev \
    libudev-dev \
    ninja-build \
    pkg-config \
    clang \
    cmake \
    git
```

<br/>

## Build rust library

```sh
$ cd src
$ ./build.sh
```
