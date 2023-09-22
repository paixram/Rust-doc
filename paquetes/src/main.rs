

//#[path = "./api/api.rs"]

//mod api;
//use crate::api::WebService;

use paquetes::*; // llama a todo aquello que este vinculado y este en lib

#[path = "./cripto/src/lib.rs"]
mod cripto;
use cripto::Cripto::Currency;

fn main() {
    dog();
    WebService::aws();
    Currency();
}
