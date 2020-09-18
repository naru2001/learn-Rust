struct Droppable;

impl Drop for Droppable{ //Dropトレイトでリソースの解法
    fn drop(&mut self){
        println!("Resource will be released.");
    }
}
fn main(){
    {
        let b=Droppable;
    }
    println!("The Droppable should bereleased at the end of block.")
}