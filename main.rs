use std::env;

fn main() {
    let vars: env::Vars = env::vars();
    // let out: Vec<(String, String)> = vars.into_iter().collect();
    // println!("{:?}", vars);
    // println!("{:?}", out);
    for (k, v) in vars {
        println!("{k}={v}");
    }
}
