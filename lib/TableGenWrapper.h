#pragma once

#include <algorithm>
#include <cstdint>
#include <string>
#include <vector>

#include <llvm/Support/CommandLine.h>
#include <llvm/Support/FileSystem.h>
#include <llvm/Support/MemoryBuffer.h>
#include <llvm/Support/SourceMgr.h>
#include <llvm/Support/ToolOutputFile.h>
#include <llvm/TableGen/Error.h>
#include <llvm/TableGen/Record.h>

#include <TableGen/TGParser.h>

struct TableGen {
  llvm::TGParser *parser;
  llvm::RecordKeeper *records;
  llvm::SourceMgr *source_mgr;
};

extern "C" {
  TableGen* tablegen_new(const char* input, size_t includesc, const char* includesv[]);
  void tablegen_destroy(const TableGen* tg);

  void tablegen_parse_file(const TableGen* tg);
}
