// DebugSplitView.tsx - Split view with synchronized VPy and ASM editors
import React, { useEffect, useRef, useState } from 'react';
import * as monaco from 'monaco-editor';
import { useEditorStore } from '../state/editorStore';
import { useDebugStore } from '../state/debugStore';

interface DebugSplitViewProps {
  vpyContent: string;
  asmContent: string;
  currentDocument: { uri: string; path: string } | null;
}

export function DebugSplitView({ vpyContent, asmContent, currentDocument }: DebugSplitViewProps) {
  const vpyEditorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const asmEditorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const vpyContainerRef = useRef<HTMLDivElement>(null);
  const asmContainerRef = useRef<HTMLDivElement>(null);
  
  const breakpoints = useEditorStore(s => s.breakpoints);
  const toggleBreakpoint = useEditorStore(s => s.toggleBreakpoint);
  
  const debugState = useDebugStore(s => s.state);
  const currentVpyLine = useDebugStore(s => s.currentVpyLine);
  const currentAsmAddress = useDebugStore(s => s.currentAsmAddress);
  const pdbData = useDebugStore(s => s.pdbData);
  
  const [vpyDecorations, setVpyDecorations] = useState<string[]>([]);
  const [asmDecorations, setAsmDecorations] = useState<string[]>([]);

  // Initialize VPy editor
  useEffect(() => {
    if (!vpyContainerRef.current) return;
    
    const editor = monaco.editor.create(vpyContainerRef.current, {
      value: vpyContent,
      language: 'python',
      theme: 'vs-dark',
      readOnly: debugState !== 'stopped', // Read-only during debugging
      minimap: { enabled: true },
      glyphMargin: true,
      lineNumbers: 'on',
      scrollBeyondLastLine: false,
    });
    
    vpyEditorRef.current = editor;
    
    // Handle breakpoint clicks on gutter
    editor.onMouseDown((e: monaco.editor.IEditorMouseEvent) => {
      if (e.target.type === monaco.editor.MouseTargetType.GUTTER_GLYPH_MARGIN) {
        const lineNumber = e.target.position?.lineNumber;
        if (lineNumber && currentDocument) {
          toggleBreakpoint(currentDocument.uri, lineNumber);
        }
      }
    });
    
    return () => editor.dispose();
  }, [vpyContent, debugState]);

  // Initialize ASM editor
  useEffect(() => {
    if (!asmContainerRef.current) return;
    
    const editor = monaco.editor.create(asmContainerRef.current, {
      value: asmContent,
      language: 'asm',
      theme: 'vs-dark',
      readOnly: true, // ASM is always read-only
      minimap: { enabled: true },
      glyphMargin: false, // No breakpoints in ASM (set them in VPy)
      lineNumbers: 'on',
      scrollBeyondLastLine: false,
    });
    
    asmEditorRef.current = editor;
    
    return () => editor.dispose();
  }, [asmContent]);

  // Update VPy decorations (breakpoints + current line)
  useEffect(() => {
    if (!vpyEditorRef.current || !currentDocument) return;
    
    const editor = vpyEditorRef.current;
    const bps = breakpoints[currentDocument.uri] || new Set<number>();
    
    const decorations: monaco.editor.IModelDeltaDecoration[] = [];
    
    // Breakpoint decorations
    Array.from(bps).forEach(lineNumber => {
      decorations.push({
        range: new monaco.Range(lineNumber, 1, lineNumber, 1),
        options: {
          isWholeLine: false,
          glyphMarginClassName: 'debug-breakpoint',
          glyphMarginHoverMessage: { value: 'Breakpoint' }
        }
      });
    });
    
    // Current line decoration (yellow arrow)
    if (currentVpyLine !== null && debugState === 'paused') {
      decorations.push({
        range: new monaco.Range(currentVpyLine, 1, currentVpyLine, 1),
        options: {
          isWholeLine: true,
          className: 'debug-current-line',
          glyphMarginClassName: 'debug-current-line-arrow',
          glyphMarginHoverMessage: { value: 'Current line' }
        }
      });
    }
    
    const newDecorations = editor.deltaDecorations(vpyDecorations, decorations);
    setVpyDecorations(newDecorations);
  }, [breakpoints, currentDocument, currentVpyLine, debugState, vpyDecorations]);

  // Update ASM decorations (current instruction)
  useEffect(() => {
    if (!asmEditorRef.current || !pdbData || currentAsmAddress === null) return;
    
    const editor = asmEditorRef.current;
    
    // Find ASM line corresponding to current address
    // This requires parsing ASM content to find address labels
    const asmLine = findAsmLineByAddress(asmContent, currentAsmAddress);
    
    if (asmLine !== null) {
      const decorations: monaco.editor.IModelDeltaDecoration[] = [{
        range: new monaco.Range(asmLine, 1, asmLine, 1),
        options: {
          isWholeLine: true,
          className: 'debug-current-asm-line',
          glyphMarginClassName: 'debug-current-asm-arrow'
        }
      }];
      
      const newDecorations = editor.deltaDecorations(asmDecorations, decorations);
      setAsmDecorations(newDecorations);
      
      // Scroll to current instruction
      editor.revealLineInCenter(asmLine);
    }
  }, [currentAsmAddress, asmContent, pdbData, asmDecorations]);

  // Synchronize scrolling (when one editor scrolls, scroll the other proportionally)
  useEffect(() => {
    if (!vpyEditorRef.current || !asmEditorRef.current) return;
    
    const vpyEditor = vpyEditorRef.current;
    const asmEditor = asmEditorRef.current;
    
    const vpyScrollDisposable = vpyEditor.onDidScrollChange((e) => {
      if (e.scrollTopChanged) {
        // Calculate proportional scroll position
        const vpyModel = vpyEditor.getModel();
        const asmModel = asmEditor.getModel();
        if (!vpyModel || !asmModel) return;
        
        const vpyLines = vpyModel.getLineCount();
        const asmLines = asmModel.getLineCount();
        const ratio = asmLines / vpyLines;
        
        const asmScrollTop = e.scrollTop * ratio;
        asmEditor.setScrollTop(asmScrollTop);
      }
    });
    
    return () => vpyScrollDisposable.dispose();
  }, []);

  return (
    <div className="debug-split-view">
      <div className="split-container">
        <div className="vpy-panel">
          <div className="panel-header">
            <span className="panel-title">🐍 VPy Source</span>
            {currentDocument && (
              <span className="panel-file">{currentDocument.path}</span>
            )}
          </div>
          <div ref={vpyContainerRef} className="editor-container" />
        </div>
        
        <div className="splitter" />
        
        <div className="asm-panel">
          <div className="panel-header">
            <span className="panel-title">⚙️ Assembly</span>
            {currentAsmAddress && (
              <span className="panel-address">PC: {currentAsmAddress}</span>
            )}
          </div>
          <div ref={asmContainerRef} className="editor-container" />
        </div>
      </div>
    </div>
  );
}

// Helper function to find ASM line by address
function findAsmLineByAddress(asmContent: string, address: string): number | null {
  const lines = asmContent.split('\n');
  const targetAddr = parseInt(address.replace('0x', ''), 16);
  
  // Parse ASM looking for address comments or labels
  // Example formats:
  //   ; Address: 0xC890
  //   START:      ; 0xC880
  //   C890: LDA #$80
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    // Match address in comment
    const commentMatch = line.match(/;\s*(?:Address:\s*)?0x([0-9A-Fa-f]+)/);
    if (commentMatch) {
      const lineAddr = parseInt(commentMatch[1], 16);
      if (lineAddr === targetAddr) {
        return i + 1; // Monaco is 1-indexed
      }
    }
    
    // Match address prefix (e.g., "C890: LDA")
    const prefixMatch = line.match(/^([0-9A-Fa-f]{4}):/);
    if (prefixMatch) {
      const lineAddr = parseInt(prefixMatch[1], 16);
      if (lineAddr === targetAddr) {
        return i + 1;
      }
    }
  }
  
  return null;
}
