use serde_json::{Result, Value};
use serde::{Deserialize};


#[derive(Deserialize, Debug)]
enum Kind {
    Pred,
    Nuc,
    Core,
    CoreP,
    ClauseP,
    Sentence,

}

#[derive(Deserialize, Debug)]
struct Top {
     pos: String,
     kind: Kind,
}

#[derive(Deserialize, Debug)]
struct Rrg {
    // Use the result of a function as the default if "resource" is
    // not included in the input.


}


fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let top_test = r#"
        {
           "pos": "V",
           "kind": "Core"
        }"#;

    // Parse the string of data into serde_json::Value.
    println!("testing");
    let v: Top = serde_json::from_str(top_test)?;
    println!("yay parsed corectly");

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {:?}", v.pos, v.kind);

    Ok(())
}


fn main() {
    untyped_example();
}
