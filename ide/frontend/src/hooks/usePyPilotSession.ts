import { useState, useEffect } from 'react';
import type { AiMessage } from '../types/ai';

interface PyPilotSession {
  id: number;
  projectPath: string;
  name: string;
  createdAt: number;
  lastActivity: number;
  isActive: number;
}

/**
 * Hook para gestionar sesiones de PyPilot
 * Maneja la persistencia en base de datos vía window.pypilot
 */
export function usePyPilotSession(projectPath: string) {
  const [currentSessionId, setCurrentSessionId] = useState<number | null>(null);
  const [messages, setMessages] = useState<AiMessage[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  // Initialize or load session when project changes
  useEffect(() => {
    if (!projectPath || !window.pypilot) {
      setIsLoading(false);
      return;
    }

    initializeSession();
  }, [projectPath]);

  const initializeSession = async () => {
    if (!window.pypilot) return;

    try {
      setIsLoading(true);

      // Try to get active session for this project
      const activeResult = await window.pypilot.getActiveSession(projectPath);
      
      let sessionId: number;
      if (activeResult.success && activeResult.session) {
        sessionId = activeResult.session.id;
        console.log('[PyPilot] Loaded existing session:', sessionId);
      } else {
        // No active session, create new one
        const createResult = await window.pypilot.createSession(
          projectPath,
          `Session ${new Date().toLocaleString()}`
        );
        
        if (!createResult.success || !createResult.session) {
          console.error('[PyPilot] Failed to create session:', createResult.error);
          return;
        }
        
        sessionId = createResult.session.id;
        console.log('[PyPilot] Created new session:', sessionId);

        // Migrate from localStorage if this is first time
        await migrateFromLocalStorage(sessionId);
      }

      setCurrentSessionId(sessionId);
      await loadMessages(sessionId);
    } catch (error) {
      console.error('[PyPilot] Error initializing session:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const loadMessages = async (sessionId: number) => {
    if (!window.pypilot) return;

    const result = await window.pypilot.getMessages(sessionId);
    if (result.success && result.messages) {
      const aiMessages: AiMessage[] = result.messages.map((msg: any) => ({
        id: `${msg.sessionId}-${msg.id}`, // Unique ID combining session and message ID
        role: msg.role,
        content: msg.content,
        timestamp: new Date(msg.timestamp)
      }));
      setMessages(aiMessages);
    }
  };

  const addMessage = async (role: 'user' | 'assistant' | 'system', content: string) => {
    if (!window.pypilot || !currentSessionId) {
      console.warn('[PyPilot] Cannot add message: no active session');
      return;
    }

    try {
      const result = await window.pypilot.saveMessage(currentSessionId, role, content);
      if (result.success) {
        const newMessage: AiMessage = {
          id: `${currentSessionId}-${Date.now()}`, // Temporary ID until message is persisted
          role,
          content,
          timestamp: new Date()
        };
        setMessages(prev => [...prev, newMessage]);
      }
    } catch (error) {
      console.error('[PyPilot] Error saving message:', error);
    }
  };

  const clearMessages = async () => {
    if (!window.pypilot || !currentSessionId) return;

    try {
      const result = await window.pypilot.clearMessages(currentSessionId);
      if (result.success) {
        setMessages([]);
      }
    } catch (error) {
      console.error('[PyPilot] Error clearing messages:', error);
    }
  };

  const switchSession = async (sessionId: number) => {
    if (!window.pypilot) return;

    try {
      const result = await window.pypilot.switchSession(sessionId);
      if (result.success) {
        setCurrentSessionId(sessionId);
        await loadMessages(sessionId);
      }
    } catch (error) {
      console.error('[PyPilot] Error switching session:', error);
    }
  };

  const createNewSession = async () => {
    if (!window.pypilot || !projectPath) return;

    try {
      const result = await window.pypilot.createSession(
        projectPath,
        `Session ${new Date().toLocaleString()}`
      );
      
      if (result.success && result.session) {
        setCurrentSessionId(result.session.id);
        setMessages([]);
        console.log('[PyPilot] Created new session:', result.session.id);
      }
    } catch (error) {
      console.error('[PyPilot] Error creating new session:', error);
    }
  };

  /**
   * One-time migration from localStorage to database
   */
  const migrateFromLocalStorage = async (sessionId: number) => {
    if (!window.pypilot) return;

    try {
      const saved = localStorage.getItem('pypilot_conversation');
      if (!saved) return;

      const parsed = JSON.parse(saved);
      if (!Array.isArray(parsed) || parsed.length === 0) return;

      console.log('[PyPilot] Migrating', parsed.length, 'messages from localStorage...');

      for (const msg of parsed) {
        await window.pypilot.saveMessage(
          sessionId,
          msg.role,
          msg.content,
          { migrated: true }
        );
      }

      // Clear localStorage after successful migration
      localStorage.removeItem('pypilot_conversation');
      console.log('[PyPilot] Migration complete');
    } catch (error) {
      console.error('[PyPilot] Migration error:', error);
    }
  };

  return {
    currentSessionId,
    messages,
    isLoading,
    addMessage,
    clearMessages,
    switchSession,
    createNewSession
  };
}
