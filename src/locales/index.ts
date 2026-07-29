import { createI18n } from "vue-i18n";
import zhCN from "./zh-CN.json";
import en from "./en.json";

function getDefaultLocale(): string {
  const saved = localStorage.getItem("ns-vpn-settings");
  if (saved) {
    try {
      const settings = JSON.parse(saved);
      if (settings.language) return settings.language;
    } catch {}
  }
  const browserLang = navigator.language;
  if (browserLang.startsWith("zh")) return "zh-CN";
  return "en";
}

const i18n = createI18n({
  legacy: false,
  locale: getDefaultLocale(),
  fallbackLocale: "zh-CN",
  messages: {
    "zh-CN": zhCN,
    en,
  },
});

export default i18n;
