import React, { useState } from 'react';

interface MenuRootProps { 
  label: string; 
  open: boolean; 
  setOpen: () => void; 
  children: React.ReactNode; 
}

export const MenuRoot: React.FC<MenuRootProps> = ({ label, open, setOpen, children }) => {
  const [hovered, setHovered] = useState(false);
  return (
    <div style={{position:'relative'}}>
      <div 
        onClick={setOpen} 
        onMouseEnter={() => setHovered(true)}
        onMouseLeave={() => setHovered(false)}
        style={{
          padding:'5px 12px', 
          cursor:'default', 
          background: open ? '#3c3c3c' : hovered ? '#2a2d2e' : 'transparent',
          borderRadius: '3px',
          margin: '2px 1px',
          fontSize: '13px',
          color: '#cccccc',
          transition: 'background 0.1s',
        }}
      >
        {label}
      </div>
      {open && (
        <div style={{
          position:'absolute', 
          top:'100%', 
          left: 0, 
          background:'#252526', 
          border:'1px solid #454545', 
          borderRadius: '4px',
          minWidth: 220, 
          zIndex: 1000, 
          boxShadow:'0 4px 16px rgba(0,0,0,0.5)',
          padding: '4px 0',
        }}>
          {children}
        </div>
      )}
    </div>
  );
};

interface MenuItemProps { 
  label: string; 
  onClick?: () => void; 
  disabled?: boolean;
  style?: React.CSSProperties;
}

export const MenuItem: React.FC<MenuItemProps> = ({ label, onClick, disabled, style }) => {
  const [hovered, setHovered] = useState(false);
  
  // Parse label to separate text from shortcut
  const parts = label.split('\t');
  const text = parts[0];
  const shortcut = parts[1] || '';
  
  return (
    <div 
      onClick={() => !disabled && onClick && onClick()} 
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      style={{
        padding:'6px 24px 6px 28px', 
        fontSize: 13, 
        cursor: disabled ? 'default' : 'pointer', 
        color: disabled ? '#6e6e6e' : '#cccccc', 
        display:'flex', 
        alignItems:'center', 
        justifyContent: 'space-between',
        gap: 16,
        background: hovered && !disabled ? '#094771' : 'transparent',
        transition: 'background 0.1s',
        ...style
      }}
    >
      <span>{text}</span>
      {shortcut && (
        <span style={{ 
          color: disabled ? '#4e4e4e' : '#858585', 
          fontSize: 12,
          fontFamily: 'monospace',
        }}>
          {shortcut}
        </span>
      )}
    </div>
  );
};

export const MenuSeparator: React.FC = () => (
  <div style={{borderTop:'1px solid #454545', margin:'4px 8px'}} />
);

interface SubMenuProps {
  label: string;
  children: React.ReactNode;
  disabled?: boolean;
}

export const SubMenu: React.FC<SubMenuProps> = ({ label, children, disabled }) => {
  const [open, setOpen] = useState(false);
  
  return (
    <div 
      style={{ position: 'relative' }}
      onMouseEnter={() => !disabled && setOpen(true)}
      onMouseLeave={() => setOpen(false)}
    >
      <div 
        style={{
          padding:'6px 24px 6px 28px', 
          fontSize: 13, 
          cursor: disabled ? 'default' : 'pointer', 
          color: disabled ? '#6e6e6e' : '#cccccc', 
          display:'flex', 
          alignItems:'center', 
          justifyContent: 'space-between',
          gap: 16,
          background: open && !disabled ? '#094771' : 'transparent',
          transition: 'background 0.1s',
        }}
      >
        <span>{label}</span>
        <span style={{ color: '#858585', fontSize: 10 }}>▶</span>
      </div>
      {open && !disabled && (
        <div style={{
          position: 'absolute',
          left: '100%',
          top: 0,
          background:'#252526', 
          border:'1px solid #454545', 
          borderRadius: '4px',
          minWidth: 180, 
          zIndex: 1001, 
          boxShadow:'0 4px 16px rgba(0,0,0,0.5)',
          padding: '4px 0',
        }}>
          {children}
        </div>
      )}
    </div>
  );
};

interface MenuCheckItemProps { 
  label: string; 
  checked?: boolean; 
  onClick: () => void; 
  badge?: string; 
}

export const MenuCheckItem: React.FC<MenuCheckItemProps> = ({ label, checked, onClick, badge }) => {
  const [hovered, setHovered] = useState(false);
  
  return (
    <div 
      onClick={onClick} 
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      style={{
        padding:'6px 24px 6px 8px', 
        fontSize: 13, 
        cursor:'pointer', 
        display:'flex', 
        alignItems:'center', 
        gap: 8,
        background: hovered ? '#094771' : 'transparent',
        transition: 'background 0.1s',
        color: '#cccccc',
      }}
    >
      <span style={{width: 18, textAlign:'center', color: checked ? '#0078d4' : 'transparent'}}>
        {checked ? '✓' : ''}
      </span>
      <span style={{flex:1}}>{label}</span>
      {badge && (
        <span style={{
          background: badge.includes('E') ? '#f14c4c' : '#cca700', 
          color:'#1e1e1e', 
          borderRadius: 10, 
          padding:'1px 6px', 
          fontSize: 10,
          fontWeight: 600,
        }}>
          {badge}
        </span>
      )}
    </div>
  );
};