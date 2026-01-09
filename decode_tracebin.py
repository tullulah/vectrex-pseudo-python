#!/usr/bin/env python3
"""Decode .tracebin produced by JSVecX full-trace binary streaming.

Record format (18 bytes):
  pc:u16, b0:u8,b1:u8,b2:u8,b3:u8, a:u8,b:u8,
  x:u16,y:u16,u:u16,s:u16, dp:u8, cc:u8
All values are big-endian for 16-bit fields.

Usage:
  python3 decode_tracebin.py path/to/file.tracebin --last 200
  python3 decode_tracebin.py path/to/file.tracebin --find-pc C000 --context 20
"""

from __future__ import annotations

import argparse
import os
import struct
from collections import deque
from dataclasses import dataclass
from typing import Deque, Iterator, Optional, Tuple

REC_SIZE = 18
REC_STRUCT = struct.Struct(">HBBBBBBHHHHBB")


def iter_records(data: bytes) -> Iterator[Tuple[int, int, int, int, int, int, int, int, int, int, int, int, int]]:
    if len(data) % REC_SIZE != 0:
        # allow partial tail (e.g. crash mid-flush)
        data = data[: len(data) - (len(data) % REC_SIZE)]
    for off in range(0, len(data), REC_SIZE):
        yield REC_STRUCT.unpack_from(data, off)


def parse_hex16(s: str) -> int:
    return int(s, 16) & 0xFFFF


def is_mapped_vectrex_pc(pc: int) -> bool:
    # Vectrex memory map (common):
    # - Cartridge ROM: 0x0000-0x7FFF
    # - RAM:          0xC800-0xCFFF
    # - I/O:          0xD000-0xD7FF
    # - BIOS ROM:     0xE000-0xFFFF
    return (
        (0x0000 <= pc <= 0x7FFF)
        or (0xC800 <= pc <= 0xCFFF)
        or (0xD000 <= pc <= 0xD7FF)
        or (0xE000 <= pc <= 0xFFFF)
    )


@dataclass(frozen=True)
class IndexedRec:
    idx: int
    rec: Tuple[int, int, int, int, int, int, int, int, int, int, int, int, int]


def fmt(rec) -> str:
    (pc, b0, b1, b2, b3, a, b, x, y, u, s, dp, cc) = rec
    return (
        f"{pc:04X} {b0:02X} {b1:02X} {b2:02X} {b3:02X} "
        f"A={a:02X} B={b:02X} X={x:04X} Y={y:04X} U={u:04X} S={s:04X} DP={dp:02X} CC={cc:02X}"
    )


def find_first_match(
    data: bytes,
    *,
    predicate,
    context: int,
) -> Optional[Tuple[IndexedRec, list[IndexedRec], list[IndexedRec]]]:
    prev: Deque[IndexedRec] = deque(maxlen=context)
    it = iter_records(data)
    for idx, rec in enumerate(it):
        pc = rec[0]
        cur = IndexedRec(idx=idx, rec=rec)
        if predicate(pc):
            pre = list(prev)
            post: list[IndexedRec] = []
            for _ in range(context):
                try:
                    nxt = next(it)
                except StopIteration:
                    break
                idx += 1
                post.append(IndexedRec(idx=idx, rec=nxt))
            return cur, pre, post
        prev.append(cur)
    return None


def find_first_entry_into_range(
    data: bytes,
    *,
    start: int,
    end: int,
    context: int,
) -> Optional[Tuple[IndexedRec, Optional[IndexedRec], list[IndexedRec], list[IndexedRec]]]:
    # Finds the first record whose PC is in [start,end] while the previous record's PC is outside.
    # Returns: (current, previous, pre_context (excluding previous), post_context)
    prev_ctx: Deque[IndexedRec] = deque(maxlen=context)
    prev: Optional[IndexedRec] = None
    it = iter_records(data)
    for idx, rec in enumerate(it):
        pc = rec[0]
        cur = IndexedRec(idx=idx, rec=rec)
        in_range = start <= pc <= end
        prev_in_range = prev is not None and (start <= prev.rec[0] <= end)
        if in_range and not prev_in_range:
            pre = list(prev_ctx)
            post: list[IndexedRec] = []
            for _ in range(context):
                try:
                    nxt = next(it)
                except StopIteration:
                    break
                idx += 1
                post.append(IndexedRec(idx=idx, rec=nxt))
            return cur, prev, pre, post
        prev_ctx.append(cur)
        prev = cur
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("path")
    ap.add_argument("--last", type=int, default=0, help="print last N records")
    ap.add_argument("--find-pc", default=None, help="hex PC to find (e.g. C000)")
    ap.add_argument(
        "--find-pc-range",
        nargs=2,
        metavar=("START", "END"),
        default=None,
        help="find first PC in inclusive hex range (e.g. 8000 BFFF)",
    )
    ap.add_argument(
        "--find-unmapped",
        action="store_true",
        help="find first PC that is outside the standard Vectrex mapped regions",
    )
    ap.add_argument(
        "--find-entry-range",
        nargs=2,
        metavar=("START", "END"),
        default=None,
        help="find first transition where PC enters the inclusive hex range from outside",
    )
    ap.add_argument("--context", type=int, default=50, help="records before/after match")
    args = ap.parse_args()

    with open(args.path, "rb") as f:
        data = f.read()

    total = len(data) // REC_SIZE
    print(f"File: {os.path.basename(args.path)}")
    print(f"Bytes: {len(data)}")
    print(f"Records (aligned): {total}")

    if args.find_unmapped:
        hit = find_first_match(data, predicate=lambda pc: not is_mapped_vectrex_pc(pc), context=args.context)
        if not hit:
            print("No unmapped PC hits")
            return 1
        cur, pre, post = hit
        lo = pre[0].idx if pre else cur.idx
        hi = post[-1].idx if post else cur.idx
        print(f"\n=== First unmapped PC hit at index {cur.idx} (showing {lo}..{hi}) ===")
        for r in pre:
            print(" " + fmt(r.rec))
        print(">" + fmt(cur.rec))
        for r in post:
            print(" " + fmt(r.rec))
        return 0

    if args.find_pc_range:
        start = parse_hex16(args.find_pc_range[0])
        end = parse_hex16(args.find_pc_range[1])
        if start > end:
            start, end = end, start
        hit = find_first_match(data, predicate=lambda pc: start <= pc <= end, context=args.context)
        if not hit:
            print(f"No PC in range {start:04X}..{end:04X} hits")
            return 1
        cur, pre, post = hit
        lo = pre[0].idx if pre else cur.idx
        hi = post[-1].idx if post else cur.idx
        print(f"\n=== First PC in range {start:04X}..{end:04X} hit at index {cur.idx} (showing {lo}..{hi}) ===")
        for r in pre:
            print(" " + fmt(r.rec))
        print(">" + fmt(cur.rec))
        for r in post:
            print(" " + fmt(r.rec))
        return 0

    if args.find_entry_range:
        start = parse_hex16(args.find_entry_range[0])
        end = parse_hex16(args.find_entry_range[1])
        if start > end:
            start, end = end, start
        hit = find_first_entry_into_range(data, start=start, end=end, context=args.context)
        if not hit:
            print(f"No entry into range {start:04X}..{end:04X} detected")
            return 1
        cur, prev, pre, post = hit
        lo = (pre[0].idx if pre else (prev.idx if prev else cur.idx))
        hi = post[-1].idx if post else cur.idx
        print(f"\n=== First entry into range {start:04X}..{end:04X} at index {cur.idx} (showing {lo}..{hi}) ===")
        if prev is not None:
            print(" " + fmt(prev.rec))
        for r in pre:
            print(" " + fmt(r.rec))
        print(">" + fmt(cur.rec))
        for r in post:
            print(" " + fmt(r.rec))
        return 0

    recs = list(iter_records(data))

    if args.last:
        n = args.last
        start = max(0, len(recs) - n)
        for r in recs[start:]:
            print(fmt(r))
        return 0

    if args.find_pc:
        target = int(args.find_pc, 16) & 0xFFFF
        hits = [i for i, r in enumerate(recs) if r[0] == target]
        if not hits:
            print(f"No PC=={target:04X} hits")
            return 1
        for idx in hits[:10]:
            lo = max(0, idx - args.context)
            hi = min(len(recs), idx + args.context + 1)
            print(f"\n=== Hit at index {idx} (showing {lo}..{hi-1}) ===")
            for j in range(lo, hi):
                prefix = ">" if j == idx else " "
                print(prefix + fmt(recs[j]))
        if len(hits) > 10:
            print(f"(Truncated hits: {len(hits)} total)")
        return 0

    # default: print a small head/tail
    for r in recs[:10]:
        print(fmt(r))
    if len(recs) > 20:
        print("...")
    for r in recs[-10:]:
        print(fmt(r))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
