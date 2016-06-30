# qmlrsng

[Qt Quick](http://doc.qt.io/qt-5/qtquick-index.html) bindings for Rust, based
on [libqmlbind](https://github.com/seanchas116/libqmlbind).

The crate `libqmlbind-sys` wraps [libqmlbind](https://github.com/seanchas116/libqmlbind)
C library in Rust and exposes an _unsafe_ API. The goal of `qmlrsng` is to create a safe
abstraction over the `libqmlbind-sys` crate.

**NOTE**: This create is empty for the moment, waiting for a workable `libqmlbind-sys` crate.


## Synopsis

Rust is a fantastic programming language. It exposes high level features that
can be found in, say, Python, while retaining the low level control that C
offers. In addition, the type system and ownership model is a strong ally and
prevents issues that only static anaylizers _might_ detect in C/C++.

C and C++ does offer great libraries though, one of them being [Qt](https://www.qt.io/)
which is increasingly used in the [embedded industry](https://www.qt.io/qt-for-device-creation/),
mostly through Qt Quick and QML. Wrapping Qt in Rust has been tried but is
[quite difficult and painful](http://endl.ch/content/cxx2rust-pains-wrapping-c-rust-example-qt5),
due to the C++ aspect of Qt.

Furthermore, I believe QML is becoming more interesting than Qt for new projects.
It is a framework to create modern applications, on any device from phone to
full car infotainment systems to desktops. Being able to use Qt Quick / QML
from Rust would certainly have a stronger impact than Qt. Additionally, since
code can be written in QML directly, shortcomings of a Rust wrapper could be
overcome by writting some QML, something which would not be possible with Qt.

While Rust has an easy way to interface with C code through its
[Foreign Function Interface (ffi)](https://doc.rust-lang.org/book/ffi.html),
C++ code cannot be used directly. This makes it hard to come up with a Rust
wrapper to QML, see for example [qmlrs](https://github.com/cyndis/qmlrs).

Fortunately, Rust is not the only language wanting to access the power of QML.
[libqmlbind](https://github.com/seanchas116/libqmlbind) was written as a C wrapper
to QML to be used by Ruby. [An issue](https://github.com/cyndis/qmlrs/issues/28)
for `qmlrs` suggested re-writting it over `libqmlbind`; Since I am looking for
excuses to write Rust code, I decided to give it a try.


## Organization

This repository contains two crates. The first one is `libqmlbind-sys` which
simply exposes `libqmlbind`'s C interface as (unsafe) Rust. It's source was
generated using [bindgen](https://github.com/crabtw/rust-bindgen).

The second crate is `qmlrsng` for `qmlrs` "next generation". I liked the `qmlrs`
name but did not wanted to confuse anybody by using `qml-rs` so simply appended `ng`.
I'm open to name suggestions!
This crate should be a safe wrapper for `libqmlbind-sys`. It is empty for now
until the unsafe wrapper is more complete.



## Licensing

The code in these two libraries is dual-licensed under the MIT license and the
Apache License (version 2.0). See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
