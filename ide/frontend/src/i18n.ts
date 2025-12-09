import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

import common_en from './locales/en/common.json' with { type: 'json' };
import editor_en from './locales/en/editor.json' with { type: 'json' };
import common_es from './locales/es/common.json' with { type: 'json' };
import editor_es from './locales/es/editor.json' with { type: 'json' };

void i18n
  .use(initReactI18next)
  .init({
    resources: {
      en: { common: common_en, editor: editor_en },
      es: { common: common_es, editor: editor_es }
    },
    lng: 'en',
    fallbackLng: 'en',
    ns: ['common', 'editor'],
    defaultNS: 'common',
    interpolation: { escapeValue: false }
  });

export default i18n;
