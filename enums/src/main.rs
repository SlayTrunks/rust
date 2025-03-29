
fn main(){
    

    value_in_cents(1);

}

fn value_in_cents(coin: u8)  {
    match coin {
        1 => {println!("hello world");}
        _ => {println!("not hello ");}
    }
}
