
fn main(){
    let x=5; //不変な変数
    let y=&x;
    let z=&x; //いくらでも参照が渡せる
    dbg!(x);
    dbg!(y);
    dbg!(z);

    let mut a=3; //可変な変数

    { //可変な参照渡し
        let b=&mut a; //1回目は可能
        let c=&mut a; //2回目は不可能
        dbg!(b);
        dbg!(c);
    }

    { //不変と可変
        let b=&a; //1回目は可能
        let c=&mut a; //2回目は不可能
        dbg!(b);
        dbg!(c);
    }
}
