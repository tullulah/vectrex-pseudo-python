import React from 'react';
import { MonacoEditorWrapper } from '../MonacoEditorWrapper';

interface EditorPanelProps { uri?: string }
export const EditorPanel: React.FC<EditorPanelProps> = ({ uri }) => {
  return <div style={{height:'100%', width:'100%'}}><MonacoEditorWrapper uri={uri} /></div>;
};
