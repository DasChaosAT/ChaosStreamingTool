use std::env;
use std::io::prelude::*;
use std::time::Duration;
use std::net::{SocketAddrV4,Ipv4Addr,TcpStream};
//use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("My path is {}.", args[0]);
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);

    teamspeak_toggle_mic();
}

fn teamspeak_toggle_mic(){
    let ts3_apikey = "K008-E1SU-AH50-NKO7-NEP0-TCV4";
    let ip_v4 = Ipv4Addr::new(127,0,0,1);
    let port = 25639;
    let socket = SocketAddrV4::new(ip_v4, port);

    let stream= TcpStream::connect(socket)
        .expect("Couldn't connect to Server!");

    //let mut buffer = String::new();
    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
    //let _ = stream.read_to_string(&mut buffer);

    let mut request = "auth apikey=".to_owned();
    request.push_str(ts3_apikey);
    let answer = command_teamspeak(request,&stream);
    if answer.contains("error id=0 msg=ok"){
        request = "whoami".to_owned();
        let answer = command_teamspeak(request,&stream);

        if answer.contains("error id=0 msg=ok"){
            let tmpv: Vec<&str> = answer.split_terminator(' ').collect();
            let tmp: Vec<&str> = tmpv[0].split_terminator("=").collect();
            let clid = tmp[1];

            request = "clientvariable clid=".to_owned();
            request.push_str(clid);
            request.push_str(" client_input_muted");
            let answer = command_teamspeak(request, &stream);

            if answer.contains("error id=0 msg=ok"){
                let tmpv: Vec<&str> = answer.split_terminator("client_input_muted=").collect();
                let tmp: Vec<&str> = tmpv[1].split_terminator("\n\r").collect();

                //Convert &str to int
                let tmp_number:Result<i32, _> = tmp[0].trim().parse();
                let mut mic_mute_status = match tmp_number {
                    Ok(mic_mute_status) => mic_mute_status,
                    Err(e) => {
                        println!("please input a number ({})", e);
                        return;
                    }
                };

                mic_mute_status ^= 1;
                let mut request = "clientupdate client_input_muted=".to_owned();
                request.push_str(mic_mute_status.to_string().as_str());
                let answer = command_teamspeak(request,&stream);
                if answer.contains("error id=0 msg=ok") {
                    println!("Teamspeak Microphone toggled");
                }else {
                    println!("Can't toggle microphone");
                }
            }
            else{
                println!("Can't get microphone mute status");
            }
        }else{
            println!("clid not found");
        }
    }else{
        println!("Authentication Failed, API-Key not working!");
    }

}

fn command_teamspeak(command: String, stream: &TcpStream) -> String{
    let mut stream = stream;

    let mut request = command.to_owned();
    request.push_str("\r\n");
    stream.write(request.as_bytes()).unwrap();

    let mut buffer_answer = String::new();
    let _ = stream.read_to_string(&mut buffer_answer);

    return buffer_answer;
}
