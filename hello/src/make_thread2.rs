use std::thread;
use std::sync::{Arc,Mutex};

fn main(){
    let mut handles=Vec::new();
    let data=Arc::new(Mutex::new(vec![1;10]));

    for i in 0..10{
        let data_ref=data.clone();
        handles.push(thread::spawn(move || {
            let mut data=data_ref.lock().unwrap();
            data[i]+=1;
        }));
    }
    for handle in handles{
        let _=handle.join();
    }

    dbg!(data);
}