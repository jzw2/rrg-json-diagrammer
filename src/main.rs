use serde_json::{Result, Value};
use serde::{Deserialize};
use graphviz_rust::dot_structures::*;
use graphviz_rust::dot_generator::*;
use graphviz_rust::{exec, parse, print};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::{PrinterContext,DotPrinter};
use graphviz_rust::attributes::*;

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


fn make_graph(phons: Vec<Phon>) -> Graph {

    let id = id!("identity");
    //let p = NodeId();
    let mut graph = graph!(id!("id"); node!("SentenceTop"),
                           node!("ClauseTop"),
                           node!("CoreTop"),
                           node!("NucTop"),
edge!(node_id!("SentenceTop") => node_id!("ClauseTop") => node_id!("CoreTop") => node_id!("NucTop"))
    );

    for (index, p) in phons.iter().enumerate() {
        graph.add_stmt(Stmt::Node(node!(index.to_string(); attr!("label", esc p.phon))));
        if let Some(t) = &p.top {
            let pos_name = index.to_string() + "pos";
            graph.add_stmt(node!(pos_name; attr!("label", esc t.pos)).into());
            graph.add_stmt(edge!(node_id!(pos_name) => node_id!(index.to_string()) => node_id!("CoreTop")).into());

        }
    }

    let x = graph.print(&mut PrinterContext::default());
    println!("{}",x );



return graph;

}

    static T1: &str = r#"

[
    {
        "phon" : "Will",
        "bot" : [

            {
                "op" : "IF",
                "kind": "Clause"
            },
            {
                "op" : "TNS",
                "kind": "Clause"
            }
        ]

    },
    {
    "phon" : "they",
    "top" :
            {
                "pos" : "NP",
                "kind": "Core"
            }
    },
    {
        "phon" : "have to",
        "bot" : [

            {
                "op" : "MOD",
                "kind": "Core"
            }
        ]

    },
    {
        "phon" : "be",
        "bot" : [

            {
                "op" : "ASP",
                "kind": "Nuc"
            }
        ]

    },
    {
        "phon" : "be",
        "top" :
                {
                    "pos" : "V",
                    "kind": "Pred"
                },
        "bot" : [

            {
                "op" : "ASP",
                "kind": "Nuc"
            }
        ]

    }

]

"#;

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

    let top_test = r#"
    {
    "phon" : "did"
}

"#;

    let v: Phon = serde_json::from_str(top_test)?;

    println!("Tetsing {:?}", v);


    let top_test = r#"
    {
        "phon" : "Will",
        "bot" : [

            {
                "op" : "IF",
                "kind": "Clause"
            },
            {
                "op" : "TNS",
                "kind": "Clause"
            }
        ]

    }

"#;

    let v: Phon = serde_json::from_str(top_test)?;

    println!("Tetsing {:?}", v);


    let top_test = r#"
[
    {
    "phon" : "what",
    "top" : {
        "pos": "NP",
        "kind": "Clause"
        }
    },
    {
    "phon" : "did"
},
    {
    "phon" : "Robin",
    "top" : {
        "pos" : "NP",
        "kind" : "Core"
    }

},
    {
    "phon" : "show",
    "top" : {
        "pos" : "V",
        "kind": "Pred"
    }
},
    {
    "phon" : "to Pat",
    "top" : {
        "pos" : "PP",
        "kind": "Core"
    }
},
    {
    "phon" : "in the library",
    "top" :
    {
        "pos" : "PP",
        "kind": "CoreP"
    }
},
    {
    "phon" : "yesterday",
    "top" :
    {
        "pos" : "ADV",
        "kind": "CoreP"
    }
}

]
"#;

    let v: Vec<Phon> = serde_json::from_str(top_test)?;

    println!("Tetsing {:?}", v);



    let top_test = r#"

[
    {
        "phon" : "Will",
        "bot" : [

            {
                "op" : "IF",
                "kind": "Clause"
            },
            {
                "op" : "TNS",
                "kind": "Clause"
            }
        ]

    },
    {
    "phon" : "they",
    "top" :
            {
                "pos" : "NP",
                "kind": "Core"
            }
    },
    {
        "phon" : "have to",
        "bot" : [

            {
                "op" : "MOD",
                "kind": "Core"
            }
        ]

    },
    {
        "phon" : "be",
        "bot" : [

            {
                "op" : "ASP",
                "kind": "Nuc"
            }
        ]

    },
    {
        "phon" : "be",
        "top" :
                {
                    "pos" : "V",
                    "kind": "Pred"
                },
        "bot" : [

            {
                "op" : "ASP",
                "kind": "Nuc"
            }
        ]

    }

]

"#;

    println!("hi");
    let v: Result<Vec<Phon>> = serde_json::from_str(top_test);
    match v {
        Err(x) => {
            println!("error: {:?}", x);
        }
        Ok(_) => println!("test test {:?}", v),
    }



    Ok(())
}

fn draw_stuff() {
    untyped_example();



 let mut g = make_graph(serde_json::from_str(T1).unwrap());

       let graph_svg = exec(g, &mut PrinterContext::default(), vec![
           CommandArg::Format(Format::Svg),
            CommandArg::Output("test.svg".to_string())
       ]).unwrap();

    println!("making graph");
    println!("{}", graph_svg);
    println!("making graph");
}

fn main() {
    //untyped_example();
    draw_stuff();
}
