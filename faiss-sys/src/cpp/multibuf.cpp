#include "faiss-sys/src/cpp/multibuf.h"
#include "faiss-sys/src/multibuf.rs.h"
#include <c_api/index_io_c.h>
#include <cassert>
#include <cstring>
#include <faiss/impl/FaissAssert.h>
#include <faiss/index_io.h>
#include <iostream>
#include <memory>

using faiss::Index;
using namespace std;

int faiss_read_index_multibuf(MultiBuf &mb, int io_flags, FaissIndex **p_out) {
  try {
    auto reader = new MultiBufReader(mb);
    auto out = faiss::read_index(reader, io_flags);
    *p_out = reinterpret_cast<FaissIndex *>(out);
  } catch (faiss::FaissException &e) {
    std::cerr << e.what() << '\n';
    // faiss_last_exception = std::make_exception_ptr(e);
    return -2;
  } catch (std::exception &e) {
    std::cerr << e.what() << '\n';
    // faiss_last_exception = std::make_exception_ptr(e);
    return -4;
  } catch (...) {
    std::cerr << "Unrecognized exception!\n";
    // faiss_last_exception =
    //     std::make_exception_ptr(std::runtime_error("Unknown error"));
    return -1;
  }
  return 0;
}

MultiBufReader::MultiBufReader(MultiBuf &mb)
    : multiBuf(mb), chunk_size(0), offset(0) {}

MultiBufReader::~MultiBufReader() { close(multiBuf); }

int MultiBufReader::fileno() {
  FAISS_THROW_MSG("MultiBufReader does not support memory mapping");
}

size_t MultiBufReader::operator()(void *ptr, size_t unitsize, size_t nitems) {
  size_t size = unitsize * nitems;
  if (size == 0)
    return 0;
  char *dst = (char *)ptr;
  size_t nb;

  { // first copy available bytes
    nb = std::min(chunk_size - offset, size);
    if (nb > 0) {
      memcpy(dst, chunk + offset, nb);
      offset += nb;
      dst += nb;
      size -= nb;
    }
  }

  // while we would like to have more data
  while (size > 0) {
    assert(offset == chunk_size); // buffer empty on input
    // try to read from main reader
    auto next_chunk_result = next_chunk(multiBuf);
    chunk_size = next_chunk_result.size();
    if (chunk_size == 0) {
      close(multiBuf);
      break;
    }
    chunk = reinterpret_cast<const char *>(next_chunk_result.data());

    offset = 0;
    // copy remaining bytes
    size_t nb2 = std::min(chunk_size, size);
    memcpy(dst, chunk, nb2);
    offset = nb2;
    nb += nb2;
    dst += nb2;
    size -= nb2;
  }
  return nb / unitsize;
}
