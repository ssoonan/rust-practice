use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

/// 서버를 실행하는 함수
fn run_server(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Server listening on {}", address);

    // 모든 클라이언트의 송신 스트림을 저장할 벡터
    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let peer_addr = stream.peer_addr()?;
        println!("New client connected: {}", peer_addr);

        // 클라이언트 목록에 등록
        let clients_clone = Arc::clone(&clients);
        {
            let mut lock = clients_clone.lock().unwrap(); // 왜 여기서 lock을 걸지? push, write를 하니까 lock을 거는 건가
            lock.push(stream.try_clone()?); // lock을 잡은 이후 새로운 steram을 추가하여 Vec<TcpStream>에 추가함
        }

        // 각 클라이언트마다 스레드를 생성해 메시지 수신 및 브로드캐스트
        let clients_clone = Arc::clone(&clients);
        thread::spawn(move || {
            handle_client(stream, clients_clone, peer_addr);
        });
    }

    Ok(())
}

/// 클라이언트에서 오는 메시지를 받으면 모든 클라이언트에게 브로드캐스트하는 함수
fn handle_client(
    stream: TcpStream,
    clients: Arc<Mutex<Vec<TcpStream>>>,
    peer_addr: std::net::SocketAddr,
) {
    let reader = BufReader::new(stream.try_clone().unwrap());

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Error reading from client: {}", peer_addr);
                break;
            }
        }; // match를 거쳐 line이 처리됨, 이걸 쓰면 됨

        let message = format!("{}: {}", peer_addr, line);

        // 브로드캐스트
        let lock = clients.lock().unwrap();
        for client_stream in lock.iter() {
            let mut cs = client_stream.try_clone().unwrap();
            if let Err(err) = writeln!(cs, "{}", message) {
                eprintln!(
                    "Failed to send message to {}: {}",
                    cs.peer_addr().unwrap(),
                    err
                );
            }
        }
    }

    // 클라이언트가 연결을 끊으면 리스트에서 제거
    {
        let mut lock = clients.lock().unwrap();
        lock.retain(|client_stream| client_stream.peer_addr().unwrap() != peer_addr);
    }

    println!("Client disconnected: {}", peer_addr);
}

/// 클라이언트 모드를 실행하는 함수
fn run_client(address: &str) -> std::io::Result<()> {
    let stream = TcpStream::connect(address)?;
    let peer_addr = stream.peer_addr()?;
    println!("Connected to server at {} as {}", address, peer_addr);

    let mut send_stream = stream.try_clone()?;
    let mut recv_stream = stream.try_clone()?;

    // 서버에서 오는 메시지를 수신하는 스레드
    thread::spawn(move || {
        let reader = BufReader::new(&mut recv_stream);
        for line in reader.lines() {
            match line {
                Ok(msg) => {
                    println!("{}", msg);
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });

    // 사용자 입력을 받아 서버에 전송
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        writeln!(send_stream, "{}", line)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // 예: cargo run -- server 127.0.0.1:9000
    //    cargo run -- client 127.0.0.1:9000

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <server|client> <address:port>", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    let address = &args[2];

    match mode.as_str() {
        "server" => run_server(address)?,
        "client" => run_client(address)?,
        _ => {
            eprintln!("Unknown mode: {}", mode);
            std::process::exit(1);
        }
    }

    Ok(())
}
