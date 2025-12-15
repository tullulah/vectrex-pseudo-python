import React from 'react';
import './ActivityBar.css';

export type ActivityBarItem = 'files' | 'git' | null;

interface ActivityBarProps {
  activeItem: ActivityBarItem;
  onItemClick: (item: ActivityBarItem) => void;
}

export const ActivityBar: React.FC<ActivityBarProps> = ({ activeItem, onItemClick }) => {
  const handleClick = (item: ActivityBarItem) => {
    // Toggle: if clicking active item, collapse (set to null)
    onItemClick(activeItem === item ? null : item);
  };

  return (
    <div className="activity-bar">
      <button
        className={`activity-bar-item ${activeItem === 'files' ? 'active' : ''}`}
        onClick={() => handleClick('files')}
        title="Files"
      >
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
          <polyline points="13 2 13 9 20 9"/>
        </svg>
      </button>
      
      <button
        className={`activity-bar-item ${activeItem === 'git' ? 'active' : ''}`}
        onClick={() => handleClick('git')}
        title="Source Control"
      >
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <circle cx="18" cy="18" r="3"/>
          <circle cx="6" cy="6" r="3"/>
          <path d="M13 6h3a2 2 0 0 1 2 2v7"/>
          <line x1="6" y1="9" x2="6" y2="21"/>
        </svg>
      </button>
    </div>
  );
};
