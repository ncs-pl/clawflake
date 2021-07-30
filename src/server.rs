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

const EPOCH: i64 = 1_564_790_400_000; // todo(n1c00o): find a good custom epoch for production cuz its fun
const WORKER_ID: i64 = 0; // todo(n1c00o): need a way to detect the worker id from Kubernetes
const DATACENTER_ID: i64 = 0; // todo(n1c00o): need a way to detect the data center id from idk

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

        let mut worker: IdWorker = IdWorker::new(EPOCH, WORKER_ID, DATACENTER_ID);

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
    let addr: SocketAddr = "[::1]:50051".parse()?; //todo(n1c00o): make sure we can manage addr, then make the Dockerfile
    let srv: MyClawflakeService = MyClawflakeService::default();

    println!("Service listening on {}", addr);

    Server::builder()
        .add_service(ClawflakeServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}
