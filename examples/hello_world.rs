extern crate qmlrsng as qmlrs;


fn main() {
    println!("qmlrsng example");

    // Create a QML engine
    let engine = qmlrs::Engine::new();

    // Create a component by loading a QML file
    let component = qmlrs::Component::load_path(&engine, "examples/hello_world.qml")
                                        .expect("Failed to create QML component from file");

    // Instanciate a component
    let instance = component.create().unwrap();

    let width = instance.get_property("width").unwrap();
    println!("width: {:?}", width);

    instance.set_property("width", 1200);


    // Start the QML engine
    engine.exec();
}
