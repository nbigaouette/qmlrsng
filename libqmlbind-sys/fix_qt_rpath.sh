#!/bin/bash

# Default values
QTMAJVER="5"
QTMINVER="${QTMINVER:-7}"
QTCOMPARCH="${QTCOMPARCH:-clang_64}"

QTDIR="${QTDIR:-${HOME}/Qt/${QTMAJVER}.${QTMINVER}/${QTCOMPARCH}}"

components=(QtCore QtNetwork QtGui QtQml QtWidgets)


for binary in target/debug/examples/*; do
    if [[ -f ${binary} ]]; then
        for component in ${components[*]}; do
            install_name_tool -change @rpath/${component}.framework/Versions/${QTMAJVER}/${component} ${QTDIR}/lib/${component}.framework/Versions/${QTMAJVER}/${component} ${binary}
        done
    fi
done
