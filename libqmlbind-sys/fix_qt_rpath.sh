#!/bin/bash

QTDIR="/Users/nicolas/Qt/5.7/clang_64"
QTMAJVER="5"
binary="target/debug/examples/main"

components=(QtCore QtNetwork QtGui QtQml QtWidgets)

for component in ${components[*]}; do
    install_name_tool -change @rpath/${component}.framework/Versions/${QTMAJVER}/${component} ${QTDIR}/lib/${component}.framework/Versions/${QTMAJVER}/${component} ${binary}
done
