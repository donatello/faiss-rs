//! Index I/O functions

use crate::error::{Error, Result};
use crate::faiss_try;
use crate::index::{CpuIndex, FromInnerPtr, IndexImpl, NativeIndex};
use faiss_sys::*;
use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr;

pub use super::io_flags::IoFlags;

pub use faiss_sys::{new_bufreceiver, new_bufsender, BufReceiver, BufSender};

pub fn read_index_br(br: &mut BufReceiver) -> Result<IndexImpl> {
    unsafe {
        let mut inner = ptr::null_mut();
        faiss_try(faiss_read_index_br(
            br,
            IoFlags::MEM_RESIDENT.into(),
            &mut inner,
        ))?;
        Ok(IndexImpl::from_inner_ptr(inner))
    }
}

pub fn write_index_bs<I>(index: &I, bs: &mut BufSender, buf_size: i32) -> Result<()>
where
    I: NativeIndex,
    I: CpuIndex,
{
    unsafe {
        faiss_try(faiss_write_index_bs(index.inner_ptr(), bs, buf_size))?;
        Ok(())
    }
}

/// Write an index to a file.
///
/// # Error
///
/// This function returns an error if the description contains any byte with the value `\0` (since
/// it cannot be converted to a C string), or if the internal index writing operation fails.
pub fn write_index<I, P>(index: &I, file_name: P) -> Result<()>
where
    I: NativeIndex,
    I: CpuIndex,
    P: AsRef<str>,
{
    unsafe {
        let f = file_name.as_ref();
        let f = CString::new(f).map_err(|_| Error::BadFilePath)?;

        faiss_try(faiss_write_index_fname(index.inner_ptr(), f.as_ptr()))?;
        Ok(())
    }
}

/// Read an index from a file.
///
/// # Error
///
/// This function returns an error if the description contains any byte with the value `\0` (since
/// it cannot be converted to a C string), or if the internal index reading operation fails.
pub fn read_index<P>(file_name: P) -> Result<IndexImpl>
where
    P: AsRef<str>,
{
    unsafe {
        let f = file_name.as_ref();
        let f = CString::new(f).map_err(|_| Error::BadFilePath)?;
        let mut inner = ptr::null_mut();
        faiss_try(faiss_read_index_fname(
            f.as_ptr(),
            IoFlags::MEM_RESIDENT.into(),
            &mut inner,
        ))?;
        Ok(IndexImpl::from_inner_ptr(inner))
    }
}

/// Read an index from a file with I/O flags.
///
/// You can memory map some index types with this.
///
/// # Error
///
/// This function returns an error if the description contains any byte with the value `\0` (since
/// it cannot be converted to a C string), or if the internal index reading operation fails.
pub fn read_index_with_flags<P>(file_name: P, io_flags: IoFlags) -> Result<IndexImpl>
where
    P: AsRef<str>,
{
    unsafe {
        let f = file_name.as_ref();
        let f = CString::new(f).map_err(|_| Error::BadFilePath)?;
        let mut inner = ptr::null_mut();
        faiss_try(faiss_read_index_fname(
            f.as_ptr(),
            io_flags.0 as c_int,
            &mut inner,
        ))?;
        Ok(IndexImpl::from_inner_ptr(inner))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::{Read, Write};

    use bytes::{Bytes, BytesMut};
    use tokio::runtime::Builder;
    use tokio::sync::mpsc::{channel, Receiver, Sender};

    use super::*;
    use crate::index::flat::FlatIndex;
    use crate::index::Index;
    const D: u32 = 8;

    #[test]
    fn write_read() {
        let mut index = FlatIndex::new_l2(D).unwrap();
        assert_eq!(index.d(), D);
        assert_eq!(index.ntotal(), 0);
        let some_data = &[
            7.5_f32, -7.5, 7.5, -7.5, 7.5, 7.5, 7.5, 7.5, -1., 1., 1., 1., 1., 1., 1., -1., 4.,
            -4., -8., 1., 1., 2., 4., -1., 8., 8., 10., -10., -10., 10., -10., 10., 16., 16., 32.,
            25., 20., 20., 40., 15.,
        ];
        index.add(some_data).unwrap();
        assert_eq!(index.ntotal(), 5);

        let filepath = ::std::env::temp_dir().join("test_write_read.index");
        let filename = filepath.to_str().unwrap();
        write_index(&index, filename).unwrap();
        let index = read_index(&filename).unwrap();
        assert_eq!(index.ntotal(), 5);
        ::std::fs::remove_file(&filepath).unwrap();
    }

    #[test]
    fn test_read_with_flags() {
        let index = read_index_with_flags("file_name", IoFlags::MEM_MAP | IoFlags::READ_ONLY);
        // we just want to ensure the method signature is right here
        assert!(index.is_err());
    }

    struct FileBufRecvr {
        f: fs::File,
        chunk_size: usize,
        tx: Sender<Option<Bytes>>,
    }

    impl FileBufRecvr {
        fn new(name: &str, chunk_size: usize) -> (Self, Receiver<Option<Bytes>>) {
            let f = fs::File::open(name).unwrap();
            let (tx, rx) = channel(1);
            (FileBufRecvr { f, chunk_size, tx }, rx)
        }

        async fn read_task(&mut self) {
            loop {
                let mut buf = BytesMut::zeroed(self.chunk_size);
                let v = self.f.read(buf.as_mut()).unwrap();
                let _ = buf.split_off(v);
                let send_buf = buf.freeze();
                if send_buf.len() == 0 {
                    self.tx.send(None).await.unwrap();
                    return;
                }
                self.tx.send(Some(send_buf)).await.unwrap();
            }
        }
    }

    #[test]
    fn write_read_bufrecv() {
        let mut index = FlatIndex::new_l2(D).unwrap();
        assert_eq!(index.d(), D);
        assert_eq!(index.ntotal(), 0);
        let some_data = &[
            7.5_f32, -7.5, 7.5, -7.5, 7.5, 7.5, 7.5, 7.5, -1., 1., 1., 1., 1., 1., 1., -1., 4.,
            -4., -8., 1., 1., 2., 4., -1., 8., 8., 10., -10., -10., 10., -10., 10., 16., 16., 32.,
            25., 20., 20., 40., 15.,
        ];
        index.add(some_data).unwrap();
        assert_eq!(index.ntotal(), 5);

        let filepath = ::std::env::temp_dir().join("test_write_read_bufrecv.index");
        let filename = filepath.to_str().unwrap();
        write_index(&index, filename).unwrap();

        let (mut fbr, recv) = FileBufRecvr::new(filename, 32);
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        let handle = rt.spawn(async move { fbr.read_task().await });
        let mut br = new_bufreceiver(recv);
        let index = read_index_br(&mut br).unwrap();
        rt.block_on(handle).unwrap();
        assert_eq!(index.ntotal(), 5);
        ::std::fs::remove_file(&filepath).unwrap();
    }

    struct FileBufSndr {
        f: fs::File,
        rx: Receiver<Option<Bytes>>,
    }

    impl FileBufSndr {
        fn new(name: &str) -> (Self, Sender<Option<Bytes>>) {
            let f = fs::File::create(name).unwrap();
            let (tx, rx) = channel(1);
            (FileBufSndr { f, rx }, tx)
        }

        async fn write_task(&mut self) {
            loop {
                let bufitem = self.rx.recv().await.unwrap();
                match bufitem {
                    Some(buf) => {
                        let v = self.f.write(buf.as_ref()).unwrap();
                        if v != buf.len() {
                            panic!("wrote only {} but had {}", v, buf.len());
                        }
                    }
                    None => return,
                }
            }
        }
    }

    #[test]
    fn write_read_bufsend() {
        let mut index = FlatIndex::new_l2(D).unwrap();
        assert_eq!(index.d(), D);
        assert_eq!(index.ntotal(), 0);
        let some_data = &[
            7.5_f32, -7.5, 7.5, -7.5, 7.5, 7.5, 7.5, 7.5, -1., 1., 1., 1., 1., 1., 1., -1., 4.,
            -4., -8., 1., 1., 2., 4., -1., 8., 8., 10., -10., -10., 10., -10., 10., 16., 16., 32.,
            25., 20., 20., 40., 15.,
        ];
        index.add(some_data).unwrap();
        assert_eq!(index.ntotal(), 5);

        let filepath = ::std::env::temp_dir().join("test_write_read_bufsend.index");
        let filename = filepath.to_str().unwrap();

        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        let (mut fbs, send) = FileBufSndr::new(filename);
        let write_handle = rt.spawn(async move { fbs.write_task().await });

        // Write the index to the bufsender
        let mut bs = new_bufsender(send);
        write_index_bs(&index, &mut bs, 32).unwrap();
        rt.block_on(write_handle).unwrap();

        // Read the index back from a bufreceiver and check.
        let (mut fbr, recv) = FileBufRecvr::new(filename, 32);
        let read_handle = rt.spawn(async move { fbr.read_task().await });
        let mut br = new_bufreceiver(recv);
        let index = read_index_br(&mut br).unwrap();
        rt.block_on(read_handle).unwrap();
        assert_eq!(index.ntotal(), 5);
        ::std::fs::remove_file(&filepath).unwrap();
    }
}
