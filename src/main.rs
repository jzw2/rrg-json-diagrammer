use std::env;


use std::fs::read_to_string;


use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};
use graphviz_rust::{exec};
use serde::Deserialize;


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

impl Kind {
    fn to_string(&self) -> String {
        let s = match &self {
            Kind::Pred => "Pred",
            Kind::Nuc => "Nuc",
            Kind::Core => "Core",
            Kind::CoreP => "CoreP",
            Kind::Clause => "Clause",
            Kind::ClauseP => "ClauseP",
            Kind::Sentence => "Sentence",
        };
        s.into()
    }

    fn get_per(&self) -> Option<Kind> {
        match self {
            Kind::CoreP => Some(Kind::Core),
            Kind::ClauseP => Some(Kind::Clause),
            _ => None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Top {
    pos: String,
    kind: Kind,
}

impl Top {
    fn make_edge(&self, node: usize) -> Stmt {
        let pos_node = node_id!(self.pos.clone() + &node.to_string());
        // let pos_node = node_id!(node.to_string() + &self.pos);
        //
        let connect_node_string = if self.kind.get_per().is_some() {
            format!("{}{}Top", self.kind.to_string(), node)
        } else {
            format!("{}Top", self.kind.to_string())
        };
        let edge = edge!(node_id!(node) => pos_node => node_id!(connect_node_string); attr!("dir", "none"));
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
    bot: Option<Vec<Bot>>,
}

fn make_graph(phons: Vec<Phon>) -> Graph {
    let mut graph = graph!(di id!("id");
                           node!("SentenceTop"; attr!("label", "Sentence"), attr!("group", "main")),
                           node!("ClauseTop"; attr!("label", "Clause"), attr!("group", "main")),
                           node!("CoreTop";attr!("label", "Core"), attr!("group", "main")),
                           node!("NucTop"; attr!("label", "Nuc"), attr!("group", "main")),
                           node!("PredTop"; attr!("label", "Pred"), attr!("group", "main")),
                           node!("SentenceBot"; attr!("label", "Sentence"), attr!("group", "main")),
                           node!("ClauseBot"; attr!("label", "Clause"), attr!("group", "main")),
                           node!("CoreBot";attr!("label", "Core"), attr!("group", "main")),
                           node!("NucBot"; attr!("label", "Nuc"), attr!("group", "main")),
                           node!("PredBot"; attr!("label", "Pred"), attr!("group", "main"))
    );

    // assume there can only be one
    let mut pred_index = None;

    // connecting each element to their pos and then to the vertical bar
    for (index, p) in phons.iter().enumerate() {
        let mut add_main = false;
        if let Some(t) = &p.top {
            let mut a = vec![attr!("label", esc t.pos)];
            if let Kind::Pred = &t.kind {
                //if it's the pred, we need add the thing to the bottom also
                pred_index = Some(index);
                a.push(attr!("group", "main"));
                add_main = true;
                graph.add_stmt(
                    node!(t.pos.to_string() + "Bot"; attr!("label", t.pos), attr!("group", "main"))
                        .into(),
                )
            } else {
                graph.add_stmt(t.make_edge(index));
            }
            // add pos
            graph.add_stmt(Stmt::Node(node!(t.pos.to_string() + &index.to_string(), a)));

            if let Some(pkind) = t.kind.get_per() {
                let main_name = format!("{}Top", pkind.to_string());
                let p_name = format!("{}{}Top", t.kind.to_string(), index);
                graph.add_stmt(Stmt::Node(node!(p_name; attr!("label", "Periphery"))));
                graph.add_stmt(edge!(node_id!(p_name) => node_id!(main_name)).into());

                let subgr = subgraph!(; attr!("rank", "same"), node!(main_name), node!(p_name));
                graph.add_stmt(subgr.into());
            }
        }
        if let Some(b) = &p.bot {
            for (op_index, proj) in b.iter().enumerate() {
                //let n = node!(proj.kind)
                let operator_node_name = format!("{}{}_{}", proj.op, index, op_index);

                graph.add_stmt(node!(operator_node_name; attr!("label", proj.op)).into());
                graph.add_stmt(edge!(node_id!(index) => node_id!(operator_node_name); attr!("style", "dotted"), attr!("dir", "none")).into());
                let bottom_name = format!("{}Bot", proj.kind.to_string());
                graph.add_stmt(edge!(node_id!(operator_node_name) => node_id!(bottom_name)).into());
                let subgr = subgraph!(; attr!("rank", "same"), node!(bottom_name), node!(operator_node_name));
                graph.add_stmt(subgr.into());
            }
        }
        let mut a = vec![attr!("label", esc p.phon)];
        if add_main {
            a.push(attr!("group", "main"));
        }
        graph.add_stmt(Stmt::Node(node!(index.to_string(), a)));
    }

    // make big ventiacl thing in the middle

    let mut v = vec![
        node_id!("SentenceTop").into(),
        node_id!("ClauseTop").into(),
        node_id!("CoreTop").into(),
        node_id!("NucTop").into(),
        node_id!("PredTop").into(),
    ];

    let index =
        pred_index.expect("Fialure didn't put the pred thing in beaucse I didn't miplemnte it yet");
    let pos = phons[index].top.as_ref().unwrap().pos.to_string();
    v.push(node_id!(pos.to_string() + &index.to_string()).into());
    v.push(node_id!(index.to_string()).into());
    v.push(node_id!(pos + "Bot").into());

    let v_bot = vec![
        node_id!("PredBot").into(),
        node_id!("NucBot").into(),
        node_id!("CoreBot").into(),
        node_id!("ClauseBot").into(),
        node_id!("SentenceBot").into(),
    ];
    v.extend(v_bot);

    //remember to add the bottom stuff here

    let big_vert = Edge {
        ty: EdgeTy::Chain(v),
        attributes: vec![attr!("dir", "none")],
    };
    graph.add_stmt(big_vert.into());

    // make subgraph to make the phon horizontal
    let num_phons = phons.len();

    let edges: Vec<_> = (0..num_phons).map(|x| Vertex::from(node_id!(x))).collect();
    let edge_stmt = Edge {
        ty: EdgeTy::Chain(edges),
        attributes: vec![attr!("style", "invis")],
    };
    let phons_sub: Subgraph = subgraph!("phons"; attr!("rank", "same"), edge_stmt);

    graph.add_stmt(phons_sub.into());

    // let enumer_top  = phons.iter().enumerate().map(|(num, p)| p.top.as_ref().map( |t| Vertex::from(node_id!(t.pos.to_string() + &num.to_string()))));

    // let edges: Vec<_> = enumer_top.filter_map(|t| t).collect();
    // let edge_stmt = Edge{ ty: EdgeTy::Chain(edges), attributes: vec![attr!("style", "invis")] };
    //
    // make a subgraph so the pos allign
    let mut v: Vec<Stmt> = vec![attr!("rank", "same").into()];
    let stmts = phons.iter().enumerate().map(|(num, p)| {
        p.top
            .as_ref()
            .map(|t| node!(t.pos.to_string() + &num.to_string()).into())
    });
    let stmts: Vec<_> = stmts.flatten().collect();
    v.extend(stmts);
    let pos_sub: Subgraph = subgraph!("all_pos", v);

    graph.add_stmt(pos_sub.into());

    let x = graph.print(&mut PrinterContext::default());
    println!("{}", x);

    graph
}

fn main() {
    //untyped_example();
    let input_file = env::args().nth(1).expect("Did not provide input file");
    let output_file = env::args().nth(2).expect("Did not provide output file");

    let error = format!("Error in reading file {}", input_file);

    let json_content = read_to_string(input_file).expect(&error);
    let g =
        make_graph(serde_json::from_str(&json_content).expect("You messed up the Json format"));

    let graph_svg = exec(
        g,
        &mut PrinterContext::default(),
        vec![
            CommandArg::Format(Format::Svg),
            CommandArg::Output(output_file),
        ],
    )
    .unwrap();

    println!("Random Debug info \n ---------- \n {}", graph_svg);
}
