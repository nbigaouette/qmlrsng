#!/bin/bash

QMLRSNG=${HOME}/Documents/perso/codes/rust/qmlrsng.git
QTDIR=${HOME}/Qt/5.7/clang_64


cargo build --example gallery

# Build resources
${QTDIR}/bin/rcc -name gallery examples/gallery.qrc -o target/debug/examples/qrc_gallery.cpp

/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang++ -c -pipe -stdlib=libc++ -O2 -std=gnu++11 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.11.sdk -mmacosx-version-min=10.8 -Wall -W -fPIC -DQT_NO_DEBUG -DQT_QUICKCONTROLS2_LIB -DQT_QUICK_LIB -DQT_GUI_LIB -DQT_QML_LIB -DQT_NETWORK_LIB -DQT_CORE_LIB -Iexamples -I. -I${QTDIR}/lib/QtQuickControls2.framework/Headers -I${QTDIR}/lib/QtQuick.framework/Headers -I${QTDIR}/lib/QtGui.framework/Headers -I${QTDIR}/lib/QtQml.framework/Headers -I${QTDIR}/lib/QtNetwork.framework/Headers -I${QTDIR}/lib/QtCore.framework/Headers -I. -I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.11.sdk/System/Library/Frameworks/OpenGL.framework/Headers -I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.11.sdk/System/Library/Frameworks/AGL.framework/Headers -I${QTDIR}/mkspecs/macx-clang -F${QTDIR}/lib -o target/debug/examples/qrc_gallery.o target/debug/examples/qrc_gallery.cpp

# Create a static library of the resources file
ar rvs target/debug/examples/libqrc_gallery.a target/debug/examples/qrc_gallery.o

# Relink the binary with the new library
rustc examples/gallery.rs --crate-name gallery --crate-type bin -g --out-dir ${QMLRSNG}/libqmlbind-sys/target/debug/examples --emit=dep-info,link -L dependency=${QMLRSNG}/libqmlbind-sys/target/debug -L dependency=${QMLRSNG}/libqmlbind-sys/target/debug/deps --extern libc=${QMLRSNG}/libqmlbind-sys/target/debug/deps/liblibc-d9dba2869ce64308.rlib --extern libqmlbind_sys=${QMLRSNG}/libqmlbind-sys/target/debug/liblibqmlbind_sys.rlib -L libqmlbind/qmlbind -lqrc_gallery  -L target/debug/examples -L framework=${QTDIR}/lib
