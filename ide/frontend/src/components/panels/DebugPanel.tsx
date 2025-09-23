import React from 'react';
import { useTranslation } from 'react-i18next';

export const DebugPanel: React.FC = () => {
  const { t } = useTranslation(['common']);
  return (
    <div style={{padding:8}}>
      <strong>{t('panel.debug')}</strong>
      <div style={{marginTop:8, color:'#888'}}>(debug info placeholder)</div>
    </div>
  );
};
