use dhat;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

mod aes_tbox;
use std::time;
use aes_tbox::*;
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _dhat = dhat::Profiler::new_heap();
    let encryption = AES128::new_from_str("Tachibana_Hinano");
    let data = vec![0x00u8; 1<<4];
    let now = time::Instant::now();
    let cipher = (encryption.encrypt)(&encryption, &data);
    println!("{:?}", now.elapsed());
}
