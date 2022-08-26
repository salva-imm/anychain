use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, Responder, Result, web::{Data}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
struct Block<'a>{
    index: u32,
    message: &'a str,
    previous_hash: &'a str,
    proof: u32,
    timestamp: u32
}

// async fn proof_of_work(last_proof: u32) -> u32 {
//     let mut proof = 1;
//     let mut check_proof = false;
//     let mut hasher = DefaultHasher::new();
//     while !check_proof {
//         proof = (proof ** 2) - (last_proof ** 2);
//         proof.hash(&mut hasher);
//         let hash_operations = hasher.finish();
//         println!("{}", hash_operations);
//         check_proof = true;
//     }
//     proof
// }

fn string_to_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}

async fn mine_block<'a>(chain: Data<Mutex<Vec<Block<'a>>>>) -> Result<impl Responder + 'static> {
    let mut t_chain = chain.lock().unwrap();
    let len = t_chain.len();

    t_chain.push(Block{
        index: 1 + len as u32,
        message: string_to_str(format!("newly mined block {}", (1 + len).to_string())),
        previous_hash: "last_hash",
        proof: 1,
        timestamp: 134465
    });
    Ok(web::Json({}))
}


async fn display_chain<'a>(chain: Data<Mutex<Vec<Block<'a>>>>) -> Result<impl Responder + 'a> {
    let t_chain = chain.lock().unwrap();
    Ok(web::Json(t_chain.to_vec()))
}


async fn valid<'a>(_chain: Data<Mutex<Vec<Block<'a>>>>) -> Result<impl Responder> {
    Ok(web::Json({}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting node on port 8000 ...");
    let chain: Data<Mutex<Vec<Block>>> = Data::new(Mutex::new(vec![Block{
        index: 1,
        message: "Genesis block",
        previous_hash: "0",
        proof: 1,
        timestamp: 134465
    }]));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&chain))
            .route("/get_chain", web::get().to(display_chain))
            .route("/mine_block", web::get().to(mine_block))
            .route("/valid", web::get().to(valid))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}