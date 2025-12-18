import Database from 'better-sqlite3';
import { join } from 'path';
import { app } from 'electron';

export interface PyPilotSession {
  id: number;
  projectPath: string;
  name: string;
  createdAt: number; // timestamp
  lastActivity: number; // timestamp
  isActive: number; // 0 or 1 (SQLite boolean)
}

export interface PyPilotMessage {
  id: number;
  sessionId: number;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: number;
  metadata?: string; // JSON string
}

let db: Database.Database | null = null;

/**
 * Initialize PyPilot sessions database
 */
export function initPyPilotDb(): Database.Database {
  if (db) return db;

  const userDataPath = app.getPath('userData');
  const dbPath = join(userDataPath, 'pypilot.db');
  
  db = new Database(dbPath);
  
  // Create sessions table
  db.exec(`
    CREATE TABLE IF NOT EXISTS sessions (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      projectPath TEXT NOT NULL,
      name TEXT NOT NULL,
      createdAt INTEGER NOT NULL,
      lastActivity INTEGER NOT NULL,
      isActive INTEGER DEFAULT 0
    )
  `);

  // Create messages table
  db.exec(`
    CREATE TABLE IF NOT EXISTS messages (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      sessionId INTEGER NOT NULL,
      role TEXT NOT NULL,
      content TEXT NOT NULL,
      timestamp INTEGER NOT NULL,
      metadata TEXT,
      FOREIGN KEY (sessionId) REFERENCES sessions(id) ON DELETE CASCADE
    )
  `);

  // Create indexes for performance
  db.exec(`
    CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(projectPath);
    CREATE INDEX IF NOT EXISTS idx_sessions_active ON sessions(isActive);
    CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(sessionId);
    CREATE INDEX IF NOT EXISTS idx_messages_timestamp ON messages(timestamp);
  `);

  console.log('[PyPilotDB] Database initialized:', dbPath);
  return db;
}

/**
 * Get active database instance
 */
export function getPyPilotDb(): Database.Database {
  if (!db) {
    throw new Error('PyPilot database not initialized. Call initPyPilotDb() first.');
  }
  return db;
}

/**
 * Create a new session
 */
export function createSession(projectPath: string, name: string): PyPilotSession {
  const db = getPyPilotDb();
  const now = Date.now();

  // Deactivate all sessions for this project
  db.prepare('UPDATE sessions SET isActive = 0 WHERE projectPath = ?').run(projectPath);

  // Create new session
  const result = db.prepare(`
    INSERT INTO sessions (projectPath, name, createdAt, lastActivity, isActive)
    VALUES (?, ?, ?, ?, 1)
  `).run(projectPath, name, now, now);

  return {
    id: result.lastInsertRowid as number,
    projectPath,
    name,
    createdAt: now,
    lastActivity: now,
    isActive: 1
  };
}

/**
 * Get all sessions for a project
 */
export function getSessions(projectPath: string): PyPilotSession[] {
  const db = getPyPilotDb();
  return db.prepare(`
    SELECT * FROM sessions 
    WHERE projectPath = ? 
    ORDER BY lastActivity DESC
  `).all(projectPath) as PyPilotSession[];
}

/**
 * Get active session for a project
 */
export function getActiveSession(projectPath: string): PyPilotSession | null {
  const db = getPyPilotDb();
  return db.prepare(`
    SELECT * FROM sessions 
    WHERE projectPath = ? AND isActive = 1 
    LIMIT 1
  `).get(projectPath) as PyPilotSession | undefined || null;
}

/**
 * Switch to a different session
 */
export function switchSession(sessionId: number): PyPilotSession {
  const db = getPyPilotDb();
  
  // Get the session to find its project
  const session = db.prepare('SELECT * FROM sessions WHERE id = ?').get(sessionId) as PyPilotSession;
  if (!session) {
    throw new Error(`Session ${sessionId} not found`);
  }

  // Deactivate all sessions for this project
  db.prepare('UPDATE sessions SET isActive = 0 WHERE projectPath = ?').run(session.projectPath);

  // Activate target session and update lastActivity
  const now = Date.now();
  db.prepare('UPDATE sessions SET isActive = 1, lastActivity = ? WHERE id = ?').run(now, sessionId);

  return { ...session, isActive: 1, lastActivity: now };
}

/**
 * Rename a session
 */
export function renameSession(sessionId: number, newName: string): void {
  const db = getPyPilotDb();
  db.prepare('UPDATE sessions SET name = ? WHERE id = ?').run(newName, sessionId);
}

/**
 * Delete a session and all its messages
 */
export function deleteSession(sessionId: number): void {
  const db = getPyPilotDb();
  db.prepare('DELETE FROM sessions WHERE id = ?').run(sessionId);
  // Messages are auto-deleted via CASCADE
}

/**
 * Save a message to a session
 */
export function saveMessage(
  sessionId: number,
  role: 'user' | 'assistant' | 'system',
  content: string,
  metadata?: any
): PyPilotMessage {
  const db = getPyPilotDb();
  const now = Date.now();

  const result = db.prepare(`
    INSERT INTO messages (sessionId, role, content, timestamp, metadata)
    VALUES (?, ?, ?, ?, ?)
  `).run(sessionId, role, content, now, metadata ? JSON.stringify(metadata) : null);

  // Update session lastActivity
  db.prepare('UPDATE sessions SET lastActivity = ? WHERE id = ?').run(now, sessionId);

  return {
    id: result.lastInsertRowid as number,
    sessionId,
    role,
    content,
    timestamp: now,
    metadata: metadata ? JSON.stringify(metadata) : undefined
  };
}

/**
 * Get all messages for a session
 */
export function getMessages(sessionId: number): PyPilotMessage[] {
  const db = getPyPilotDb();
  return db.prepare(`
    SELECT * FROM messages 
    WHERE sessionId = ? 
    ORDER BY timestamp ASC
  `).all(sessionId) as PyPilotMessage[];
}

/**
 * Clear all messages from a session
 */
export function clearMessages(sessionId: number): void {
  const db = getPyPilotDb();
  db.prepare('DELETE FROM messages WHERE sessionId = ?').run(sessionId);
}

/**
 * Get message count for a session
 */
export function getMessageCount(sessionId: number): number {
  const db = getPyPilotDb();
  const result = db.prepare('SELECT COUNT(*) as count FROM messages WHERE sessionId = ?').get(sessionId) as { count: number };
  return result.count;
}

/**
 * Close database connection
 */
export function closePyPilotDb(): void {
  if (db) {
    db.close();
    db = null;
    console.log('[PyPilotDB] Database closed');
  }
}
