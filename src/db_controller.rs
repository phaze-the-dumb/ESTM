use std::{ env, fs, process::{ Command, Stdio } };
use rand::Rng;

#[derive(Debug, Clone)]
pub struct DBController{
  port: u16
}

impl DBController{
  pub fn new() -> Self{
    if fs::metadata("dbdata").is_err(){ fs::create_dir("dbdata").unwrap(); }
    let port: u16 = 27021; //rand::thread_rng().gen_range(27021..27061);

    Command::new("C:\\Program Files\\MongoDB\\Server\\7.0\\bin\\mongod.exe")
      .current_dir("C:\\Program Files\\MongoDB\\Server\\7.0\\bin")
      .arg(format!("--dbpath={}", env::current_dir().unwrap().join("dbdata").to_str().unwrap()))
      .arg(format!("--port={}", port))
      .stdout(Stdio::null())
      .spawn().unwrap();

    println!("MongoDB listening on {}", port);
    Self { port }
  }

  pub fn get_connection_uri( &self ) -> String{
    format!("mongodb://127.0.0.1:{}/estm", self.port)
  }
}