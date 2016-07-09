use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::process::Command;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    // http://doc.crates.io/build-script.html#inputs-to-the-build-script
    // "the build script’s current directory is the source directory of the build script’s package."
    let current_dir = env::current_dir().unwrap();
    let src_dir = current_dir.join("src");

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

    let qmake = qt_dir.join("bin").join("qmake");

    // Run qmake to create a library with the reousrce file
    let output = Command::new(qmake).args(&[src_dir])
                                  .current_dir(&Path::new(&out_dir))
                                  .output()
                                  .expect("failed to execute 'qmake' process");

    println!("output.status: {}", output.status);
    println!("output.stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("output.stderr: {}", String::from_utf8_lossy(&output.stderr));

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
}
