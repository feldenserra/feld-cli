// Utils  
// feld-cli
// -------------------------------------------------------- //
use std::time::{ SystemTime, UNIX_EPOCH };

pub fn getRandomIndex(arrLen: usize) -> usize {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() % 1000;

    return (now % arrLen as u128) as usize;
}

//#[macro_export]
//macro_rules! printNL {
//    (_) =>{
//        println!("\n");
//    }
//}
