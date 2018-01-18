extern crate hyper;
extern crate futures;

use std::collections::HashMap;
use std::fs::{File};
use std::io;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use futures::future::Future;
use futures::future::ok;

use hyper::{Get, Post, Put, Delete, StatusCode};
use hyper::server::{Service, NewService, Request, Response};
use hyper::header::{ContentLength};



pub mod Pony;
