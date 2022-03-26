#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

pub mod rpc {
    tonic::include_proto!("auth");
}

pub mod db;
