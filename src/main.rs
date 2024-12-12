fn main() {
    println!("Hello, world!");
    println!("lists all config options for our beloved cloudnine");
    println!("plates, foams, tadpoles, weights");

    let plates = vec!["Aluminum", "PolyPropelyne"];
    let foams = vec!["Plate", "Case", "None"];
    let mounts = vec!["Blue", "White", "Orange", "Black"];
    let weights = vec!["One", "Two", "Three", "Four"];

    for plate in &plates {
        for foam in &foams {
            for mount in &mounts {
                for weight in &weights {
                    println!("{}, {}, {}, {}", plate, foam, mount, weight);
                }
            }
        }
    }
}