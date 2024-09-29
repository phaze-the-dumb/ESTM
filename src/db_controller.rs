use std::{ env, fs, process::{ Command, Stdio } };
// use rand::Rng;

#[derive(Debug, Clone)]
pub struct DBController{
  port: u16
}

impl DBController{
  pub fn new() -> Self{
    // Check if the data folder for the database exists, if not, create a new one
    if fs::metadata("dbdata").is_err(){ fs::create_dir("dbdata").unwrap(); }

    //              v The database will listen on this port. We can randomise this by replacing "27021" with "rand::thread_rng().gen_range(27021..27061);"
    let port: u16 = 27021;

    // Start MongoDB as a child process, this will make sure it is always running when we want it to be.
    Command::new("C:\\Program Files\\MongoDB\\Server\\7.0\\bin\\mongod.exe")
      .current_dir("C:\\Program Files\\MongoDB\\Server\\7.0\\bin")
      .arg(format!("--dbpath={}", env::current_dir().unwrap().join("dbdata").to_str().unwrap())) // Tell mongo to use our data path
      .arg(format!("--port={}", port)) // Tell mongo to use the port that we have assigned to it
      .arg("--bind_ip=127.0.0.1") // Tell mongo to listen on 127.0.0.1 meaning only this local machine can access it
      .stdout(Stdio::null()) // Set the stdio to null so mongo doesn't spam our logs
      .spawn().unwrap();

    // Tell the user what port mongodb is listening on, mainly for dev purposes, we can ommit this in prod
    println!("MongoDB listening on {}", port);
    Self { port }
  }

  pub fn get_connection_uri( &self ) -> String{
    // Generate a MongoDB connection uri
    format!("mongodb://127.0.0.1:{}/estm", self.port)
  }
}