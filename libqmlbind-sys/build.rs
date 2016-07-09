use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::env;


fn main() {
    let target = env::var("TARGET").unwrap();

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


    let qt_dir = env::var("QT_DIR").map(|p| PathBuf::from(p)).unwrap_or({
        println!("Environnement variable 'QT_DIR' not set!");
        println!("Defaulting to ${{HOME}}/Qt/${{QT_VER}}/${{QT_COMP}} where:");
        let home_dir = env::home_dir().map(|p| PathBuf::from(p)).unwrap();
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

    let qt_lib_dir = qt_dir.join("lib").to_str().unwrap().to_string();

    let osx_framework = if target.contains("darwin") { "=framework" }
                        else  { "" };

    println!("cargo:rustc-link-search{}={}", osx_framework, qt_lib_dir);

    println!("cargo:rustc-link-lib{}=QtCore", osx_framework);
    println!("cargo:rustc-link-lib{}=QtNetwork", osx_framework);
    println!("cargo:rustc-link-lib{}=QtGui", osx_framework);
    println!("cargo:rustc-link-lib{}=QtQml", osx_framework);
    println!("cargo:rustc-link-lib{}=QtWidgets", osx_framework);

    println!("cargo:rustc-flags=-l stdc++");
}
