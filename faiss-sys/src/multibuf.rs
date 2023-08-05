use crate::FaissIndex;

use cxx::{type_id, ExternType};

unsafe impl ExternType for FaissIndex {
    type Id = type_id!("FaissIndex");
    type Kind = cxx::kind::Opaque;
}

// use crate::faiss_sys;

#[cxx::bridge()]
pub mod ffi {
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
        fn close(buf: &mut MultiBuf);
    }

    unsafe extern "C++" {
        include!("faiss-sys/src/cpp/multibuf.h");

        type FaissIndex = crate::FaissIndex;
        unsafe fn faiss_read_index_multibuf(
            mb: &mut MultiBuf,
            io_flags: i32,
            p_out: *mut *mut FaissIndex,
        ) -> i32;
    }
}

// An iterator over contiguous chunks of a discontiguous file object.
//
// Toy implementation uses a Vec<Vec<u8>> but in reality this might be iterating
// over some more complex Rust data structure like a rope, or maybe loading
// chunks lazily from somewhere.
pub struct MultiBuf {
    pub chunks: Vec<Vec<u8>>,
    pub pos: usize,
    pub is_closed: bool,
}

pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}

pub fn close(buf: &mut MultiBuf) {
    buf.is_closed = true;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
