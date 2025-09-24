// Utility helpers for project & artifact naming.
// Current logic: derive binary name from active document URI.
// Rules (current stage):
//  - Take last path segment
//  - Strip query/hash if present
//  - Remove known source extensions (.vpy, .pseudo, .py-like) (configurable list)
//  - Fallback to 'output'
//  - Append .bin suffix ALWAYS

const STRIP_EXT = ['.vpy', '.pseudo', '.py'];

export function deriveBinaryName(uri: string): string {
  try {
    if (!uri) return 'output.bin';
    // Remove schema (e.g., inmemory://)
    const withoutScheme = uri.replace(/^[a-zA-Z0-9+.-]+:\/\//, '');
    // Split on / and \\
    const parts = withoutScheme.split(/[\\/]/);
    let last = parts[parts.length - 1] || 'output';
    // Drop any query/hash
    last = last.split('?')[0].split('#')[0];
    for (const ext of STRIP_EXT) {
      if (last.toLowerCase().endsWith(ext)) {
        last = last.slice(0, -ext.length);
        break;
      }
    }
    if (!last) last = 'output';
    return `${last}.bin`;
  } catch {
    return 'output.bin';
  }
}
