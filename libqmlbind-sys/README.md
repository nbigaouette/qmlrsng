# libqmlbind-sys

Unsafe Rust binding to [libqmlbind](https://github.com/seanchas116/libqmlbind/).


# Initial Generation

[bindgen](https://github.com/crabtw/rust-bindgen) is used:

```sh
sed -ibak 's|.*#include <stdlib.h>|// #include <stdlib.h>|' path/to/libqmlbind.git/qmlbind/include/qmlbind/value.h
bindgen --builtins path/to/libqmlbind.git/qmlbind/include/qmlbind.h > src/lib.rs
sed -ibak 's|.*#include <stdlib.h>|#include <stdlib.h>|' path/to/libqmlbind.git/qmlbind/include/qmlbind/value.h
```

Build exmple:
```
cargo build --example main
```

rpath problem: `LC_RPATH` missing?
```
otool -l target/debug/examples/main | grep LC_RPATH
```

Fix Qt linking:
```
install_name_tool -change @rpath/QtCore.framework/Versions/5/QtCore /Users/nicolas/Qt/5.7/clang_64/lib/QtCore.framework/Versions/5/QtCore target/debug/examples/main
install_name_tool -change @rpath/QtNetwork.framework/Versions/5/QtNetwork /Users/nicolas/Qt/5.7/clang_64/lib/QtNetwork.framework/Versions/5/QtNetwork target/debug/examples/main
install_name_tool -change @rpath/QtGui.framework/Versions/5/QtGui /Users/nicolas/Qt/5.7/clang_64/lib/QtGui.framework/Versions/5/QtGui target/debug/examples/main
install_name_tool -change @rpath/QtQml.framework/Versions/5/QtQml /Users/nicolas/Qt/5.7/clang_64/lib/QtQml.framework/Versions/5/QtQml target/debug/examples/main
install_name_tool -change @rpath/QtWidgets.framework/Versions/5/QtWidgets /Users/nicolas/Qt/5.7/clang_64/lib/QtWidgets.framework/Versions/5/QtWidgets target/debug/examples/main
```

Run!
```
cargo run --example main
```

Or simply:
```
cargo build --example main && ./fix_qt_rpath.sh && cargo run --example main
```
