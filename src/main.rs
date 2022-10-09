use serde_json::{Result, Value};
use serde::{Deserialize};


#[derive(Deserialize, Debug)]
enum Kind {
    Pred,
    Nuc,
    Core,
    CoreP,
    Clause,
    ClauseP,
    Sentence,

}

#[derive(Deserialize, Debug)]
struct Top {
     pos: String,
     kind: Kind,
}

#[derive(Deserialize, Debug)]
struct Bot {
     op: String,
     kind: Kind,
}


#[derive(Deserialize, Debug)]
struct Phon {
    // Use the result of a function as the default if "resource" is
    // not included in the input.
    phon: String,
    top: Option<Top>,
    bot: Option<Vec<Bot>>

}


fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let top_test = r#"
        {
           "pos": "V",
           "kind": "Core"
        }"#;

    let v: Top = serde_json::from_str(top_test)?;

    println!("Please call {} at the number {:?}", v.pos, v.kind);

    let bot_test = r#"
        {
           "op": "V",
           "kind": "CoreP"
        }"#;

    println!("testing");
    let v: Bot = serde_json::from_str(bot_test)?;
    println!("yay parsed corectly");

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {:?}", v.op,  v.kind);

    let top_test = r#"
{
        "pos": "NP",
        "kind": "Clause"
        }

"#;

    let v: Top = serde_json::from_str(top_test)?;

    println!("Please call {} at the number {:?}", v.pos, v.kind);



    let top_test = r#"
    {
    "phon" : "what",
    "top" : {
        "pos": "NP",
        "kind": "Clause"
        }
    }

"#;

    let v: Phon = serde_json::from_str(top_test)?;

    println!("Tetsing {:?}", v);
    Ok(())
}


fn main() {
    untyped_example();
}
