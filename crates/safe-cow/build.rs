use ethers::contract::{MultiAbigen, SelectContracts};

fn main () {
    println!("cargo:rerun-if-changed=build.rs");
    let gen = MultiAbigen::from_json_files("./abi").unwrap();

    let bindings = gen.build().unwrap();

    bindings.write_to_module("./src/contracts", false);
}
