use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, Responder, Result, web::{Data}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
struct Block<'a>{
    index: u32,
    message: &'a str,
    previous_hash: &'a str,
    nonce: u64,
    timestamp: u32
}
// 00000d27548b48fb9948dec841504bb2dfe0ad4812f0f6c049f2cd02dada6dcd
async fn proof_of_work<'a>(last_block: &Block<'a>) -> u64 {
    let mut nonce: u64 = 1;
    let mut check_proof = false;

    while !check_proof {
        let calc_proof: String = format!("{}{}{}",
                                         nonce,
                                         last_block.previous_hash,
                                         last_block.message);
        let mut hasher = Sha256::new();
        hasher.update(calc_proof.to_string().as_bytes());
        let hash_operation = hasher.finalize();
        let hash_digest = base16ct::lower::encode_string(&hash_operation);
        if &hash_digest[0..5] == "00000"{
            check_proof = true;
        }else{
            nonce += 1;
        }
    }
    nonce
}

fn string_to_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}

async fn mine_block<'a>(chain: Data<Mutex<Vec<Block<'a>>>>) -> Result<impl Responder + 'static> {
    let mut t_chain = chain.lock().unwrap();
    let len = t_chain.len();
    let message = string_to_str(format!("newly mined block {}", (1 + len).to_string()));
    let nonce = proof_of_work(t_chain.last().unwrap()).await;
    let mut d_hasher = DefaultHasher::new();
    t_chain.last().unwrap().hash(&mut d_hasher);
    let mut hasher = Sha256::new();
    hasher.update(d_hasher.finish().to_string().as_bytes());
    let hash_operation = hasher.finalize();
    let hash_digest = base16ct::lower::encode_string(&hash_operation);
    println!("{}", hash_digest);

    t_chain.push(Block{
        index: 1 + len as u32,
        message: message,
        previous_hash: string_to_str(hash_digest),
        nonce: nonce,
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
        nonce: 1,
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