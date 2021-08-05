use std::env;
use std::net::SocketAddr;
use std::process::exit;

use log::info;
mod logger;
use tonic::{transport::Server, Request, Response, Status};

pub mod clawflake {
  tonic::include_proto!("clawflake");
}

use clawflake::clawflake_server::{Clawflake, ClawflakeServer};
use clawflake::{IdReply, IdRequest};

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

        let mut worker: IdWorker = IdWorker::new(
          env::var("CLAWFLAKE_EPOCH").expect("Missing env `CLAWFLAKE_EPOCH`").parse::<i64>().unwrap(), 
          env::var("CLAWFLAKE_WORKER_ID").expect("Missing env `CLAWFLAKE_WORKER_ID`").parse::<i64>().unwrap(),
          env::var("CLAWFLAKE_DATACENTER_ID").expect("Missing env `CLAWFLAKE_DATACENTER_ID`").parse::<i64>().unwrap()
        );

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

    // init tonic_health
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<ClawflakeServer<MyClawflakeService>>().await;

    // init tonic and IdWorker
    let addr: SocketAddr = "[::0]:50051".parse()?;
    let srv: MyClawflakeService = MyClawflakeService::default();

    println!("Service listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(ClawflakeServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}
