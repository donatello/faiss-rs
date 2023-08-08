use crate::FaissIndex;

use bytes::Bytes;
use cxx::{type_id, ExternType};
use tokio::sync::mpsc::{Receiver, Sender};

unsafe impl ExternType for FaissIndex {
    type Id = type_id!("FaissIndex");
    type Kind = cxx::kind::Opaque;
}

// use crate::faiss_sys;

#[cxx::bridge()]
pub mod ffi {
    extern "Rust" {
        type BufReceiver;
        fn recv_chunk(buf: &mut BufReceiver) -> &[u8];

        type BufSender;
        fn send_chunk(buf: &mut BufSender, chunk: &[u8]) -> bool;
    }

    unsafe extern "C++" {
        include!("faiss-sys/src/cpp/multibuf.h");

        type FaissIndex = crate::FaissIndex;
        unsafe fn faiss_read_index_br(
            br: &mut BufReceiver,
            io_flags: i32,
            p_out: *mut *mut FaissIndex,
        ) -> i32;

        unsafe fn faiss_write_index_bs(idx: *const FaissIndex, bs: &mut BufSender, bsz: i32)
            -> i32;
    }
}

pub struct BufReceiver {
    recv: Receiver<Option<Bytes>>,

    curr_chunk: Bytes,
}

pub fn new_bufreceiver(recv: Receiver<Option<Bytes>>) -> BufReceiver {
    BufReceiver {
        recv,
        curr_chunk: Bytes::new(),
    }
}

pub fn recv_chunk(buf: &mut BufReceiver) -> &[u8] {
    // let next = (buf.callback)();
    let next = buf.recv.blocking_recv();
    match next {
        Some(Some(v)) => {
            buf.curr_chunk = v;
            &buf.curr_chunk[..]
        }
        _ => {
            buf.recv.close();
            &[]
        }
    }
}

pub struct BufSender {
    send: Sender<Option<Bytes>>,
}

pub fn new_bufsender(send: Sender<Option<Bytes>>) -> BufSender {
    BufSender { send }
}

// An empty chunk here indicates to the recv end that write has finished.
pub fn send_chunk(buf: &mut BufSender, chunk: &[u8]) -> bool {
    let b = Bytes::copy_from_slice(chunk);
    let v = {
        if b.len() == 0 {
            None
        } else {
            Some(b)
        }
    };
    match buf.send.blocking_send(v) {
        Ok(_) => true,
        Err(_) => false,
    }
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
