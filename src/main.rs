use serde_json::{Result, Value};
use serde::{Deserialize};
use graphviz_rust::dot_structures::*;
use graphviz_rust::dot_generator::*;
use graphviz_rust::{exec, parse, print};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::{PrinterContext,DotPrinter};
use graphviz_rust::attributes::*;

#[derive(Deserialize, Debug, Clone)]
enum Kind {
    Pred,
    Nuc,
    Core,
    CoreP,
    Clause,
    ClauseP,
    Sentence,

}

#[derive(Deserialize, Debug, Clone)]
struct Top {
     pos: String,
     kind: Kind,
}

impl Top {
    fn make_edge(&self, node: usize) -> Stmt {

        let pos_node = node_id!(self.pos.clone() + &node.to_string() );
        // let pos_node = node_id!(node.to_string() + &self.pos);
        let edge  = match self.kind {
            Kind::Pred => edge!(node_id!(node) => pos_node => node_id!("PredTop")),
            Kind::Nuc => edge!(node_id!(node) => pos_node => node_id!("NucTop")),
            Kind::Core => edge!(node_id!(node) => pos_node => node_id!("CoreTop")),
            Kind::CoreP => edge!(node_id!(node) => pos_node => node_id!("PredTop")),// not correct
            Kind::Clause => edge!(node_id!(node) => pos_node => node_id!("ClauseTop")),
            Kind::ClauseP => edge!(node_id!(node) => pos_node => node_id!("PredTop")),  // not correct
            Kind::Sentence => edge!(node_id!(node) => pos_node => node_id!("SentenceTop"))
        };
        edge.into()
    }
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

    let mut graph = graph!(id!("id"); node!("SentenceTop"),
                           node!("ClauseTop"),
                           node!("CoreTop"),
                           node!("NucTop"),
                           node!("PredTop"),
edge!(node_id!("SentenceTop") => node_id!("ClauseTop") => node_id!("CoreTop") => node_id!("NucTop") => node_id!("PredTop"); attr!("weight", "10"))
    );






    for (index, p) in phons.iter().enumerate() {
        graph.add_stmt(Stmt::Node(node!(index.to_string(); attr!("label", esc p.phon))));
        if let Some(t) = &p.top {
            graph.add_stmt(Stmt::Node(node!(t.pos.to_string() + &index.to_string(); attr!("label", esc t.pos))));
            graph.add_stmt(t.make_edge(index));

        }
    }

    let num_phons = phons.len();

    let edges: Vec<_> = (0..num_phons).map(|x| Vertex::from(node_id!(x))).collect();
    let edge_stmt = Edge{ ty: EdgeTy::Chain(edges), attributes: vec![attr!("style", "invis")] };
    let phons_sub: Subgraph = subgraph!("phons"; attr!("rank", "same"), edge_stmt);

    graph.add_stmt(phons_sub.into());


    // let enumer_top  = phons.iter().enumerate().map(|(num, p)| p.top.as_ref().map( |t| Vertex::from(node_id!(t.pos.to_string() + &num.to_string()))));

    // let edges: Vec<_> = enumer_top.filter_map(|t| t).collect();
    // let edge_stmt = Edge{ ty: EdgeTy::Chain(edges), attributes: vec![attr!("style", "invis")] };
    //
    let mut v: Vec<Stmt> = vec![attr!("rank", "same").into()];
    let stmts  = phons.iter().enumerate().map(|(num, p)| p.top.as_ref().map( |t| node!(t.pos.to_string() + &num.to_string()).into()));
    let stmts: Vec<_> = stmts.filter_map(|t| t).collect();
    v.extend(stmts);
    let pos_sub: Subgraph = subgraph!("all_pos", v);

    graph.add_stmt(pos_sub.into());

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
        "phon" : "leaving",
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
