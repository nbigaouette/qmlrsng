extern crate qmlrsng as qmlrs;


fn main() {
    println!("qmlrsng example");

    // Create a QML engine
    let engine = qmlrs::Engine::new();

    // Create a component by loading a QML file
    let component = qmlrs::Component::load_path(&engine, "examples/hello_orld.qml")
                                        .expect("Failed to create QML component from file");

    // Instanciate a component
    let instance = component.create();

    // Start the QML engine
    engine.start();
}
