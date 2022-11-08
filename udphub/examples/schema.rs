#[cfg(not(feature="schemars"))]
fn main() { eprintln!("Schema generation requires `schemars` Cargo feature enabled"); std::process::exit(1); }

#[cfg(feature="schemars")]
fn main () {
    let schema = schemars::schema_for!(udphub::config::Config);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
