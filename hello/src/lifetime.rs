
fn main(){
    let y;

    {
        let x=5; // x~

        y=&x; // y~

        dbg!(x); // ~x
    }

    dbg!(y); // ~y(xのライフタイムを超えることはできない)
}
