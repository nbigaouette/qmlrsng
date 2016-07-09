use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::process::Command;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    // http://doc.crates.io/build-script.html#inputs-to-the-build-script
    // "the build script’s current directory is the source directory of the build script’s package."
    let pwd = env::current_dir().unwrap();
    let src = pwd.join("src");

    let home_dir = env::home_dir().map(|p| PathBuf::from(p)).unwrap();

    let qt_dir = env::var("QT_DIR").map(|p| PathBuf::from(p)).unwrap_or({
        println!("Environnement variable 'QT_DIR' not set!");
        println!("Defaulting to ${{HOME}}/Qt/${{QT_VER}}/${{QT_COMP}} where:");
        let default_qt_ver = "5.7".to_string();
        let default_qt_comp = if target.contains("linux") {
            "gcc_64".to_string()
        } else if target.contains("darwin") {
            "clang_64".to_string()
        } else {
            panic!("Unsuported platform in gallery's build.rs!")
        };
        println!("    QT_VER:  {}", default_qt_ver);
        println!("    QT_COMP: {}", default_qt_comp);
        let qt_dir_default = home_dir.join("Qt")
                                  .join(env::var("QT_VER").unwrap_or(default_qt_ver))
                                  .join(env::var("QT_COMP").unwrap_or(default_qt_comp));
        qt_dir_default
    });
    println!("Using Qt directory: {:?}", qt_dir);
    println!("Use QT_DIR environment variable to ovewrite.");

    let qmake = qt_dir.join("bin").join("qmake");

    // let rcc = Path::new(&qt_dir).join("bin").join("rcc");
    // println!("rcc: {:?}", rcc);

    // let qrc_output = Path::new(&out_dir).join("qrc_gallery.cpp");
    //
    // // Run the Qt Resource Compiler (rcc)
    // // ${QTDIR}/bin/rcc -name gallery examples/gallery.qrc -o target/debug/examples/qrc_gallery.cpp
    // let output = Command::new(rcc).args(&["-name", "gallery", "src/gallery.qrc",
    //                                       "-o", qrc_output.to_str().unwrap()])
    //                               .output()
    //                               .expect("failed to execute 'rcc' process");

    // Run qmake to create a library with the reousrce file
    let output = Command::new(qmake).args(&[src])
                                  .current_dir(&Path::new(&out_dir))
                                  .output()
                                  .expect("failed to execute 'qmake' process");

    // println!("output.status: {}", output.status);
    // println!("output.stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("output.stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "failed to execute qmake process");

    // Call 'make'
    let output = Command::new("make").current_dir(&Path::new(&out_dir))
                                  .output()
                                  .expect("failed to execute 'make' process");

    println!("output.status: {}", output.status);
    println!("output.stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("output.stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "failed to execute make process");

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=galleryresources");




    // // Initialize git submodule
    // if !Path::new("libqmlbind/.git").exists() {
    //     let _ = Command::new("git").args(&["submodule", "update", "--init"])
    //                                .status();
    // }
    //
    // Command::new("qmake").current_dir("libqmlbind").output().unwrap();
    // Command::new("make").current_dir("libqmlbind").output().unwrap();
    // Command::new("make").arg("staticlib").current_dir("libqmlbind/qmlbind").output().unwrap();
    //
    //
    // println!("cargo:rustc-flags=-L libqmlbind/qmlbind");
    // println!("cargo:rustc-link-lib=static=qmlbind");
    // // println!("cargo:rustc-link-lib=qmlbind");
    //
    //
    // // https://github.com/bitcoin/bitcoin/issues/7714
    // // println!("cargo:rustc-link-lib=Qt");
    // // HACK - missing pkgconfig files in Qt5.6
    // // QT_CFLAGS="-I${QTDIR}/include -I${QTDIR}/include/QtWidgets -I${QTDIR}/include/QtCore \
    // //            -I${QTDIR}/include/QtGui -I${QTDIR}/include/QtNetwork"
    // // QT_LIBS="-F${QTDIR}/lib -framework QtCore -framework QtNetwork -framework QtGui -framework QtWidgets" ./configure --with-gui=qt5
    // // QT_LIBS="-F${QTDIR}/lib -framework QtCore -framework QtNetwork -framework QtGui -framework QtWidgets"
    // // let qtdir = String::from("/Users/nicolas/Qt/5.6/clang_64/lib");
    // let qtdir = String::from("/Users/nicolas/Qt/5.7/clang_64/lib");
    //
    // println!("cargo:rustc-link-search=framework={}", qtdir);
    // // println!("cargo:rustc-flags=-L framework={}", qtdir);
    //
    // println!("cargo:rustc-link-lib=framework=QtCore");
    // println!("cargo:rustc-link-lib=framework=QtNetwork");
    // println!("cargo:rustc-link-lib=framework=QtGui");
    // println!("cargo:rustc-link-lib=framework=QtQml");
    // println!("cargo:rustc-link-lib=framework=QtWidgets");
    //
    // println!("cargo:rustc-flags=-l stdc++");

}
