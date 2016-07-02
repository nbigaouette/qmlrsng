#!/bin/bash

# Default values
QTMAJVER="5"
QTMINVER="${QTMINVER:-7}"
QTCOMPARCH="${QTCOMPARCH:-clang_64}"

QTDIR="${QTDIR:-${HOME}/Qt/${QTMAJVER}.${QTMINVER}/${QTCOMPARCH}}"

components=(QtCore QtNetwork QtGui QtQml QtWidgets)


for example in examples/*.rs; do
    binary=`echo ${example} | sed 's|examples/\(.*\).rs|\1|g'`
    for build in "debug" "release"; do
        binary="target/${build}/exmaples/${binary}"
        if [[ -f ${binary} ]]; then
            for component in ${components[*]}; do
                install_name_tool -change @rpath/${component}.framework/Versions/${QTMAJVER}/${component} ${QTDIR}/lib/${component}.framework/Versions/${QTMAJVER}/${component} ${binary}
            done
        fi
    done
done
