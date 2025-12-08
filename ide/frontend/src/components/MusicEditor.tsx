// Interface para notas escaladas usadas en mergedTracksToVmus
interface ScaledNote {
  note: number;
  start: number;
  end: number;
  duration: number;
  velocity: number;
  originalTrack: string;
}
/**
 * MusicEditor - AY-3-8910 PSG Music Editor for Vectrex
 * Piano Roll style editor with timeline view
 * 
 * AY-3-8910 capabilities:
 * - 3 tone channels (A, B, C) - square waves
 * - 1 noise generator (can be mixed with any channel)
 * - 1 envelope generator (shared)
 */

import React, { useRef, useEffect, useState, useCallback } from 'react';

// ============================================
// Types
// ============================================

interface NoteEvent {
  id: string;
  note: number;      // MIDI note (0-127)
  start: number;     // Start time in ticks
  duration: number;  // Duration in ticks
  velocity: number;  // Volume 0-15
  channel: number;   // 0=A, 1=B, 2=C
}

interface NoiseEvent {
  id: string;
  start: number;
  duration: number;
  period: number;    // 0-31
  channels: number;  // Bitmask: which channels use noise
}

interface MusicResource {
  version: string;
  name: string;
  author: string;
  tempo: number;
  ticksPerBeat: number;
  totalTicks: number;
  notes: NoteEvent[];
  noise: NoiseEvent[];
  loopStart: number;
  loopEnd: number;
}

interface MusicEditorProps {
  resource?: MusicResource;
  onChange?: (resource: MusicResource) => void;
  width?: number;
  height?: number;
}

// ============================================
// Constants
// ============================================

const NOTE_NAMES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
// Full piano range: A0 (MIDI 21) to C8 (MIDI 108) = 88 keys
// AY-3-8910 supports ~22 Hz to 93 kHz, so this is well within range
const START_NOTE = 21;  // A0
const END_NOTE = 108;   // C8
const NOTES_COUNT = END_NOTE - START_NOTE + 1;  // 88 notes

const PIANO_KEY_HEIGHT = 14;
const PIANO_KEY_WIDTH = 60;
const TICK_WIDTH = 4;  // Smaller since we have more ticks now
const DEFAULT_TICKS = 384;  // 16 bars * 24 ticks/beat * 4 beats/bar / 4
const TICKS_PER_BEAT = 24;  // Higher resolution for better MIDI import (24 = common MIDI resolution)

const CHANNEL_COLORS = ['#ff6b6b', '#51cf66', '#339af0']; // A=red, B=green, C=blue

const isBlackKey = (note: number) => {
  const n = note % 12;
  return n === 1 || n === 3 || n === 6 || n === 8 || n === 10;
};

const noteToString = (note: number): string => {
  const octave = Math.floor(note / 12) - 1;
  const noteName = NOTE_NAMES[note % 12];
  return `${noteName}${octave}`;
};

const generateId = () => Math.random().toString(36).substr(2, 9);

// Default resource
const createDefaultResource = (): MusicResource => ({
  version: '1.0',
  name: 'Untitled',
  author: '',
  tempo: 120,
  ticksPerBeat: TICKS_PER_BEAT,
  totalTicks: DEFAULT_TICKS,
  notes: [],
  noise: [],
  loopStart: 0,
  loopEnd: DEFAULT_TICKS,
});

// ============================================
// MIDI Import Dialog Types
// ============================================

interface MidiTrackInfo {
  key: string;           // "track-channel" identifier
  track: number;
  channel: number;
  noteCount: number;
  notes: MidiNote[];
  signature: string;     // For duplicate detection
  minNote: number;       // Lowest note (for range display)
  maxNote: number;       // Highest note
  startBeat: number;     // When first note starts (in beats)
  isDuplicate: boolean;
}

interface MidiImportData {
  tracks: MidiTrackInfo[];
  tempo: number;
  ticksPerBeat: number;
  totalTicks: number;
}

// ============================================
// MIDI Parser (Robust implementation)
// ============================================

interface MidiNote {
  note: number;
  start: number;  // in MIDI ticks
  duration: number;
  velocity: number;
  channel: number; // MIDI channel (0-15)
  track: number;   // Track number
}

function parseMidi(arrayBuffer: ArrayBuffer): { notes: MidiNote[]; tempo: number; ticksPerBeat: number; totalTicks: number } {
  const data = new Uint8Array(arrayBuffer);
  let pos = 0;
  
  const readUint16 = () => { const v = (data[pos] << 8) | data[pos + 1]; pos += 2; return v; };
  const readUint32 = () => { const v = (data[pos] << 24) | (data[pos + 1] << 16) | (data[pos + 2] << 8) | data[pos + 3]; pos += 4; return v; };
  const readVarLen = () => { 
    let v = 0; 
    let b; 
    let count = 0;
    do { 
      b = data[pos++]; 
      v = (v << 7) | (b & 0x7f); 
      count++;
      if (count > 4) break; // Safety: max 4 bytes for variable length
    } while (b & 0x80); 
    return v; 
  };
  
  // Read header
  const headerChunk = String.fromCharCode(data[0], data[1], data[2], data[3]);
  if (headerChunk !== 'MThd') throw new Error('Not a valid MIDI file');
  pos = 4;
  const headerLength = readUint32();
  const format = readUint16();
  const numTracks = readUint16();
  const ticksPerBeat = readUint16();
  pos = 8 + headerLength;
  
  console.log(`MIDI: Format ${format}, ${numTracks} tracks, ${ticksPerBeat} ticks/beat`);
  
  const allNotes: MidiNote[] = [];
  let tempo = 120;
  let maxTick = 0;
  
  // Read all tracks
  for (let t = 0; t < numTracks; t++) {
    if (pos + 8 > data.length) break;
    
    const trackChunk = String.fromCharCode(data[pos], data[pos + 1], data[pos + 2], data[pos + 3]);
    if (trackChunk !== 'MTrk') { 
      console.warn(`Track ${t}: Invalid header, skipping`);
      pos += 8; 
      continue; 
    }
    pos += 4;
    const trackLength = readUint32();
    const trackEnd = pos + trackLength;
    
    const activeNotes: Map<string, { note: number; start: number; velocity: number; channel: number }> = new Map();
    let tick = 0;
    let runningStatus = 0;
    let trackNoteCount = 0;
    
    while (pos < trackEnd && pos < data.length) {
      const delta = readVarLen();
      tick += delta;
      maxTick = Math.max(maxTick, tick);
      
      if (pos >= data.length) break;
      
      let status = data[pos];
      if (status < 0x80) {
        // Running status - use previous status
        status = runningStatus;
      } else {
        pos++;
        if (status >= 0x80 && status < 0xf0) {
          runningStatus = status;
        }
      }
      
      const type = status & 0xf0;
      const ch = status & 0x0f;
      
      if (type === 0x90) { // Note on
        const note = data[pos++];
        const velocity = data[pos++];
        const key = `${t}-${ch}-${note}`;
        
        if (velocity > 0) {
          activeNotes.set(key, { note, start: tick, velocity, channel: ch });
        } else {
          // Velocity 0 = note off
          const active = activeNotes.get(key);
          if (active) {
            allNotes.push({
              note: active.note,
              start: active.start,
              duration: Math.max(1, tick - active.start),
              velocity: Math.round(active.velocity / 8),
              channel: active.channel,
              track: t,
            });
            activeNotes.delete(key);
            trackNoteCount++;
          }
        }
      } else if (type === 0x80) { // Note off
        const note = data[pos++];
        pos++; // velocity (ignored for note off)
        const key = `${t}-${ch}-${note}`;
        
        const active = activeNotes.get(key);
        if (active) {
          allNotes.push({
            note: active.note,
            start: active.start,
            duration: Math.max(1, tick - active.start),
            velocity: Math.round(active.velocity / 8),
            channel: active.channel,
            track: t,
          });
          activeNotes.delete(key);
          trackNoteCount++;
        }
      } else if (type === 0xa0) { pos += 2; } // Aftertouch
      else if (type === 0xb0) { pos += 2; } // Control change
      else if (type === 0xc0) { pos += 1; } // Program change
      else if (type === 0xd0) { pos += 1; } // Channel pressure
      else if (type === 0xe0) { pos += 2; } // Pitch bend
      else if (status === 0xff) { // Meta event
        const metaType = data[pos++];
        const metaLen = readVarLen();
        if (metaType === 0x51 && metaLen === 3) { // Tempo
          const microsPerBeat = (data[pos] << 16) | (data[pos + 1] << 8) | data[pos + 2];
          tempo = Math.round(60000000 / microsPerBeat);
          console.log(`MIDI: Tempo ${tempo} BPM`);
        }
        pos += metaLen;
      } else if (status === 0xf0 || status === 0xf7) { // SysEx
        const len = readVarLen();
        pos += len;
      }
    }
    
    // Close any notes still active at end of track
    for (const [key, active] of activeNotes) {
      allNotes.push({
        note: active.note,
        start: active.start,
        duration: Math.max(1, tick - active.start),
        velocity: Math.round(active.velocity / 8),
        channel: active.channel,
        track: t,
      });
      trackNoteCount++;
    }
    
    console.log(`Track ${t}: ${trackNoteCount} notes`);
    pos = trackEnd;
  }
  
  console.log(`MIDI: Total ${allNotes.length} notes, max tick ${maxTick}`);
  return { notes: allNotes, tempo, ticksPerBeat, totalTicks: maxTick };
}

// Analyze MIDI and return track information for selection dialog
function analyzeMidi(arrayBuffer: ArrayBuffer): MidiImportData {
  const midiData = parseMidi(arrayBuffer);
  
  // Group notes by track+channel
  const trackChannelNotes: Map<string, MidiNote[]> = new Map();
  for (const note of midiData.notes) {
    const key = `${note.track}-${note.channel}`;
    if (!trackChannelNotes.has(key)) trackChannelNotes.set(key, []);
    trackChannelNotes.get(key)!.push(note);
  }
  
  // Create signatures and detect duplicates
  const signatureToFirst: Map<string, string> = new Map();
  const tracks: MidiTrackInfo[] = [];
  
  for (const [key, notes] of trackChannelNotes) {
    if (notes.length === 0) continue;
    
    const [trackStr, channelStr] = key.split('-');
    const sig = notes.slice(0, 20).map(n => `${n.start}-${n.note}`).join(',');
    
    const isDuplicate = signatureToFirst.has(sig);
    if (!isDuplicate) signatureToFirst.set(sig, key);
    
    const noteValues = notes.map(n => n.note);
    const firstNoteStart = notes[0].start;
    
    tracks.push({
      key,
      track: parseInt(trackStr),
      channel: parseInt(channelStr),
      noteCount: notes.length,
      notes,
      signature: sig,
      minNote: Math.min(...noteValues),
      maxNote: Math.max(...noteValues),
      startBeat: firstNoteStart / midiData.ticksPerBeat,
      isDuplicate,
    });
  }
  
  // Sort by note count (most notes first)
  tracks.sort((a, b) => b.noteCount - a.noteCount);
  
  return {
    tracks,
    tempo: midiData.tempo,
    ticksPerBeat: midiData.ticksPerBeat,
    totalTicks: midiData.totalTicks,
  };
}

// Convert selected tracks to vmus format
function selectedTracksToVmus(
  importData: MidiImportData, 
  selectedTracks: string[]  // Array of track keys to import (max 3)
): MusicResource {
  const vmusNotes: NoteEvent[] = [];
  const tickScale = TICKS_PER_BEAT / importData.ticksPerBeat;
  selectedTracks.slice(0, 3).forEach((trackKey, psgChannel) => {
    const trackInfo = importData.tracks.find(t => t.key === trackKey);
    if (!trackInfo) return;
    for (const note of trackInfo.notes) {
      vmusNotes.push({
        id: generateId(),
        note: note.note,
        start: Math.round(note.start * tickScale),
        duration: Math.max(1, Math.round(note.duration * tickScale)),
        velocity: Math.min(15, Math.max(1, note.velocity)),
        channel: psgChannel,
      });
    }
  });
  // Eliminar espacio en blanco inicial
  shiftNotesToZero(vmusNotes);
  // El totalTicks debe ser el máximo entre el final de la última nota y el totalTicks original
  const lastNoteEnd = vmusNotes.length ? Math.max(...vmusNotes.map(n => n.start + n.duration)) : 0;
  const totalTicks = Math.max(DEFAULT_TICKS, Math.round(importData.totalTicks * tickScale) + 16, lastNoteEnd + 8);
  return {
    version: '1.0',
    name: 'Imported MIDI',
    author: '',
    tempo: importData.tempo,
    ticksPerBeat: TICKS_PER_BEAT,
    totalTicks,
    notes: vmusNotes,
    noise: [],
    loopStart: 0,
    loopEnd: totalTicks,
  };
}

// NEW: Merge all MIDI tracks into PSG channels based on polyphony
// All notes go into a single timeline, then dynamically assigned to A/B/C based on overlap
function mergedTracksToVmus(
  importData: MidiImportData,
  selectedTracks: string[]
): MusicResource {
  const tickScale = TICKS_PER_BEAT / importData.ticksPerBeat;
  const allNotes: ScaledNote[] = [];
  for (const trackKey of selectedTracks) {
    const trackInfo = importData.tracks.find(t => t.key === trackKey);
    if (!trackInfo) continue;
    for (const note of trackInfo.notes) {
      const start = Math.round(note.start * tickScale);
      const duration = Math.max(1, Math.round(note.duration * tickScale));
      allNotes.push({
        note: note.note,
        start,
        end: start + duration,
        duration,
        velocity: Math.min(15, Math.max(1, note.velocity)),
        originalTrack: trackKey,
      });
    }
  }
  // Eliminar espacio en blanco inicial
  shiftNotesToZero(allNotes);
  // Sort by start time, then by pitch (higher notes have priority for melody)
  allNotes.sort((a, b) => a.start - b.start || b.note - a.note);
  // Assign notes to PSG channels dinámicamente
  const channelEndTime: number[] = [0, 0, 0];
  const vmusNotes: NoteEvent[] = [];
  let discardedNotes = 0;
  for (const note of allNotes) {
    let assignedChannel = -1;
    for (let ch = 0; ch < 3; ch++) {
      if (channelEndTime[ch] <= note.start) {
        assignedChannel = ch;
        break;
      }
    }
    if (assignedChannel === -1) {
      discardedNotes++;
      continue;
    }
    channelEndTime[assignedChannel] = note.end;
    vmusNotes.push({
      id: generateId(),
      note: note.note,
      start: note.start,
      duration: note.duration,
      velocity: note.velocity,
      channel: assignedChannel,
    });
  }
  if (discardedNotes > 0) {
    console.log(`Merged import: ${discardedNotes} notes discarded (>3 simultaneous)`);
  }
  // El totalTicks debe ser el máximo entre el final de la última nota y el totalTicks original
  const lastNoteEnd = vmusNotes.length ? Math.max(...vmusNotes.map(n => n.start + n.duration)) : 0;
  const totalTicks = Math.max(DEFAULT_TICKS, Math.round(importData.totalTicks * tickScale) + 16, lastNoteEnd + 8);
  return {
    version: '1.0',
    name: 'Imported MIDI',
    author: '',
    tempo: importData.tempo,
    ticksPerBeat: TICKS_PER_BEAT,
    totalTicks,
    notes: vmusNotes,
    noise: [],
    loopStart: 0,
    loopEnd: totalTicks,
  };
}

function midiToVmus(midiData: { notes: MidiNote[]; tempo: number; ticksPerBeat: number; totalTicks: number }): MusicResource {
  // Group notes by track+channel combination
  const trackChannelNotes: Map<string, MidiNote[]> = new Map();
  
  for (const note of midiData.notes) {
    const key = `${note.track}-${note.channel}`;
    if (!trackChannelNotes.has(key)) trackChannelNotes.set(key, []);
    trackChannelNotes.get(key)!.push(note);
  }
  
  // Create signature for each track (first 20 notes) to detect duplicates
  const trackSignatures: Map<string, string> = new Map();
  const signatureToTracks: Map<string, string[]> = new Map();
  
  for (const [key, notes] of trackChannelNotes) {
    // Create signature from first 20 notes (start time + note number)
    const sig = notes.slice(0, 20).map(n => `${n.start}-${n.note}`).join(',');
    trackSignatures.set(key, sig);
    
    if (!signatureToTracks.has(sig)) signatureToTracks.set(sig, []);
    signatureToTracks.get(sig)!.push(key);
  }
  
  // Filter out duplicate tracks (keep only one per unique signature)
  const uniqueTracks: Array<[string, MidiNote[]]> = [];
  const usedSignatures = new Set<string>();
  
  // Sort by note count first, so we keep the "best" version of each pattern
  const sortedTracks = Array.from(trackChannelNotes.entries())
    .sort((a, b) => b[1].length - a[1].length);
  
  for (const [key, notes] of sortedTracks) {
    const sig = trackSignatures.get(key)!;
    if (!usedSignatures.has(sig)) {
      usedSignatures.add(sig);
      uniqueTracks.push([key, notes]);
    } else {
      console.log(`MIDI: Skipping duplicate track ${key} (same pattern as another)`);
    }
  }
  
  // Take top 3 unique tracks
  const selectedChannels = uniqueTracks.slice(0, 3);
  
  console.log('Selected unique channels:', selectedChannels.map(([k, n]) => `${k}: ${n.length} notes`));
  
  // Create mapping from track-channel to PSG channel (0, 1, 2)
  const channelMap = new Map<string, number>();
  selectedChannels.forEach(([key], index) => channelMap.set(key, index));
  
  const vmusNotes: NoteEvent[] = [];
  
  // Scale ticks to our resolution (TICKS_PER_BEAT = 4)
  const tickScale = TICKS_PER_BEAT / midiData.ticksPerBeat;
  
  // Only include notes from the top 3 channels
  for (const note of midiData.notes) {
    const key = `${note.track}-${note.channel}`;
    const psgChannel = channelMap.get(key);
    if (psgChannel === undefined) continue; // Skip notes from other channels
    
    vmusNotes.push({
      id: generateId(),
      note: note.note,
      start: Math.round(note.start * tickScale),
      duration: Math.max(1, Math.round(note.duration * tickScale)),
      velocity: Math.min(15, Math.max(1, note.velocity)),
      channel: psgChannel,
    });
  }
  
  const totalTicks = Math.max(DEFAULT_TICKS, Math.round(midiData.totalTicks * tickScale) + 16);
  
  return {
    version: '1.0',
    name: 'Imported MIDI',
    author: '',
    tempo: midiData.tempo,
    ticksPerBeat: TICKS_PER_BEAT,
    totalTicks,
    notes: vmusNotes,
    noise: [],
    loopStart: 0,
    loopEnd: totalTicks,
  };
}

// Helper para eliminar espacio en blanco inicial en arrays de notas
function shiftNotesToZero<T extends { start: number; end?: number }>(notes: T[]): number {
  if (!notes.length) return 0;
  const minStart = Math.min(...notes.map(n => n.start));
  if (minStart > 0) {
    for (const n of notes) {
      n.start -= minStart;
      if (typeof n.end === 'number') n.end -= minStart;
    }
  }
  return minStart;
}

// ============================================
// MIDI Import Dialog Component
// ============================================

interface MidiImportDialogProps {
  importData: MidiImportData;
  onImport: (selectedTracks: string[], mergeMode: boolean) => void;
  onCancel: () => void;
}

const MidiImportDialog: React.FC<MidiImportDialogProps> = ({ importData, onImport, onCancel }) => {
  const [selectedTracks, setSelectedTracks] = useState<string[]>([]);
  const [previewTrack, setPreviewTrack] = useState<string | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [mergeMode, setMergeMode] = useState(true); // NEW: merge all tracks by default
  const psgRef = useRef<PSGEngine | null>(null);
  const playIntervalRef = useRef<number | null>(null);
  
  useEffect(() => {
    psgRef.current = new PSGEngine();
    psgRef.current.init();
    
    // Auto-select top 3 non-duplicate tracks (or more in merge mode)
    const autoSelected = importData.tracks
      .filter(t => !t.isDuplicate)
      .slice(0, mergeMode ? 10 : 3) // In merge mode, allow more tracks
      .map(t => t.key);
    setSelectedTracks(autoSelected);
    
    return () => {
      if (playIntervalRef.current) clearInterval(playIntervalRef.current);
      psgRef.current?.destroy();
    };
  }, [importData]);
  
  // Update selection when merge mode changes
  const handleMergeModeChange = (newMergeMode: boolean) => {
    setMergeMode(newMergeMode);
    if (newMergeMode) {
      // In merge mode, select all non-duplicate tracks
      const allTracks = importData.tracks.filter(t => !t.isDuplicate).map(t => t.key);
      setSelectedTracks(allTracks);
    } else {
      // In separate mode, limit to 3
      setSelectedTracks(prev => prev.slice(0, 3));
    }
  };
  
  const toggleTrack = (key: string) => {
    setSelectedTracks(prev => {
      if (prev.includes(key)) {
        return prev.filter(k => k !== key);
      } else if (mergeMode || prev.length < 3) {
        return [...prev, key];
      }
      return prev; // Max 3 in separate mode
    });
  };
  
  const previewPlay = (trackInfo: MidiTrackInfo) => {
    if (isPlaying && previewTrack === trackInfo.key) {
      // Stop
      if (playIntervalRef.current) clearInterval(playIntervalRef.current);
      psgRef.current?.stopAll();
      setIsPlaying(false);
      setPreviewTrack(null);
      return;
    }
    
    // Stop any current playback
    if (playIntervalRef.current) clearInterval(playIntervalRef.current);
    psgRef.current?.stopAll();
    
    setIsPlaying(true);
    setPreviewTrack(trackInfo.key);
    
    const tickScale = TICKS_PER_BEAT / importData.ticksPerBeat;
    const msPerTick = (60000 / importData.tempo) / TICKS_PER_BEAT;
    
    // Start from where the notes actually begin
    const firstNoteStart = trackInfo.notes.length > 0 ? 
      Math.round(trackInfo.notes[0].start * tickScale) : 0;
    let pos = firstNoteStart;
    
    const playing = new Map<string, MidiNote>();
    const lastNoteEnd = trackInfo.notes.length > 0 ? 
      Math.round((trackInfo.notes[trackInfo.notes.length - 1].start + 
                  trackInfo.notes[trackInfo.notes.length - 1].duration) * tickScale) : 200;
    // Play for max 300 ticks from first note (about 3 seconds at 120bpm)
    const maxTicks = Math.min(firstNoteStart + 300, lastNoteEnd + 10);
    
    playIntervalRef.current = window.setInterval(() => {
      for (const note of trackInfo.notes) {
        const scaledStart = Math.round(note.start * tickScale);
        if (scaledStart === pos && !playing.has(`${note.start}-${note.note}`)) {
          psgRef.current?.playNote(0, note.note, note.velocity);
          playing.set(`${note.start}-${note.note}`, note);
        }
      }
      for (const [id, note] of playing) {
        const scaledEnd = Math.round((note.start + note.duration) * tickScale);
        if (pos >= scaledEnd) {
          psgRef.current?.stopChannel(0);
          playing.delete(id);
        }
      }
      pos++;
      if (pos >= maxTicks) {
        if (playIntervalRef.current) clearInterval(playIntervalRef.current);
        psgRef.current?.stopAll();
        setIsPlaying(false);
        setPreviewTrack(null);
      }
    }, msPerTick);
  };
  
  const getAssignment = (key: string): string => {
    const idx = selectedTracks.indexOf(key);
    if (idx === -1) return '';
    return ['A', 'B', 'C'][idx];
  };
  
  const noteToName = (note: number): string => {
    const names = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    const octave = Math.floor(note / 12) - 1;
    return `${names[note % 12]}${octave}`;
  };
  
  const dialogStyle: React.CSSProperties = {
    position: 'fixed', top: 0, left: 0, right: 0, bottom: 0,
    background: 'rgba(0,0,0,0.8)', display: 'flex', alignItems: 'center', justifyContent: 'center',
    zIndex: 9999,
  };
  
  const panelStyle: React.CSSProperties = {
    background: '#1e1e2e', borderRadius: '8px', padding: '20px', minWidth: '500px', maxWidth: '700px',
    maxHeight: '80vh', overflow: 'auto', color: '#fff', border: '1px solid #444',
  };
  
  const rowStyle = (isSelected: boolean, isDuplicate: boolean): React.CSSProperties => ({
    display: 'flex', alignItems: 'center', gap: '10px', padding: '8px 12px', margin: '4px 0',
    background: isSelected ? '#3a4a6e' : isDuplicate ? '#2a2a3e' : '#252535',
    borderRadius: '4px', cursor: 'pointer', opacity: isDuplicate ? 0.6 : 1,
    border: isSelected ? '2px solid #6a8abe' : '2px solid transparent',
  });
  
  const btnStyle: React.CSSProperties = {
    padding: '8px 16px', background: '#4a5a8e', color: '#fff', border: 'none',
    borderRadius: '4px', cursor: 'pointer', fontSize: '14px',
  };
  
  return (
    <div style={dialogStyle} onClick={onCancel}>
      <div style={panelStyle} onClick={e => e.stopPropagation()}>
        <h2 style={{ margin: '0 0 16px 0', fontSize: '18px' }}>🎵 Import MIDI - Select Channels</h2>
        
        {/* Mode selector */}
        <div style={{ display: 'flex', gap: '12px', marginBottom: '16px', padding: '12px', background: '#2a2a3e', borderRadius: '6px' }}>
          <label style={{ display: 'flex', alignItems: 'center', gap: '8px', cursor: 'pointer' }}>
            <input 
              type="radio" 
              checked={mergeMode} 
              onChange={() => handleMergeModeChange(true)}
              style={{ accentColor: '#4a4' }}
            />
            <div>
              <div style={{ fontWeight: '500', color: '#fff' }}>🔀 Merge Mode (Recommended)</div>
              <div style={{ fontSize: '11px', color: '#888' }}>All tracks → dynamic A/B/C based on polyphony</div>
            </div>
          </label>
          <label style={{ display: 'flex', alignItems: 'center', gap: '8px', cursor: 'pointer' }}>
            <input 
              type="radio" 
              checked={!mergeMode} 
              onChange={() => handleMergeModeChange(false)}
              style={{ accentColor: '#4a4' }}
            />
            <div>
              <div style={{ fontWeight: '500', color: '#fff' }}>📊 Separate Mode</div>
              <div style={{ fontSize: '11px', color: '#888' }}>Each track → fixed channel (max 3)</div>
            </div>
          </label>
        </div>
        
        <p style={{ color: '#888', fontSize: '13px', margin: '0 0 8px 0' }}>
          {mergeMode 
            ? 'Select tracks to merge. Notes will be dynamically assigned to A/B/C.' 
            : 'Select up to 3 tracks. Each becomes a fixed channel.'}
        </p>
        <p style={{ color: '#6a8', fontSize: '12px', margin: '0 0 12px 0' }}>
          Tempo: {importData.tempo} BPM | Tracks: {importData.tracks.length} | 
          Selected: {selectedTracks.length}
        </p>
        
        <div style={{ maxHeight: '350px', overflow: 'auto' }}>
          {importData.tracks.map(track => {
            const assignment = mergeMode ? (selectedTracks.includes(track.key) ? '✓' : '') : getAssignment(track.key);
            const isSelected = selectedTracks.includes(track.key);
            return (
              <div key={track.key} style={rowStyle(isSelected, track.isDuplicate)} onClick={() => toggleTrack(track.key)}>
                <div style={{ 
                  width: '28px', height: '28px', borderRadius: '4px', 
                  background: mergeMode 
                    ? (isSelected ? '#4a4' : '#444')
                    : (assignment ? CHANNEL_COLORS[['A','B','C'].indexOf(assignment)] : '#444'),
                  display: 'flex', alignItems: 'center', justifyContent: 'center',
                  fontWeight: 'bold', fontSize: '14px'
                }}>
                  {assignment || '—'}
                </div>
                <div style={{ flex: 1 }}>
                  <div style={{ fontWeight: '500' }}>
                    Track {track.track}, Ch {track.channel}
                    {track.isDuplicate && <span style={{ color: '#a88', marginLeft: '8px', fontSize: '11px' }}>(duplicate)</span>}
                  </div>
                  <div style={{ fontSize: '11px', color: '#888' }}>
                    {track.noteCount} notes | Range: {noteToName(track.minNote)} - {noteToName(track.maxNote)}
                    {track.startBeat > 0 && <span style={{ color: '#a8a', marginLeft: '6px' }}>| Starts at beat {Math.round(track.startBeat)}</span>}
                  </div>
                </div>
                <button 
                  onClick={(e) => { e.stopPropagation(); previewPlay(track); }}
                  style={{ ...btnStyle, background: previewTrack === track.key ? '#c44' : '#4a4', minWidth: '50px' }}
                >
                  {previewTrack === track.key ? '⏹' : '▶'}
                </button>
              </div>
            );
          })}
        </div>
        
        <div style={{ display: 'flex', gap: '12px', marginTop: '20px', justifyContent: 'flex-end' }}>
          <button onClick={onCancel} style={{ ...btnStyle, background: '#555' }}>Cancel</button>
          <button 
            onClick={() => onImport(selectedTracks, mergeMode)} 
            disabled={selectedTracks.length === 0}
            style={{ ...btnStyle, background: selectedTracks.length > 0 ? '#4a4' : '#333' }}
          >
            {mergeMode 
              ? `🔀 Merge ${selectedTracks.length} Track${selectedTracks.length !== 1 ? 's' : ''}`
              : `Import ${selectedTracks.length} Channel${selectedTracks.length !== 1 ? 's' : ''}`
            }
          </button>
        </div>
      </div>
    </div>
  );
};

// ============================================
// PSG Audio Engine
// ============================================

class PSGEngine {
  private ctx: AudioContext | null = null;
  private oscillators: OscillatorNode[] = [];
  private gains: GainNode[] = [];
  private masterGain: GainNode | null = null;
  
  async init() {
    if (this.ctx) return;
    this.ctx = new AudioContext();
    
    this.masterGain = this.ctx.createGain();
    this.masterGain.gain.value = 0.2;
    this.masterGain.connect(this.ctx.destination);
    
    for (let i = 0; i < 3; i++) {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'square';
      osc.frequency.value = 440;
      gain.gain.value = 0;
      osc.connect(gain);
      gain.connect(this.masterGain);
      osc.start();
      this.oscillators.push(osc);
      this.gains.push(gain);
    }
  }
  
  noteToFrequency(note: number): number {
    return 440 * Math.pow(2, (note - 69) / 12);
  }
  
  playNote(channel: number, note: number, velocity: number) {
    if (!this.ctx || channel < 0 || channel > 2) return;
    const freq = this.noteToFrequency(note);
    this.oscillators[channel].frequency.setValueAtTime(freq, this.ctx.currentTime);
    this.gains[channel].gain.setValueAtTime(velocity / 15 * 0.3, this.ctx.currentTime);
  }
  
  stopChannel(channel: number) {
    if (channel >= 0 && channel < 3 && this.gains[channel]) {
      this.gains[channel].gain.value = 0;
    }
  }
  
  stopAll() {
    this.gains.forEach(g => { if (g) g.gain.value = 0; });
  }
  
  destroy() {
    this.oscillators.forEach(o => { try { o.stop(); } catch {} });
    this.ctx?.close();
  }
}

// ============================================
// Main Component
// ============================================

// Ensure resource has all required fields with defaults
const ensureValidResource = (r?: Partial<MusicResource>): MusicResource => {
  const defaults = createDefaultResource();
  if (!r) return defaults;
  return {
    version: r.version || defaults.version,
    name: r.name || defaults.name,
    author: r.author || defaults.author,
    tempo: r.tempo || defaults.tempo,
    ticksPerBeat: r.ticksPerBeat || defaults.ticksPerBeat,
    totalTicks: r.totalTicks || defaults.totalTicks,
    notes: Array.isArray(r.notes) ? r.notes : defaults.notes,
    noise: Array.isArray(r.noise) ? r.noise : defaults.noise,
    loopStart: r.loopStart ?? defaults.loopStart,
    loopEnd: r.loopEnd ?? defaults.loopEnd,
  };
};

export const MusicEditor: React.FC<MusicEditorProps> = ({
  resource: initialResource,
  onChange,
  width: propWidth,
  height: propHeight,
}) => {
  const [resource, setResource] = useState<MusicResource>(() => ensureValidResource(initialResource));
  const [currentChannel, setCurrentChannel] = useState(0);
  const [viewChannel, setViewChannel] = useState<number | 'all'>('all'); // Filter: 'all' or 0/1/2
  const [isPlaying, setIsPlaying] = useState(false);
  const [playheadPosition, setPlayheadPosition] = useState(0);
  const [scrollX, setScrollX] = useState(0);
  const [scrollY, setScrollY] = useState(NOTES_COUNT * PIANO_KEY_HEIGHT / 2 - 200);
  const [tool, setTool] = useState<'draw' | 'select' | 'erase'>('draw');
  const [containerSize, setContainerSize] = useState({ width: propWidth || 1000, height: propHeight || 600 });

  // Sync with external resource changes (e.g., file loaded from disk)
  useEffect(() => {
    if (initialResource) {
      const validated = ensureValidResource(initialResource);
      // Only update if notes array is different (avoid loops)
      if (JSON.stringify(validated.notes) !== JSON.stringify(resource.notes) ||
          validated.tempo !== resource.tempo ||
          validated.totalTicks !== resource.totalTicks) {
        setResource(validated);
      }
    }
  }, [initialResource]);
  const [selectedNotes, setSelectedNotes] = useState<Set<string>>(new Set());
  const [dragStart, setDragStart] = useState<{ tick: number; note: number } | null>(null);
  const [zoom, setZoom] = useState(1);
  const [midiImportData, setMidiImportData] = useState<MidiImportData | null>(null);
  const [playbackSpeed, setPlaybackSpeed] = useState(100); // Percentage: 100 = normal, 50 = half speed
  const [autoScroll, setAutoScroll] = useState(true);
  
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const psgRef = useRef<PSGEngine | null>(null);
  const playIntervalRef = useRef<number | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Initialize audio
  useEffect(() => {
    psgRef.current = new PSGEngine();
    psgRef.current.init();
    return () => psgRef.current?.destroy();
  }, []);

  // Resize observer to fill container
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    
    const resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width, height } = entry.contentRect;
        if (width > 0 && height > 0) {
          setContainerSize({ width, height });
        }
      }
    });
    
    resizeObserver.observe(container);
    return () => resizeObserver.disconnect();
  }, []);

  // Use container size or props
  const width = containerSize.width;
  const height = containerSize.height;

  // Update parent
  const updateResource = useCallback((newResource: MusicResource) => {
    setResource(newResource);
    onChange?.(newResource);
  }, [onChange]);

  // Calculate dimensions
  const gridHeight = NOTES_COUNT * PIANO_KEY_HEIGHT;
  const visibleWidth = width - PIANO_KEY_WIDTH - 20;
  const visibleHeight = height - 160;

  // Draw the piano roll
  const draw = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const tickW = TICK_WIDTH * zoom;

    // Draw grid background
    for (let i = 0; i < NOTES_COUNT; i++) {
      const y = i * PIANO_KEY_HEIGHT - scrollY;
      if (y < -PIANO_KEY_HEIGHT || y > visibleHeight) continue;
      
      const note = START_NOTE + (NOTES_COUNT - 1 - i);
      const isBlack = isBlackKey(note);
      
      ctx.fillStyle = isBlack ? '#1a1a2e' : '#252535';
      ctx.fillRect(0, y, visibleWidth, PIANO_KEY_HEIGHT);
      
      ctx.strokeStyle = '#333';
      ctx.lineWidth = 0.5;
      ctx.beginPath();
      ctx.moveTo(0, y + PIANO_KEY_HEIGHT);
      ctx.lineTo(visibleWidth, y + PIANO_KEY_HEIGHT);
      ctx.stroke();
    }

    // Draw beat lines
    for (let tick = 0; tick < resource.totalTicks; tick++) {
      const x = tick * tickW - scrollX;
      if (x < -tickW || x > visibleWidth) continue;
      
      ctx.strokeStyle = tick % (TICKS_PER_BEAT * 4) === 0 ? '#555' : 
                        tick % TICKS_PER_BEAT === 0 ? '#3a3a4e' : '#2a2a3e';
      ctx.lineWidth = tick % (TICKS_PER_BEAT * 4) === 0 ? 2 : 1;
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, visibleHeight);
      ctx.stroke();
    }

    // Draw notes (filtered by viewChannel)
    const visibleNotes = viewChannel === 'all' 
      ? resource.notes 
      : resource.notes.filter(n => n.channel === viewChannel);
    
    for (const note of visibleNotes) {
      const noteIndex = NOTES_COUNT - 1 - (note.note - START_NOTE);
      const y = noteIndex * PIANO_KEY_HEIGHT - scrollY;
      const x = note.start * tickW - scrollX;
      const w = note.duration * tickW;
      
      if (y < -PIANO_KEY_HEIGHT || y > visibleHeight) continue;
      if (x + w < 0 || x > visibleWidth) continue;
      
      const isSelected = selectedNotes.has(note.id);
      const channelIdx = typeof note.channel === 'number' ? note.channel : 0;
      const color = CHANNEL_COLORS[channelIdx] || CHANNEL_COLORS[0];
      ctx.fillStyle = isSelected ? '#fff' : color;
      ctx.strokeStyle = isSelected ? '#ff0' : '#000';
      ctx.lineWidth = isSelected ? 2 : 1;
      
      const radius = 3;
      ctx.beginPath();
      ctx.roundRect(x + 1, y + 2, Math.max(w - 2, 4), PIANO_KEY_HEIGHT - 4, radius);
      ctx.fill();
      ctx.stroke();
      
      if (w > 25) {
        ctx.fillStyle = '#000';
        ctx.font = '9px monospace';
        ctx.fillText(noteToString(note.note), x + 4, y + PIANO_KEY_HEIGHT - 4);
      }
    }

    // Draw playhead
    const playX = playheadPosition * tickW - scrollX;
    if (playX >= 0 && playX <= visibleWidth) {
      ctx.strokeStyle = '#f00';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.moveTo(playX, 0);
      ctx.lineTo(playX, visibleHeight);
      ctx.stroke();
    }

    // Draw loop region
    const loopStartX = resource.loopStart * tickW - scrollX;
    const loopEndX = resource.loopEnd * tickW - scrollX;
    ctx.fillStyle = 'rgba(100, 200, 100, 0.15)';
    ctx.fillRect(loopStartX, 0, loopEndX - loopStartX, visibleHeight);

  }, [resource, scrollX, scrollY, visibleWidth, visibleHeight, zoom, playheadPosition, selectedNotes, viewChannel]);

  useEffect(() => {
    draw();
  }, [draw]);

  // Mouse handlers
  const handleMouseDown = (e: React.MouseEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    const x = e.clientX - rect.left + scrollX;
    const y = e.clientY - rect.top + scrollY;
    const tickW = TICK_WIDTH * zoom;
    
    const tick = Math.floor(x / tickW);
    const noteIndex = Math.floor(y / PIANO_KEY_HEIGHT);
    const note = START_NOTE + (NOTES_COUNT - 1 - noteIndex);
    
    if (note < START_NOTE || note >= START_NOTE + NOTES_COUNT) return;
    
    if (tool === 'draw') {
      setDragStart({ tick, note });
      psgRef.current?.playNote(currentChannel, note, 15);
    } else if (tool === 'erase') {
      const newNotes = resource.notes.filter(n => 
        !(n.note === note && n.start <= tick && n.start + n.duration > tick)
      );
      updateResource({ ...resource, notes: newNotes });
    } else if (tool === 'select') {
      const clickedNote = resource.notes.find(n =>
        n.note === note && n.start <= tick && n.start + n.duration > tick
      );
      if (clickedNote) {
        if (e.shiftKey) {
          const newSelected = new Set(selectedNotes);
          newSelected.has(clickedNote.id) ? newSelected.delete(clickedNote.id) : newSelected.add(clickedNote.id);
          setSelectedNotes(newSelected);
        } else {
          setSelectedNotes(new Set([clickedNote.id]));
        }
      } else {
        setSelectedNotes(new Set());
      }
    }
  };

  const handleMouseUp = (e: React.MouseEvent) => {
    if (dragStart && tool === 'draw') {
      const rect = canvasRef.current?.getBoundingClientRect();
      if (rect) {
        const x = e.clientX - rect.left + scrollX;
        const tickW = TICK_WIDTH * zoom;
        const endTick = Math.max(dragStart.tick + 1, Math.floor(x / tickW) + 1);
        
        const newNote: NoteEvent = {
          id: generateId(),
          note: dragStart.note,
          start: dragStart.tick,
          duration: Math.max(1, endTick - dragStart.tick),
          velocity: 15,
          channel: currentChannel,
        };
        
        updateResource({ ...resource, notes: [...resource.notes, newNote] });
      }
      psgRef.current?.stopChannel(currentChannel);
    }
    setDragStart(null);
  };

  // Playback
  const togglePlay = useCallback(() => {
    if (isPlaying) {
      if (playIntervalRef.current) clearInterval(playIntervalRef.current);
      psgRef.current?.stopAll();
      setIsPlaying(false);
    } else {
      setIsPlaying(true);
      let pos = playheadPosition;
      // Adjust speed: 100% = normal, 50% = double time (slower), 200% = half time (faster)
      const speedFactor = 100 / playbackSpeed;
      const msPerTick = ((60000 / resource.tempo) / TICKS_PER_BEAT) * speedFactor;
      
      // Filter notes based on viewChannel
      const notesToPlay = viewChannel === 'all' 
        ? resource.notes 
        : resource.notes.filter(n => n.channel === viewChannel);
      
      const tickWidth = TICK_WIDTH * zoom;
      
      // Track active notes per channel (only one note per channel at a time)
      const activeNotePerChannel: (NoteEvent | null)[] = [null, null, null];
      
      playIntervalRef.current = window.setInterval(() => {
        // Find notes that start at current position
        for (const note of notesToPlay) {
          if (note.start === pos) {
            // Play this note on its channel (replaces any previous note on same channel)
            psgRef.current?.playNote(note.channel, note.note, note.velocity);
            activeNotePerChannel[note.channel] = note;
          }
        }
        
        // Check if any active notes have ended
        for (let ch = 0; ch < 3; ch++) {
          const activeNote = activeNotePerChannel[ch];
          if (activeNote && pos >= activeNote.start + activeNote.duration) {
            // Check if there's another note starting at this exact tick
            const newNote = notesToPlay.find(n => n.channel === ch && n.start === pos);
            if (!newNote) {
              psgRef.current?.stopChannel(ch);
              activeNotePerChannel[ch] = null;
            }
          }
        }
        
        pos++;
        if (pos >= resource.loopEnd) pos = resource.loopStart;
        setPlayheadPosition(pos);
        
        // Auto-scroll to keep playhead visible
        if (autoScroll) {
          const playheadX = pos * tickWidth;
          const margin = visibleWidth * 0.3; // Keep playhead in left 30%
          if (playheadX > scrollX + visibleWidth - margin) {
            setScrollX(Math.max(0, playheadX - margin));
          } else if (playheadX < scrollX + margin) {
            setScrollX(Math.max(0, playheadX - margin));
          }
        }
      }, msPerTick);
    }
  }, [isPlaying, playheadPosition, resource, viewChannel, playbackSpeed, autoScroll, zoom, scrollX, visibleWidth]);

  useEffect(() => () => { if (playIntervalRef.current) clearInterval(playIntervalRef.current); }, []);

  const deleteSelected = useCallback(() => {
    updateResource({ ...resource, notes: resource.notes.filter(n => !selectedNotes.has(n.id)) });
    setSelectedNotes(new Set());
  }, [resource, selectedNotes, updateResource]);

  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    if (e.key === ' ') { e.preventDefault(); togglePlay(); }
    else if (e.key === 'Delete' || e.key === 'Backspace') deleteSelected();
    else if (e.key === '1') setCurrentChannel(0);
    else if (e.key === '2') setCurrentChannel(1);
    else if (e.key === '3') setCurrentChannel(2);
    else if (e.key === 'd') setTool('draw');
    else if (e.key === 's') setTool('select');
    else if (e.key === 'e') setTool('erase');
  }, [togglePlay, deleteSelected]);

  const handleWheel = (e: React.WheelEvent) => {
    e.shiftKey 
      ? setScrollX(Math.max(0, scrollX + e.deltaY))
      : setScrollY(Math.max(0, Math.min(gridHeight - visibleHeight, scrollY + e.deltaY)));
  };

  // Import MIDI file
  const importMidi = useCallback(() => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.mid,.midi';
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;
      try {
        const arrayBuffer = await file.arrayBuffer();
        const importData = analyzeMidi(arrayBuffer);
        console.log('MIDI analyzed:', importData.tracks.length, 'tracks');
        setMidiImportData(importData);
      } catch (err) {
        console.error('Failed to parse MIDI:', err);
        alert('Failed to parse MIDI file. Make sure it is a valid MIDI file.');
      }
    };
    input.click();
  }, []);

  // Handle import from dialog
  const handleMidiImport = useCallback((selectedTracks: string[], mergeMode: boolean) => {
    if (!midiImportData) return;
    
    const vmusData = mergeMode 
      ? mergedTracksToVmus(midiImportData, selectedTracks)
      : selectedTracksToVmus(midiImportData, selectedTracks);
    
    console.log('Imported:', vmusData.notes.length, 'notes', mergeMode ? '(merged)' : '(separate)');
    
    // Show summary
    const chA = vmusData.notes.filter(n => n.channel === 0).length;
    const chB = vmusData.notes.filter(n => n.channel === 1).length;
    const chC = vmusData.notes.filter(n => n.channel === 2).length;
    console.log(`Channels: A=${chA}, B=${chB}, C=${chC}`);
    
    updateResource(vmusData);
    setScrollX(0);
    setScrollY(NOTES_COUNT * PIANO_KEY_HEIGHT / 2 - 200);
    setPlayheadPosition(0);
    setMidiImportData(null);
  }, [midiImportData, updateResource]);

  // Clear all notes
  const clearAll = useCallback(() => {
    if (confirm('Clear all notes?')) {
      updateResource({ ...resource, notes: [], noise: [] });
    }
  }, [resource, updateResource]);

  // Components
  const btnStyle: React.CSSProperties = { padding: '6px 12px', background: '#3a3a5e', color: '#fff', border: 'none', borderRadius: '4px', cursor: 'pointer', fontSize: '12px' };

  return (
    <div ref={containerRef} tabIndex={0} onKeyDown={handleKeyDown}
      style={{ display: 'flex', flexDirection: 'column', width: '100%', height: '100%', background: '#1e1e2e', outline: 'none', overflow: 'hidden' }}>
      
      {/* Toolbar */}
      <div style={{ display: 'flex', gap: '8px', padding: '8px', background: '#252530', borderBottom: '1px solid #3a3a4e', alignItems: 'center', flexWrap: 'wrap', flexShrink: 0 }}>
        <button onClick={togglePlay} style={{ ...btnStyle, background: isPlaying ? '#c44' : '#4a4', fontWeight: 'bold' }}>
          {isPlaying ? '⏹ Stop' : '▶ Play'}
        </button>
        <button onClick={() => setPlayheadPosition(0)} style={btnStyle}>⏮</button>
        
        {/* Speed control */}
        <select 
          value={playbackSpeed} 
          onChange={(e) => setPlaybackSpeed(parseInt(e.target.value))}
          style={{ padding: '4px 8px', background: '#3a3a5e', color: '#fff', border: 'none', borderRadius: '4px', fontSize: '11px' }}
        >
          <option value={25}>25% (4x slower)</option>
          <option value={50}>50% (2x slower)</option>
          <option value={75}>75%</option>
          <option value={100}>100% (Normal)</option>
          <option value={150}>150%</option>
          <option value={200}>200% (2x faster)</option>
        </select>
        
        {/* Auto-scroll toggle */}
        <button 
          onClick={() => setAutoScroll(!autoScroll)} 
          style={{ ...btnStyle, background: autoScroll ? '#5a5a8e' : '#3a3a5e', fontSize: '11px', padding: '4px 8px' }}
          title="Auto-scroll during playback"
        >
          {autoScroll ? '📍 Auto' : '📍 Off'}
        </button>
        
        <div style={{ width: '1px', height: '24px', background: '#4a4a6e' }} />
        
        <button onClick={() => setTool('draw')} style={{ ...btnStyle, background: tool === 'draw' ? '#5a5a8e' : '#3a3a5e' }}>✏️ Draw</button>
        <button onClick={() => setTool('select')} style={{ ...btnStyle, background: tool === 'select' ? '#5a5a8e' : '#3a3a5e' }}>⬚ Select</button>
        <button onClick={() => setTool('erase')} style={{ ...btnStyle, background: tool === 'erase' ? '#5a5a8e' : '#3a3a5e' }}>🗑 Erase</button>
        
        <div style={{ width: '1px', height: '24px', background: '#4a4a6e' }} />
        
        <span style={{ color: '#888', fontSize: '12px' }}>Draw:</span>
        {['A', 'B', 'C'].map((ch, i) => (
          <button key={ch} onClick={() => setCurrentChannel(i)}
            style={{ ...btnStyle, background: currentChannel === i ? CHANNEL_COLORS[i] : '#3a3a5e', fontWeight: currentChannel === i ? 'bold' : 'normal', minWidth: '28px', padding: '6px 8px' }}>
            {ch}
          </button>
        ))}
        
        <div style={{ width: '1px', height: '24px', background: '#4a4a6e' }} />
        
        <span style={{ color: '#888', fontSize: '12px' }}>View:</span>
        <button onClick={() => setViewChannel('all')}
          style={{ ...btnStyle, background: viewChannel === 'all' ? '#6a6a9e' : '#3a3a5e', fontWeight: viewChannel === 'all' ? 'bold' : 'normal', minWidth: '36px' }}>
          ALL
        </button>
        {['A', 'B', 'C'].map((ch, i) => (
          <button key={`view-${ch}`} onClick={() => setViewChannel(i)}
            style={{ ...btnStyle, background: viewChannel === i ? CHANNEL_COLORS[i] : '#3a3a5e', fontWeight: viewChannel === i ? 'bold' : 'normal', minWidth: '28px', padding: '6px 8px' }}>
            {ch}
          </button>
        ))}
        
        <div style={{ width: '1px', height: '24px', background: '#4a4a6e' }} />
        
        <span style={{ color: '#888', fontSize: '12px' }}>BPM:</span>
        <input type="number" value={resource.tempo} onChange={(e) => updateResource({ ...resource, tempo: parseInt(e.target.value) || 120 })}
          style={{ width: '50px', background: '#1a1a2e', color: '#fff', border: '1px solid #4a4a6e', borderRadius: '4px', padding: '4px' }} />
        
        <span style={{ color: '#888', fontSize: '12px' }}>Zoom:</span>
        <input type="range" min="0.5" max="3" step="0.1" value={zoom} onChange={(e) => {
          const newZoom = parseFloat(e.target.value);
          setZoom(newZoom);
          // Auto-scroll para mantener playhead visible
          setTimeout(() => {
            const tickWidth = TICK_WIDTH * newZoom;
            const playheadX = playheadPosition * tickWidth;
            const margin = (containerSize.width - PIANO_KEY_WIDTH - 20) * 0.3;
            if (playheadX > scrollX + (containerSize.width - PIANO_KEY_WIDTH - 20) - margin || playheadX < scrollX + margin) {
              setScrollX(Math.max(0, playheadX - margin));
            }
          }, 0);
        }} style={{ width: '80px' }} />
        
        <div style={{ width: '1px', height: '24px', background: '#4a4a6e' }} />
        
        <button onClick={importMidi} style={{ ...btnStyle, background: '#5a4a6e' }} title="Import MIDI file">📥 Import MIDI</button>
        <button onClick={clearAll} style={{ ...btnStyle, background: '#6a4a4e' }} title="Clear all notes">🗑️ Clear</button>
        
        <div style={{ flex: 1 }} />
        <span style={{ color: '#666', fontSize: '11px' }}>Notes: {resource.notes.length} | Tick: {playheadPosition}</span>
      </div>

      {/* Timeline header */}
      <div style={{ height: '20px', marginLeft: PIANO_KEY_WIDTH, background: '#252530', borderBottom: '1px solid #444', overflow: 'hidden', flexShrink: 0 }}>
        <div style={{ transform: `translateX(${-scrollX}px)`, display: 'flex' }}>
          {(() => {
            // Calcular el número de compases hasta el final real de la canción
            const lastNoteEnd = resource.notes.length ? Math.max(...resource.notes.map(n => n.start + n.duration)) : 0;
            const bars = Math.ceil(Math.max(resource.totalTicks, lastNoteEnd) / (TICKS_PER_BEAT * 4));
            return Array.from({ length: bars }, (_, i) => {
              const tick = i * TICKS_PER_BEAT * 4;
              return (
                <div
                  key={i}
                  style={{
                    width: TICKS_PER_BEAT * 4 * TICK_WIDTH * zoom,
                    color: '#888', fontSize: '10px', paddingLeft: '4px', borderLeft: '1px solid #555',
                    cursor: 'pointer', userSelect: 'none',
                    background: playheadPosition >= tick && playheadPosition < tick + TICKS_PER_BEAT * 4 ? '#2a2a3e' : undefined
                  }}
                  onClick={() => {
                    setPlayheadPosition(tick);
                    if (isPlaying) {
                      if (playIntervalRef.current) clearInterval(playIntervalRef.current);
                      setTimeout(() => togglePlay(), 0);
                    }
                  }}
                  title={`Bar ${i + 1} (tick ${tick})`}
                >
                  {i + 1}
                </div>
              );
            });
          })()}
        </div>
      </div>

      {/* Main area */}
      <div style={{ display: 'flex', flex: 1, overflow: 'hidden', minHeight: 0 }}>
        {/* Piano keys */}
        <div style={{ width: PIANO_KEY_WIDTH, flexShrink: 0, overflow: 'hidden', background: '#1a1a2e', borderRight: '2px solid #333' }}>
          <div style={{ transform: `translateY(${-scrollY}px)` }}>
            {Array.from({ length: NOTES_COUNT }, (_, i) => {
              const note = START_NOTE + (NOTES_COUNT - 1 - i);
              const isBlack = isBlackKey(note);
              const isC = note % 12 === 0;
              return (
                <div key={i}
                  onMouseDown={() => psgRef.current?.playNote(currentChannel, note, 15)}
                  onMouseUp={() => psgRef.current?.stopChannel(currentChannel)}
                  onMouseLeave={() => psgRef.current?.stopChannel(currentChannel)}
                  style={{
                    height: PIANO_KEY_HEIGHT, background: isBlack ? '#222' : (isC ? '#ddd' : '#fff'),
                    borderBottom: '1px solid #333', display: 'flex', alignItems: 'center',
                    paddingLeft: isBlack ? '4px' : '24px', fontSize: '9px', color: isBlack ? '#888' : '#333',
                    cursor: 'pointer', fontWeight: isC ? 'bold' : 'normal',
                  }}>
                  {noteToString(note)}
                </div>
              );
            })}
          </div>
        </div>

        {/* Canvas */}
        <canvas ref={canvasRef} width={visibleWidth} height={visibleHeight}
          onMouseDown={handleMouseDown} onMouseUp={handleMouseUp} onWheel={handleWheel}
          style={{ flex: 1, cursor: tool === 'draw' ? 'crosshair' : tool === 'erase' ? 'not-allowed' : 'default' }} />
      </div>

      {/* Bottom piano - full range A0 to C8 */}
      <div style={{ height: '55px', background: '#1a1a2e', borderTop: '1px solid #3a3a4e', display: 'flex', alignItems: 'flex-end', padding: '4px 8px', overflow: 'auto', flexShrink: 0 }}>
        <div style={{ position: 'relative', display: 'flex' }}>
          {Array.from({ length: NOTES_COUNT }, (_, i) => {
            const note = START_NOTE + i; // A0 (21) to C8 (108)
            if (isBlackKey(note)) return null;
            return (
              <div key={note}
                onMouseDown={() => psgRef.current?.playNote(currentChannel, note, 15)}
                onMouseUp={() => psgRef.current?.stopChannel(currentChannel)}
                onMouseLeave={() => psgRef.current?.stopChannel(currentChannel)}
                style={{ width: '20px', height: '45px', background: '#fff', border: '1px solid #333', borderRadius: '0 0 3px 3px', cursor: 'pointer',
                  display: 'flex', alignItems: 'flex-end', justifyContent: 'center', paddingBottom: '2px', fontSize: '7px', color: '#666' }}>
                {note % 12 === 0 ? `C${Math.floor(note / 12) - 1}` : (note === START_NOTE ? 'A0' : '')}
              </div>
            );
          })}
          {Array.from({ length: NOTES_COUNT }, (_, i) => {
            const note = START_NOTE + i;
            if (!isBlackKey(note)) return null;
            const whites = Array.from({ length: i }, (_, j) => !isBlackKey(START_NOTE + j)).filter(Boolean).length;
            return (
              <div key={note}
                onMouseDown={() => psgRef.current?.playNote(currentChannel, note, 15)}
                onMouseUp={() => psgRef.current?.stopChannel(currentChannel)}
                onMouseLeave={() => psgRef.current?.stopChannel(currentChannel)}
                style={{ position: 'absolute', left: whites * 20 - 7, width: '14px', height: '28px', background: '#222', border: '1px solid #111', borderRadius: '0 0 2px 2px', cursor: 'pointer', zIndex: 1 }} />
            );
          })}
        </div>
      </div>

      {/* Help */}
      <div style={{ padding: '4px 8px', background: '#1a1a2e', borderTop: '1px solid #3a3a4e', fontSize: '10px', color: '#666', display: 'flex', gap: '16px', flexWrap: 'wrap', flexShrink: 0 }}>
        <span><b>Space</b>: Play/Stop</span>
        <span><b>1/2/3</b>: Channel A/B/C</span>
        <span><b>D</b>: Draw</span>
        <span><b>S</b>: Select</span>
        <span><b>E</b>: Erase</span>
        <span><b>Del</b>: Delete</span>
        <span><b>Scroll</b>: V | <b>Shift+Scroll</b>: H</span>
      </div>
      {/* MIDI Import Dialog */}
      {midiImportData && (
        <MidiImportDialog importData={midiImportData} onImport={handleMidiImport} onCancel={() => setMidiImportData(null)} />
      )}
    </div>
  );
};
