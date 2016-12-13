#include <algorithm>
#include <iostream>
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

#include "TableGenWrapper.h"

using namespace llvm;

int main() {
  // RecordKeeper Records;
  // SourceMgr SrcMgr;

  auto input_filename = "/home/astocko/dev/public/tablegen-rs/lib/third-party/llvm-3.9.1/lib/Target/X86/X86.td";

  // ErrorOr<std::unique_ptr<MemoryBuffer>> FileOrErr =
  //   MemoryBuffer::getFileOrSTDIN(input_filename);

  // if (std::error_code EC = FileOrErr.getError()) {
  //   std::cout << "Could not open input file: " << input_filename << std::endl;
  //   return 1;
  // }

  // SrcMgr.AddNewSourceBuffer(std::move(*FileOrErr), SMLoc());

  // std::vector<std::string> includes;
  // includes.push_back("/home/astocko/dev/public/tablegen-rs/lib/third-party/llvm-3.9.1/lib/Target/X86");
  // includes.push_back("/home/astocko/dev/public/tablegen-rs/lib/third-party/llvm-3.9.1/include");

  // SrcMgr.setIncludeDirs(includes);

  // TGParser Parser(SrcMgr, Records);

  std::cout << "Parsing" << std::endl;

  auto inc1 = "/home/astocko/dev/public/tablegen-rs/lib/third-party/llvm-3.9.1/lib/Target/X86";
  auto inc2 = "/home/astocko/dev/public/tablegen-rs/lib/third-party/llvm-3.9.1/include";

  const char* includesv[] = {inc1, inc2};


  auto tg = tablegen_new(input_filename, 2, includesv);

  if (tg != nullptr) {
    std::cout << "It worked!" << std::endl;
    tg->parser->ParseFile();
  } else {
    std::cout << "It didn't work!!!" << std::endl;
  }

  tablegen_destroy(tg);

  // if (Parser.ParseFile())
  //   return 1;

  // for (const auto &d : Records.getDefs()) {
  //   auto cz = d.second.get()->getValue("CodeSize");
  //   if (cz != nullptr) {
  //     std::cout << cz->getName() << std::endl;
  //     std::cout << cz->getType()->getAsString() << std::endl;
  //   }
  // }


  std::cout << "Done" << std::endl;




  return 0;
}
