#!/bin/bash

# Default values
QTMAJVER="5"
QTMINVER="${QTMINVER:-7}"
QTCOMPARCH="${QTCOMPARCH:-clang_64}"

QTDIR="${QTDIR:-${HOME}/Qt/${QTMAJVER}.${QTMINVER}/${QTCOMPARCH}}"

components=(QtCore QtNetwork QtGui QtQml QtWidgets)

binary="gallery"
for build in "debug" "release"; do
    echo "    Build: ${build}"
    bin="target/${build}/${binary}"
    echo "        ${bin}"
    if [[ -f ${bin} ]]; then
        echo "        bin: ${bin}"
        for component in ${components[*]}; do
            echo "            component: ${component}"
            install_name_tool -change @rpath/${component}.framework/Versions/${QTMAJVER}/${component} ${QTDIR}/lib/${component}.framework/Versions/${QTMAJVER}/${component} ${bin}
        done
    fi
done
