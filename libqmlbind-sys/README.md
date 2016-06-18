# libqmlbind-sys

Unsafe Rust binding to [libqmlbind](https://github.com/seanchas116/libqmlbind/).


# Initial Generation

[bindgen](https://github.com/crabtw/rust-bindgen) is used:

```sh
sed -ibak 's|.*#include <stdlib.h>|// #include <stdlib.h>|' path/to/libqmlbind.git/qmlbind/include/qmlbind/value.h
bindgen --builtins path/to/libqmlbind.git/qmlbind/include/qmlbind.h > src/lib.rs
sed -ibak 's|.*#include <stdlib.h>|#include <stdlib.h>|' path/to/libqmlbind.git/qmlbind/include/qmlbind/value.h
```
