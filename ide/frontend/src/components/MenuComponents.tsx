import React from 'react';

interface MenuRootProps { 
  label: string; 
  open: boolean; 
  setOpen: () => void; 
  children: React.ReactNode; 
}

export const MenuRoot: React.FC<MenuRootProps> = ({ label, open, setOpen, children }) => {
  return (
    <div style={{position:'relative'}}>
      <div onClick={setOpen} style={{padding:'4px 10px', cursor:'default', background: open? '#333':'transparent'}}>
        {label}
      </div>
      {open && (
        <div style={{
          position:'absolute', top:'100%', left:0, background:'#2d2d2d', border:'1px solid #444', 
          minWidth:180, zIndex:1000, boxShadow:'0 2px 6px rgba(0,0,0,0.4)'
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
}

export const MenuItem: React.FC<MenuItemProps> = ({ label, onClick, disabled }) => (
  <div onClick={() => !disabled && onClick && onClick()} style={{
    padding:'4px 10px', 
    fontSize:12, 
    cursor: disabled? 'not-allowed':'default', 
    color: disabled? '#666':'#eee', 
    display:'flex', 
    alignItems:'center', 
    gap:8
  }}>
    {label}
  </div>
);

export const MenuSeparator: React.FC = () => (
  <div style={{borderTop:'1px solid #444', margin:'4px 0'}} />
);

interface MenuCheckItemProps { 
  label: string; 
  checked?: boolean; 
  onClick: () => void; 
  badge?: string; 
}

export const MenuCheckItem: React.FC<MenuCheckItemProps> = ({ label, checked, onClick, badge }) => (
  <div onClick={onClick} style={{
    padding:'4px 10px', 
    fontSize:12, 
    cursor:'default', 
    display:'flex', 
    alignItems:'center', 
    gap:8
  }}>
    <span style={{width:14, textAlign:'center', color:'#bbb'}}>
      {checked ? '✓' : ''}
    </span>
    <span style={{flex:1}}>{label}</span>
    {badge && (
      <span style={{
        background: badge.includes('E')? '#F14C4C':'#CCA700', 
        color:'#fff', 
        borderRadius:8, 
        padding:'0 4px', 
        fontSize:10
      }}>
        {badge}
      </span>
    )}
  </div>
);