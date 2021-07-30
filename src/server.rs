use std::net::SocketAddr;
use std::process::exit;

use log::info;
mod logger;
use tonic::{transport::Server, Request, Response, Status};

use clawflake::clawflake_server::{Clawflake, ClawflakeServer};
use clawflake::{IdReply, IdRequest};

pub mod clawflake {
    tonic::include_proto!("clawflake");
}

mod id_worker;
use id_worker::IdWorker;

#[derive(Debug, Default)]
pub struct MyClawflakeService {
}

#[tonic::async_trait]
impl Clawflake for MyClawflakeService {
    async fn get_id(
        &self,
        _: Request<IdRequest>,
    ) -> Result<Response<IdReply>, Status> {
        info!("request on GetID");

        let mut worker: IdWorker = IdWorker::new(1_564_790_400_000, 0, 0);

        let reply: IdReply = clawflake::IdReply {
            id: format!("{}", worker.next_id()).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logger
    match logger::init() {
      Err(e) => {
        eprintln!("failed to init logger: {}", &e);
        exit(1);
      }
      _ => {}
    }

    // init tonic and IdWorker
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let srv: MyClawflakeService = MyClawflakeService::default();

    println!("Service listening on {}", addr);

    Server::builder()
        .add_service(ClawflakeServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}
