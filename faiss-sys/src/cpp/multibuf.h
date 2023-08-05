#ifndef MULTIBUF_H_
#define MULTIBUF_H_

#pragma once
#include "rust/cxx.h"
#include <c_api/index_io_c.h>
#include <faiss/impl/io.h>
#include <memory>

using namespace faiss;

/* Defined in Rust code */
struct MultiBuf;

// std::vector<char> next_chunk(MultiBuf *mb);

// void close(MultiBuf *mb);

/* Read an index using bytes from MultiBuf - called from Rust */
int faiss_read_index_multibuf(MultiBuf &mb, int io_flags, FaissIndex **p_out);

/* MultiBufReader provides a way to read an index from a byte-stream
 * provided from rust code via the `MultiBuf` type. */
struct MultiBufReader : IOReader {
  MultiBuf &multiBuf;

  size_t chunk_size, offset;
  const char *chunk = nullptr;

  MultiBufReader(MultiBuf &mb);
  ~MultiBufReader() override;

  size_t operator()(void *ptr, size_t size, size_t nitems) override;
  int fileno() override;
};

#endif // MULTIBUF_H_
