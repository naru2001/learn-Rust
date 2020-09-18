

fn main(){
    let imp="Hello world".to_string();
    calc_data(&imp); //値の参照を渡している(アクセスを許す)
    println!("{}",imp);
}

fn calc_data(data:&String){
    println!("{}",data);
}