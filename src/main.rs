use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};
use tokio::sync::RwLock;
use actix_web::{web, App, HttpServer, Responder, Result, web::{Data}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
struct Block{
    index: u32,
    message: String,
    previous_hash: String,
    nonce: u64,
    timestamp: u32
}
// 00000d27548b48fb9948dec841504bb2dfe0ad4812f0f6c049f2cd02dada6dcd
async fn proof_of_work(last_block: &Block) -> u64 {
    let mut nonce: u64 = 1;
    let mut check_proof = false;
    // TODO: make it parallel / future plan

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


fn _string_to_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}

async fn mine_block(chain: Data<RwLock<Vec<Block>>>) -> Result<impl Responder> {
    let t_chain = chain.read().await;

    let len = t_chain.len();

    let nonce = proof_of_work(t_chain.last().unwrap()).await;

    let mut d_hasher = DefaultHasher::new();
    t_chain.last().unwrap().hash(&mut d_hasher);
    let mut hasher = Sha256::new();

    hasher.update(d_hasher.finish().to_string().as_bytes());
    let hash_operation = hasher.finalize();
    let hash_digest = base16ct::lower::encode_string(&hash_operation);
    drop(t_chain);
    let mut t_chain = chain.write().await;

    t_chain.push(Block{
        index: 1 + len as u32,
        message: format!("newly mined block {}", (1 + len)),
        previous_hash: hash_digest,
        nonce: nonce,
        timestamp: 134465
    });
    Ok(web::Json({}))
}


async fn display_chain(chain: Data<RwLock<Vec<Block>>>) -> Result<impl Responder> {
    let t_chain = chain.read().await;
    Ok(web::Json(t_chain.to_vec()))
}


async fn valid(_chain: Data<RwLock<Vec<Block>>>) -> Result<impl Responder> {
    Ok(web::Json({}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting node on port 8000 ...");

    let chain: Data<RwLock<Vec<Block>>> = Data::new(RwLock::new(vec![Block{
        index: 1,
        message: "Genesis block".to_string(),
        previous_hash: "0".to_string(),
        nonce: 1,
        timestamp: 134465
    }]));
    HttpServer::new(move || {
        App::new()
            .app_data(chain.clone())
            .route("/get_chain", web::get().to(display_chain))
            .route("/mine_block", web::get().to(mine_block))
            .route("/valid", web::get().to(valid))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
