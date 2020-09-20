use std::thread;

fn main(){
    let mut handles=Vec::new();

    for x in 0..10{
        handles.push(thread::spawn(move || { // xを参照したスレッドがxより長生きする可能性があるためmoveで所有権を移す
            println!("Hello : {}.",x);
        }));
    }

    for h in handles{ //マルチスレッドプログラムは各スレッドの実行順序が不確定
        let _ =h.join();
    }
}