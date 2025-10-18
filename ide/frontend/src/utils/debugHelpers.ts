// debugHelpers.ts - Debug utilities for address/line mapping
// Phase 5: Centralized helper functions for debugging operations

import type { PdbData } from '../state/debugStore';

/**
 * Convert VPy source line to ASM address using .pdb lineMap
 * @param line - VPy source line number (1-indexed)
 * @param pdb - Loaded .pdb data
 * @returns ASM address as number, or null if not found
 */
export function vpyLineToAsmAddress(line: number, pdb: PdbData | null): number | null {
  if (!pdb || !pdb.lineMap) return null;
  
  const address = pdb.lineMap[line.toString()];
  if (!address) return null;
  
  const addr = parseInt(address, 16);
  return isNaN(addr) ? null : addr;
}

/**
 * Convert ASM address to VPy source line (reverse lookup)
 * @param address - ASM address as number
 * @param pdb - Loaded .pdb data
 * @returns VPy line number, or null if not found
 */
export function asmAddressToVpyLine(address: number, pdb: PdbData | null): number | null {
  if (!pdb || !pdb.lineMap) return null;
  
  const addrStr = `0x${address.toString(16).padStart(4, '0').toUpperCase()}`;
  
  // Search lineMap for matching address
  for (const [lineStr, addr] of Object.entries(pdb.lineMap)) {
    if (addr.toUpperCase() === addrStr) {
      const line = parseInt(lineStr);
      return isNaN(line) ? null : line;
    }
  }
  
  return null;
}

/**
 * Get function name at given ASM address
 * @param address - ASM address as number
 * @param pdb - Loaded .pdb data
 * @returns Function name, or null if address not in any function
 */
export function getFunctionAtAddress(address: number, pdb: PdbData | null): string | null {
  if (!pdb || !pdb.functions) return null;
  
  const addrStr = `0x${address.toString(16).padStart(4, '0').toUpperCase()}`;
  
  // Direct match: address is function entry point
  for (const [name, info] of Object.entries(pdb.functions)) {
    if (info.address.toUpperCase() === addrStr) {
      return name;
    }
  }
  
  // TODO: Range check if we add function size info to .pdb
  // For now, only match exact entry points
  
  return null;
}

/**
 * Get function info at given ASM address (extended version)
 * @param address - ASM address as number
 * @param pdb - Loaded .pdb data
 * @returns Function info object, or null if not found
 */
export function getFunctionInfoAtAddress(
  address: number, 
  pdb: PdbData | null
): { name: string; address: string; type: 'vpy' | 'native' } | null {
  if (!pdb || !pdb.functions) return null;
  
  const addrStr = `0x${address.toString(16).padStart(4, '0').toUpperCase()}`;
  
  for (const [name, info] of Object.entries(pdb.functions)) {
    if (info.address.toUpperCase() === addrStr) {
      return {
        name,
        address: info.address,
        type: info.type || 'vpy'
      };
    }
  }
  
  return null;
}

/**
 * Check if VPy line contains a native function call
 * @param line - VPy source line number
 * @param pdb - Loaded .pdb data
 * @returns Native function name, or null if no native call
 */
export function getNativeCallAtLine(line: number, pdb: PdbData | null): string | null {
  if (!pdb || !pdb.nativeCalls) return null;
  
  return pdb.nativeCalls[line.toString()] || null;
}

/**
 * Get all native calls in current .pdb
 * @param pdb - Loaded .pdb data
 * @returns Array of { line, function } objects
 */
export function getAllNativeCalls(pdb: PdbData | null): Array<{ line: number; function: string }> {
  if (!pdb || !pdb.nativeCalls) return [];
  
  return Object.entries(pdb.nativeCalls).map(([lineStr, func]) => ({
    line: parseInt(lineStr),
    function: func
  }));
}

/**
 * Get all functions in current .pdb
 * @param pdb - Loaded .pdb data
 * @returns Array of function info objects
 */
export function getAllFunctions(pdb: PdbData | null): Array<{
  name: string;
  address: string;
  type: 'vpy' | 'native';
  startLine: number;
  endLine: number;
}> {
  if (!pdb || !pdb.functions) return [];
  
  return Object.entries(pdb.functions).map(([name, info]) => ({
    name,
    address: info.address,
    type: info.type || 'vpy',
    startLine: info.startLine,
    endLine: info.endLine
  }));
}

/**
 * Get symbol address by name
 * @param symbol - Symbol name (e.g., "START", "MAIN", "LOOP_BODY")
 * @param pdb - Loaded .pdb data
 * @returns ASM address as number, or null if not found
 */
export function getSymbolAddress(symbol: string, pdb: PdbData | null): number | null {
  if (!pdb || !pdb.symbols) return null;
  
  const address = pdb.symbols[symbol];
  if (!address) return null;
  
  const addr = parseInt(address, 16);
  return isNaN(addr) ? null : addr;
}

/**
 * Format address as hex string
 * @param address - Address as number
 * @param pad - Number of digits to pad (default 4)
 * @returns Formatted hex string (e.g., "0x06C3")
 */
export function formatAddress(address: number, pad: number = 4): string {
  return `0x${address.toString(16).padStart(pad, '0').toUpperCase()}`;
}

/**
 * Parse address from hex string
 * @param addressStr - Hex string (with or without 0x prefix)
 * @returns Address as number, or null if invalid
 */
export function parseAddress(addressStr: string): number | null {
  const cleaned = addressStr.trim().replace(/^0x/i, '');
  const addr = parseInt(cleaned, 16);
  return isNaN(addr) ? null : addr;
}

/**
 * Get entry point address from .pdb
 * @param pdb - Loaded .pdb data
 * @returns Entry point address as number, or null if not found
 */
export function getEntryPoint(pdb: PdbData | null): number | null {
  if (!pdb || !pdb.entry_point) return null;
  return parseAddress(pdb.entry_point);
}

/**
 * Check if address is valid (within typical Vectrex range)
 * @param address - Address to validate
 * @returns true if address is in valid range
 */
export function isValidAddress(address: number): boolean {
  // Vectrex typical ranges:
  // RAM: 0xC800-0xCFFF
  // ROM: 0x0000-0x7FFF (cartridge)
  // BIOS: 0xF000-0xFFFF
  return (address >= 0x0000 && address <= 0xFFFF);
}

/**
 * Get memory region name for address
 * @param address - Address to check
 * @returns Region name ("RAM", "ROM", "BIOS", "INVALID")
 */
export function getMemoryRegion(address: number): 'RAM' | 'ROM' | 'BIOS' | 'INVALID' {
  if (address >= 0xC800 && address <= 0xCFFF) return 'RAM';
  if (address >= 0xF000 && address <= 0xFFFF) return 'BIOS';
  if (address >= 0x0000 && address <= 0x7FFF) return 'ROM';
  return 'INVALID';
}

/**
 * Build reverse lineMap (address -> line) for faster lookups
 * @param pdb - Loaded .pdb data
 * @returns Map of address (number) -> line (number)
 */
export function buildReverseLineMap(pdb: PdbData | null): Map<number, number> {
  const map = new Map<number, number>();
  
  if (!pdb || !pdb.lineMap) return map;
  
  for (const [lineStr, addrStr] of Object.entries(pdb.lineMap)) {
    const line = parseInt(lineStr);
    const addr = parseAddress(addrStr);
    
    if (!isNaN(line) && addr !== null) {
      map.set(addr, line);
    }
  }
  
  return map;
}

/**
 * Build reverse symbol map (address -> symbol name) for faster lookups
 * @param pdb - Loaded .pdb data
 * @returns Map of address (number) -> symbol name
 */
export function buildReverseSymbolMap(pdb: PdbData | null): Map<number, string> {
  const map = new Map<number, string>();
  
  if (!pdb || !pdb.symbols) return map;
  
  for (const [symbol, addrStr] of Object.entries(pdb.symbols)) {
    const addr = parseAddress(addrStr);
    if (addr !== null) {
      map.set(addr, symbol);
    }
  }
  
  return map;
}
