#!/bin/bash

# Default values
QTMAJVER="5"
QTMINVER="${QTMINVER:-7}"
QTCOMPARCH="${QTCOMPARCH:-clang_64}"

QTDIR="${QTDIR:-${HOME}/Qt/${QTMAJVER}.${QTMINVER}/${QTCOMPARCH}}"
binary="target/debug/examples/hello_world"

components=(QtCore QtNetwork QtGui QtQml QtWidgets)

for component in ${components[*]}; do
    install_name_tool -change @rpath/${component}.framework/Versions/${QTMAJVER}/${component} ${QTDIR}/lib/${component}.framework/Versions/${QTMAJVER}/${component} ${binary}
done
