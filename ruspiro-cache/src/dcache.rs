/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # Data Cache Operations
//!

use ruspiro_arch_aarch64::register::{
  el0::ctr_el0, el1::ccsidr_el1, el1::clidr_el1, el1::csselr_el1,
};

use ruspiro_arch_aarch64::instructions::{dsb, isb};

#[derive(Clone)]
enum CacheOperation {
  Clean,
  Invalidate,
  Flush,
}

#[derive(Clone, Copy)]
#[repr(u64)]
enum CacheLevel {
  L1 = 0,
  L2 = 1,
  L3 = 2,
}

pub unsafe fn invalidate_dcache() {
  dsb();

  // get level of coherency
  let loc = clidr_el1::read(clidr_el1::LOC::Field);
  if loc.value() != 0 {
    maintain_dcache(CacheOperation::Invalidate, CacheLevel::L1);
    maintain_dcache(CacheOperation::Invalidate, CacheLevel::L2);
    maintain_dcache(CacheOperation::Invalidate, CacheLevel::L3);
  }

  csselr_el1::write(csselr_el1::LEVEL::L1);
  dsb();
  isb();
}

pub unsafe fn clean_dcache() {
  dsb();

  // get level of coherency
  let loc = clidr_el1::read(clidr_el1::LOC::Field);
  if loc.value() != 0 {
    maintain_dcache(CacheOperation::Clean, CacheLevel::L1);
    maintain_dcache(CacheOperation::Clean, CacheLevel::L2);
    maintain_dcache(CacheOperation::Clean, CacheLevel::L3);
  }

  csselr_el1::write(csselr_el1::LEVEL::L1);
  dsb();
  isb();
}

pub unsafe fn flush_dcache() {
  dsb();

  // get level of coherency
  let loc = clidr_el1::read(clidr_el1::LOC::Field);
  if loc.value() != 0 {
    maintain_dcache(CacheOperation::Flush, CacheLevel::L1);
    maintain_dcache(CacheOperation::Flush, CacheLevel::L2);
    maintain_dcache(CacheOperation::Flush, CacheLevel::L3);
  }

  csselr_el1::write(csselr_el1::LEVEL::L1);
  dsb();
  isb();
}

unsafe fn maintain_dcache(operation: CacheOperation, level: CacheLevel) {
  // get the cache type for the requested cache level
  let cache_type = match level {
    CacheLevel::L1 => clidr_el1::read(clidr_el1::CTYPE1::Field).value(),
    CacheLevel::L2 => clidr_el1::read(clidr_el1::CTYPE2::Field).value(),
    CacheLevel::L3 => clidr_el1::read(clidr_el1::CTYPE3::Field).value(),
  };
  // if cache type is "no cache" or "instruction cache only" nothing to do, otherwise
  // process this cache leval
  if cache_type >= 0x2 {
    // select the cache level for the cache operations
    match level {
      CacheLevel::L1 => csselr_el1::write(csselr_el1::LEVEL::L1),
      CacheLevel::L2 => csselr_el1::write(csselr_el1::LEVEL::L2),
      CacheLevel::L3 => csselr_el1::write(csselr_el1::LEVEL::L3),
    }
    // instruction barrier to ensure the cache level has been choosen be the
    // previous instruction
    isb();
    let cache_line_size = ccsidr_el1::read(ccsidr_el1::LINESIZE::Field).value();
    let cache_line_size_x2 = cache_line_size + 4;
    let assoc = ccsidr_el1::read(ccsidr_el1::ASSOC::Field).value();
    // TODO: find bit position clz w5, w4?
    let assoc_bit_x5 = core::intrinsics::ctlz(assoc); //??
    let num_sets_x7 = ccsidr_el1::read(ccsidr_el1::NUMSETS::Field).value();
    for num_sets in (0..num_sets_x7).rev() {
      for way in (0..assoc).rev() {
        let x6 = way << assoc_bit_x5;
        let x11 = ((level as u64) << 1) | x6;
        let x6 = num_sets << cache_line_size_x2;
        let x11 = x11 | x6;
        // invalidate data cache by set/way
        match operation {
          CacheOperation::Invalidate => llvm_asm!("dc isw, $0"::"r"(x11)::"volatile"),
          CacheOperation::Clean => llvm_asm!("dc csw, $0"::"r"(x11)::"volatile"),
          CacheOperation::Flush => llvm_asm!("dc cisw, $0"::"r"(x11)::"volatile"),
        }
      }
    }
  }
}

pub unsafe fn flush_dcache_range(from: usize, size: usize) {
  dsb();
  let dcls = dcache_line_size();
  let end = from + size;
  let start = from & !(dcls - 1);
  let mut current = start;
  while current < end {
    // clean & invalidate D line / unified line
    llvm_asm!("dc civac, $0"::"r"(current)::"volatile");
    current += dcls;
  }
  dsb();
}

unsafe fn dcache_line_size() -> usize {
  //let mut cls: usize;
  let dcls_log2 = ctr_el0::read(ctr_el0::DminLine::Field);
  (4 << dcls_log2.value()) as usize
}
