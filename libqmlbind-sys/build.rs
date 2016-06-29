use std::path::Path;
use std::process::Command;


fn main() {

    // Initialize git submodule
    if !Path::new("libqmlbind/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    Command::new("qmake").current_dir("libqmlbind").output().unwrap();
    Command::new("make").current_dir("libqmlbind").output().unwrap();
    Command::new("make").arg("staticlib").current_dir("libqmlbind/qmlbind").output().unwrap();


    println!("cargo:rustc-flags=-L libqmlbind/qmlbind");
    println!("cargo:rustc-link-lib=static=qmlbind");
    // println!("cargo:rustc-link-lib=qmlbind");


    // https://github.com/bitcoin/bitcoin/issues/7714
    // println!("cargo:rustc-link-lib=Qt");
    // HACK - missing pkgconfig files in Qt5.6
    // QT_CFLAGS="-I${QTDIR}/include -I${QTDIR}/include/QtWidgets -I${QTDIR}/include/QtCore \
    //            -I${QTDIR}/include/QtGui -I${QTDIR}/include/QtNetwork"
    // QT_LIBS="-F${QTDIR}/lib -framework QtCore -framework QtNetwork -framework QtGui -framework QtWidgets" ./configure --with-gui=qt5
    // QT_LIBS="-F${QTDIR}/lib -framework QtCore -framework QtNetwork -framework QtGui -framework QtWidgets"
    // let qtdir = String::from("/Users/nicolas/Qt/5.6/clang_64/lib");
    let qtdir = String::from("/Users/nicolas/Qt/5.7/clang_64/lib");

    println!("cargo:rustc-link-search=framework={}", qtdir);
    // println!("cargo:rustc-flags=-L framework={}", qtdir);

    println!("cargo:rustc-link-lib=framework=QtCore");
    println!("cargo:rustc-link-lib=framework=QtNetwork");
    println!("cargo:rustc-link-lib=framework=QtGui");
    println!("cargo:rustc-link-lib=framework=QtWidgets");

    println!("cargo:rustc-flags=-l stdc++");

}
