use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect(); // コマンドライン引数取得
    let addr = &args[1];
    println!("Address: {}", addr);
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?; // [1]: リッスンモードのソケットを指定アドレスで作成
    loop {
        let (mut stream, _) = listener.accept()?; // [2]: スレッドをブロックし，クライアントからのコネクション確率要求を待機．TCPのスリーウェイハンドシェイクでコネクションが確立されたらブロックを解除．
        thread::spawn(move || {
            // [3]: 新たにスレッドを生成し起動．メインスレッドは[2]に戻り，新たなスレッドは[4]へ移る．
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).unwrap(); // [4]
                if nbytes == 0 {
                    // [6]
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                stream.write_all(&buffer[..nbytes]).unwrap(); // [5]
            }
        });
    }
}
