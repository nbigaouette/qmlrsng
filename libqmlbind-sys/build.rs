use std::path::PathBuf;
use std::process::Command;
use std::env;


fn main() {
    let target = env::var("TARGET").expect("Environnement variable TARGET not set");
    // let out_dir = env::var("OUT_DIR").expect("Failed to get output directory");
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // println!("target:      {:?}", target);
    // println!("out_dir:     {:?}", out_dir);
    // println!("current_dir: {:?}", current_dir);

    let libqmlbind_dir = current_dir.join("libqmlbind");
    let libqmlbind_build_dir = libqmlbind_dir.join("build");

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
    println!("Use QT_DIR environment variable to overwrite.");

    let qt_lib_dir = qt_dir.join("lib");
    let qt_bin_dir = qt_dir.join("bin");

    // Initialize git submodule
    if !libqmlbind_dir.join(".git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status()
                                   .unwrap_or_else(|e| panic!("Failed to initialize git submodule: {}", e));
    }

    std::fs::create_dir_all(&libqmlbind_build_dir).unwrap_or_else(|e| panic!("Failed to create libqmlbind build directory: {}", e));

    println!("current_dir: {:?}", current_dir);
    println!("libqmlbind_dir: {:?}", libqmlbind_dir);
    println!("libqmlbind_build_dir: {:?}", libqmlbind_build_dir);
    println!("qt_lib_dir: {:?}", qt_lib_dir);
    println!("qt_bin_dir: {:?}", qt_bin_dir);

    let output = Command::new(qt_bin_dir.join("qmake")).arg("../qmlbind")
                                      .current_dir(&libqmlbind_build_dir)
                                      .output()
                                      .unwrap_or_else(|e| panic!("Failed execute qmake: {}", e));
    println!("output.status: {}", output.status);
    println!("output.stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("output.stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success(), "failed to execute qmake process");

    let output = Command::new("make")
                                .current_dir(&libqmlbind_build_dir)
                                .output()
                                .unwrap_or_else(|e| panic!("Failed execute make : {}", e));
    println!("output.status: {}", output.status);
    println!("output.stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("output.stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success(), "failed to execute make process");

    let output = Command::new("make").arg("staticlib")
                                     .current_dir(&libqmlbind_build_dir)
                                     .output()
                                     .unwrap_or_else(|e| panic!("Failed execute 'make staticlib': {}", e));
    println!("output.status: {}", output.status);
    println!("output.stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("output.stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success(), "failed to execute 'make staticlib' process");


    println!("cargo:rustc-link-search={}", libqmlbind_build_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=qmlbind");

    let osx_framework = if target.contains("darwin") { "=framework" }
                        else  { "" };
    // On Linux, libraries are name "Qt5Core", not "QtCore" as on OSX
    let linux_qt_lib_ver = if target.contains("linux") { "5" }
                           else  { "" };

    println!("cargo:rustc-link-search{}={}", osx_framework, qt_lib_dir.to_str().unwrap());

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
