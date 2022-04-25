fn look_at_apples(_apples: i32) {
    if _apples >= 10 && _apples <100 {
           println!("Count: normal consecutive apples");
   } else {
           if _apples >= 100 && _apples <1000 {
               println!("Count: good number of apples");
   } else {
           if _apples >= 1000 && _apples <9000 {
                   println!("Count: lots of apples");
                   } else {
           if _apples >= 9000 {
                   println!("Count: over nine thousand");
           }
        }
     }
   }
   }               

   fn look_at_apples1(apples: i32) {
    let count = if apples > 9000 {
        "over nine thousand"
    } else if apples >= 1000 {
        "lots of apples"
    } else if apples >= 100 {
        "good number of apples"
    } else {
        "normal consecutive apples"
    };

    println!("Count: {}", count);
}
fn main() {
    look_at_apples1(10);
    look_at_apples(100);
    look_at_apples(1000);
    look_at_apples(9001);

}