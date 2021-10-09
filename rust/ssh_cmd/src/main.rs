use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
extern crate rpassword;
use rpassword::read_password;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "structopt example", about = "using structop")]
struct Args {
    #[structopt(short = "h", long)]
    host: String,
    #[structopt(short = "c", long)]
    command: String,
    #[structopt(short = "u", long)]
    username: String,
}

fn main() {
    println!("Enter your password: ");
    let password = read_password().unwrap();
    let args = Args::from_args();
    println!(
        "Running command {} against host {}:\n",
        args.command, args.host
    );
    let tcp = TcpStream::connect(args.host + ":22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&args.username, &password).unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec(&args.command).unwrap();
    let mut command_output = String::new();
    channel.read_to_string(&mut command_output).unwrap();
    println!("{}", command_output);
    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}