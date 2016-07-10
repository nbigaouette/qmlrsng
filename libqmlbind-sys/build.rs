use std::path::PathBuf;
use std::process::Command;
use std::env;


fn main() {
    let target = env::var("TARGET").unwrap();
    // let out_dir = env::var("OUT_DIR").unwrap();
    let current_dir = env::current_dir().unwrap();

    // println!("target:      {:?}", target);
    // println!("out_dir:     {:?}", out_dir);
    // println!("current_dir: {:?}", current_dir);

    let libqmlbind_dir = current_dir.join("libqmlbind");
    let libqmlbind_build_dir = libqmlbind_dir.join("qmlbind");

    // Initialize git submodule
    if !libqmlbind_dir.join(".git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    Command::new("qmake").current_dir(&libqmlbind_dir).output().unwrap();
    Command::new("make").current_dir(&libqmlbind_dir).output().unwrap();
    Command::new("make").arg("staticlib").current_dir(&libqmlbind_build_dir).output().unwrap();


    println!("cargo:rustc-link-search={}", libqmlbind_build_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=qmlbind");


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
    // On Linux, libraries are name "Qt5Core", not "QtCore" as on OSX
    let linux_qt_lib_ver = if target.contains("linux") { "5" }
                           else  { "" };

    println!("cargo:rustc-link-search{}={}", osx_framework, qt_lib_dir);

    println!("cargo:rustc-link-lib{}=Qt{}Core", osx_framework, linux_qt_lib_ver);
    println!("cargo:rustc-link-lib{}=Qt{}Network", osx_framework, linux_qt_lib_ver);
    println!("cargo:rustc-link-lib{}=Qt{}Gui", osx_framework, linux_qt_lib_ver);
    println!("cargo:rustc-link-lib{}=Qt{}Qml", osx_framework, linux_qt_lib_ver);
    println!("cargo:rustc-link-lib{}=Qt{}Widgets", osx_framework, linux_qt_lib_ver);

    // On Linux, Qt needs icui18n, icuuc and icudata
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=icui18n");
        println!("cargo:rustc-link-lib=icuuc");
        println!("cargo:rustc-link-lib=icudata");
    }

    println!("cargo:rustc-flags=-l stdc++");
}
