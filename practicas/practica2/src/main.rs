
#[derive(Debug)]
struct WebService {
    ttype: String,
    timestamp: String,
    year: u32,
    desc: String,
}


impl WebService {
    fn New(ttype:  String, desc: String) -> WebService {
        let time: String = String::from("27/07/2021");
        let year: u32 = 2021;

        WebService {
            ttype,
            desc,
            timestamp: time,
            year,
        }
    }

    fn Check_service(&self) -> bool { // Tomar prestado La estructura WebService, ya que es del monton       
    
        false
    }
}

fn main() {
    /*let aws: WebService = WebService {
        ttype: String::from("Cloud"),
        timestamp: String::from("27/07/2021"),
        year: 2021,
        desc: String::from("Servicio como infraestructura perteneciente a Amazon INC."),
    };*/

    let aws: WebService = WebService::New(String::from("Cloud"), String::from("Servicio como infraestructura perteneciente a Amazon INC."));
    
    

    println!("{:?}", aws);
}
