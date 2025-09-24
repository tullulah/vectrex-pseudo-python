import React from 'react';
import { useTranslation } from 'react-i18next';

export const FileTreePanel: React.FC = () => {
  const { t } = useTranslation(['common']);
  return (
    <div style={{padding:8, fontSize:13}}>
      <strong>{t('menu.file')}</strong>
      <div style={{marginTop:8, color:'#888'}}>(file tree placeholder)</div>
    </div>
  );
};
