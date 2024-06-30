fn main() {
    let _plugins = [
        "swagger"
    ];
    let profile = std::env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" => {

        },
        "release" => (),
        _ => (),
    }
}