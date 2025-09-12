import React from 'react';
import { useTranslation } from 'react-i18next';

export const EmulatorPanel: React.FC = () => {
  const { t } = useTranslation(['common']);
  return (
    <div style={{padding:8}}>
      <strong>{t('panel.emulator')}</strong>
      <div style={{marginTop:8, color:'#888'}}>(emulator placeholder)</div>
    </div>
  );
};
