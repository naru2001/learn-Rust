use std::sync::mpsc;
use std::thread;

fn main(){
    let (tx,rx)=mpsc::channel(); //(送信,受信)のタプルが戻り値
    let handle = thread::spawn(move ||{
        let data=rx.recv().unwrap(); //recvで受け取る
        println!("{}",data);
    });
    let _=tx.send("Hello"); //sendで送る
    let _ = handle.join();
}