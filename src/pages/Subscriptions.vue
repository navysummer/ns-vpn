<script setup lang="ts">
import { ref, watch, computed, onMounted, onUnmounted } from "vue";
import { RefreshCw, Trash2, Clipboard, GripVertical, FileCode, Edit, FolderOpen, Upload, Link, FileText, FileJson, ArrowDownToLine, ArrowUpFromLine, Search, X, Plus } from "lucide-vue-next";
import { useToast } from "@/utils/toast";
import { useI18n } from "vue-i18n";
import { useAppStore } from "@/stores/app";
import ConfirmDialog from "@/components/ConfirmDialog.vue";
import ContextMenu from "@/components/ContextMenu.vue";
import QrViewer from "@/components/QrViewer.vue";
import { writeConfigOnly, fetchSubscriptionUrl, openAppDir, convertContent } from "@/utils/tauri";

const { show } = useToast();
const { t } = useI18n();
const app = useAppStore();
const ACTIVE_KEY = "ns-vpn-active-sub";
const activeSubId = ref(localStorage.getItem(ACTIVE_KEY));
const selectedSub = ref<string | null>(activeSubId.value);

type ConfigFormat = "clash" | "v2rayn" | "singbox";
type ProfileType = "remote" | "local" | "script";

interface Subscription {
  id: string;
  name: string;
  description: string;
  type: ProfileType;
  url: string;
  format: ConfigFormat;
  userAgent: string;
  httpTimeout: number;
  updateInterval: number;
  useSystemProxy: boolean;
  useCoreProxy: boolean;
  allowInsecure: boolean;
  allowAutoUpdate: boolean;
  lastUpdate: string;
  timeAgo: string;
  fileName?: string;
  fileContent?: string;
  pasteContent?: string;
  rawContent?: string;
}

const STORAGE_KEY = "ns-vpn-subscriptions";

function loadSubscriptions(): Subscription[] {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    const subs: Subscription[] = saved ? JSON.parse(saved) : [];
    return subs.map(s => ({ ...s, id: s.id || crypto.randomUUID() }));
  } catch {
    return [];
  }
}

const subscriptions = ref<Subscription[]>(loadSubscriptions());

const searchQuery = ref("");
const filterFormat = ref<string[]>(["all"]);
const showFilterPanel = ref(false);
const formatOptions = [
  { label: t("subscriptions.formatClash"), value: "clash" },
  { label: t("subscriptions.formatV2rayN"), value: "v2rayn" },
  { label: t("subscriptions.formatSingbox"), value: "singbox" },
];

const filterLabel = computed(() => {
  if (filterFormat.value.includes("all") || filterFormat.value.length === 0) return t("common.all");
  return filterFormat.value.map(v => formatOptions.find(o => o.value === v)?.label || v).join(", ");
});

function toggleFilter(value: string) {
  if (value === "all") {
    filterFormat.value = ["all"];
    return;
  }
  const current = filterFormat.value.filter(v => v !== "all");
  const idx = current.indexOf(value);
  if (idx >= 0) {
    current.splice(idx, 1);
  } else {
    current.push(value);
  }
  filterFormat.value = current.length === 0 ? ["all"] : current;
}

function onFilterClickOutside(e: MouseEvent) {
  const target = e.target as HTMLElement;
  if (!target.closest(".sub-filter-dropdown")) {
    showFilterPanel.value = false;
  }
}

onMounted(() => document.addEventListener("click", onFilterClickOutside));
onUnmounted(() => document.removeEventListener("click", onFilterClickOutside));

const filteredSubscriptions = computed(() => {
  return subscriptions.value.filter(sub => {
    if (!filterFormat.value.includes("all") && !filterFormat.value.includes(sub.format)) return false;
    if (searchQuery.value) {
      const q = searchQuery.value.toLowerCase();
      return sub.name.toLowerCase().includes(q) || sub.url.toLowerCase().includes(q);
    }
    return true;
  });
});

watch(subscriptions, (val) => {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(val));
}, { deep: true });

const showDeleteDialog = ref(false);
const deleteTarget = ref<Subscription | null>(null);
const updating = ref<string | null>(null);

const showContextMenu = ref(false);
const contextMenuX = ref(0);
const contextMenuY = ref(0);
const contextMenuTarget = ref<Subscription | null>(null);
const contextMenuItems = ref<{ label: string; danger?: boolean; divider?: boolean }[]>([]);

const showCreateDialog = ref(false);
const newName = ref("");
const newDescription = ref("");
const newType = ref<ProfileType>("remote");
const newUrl = ref("");
const newFormat = ref<ConfigFormat>("clash");
const newUserAgent = ref("");
const newHttpTimeout = ref(10);
const newUpdateInterval = ref(120);
const newUseSystemProxy = ref(false);
const newUseCoreProxy = ref(false);
const newAllowInsecure = ref(false);
const newAllowAutoUpdate = ref(false);
const newPasteContent = ref("");
const newFileName = ref("");
const newFileContent = ref("");
const isDragOver = ref(false);
const fileInput = ref<HTMLInputElement | null>(null);

function resetCreateForm() {
  newName.value = "";
  newDescription.value = "";
  newType.value = "remote";
  newUrl.value = "";
  newFormat.value = "clash";
  newUserAgent.value = "";
  newHttpTimeout.value = 10;
  newUpdateInterval.value = 120;
  newUseSystemProxy.value = false;
  newUseCoreProxy.value = false;
  newAllowInsecure.value = false;
  newAllowAutoUpdate.value = false;
  newPasteContent.value = "";
  newFileName.value = "";
  newFileContent.value = "";
}

function createNew() {
  resetCreateForm();
  showCreateDialog.value = true;
}

function handleFileSelect(e: Event) {
  const input = e.target as HTMLInputElement;
  if (input.files && input.files[0]) {
    readFile(input.files[0]);
  }
}

function readFile(file: File) {
  newFileName.value = file.name;
  const reader = new FileReader();
  reader.onload = (e) => {
    newFileContent.value = (e.target?.result as string) || "";
  };
  reader.readAsText(file);
}

function onDragOver(e: DragEvent) {
  e.preventDefault();
  isDragOver.value = true;
}

function onDragLeave() {
  isDragOver.value = false;
}

function onDrop(e: DragEvent) {
  e.preventDefault();
  isDragOver.value = false;
  const file = e.dataTransfer?.files[0];
  if (file) readFile(file);
}

function clearFile() {
  newFileName.value = "";
  newFileContent.value = "";
  if (fileInput.value) fileInput.value.value = "";
}

async function doCreate() {
  if (!newName.value.trim()) {
    show(t("profiles.enterName"), "error");
    return;
  }
  if (newType.value === "remote" && !newUrl.value.trim()) {
    show(t("subscriptions.enterSubUrl"), "error");
    return;
  }
  if (newType.value === "local" && !newFileContent.value) {
    show(t("subscriptions.importFile"), "error");
    return;
  }
  if (newType.value === "script" && !newPasteContent.value.trim()) {
    show(t("subscriptions.pasteContent"), "error");
    return;
  }
let rawContent: string | undefined;
  let fileContent: string | undefined;
  if (newType.value === "local" && newFileContent.value) {
    rawContent = newFileContent.value;
    if (newFormat.value === "clash") {
      fileContent = rawContent;
    } else {
      try {
        fileContent = await convertContent(rawContent, newFormat.value);
      } catch (e: any) {
        show(`${e}`, "error");
        return;
      }
    }
  } else if (newType.value === "script" && newPasteContent.value) {
    rawContent = newPasteContent.value;
    if (newFormat.value === "clash") {
      fileContent = rawContent;
    } else {
      try {
        fileContent = await convertContent(rawContent, newFormat.value);
      } catch (e: any) {
        show(`${e}`, "error");
        return;
      }
    }
  }
  const newSub = {
    id: crypto.randomUUID(),
    name: newName.value.trim(),
    description: newDescription.value.trim(),
    type: newType.value,
    url: newUrl.value.trim(),
    format: newFormat.value,
    userAgent: newUserAgent.value.trim(),
    httpTimeout: newHttpTimeout.value,
    updateInterval: newUpdateInterval.value,
    useSystemProxy: newUseSystemProxy.value,
    useCoreProxy: newUseCoreProxy.value,
    allowInsecure: newAllowInsecure.value,
    allowAutoUpdate: newAllowAutoUpdate.value,
    lastUpdate: new Date().toISOString().split("T")[0],
    timeAgo: t("subscriptions.justNow"),
    fileName: newFileName.value || undefined,
    fileContent,
    pasteContent: fileContent,
    rawContent,
  };
  subscriptions.value = [...subscriptions.value, newSub];
  show(t("subscriptions.added", { name: newName.value }), "success");
  showCreateDialog.value = false;
}

async function refreshSubscription(id: string) {
  const sub = subscriptions.value.find(s => s.id === id);
  if (!sub) return;
  updating.value = id;
  try {
    if (sub.type === "remote" && sub.url) {
      let content = await fetchSubscriptionUrl(sub.url);
      if (sub.format !== "clash") {
        content = await convertContent(content, sub.format);
      }
      sub.fileContent = content;
    }
    sub.lastUpdate = new Date().toISOString().split("T")[0];
    sub.timeAgo = t("subscriptions.justNow");
    show(`${t('subscriptions.refresh')}: ${sub.name}`, "success");
    if (sub.id === activeSubId.value) {
      await applySub(sub, true);
    }
  } catch (e: any) {
    show(`${sub.name}: ${e}`, "error");
  } finally {
    updating.value = null;
  }
}

function refreshAll() {
  subscriptions.value.forEach(sub => refreshSubscription(sub.id));
}

function clearAllSubs() {
  subscriptions.value = [];
  show(t("subscriptions.clearAll") || "已清空所有订阅", "success");
}

const applying = ref<string | null>(null);

async function applySub(sub: Subscription, silent = false) {
  selectedSub.value = sub.id;
  activeSubId.value = sub.id;
  app.activeSubId = sub.id;
  app.activeSubName = sub.name;
  app.activeSubUrl = sub.url || "";
  app.activeSubUpdateTime = sub.lastUpdate || "";
  localStorage.setItem(ACTIVE_KEY, sub.id);
  applying.value = sub.id;
  try {
    let content = sub.fileContent || sub.pasteContent || "";
    let needConvert = sub.format !== "clash" && !sub.fileContent;
    if (!content && sub.type === "remote" && sub.url) {
      content = await fetchSubscriptionUrl(sub.url);
      needConvert = true;
    }
    if (needConvert && content) {
      try {
        content = await convertContent(content, sub.format);
      } catch (e: any) {
        show(`${sub.name}: ${e}`, "error");
        sub.fileContent = "";
        applying.value = null;
        return;
      }
      sub.fileContent = content;
    }
    if (!content) {
      show(t("subscriptions.enterSubUrl"), "error");
      return;
    }
    let finalContent = content;
    if (app.overrideMergeContent && app.overrideMergeContent !== "# override config") {
      finalContent = content + "\n" + app.overrideMergeContent;
    }
    await writeConfigOnly(finalContent);
    if (app.proxyRunning) {
      try { await app.stopCoreCmd(); } catch {}
    }
    try { await app.startCoreCmd(); } catch {}
    sub.lastUpdate = new Date().toISOString().split("T")[0];
    sub.timeAgo = t("subscriptions.justNow");
    if (!silent) {
      show(t("subscriptions.switchedTo", { name: sub.name }), "success");
    }
    localStorage.setItem("ns-vpn-active-sub", sub.id);
  } catch (e: any) {
    show(`${sub.name}: ${e}`, "error");
  } finally {
    applying.value = null;
  }
}

function confirmDelete(sub: Subscription) {
  deleteTarget.value = sub;
  showDeleteDialog.value = true;
}

function doDelete() {
  if (deleteTarget.value) {
    const name = deleteTarget.value.name;
    subscriptions.value = subscriptions.value.filter(s => s.id !== deleteTarget.value!.id);
    show(`${t('subscriptions.delete')}: ${name}`, "success");
  }
  showDeleteDialog.value = false;
  deleteTarget.value = null;
}

const showEditDialog = ref(false);
const editTarget = ref<Subscription | null>(null);
const editName = ref("");
const editDescription = ref("");
const editUrl = ref("");
const editFormat = ref<ConfigFormat>("clash");

const showEditFileDialog = ref(false);
const editFileContent = ref("");

const showEditRules = ref(false);
const editRulesContent = ref("");

const showEditProxies = ref(false);
const editProxiesContent = ref("");

const showEditGroups = ref(false);
const editGroupsContent = ref("");

const showEditMerge = ref(false);
const editMergeContent = ref("");

const showEditScript = ref(false);
const editScriptContent = ref("");

const editFileTarget = ref<Subscription | null>(null);
const editSectionType = ref("");

function openEditDialog(sub: Subscription) {
  editTarget.value = sub;
  editName.value = sub.name;
  editDescription.value = sub.description;
  editUrl.value = sub.url;
  editFormat.value = sub.format;
  showEditDialog.value = true;
}

function doEdit() {
  if (!editTarget.value) return;
  if (!editName.value.trim()) {
    show(t("profiles.enterName"), "error");
    return;
  }
  editTarget.value.name = editName.value.trim();
  editTarget.value.description = editDescription.value.trim();
  editTarget.value.url = editUrl.value.trim();
  editTarget.value.format = editFormat.value;
  showEditDialog.value = false;
}

async function openEditFileDialog(sub: Subscription) {
  editTarget.value = sub;
  if (sub.type === "remote" && sub.url && !sub.rawContent) {
    show(t("common.loading"), "info");
    try {
      const raw = await fetchSubscriptionUrl(sub.url);
      sub.rawContent = raw;
      sub.fileContent = sub.format === "clash"
        ? raw
        : await convertContent(raw, sub.format);
    } catch (e: any) {
      editFileContent.value = sub.fileContent || "";
      show(`${e}`, "error");
      showEditFileDialog.value = true;
      return;
    }
  }
  editFileContent.value = sub.rawContent || sub.fileContent || sub.pasteContent || "";
  showEditFileDialog.value = true;
}

async function doEditFile() {
  if (!editTarget.value) return;
  const isClash = editTarget.value.format === "clash";
  if (isClash) {
    editTarget.value.fileContent = editFileContent.value;
    editTarget.value.rawContent = undefined;
  } else {
    editTarget.value.rawContent = editFileContent.value;
    try {
      editTarget.value.fileContent = await convertContent(editFileContent.value, editTarget.value.format);
    } catch (e: any) {
      show(`转换失败: ${e}`, "error");
      return;
    }
  }
  showEditFileDialog.value = false;
  show(t("subscriptions.save") + ": " + editTarget.value.name, "success");
}

interface ParsedRule {
  type: string;
  content: string;
  proxy: string;
}

const RULE_TYPES = [
  "DOMAIN", "DOMAIN-SUFFIX", "DOMAIN-KEYWORD",
  "IP-CIDR", "IP-CIDR6", "SRC-IP-CIDR",
  "SRC-PORT", "DST-PORT",
  "PROCESS-NAME", "PROCESS-PATH",
  "GEOIP", "GEOSITE", "RULE-SET", "MATCH",
];

const SINGBOX_RULE_TYPES = [
  "domain", "domain_suffix", "domain_keyword",
  "ip_cidr", "ip_ispn", "src_ip_cidr",
  "src_port", "dst_port",
  "process_name", "process_path",
  "geoip", "geosite", "rule_set", "final",
];

const parsedRules = ref<ParsedRule[]>([]);
const parsedFormat = ref<ConfigFormat>("clash");
const ruleFilter = ref("");
const ruleFilterCase = ref(false);
const ruleFilterRegex = ref(false);
const newRuleType = ref("DOMAIN");
const newRuleContent = ref("");
const newRuleProxy = ref("DIRECT");

const filteredRules = computed(() => {
  const q = ruleFilter.value;
  if (!q) return parsedRules.value;
  return parsedRules.value.filter(r => {
    const hay = `${r.type} ${r.content} ${r.proxy}`;
    if (ruleFilterRegex.value) {
      try {
        const re = new RegExp(q, ruleFilterCase.value ? "" : "i");
        return re.test(hay);
      } catch { return true; }
    }
    if (ruleFilterCase.value) return hay.includes(q);
    return hay.toLowerCase().includes(q.toLowerCase());
  });
});

const RULE_TYPE_MAP_CLASH_TO_SBOX: Record<string, string> = {
  "DOMAIN": "domain",
  "DOMAIN-SUFFIX": "domain_suffix",
  "DOMAIN-KEYWORD": "domain_keyword",
  "IP-CIDR": "ip_cidr",
  "IP-CIDR6": "ip_cidr",
  "SRC-IP-CIDR": "src_ip_cidr",
  "SRC-PORT": "src_port",
  "DST-PORT": "dst_port",
  "PROCESS-NAME": "process_name",
  "PROCESS-PATH": "process_path",
  "GEOIP": "geoip",
  "GEOSITE": "geosite",
  "RULE-SET": "rule_set",
  "MATCH": "final",
};

const RULE_TYPE_MAP_SBOX_TO_CLASH: Record<string, string> = {};
for (const [k, v] of Object.entries(RULE_TYPE_MAP_CLASH_TO_SBOX)) {
  RULE_TYPE_MAP_SBOX_TO_CLASH[v] = k;
}

function parseRules(content: string, format: ConfigFormat): ParsedRule[] {
  switch (format) {
    case "clash":
      return parseRulesFromYaml(content);
    case "singbox":
      return parseRulesFromSingbox(content);
    case "v2rayn":
      return parseRulesFromV2rayN(content);
    default:
      return parseRulesFromYaml(content);
  }
}

function parseRulesFromYaml(yaml: string): ParsedRule[] {
  const lines = yaml.split("\n");
  const result: ParsedRule[] = [];
  let inRules = false;
  let rulesIndent = 0;
  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.startsWith("rules:")) {
      inRules = true;
      rulesIndent = line.search(/\S/);
      continue;
    }
    if (inRules) {
      const indent = line.search(/\S/);
      if (indent !== -1 && indent <= rulesIndent && !trimmed.startsWith("-")) {
        break;
      }
      if (trimmed.startsWith("- ")) {
        const ruleStr = trimmed.slice(2).trim();
        const parts = ruleStr.split(",").map(s => s.trim());
        if (parts.length >= 2) {
          const type = parts[0];
          const proxy = parts[parts.length - 1];
          const content = parts.slice(1, -1).join(",");
          result.push({ type, content, proxy });
        }
      }
    }
  }
  if (result.length === 0) {
    result.push({ type: "MATCH", content: "", proxy: "Proxy" });
  }
  return result;
}

function parseRulesFromSingbox(json: string): ParsedRule[] {
  try {
    const obj = JSON.parse(json);
    const rules = obj.route?.rules || obj.rules || [];
    if (!Array.isArray(rules)) return [];
    const result: ParsedRule[] = [];
    for (const r of rules) {
      const sboxType = r.type || "";
      const clashType = RULE_TYPE_MAP_SBOX_TO_CLASH[sboxType] || sboxType.toUpperCase();
      const proxy = r.outbound || "DIRECT";
      let content = "";
      if (r.domain) content = (Array.isArray(r.domain) ? r.domain : [r.domain]).join(",");
      else if (r.ip_cidr) content = (Array.isArray(r.ip_cidr) ? r.ip_cidr : [r.ip_cidr]).join(",");
      else if (r.src_ip_cidr) content = (Array.isArray(r.src_ip_cidr) ? r.src_ip_cidr : [r.src_ip_cidr]).join(",");
      else if (r.port) content = (Array.isArray(r.port) ? r.port.join(",") : String(r.port));
      else if (r.process_name) content = (Array.isArray(r.process_name) ? r.process_name : [r.process_name]).join(",");
      result.push({ type: clashType, content, proxy });
    }
    return result;
  } catch {
    return [];
  }
}

function parseRulesFromV2rayN(content: string): ParsedRule[] {
  const lines = content.split("\n").filter(l => l.trim());
  if (!lines.length) return [];
  let decoded = content.trim();
  try {
    const bytes = Uint8Array.from(atob(decoded.replace(/\s/g, "")), c => c.charCodeAt(0));
    decoded = new TextDecoder().decode(bytes);
  } catch { /* already raw */ }
  const uris = decoded.split("\n").map(l => l.trim()).filter(l => l.includes("://"));
  if (!uris.length) return [];
  return [{ type: "MATCH", content: "", proxy: "Proxy" }];
}

function serializeRules(rules: ParsedRule[], format: ConfigFormat): string {
  switch (format) {
    case "clash":
      return serializeRulesAsYaml(rules);
    case "singbox":
      return serializeRulesAsSingbox(rules);
    case "v2rayn":
      return "";
    default:
      return serializeRulesAsYaml(rules);
  }
}

function serializeRulesAsYaml(rules: ParsedRule[]): string {
  if (!rules.length) return "";
  return "rules:\n" + rules.map(r => {
    if (r.type === "MATCH") return `  - ${r.type},${r.proxy}`;
    return `  - ${r.type},${r.content},${r.proxy}`;
  }).join("\n");
}

function serializeRulesAsSingbox(rules: ParsedRule[]): string {
  const sboxRules = rules.map(r => {
    const sboxType = RULE_TYPE_MAP_CLASH_TO_SBOX[r.type.toLowerCase()] || r.type.toLowerCase();
    const rule: any = { type: sboxType, outbound: r.proxy };
    const parts = r.content ? r.content.split(",").map(s => s.trim()).filter(Boolean) : [];
    if (sboxType === "domain" || sboxType === "domain_suffix" || sboxType === "domain_keyword") {
      rule.domain = parts;
    } else if (sboxType === "ip_cidr" || sboxType === "ip_ispn") {
      rule.ip_cidr = parts;
    } else if (sboxType === "src_ip_cidr") {
      rule.src_ip_cidr = parts;
    } else if (sboxType === "src_port" || sboxType === "dst_port") {
      rule[sboxType === "src_port" ? "source_port" : "port"] = parts.map(Number);
    } else if (sboxType === "process_name" || sboxType === "process_path") {
      rule.process_name = parts;
    } else if (sboxType === "final") {
      delete rule.type;
      rule.type = "final";
    }
    return rule;
  });
  return JSON.stringify({ route: { rules: sboxRules } }, null, 2);
}

function addRule(prepend: boolean) {
  if (!newRuleContent.value.trim() && newRuleType.value !== "MATCH") return;
  const rule: ParsedRule = {
    type: newRuleType.value,
    content: newRuleContent.value.trim(),
    proxy: newRuleProxy.value,
  };
  if (prepend) {
    parsedRules.value.unshift(rule);
  } else {
    parsedRules.value.push(rule);
  }
  newRuleContent.value = "";
}

function removeRule(index: number) {
  parsedRules.value.splice(index, 1);
}

async function openEditSection(sub: Subscription, section: string) {
  editFileTarget.value = sub;
  editSectionType.value = section;
  let content = sub.rawContent || sub.fileContent || sub.pasteContent || "";
  if (!content && sub.type === "remote" && sub.url) {
    try {
      const raw = await fetchSubscriptionUrl(sub.url);
      sub.rawContent = raw;
      sub.fileContent = sub.format === "clash"
        ? raw
        : await convertContent(raw, sub.format);
      content = raw;
    } catch (e: any) {
      show(`${e}`, "error");
      return;
    }
  }
  const sectionContent = extractSection(content, section);
  switch (section) {
    case "rules":
      parsedFormat.value = "clash";
      parsedRules.value = parseRules(sub.fileContent || content, "clash");
      ruleFilter.value = "";
      showEditRules.value = true;
      break;
    case "proxies": editProxiesContent.value = sectionContent; showEditProxies.value = true; break;
    case "proxy-groups": editGroupsContent.value = sectionContent; showEditGroups.value = true; break;
    case "merge": editMergeContent.value = sectionContent; showEditMerge.value = true; break;
    case "script": editScriptContent.value = sectionContent; showEditScript.value = true; break;
  }
}

function extractSection(content: string, section: string): string {
  if (!content) return "";
  const lines = content.split("\n");
  const startIdx = lines.findIndex(l => l.trim().startsWith(section + ":"));
  if (startIdx === -1) return "";
  const indent = lines[startIdx].search(/\S/);
  let endIdx = lines.length;
  for (let i = startIdx + 1; i < lines.length; i++) {
    const lineIndent = lines[i].search(/\S/);
    if (lineIndent !== -1 && lineIndent <= indent) { endIdx = i; break; }
  }
  return lines.slice(startIdx, endIdx).join("\n").trim();
}

async function saveSection(sub: Subscription, section: string, value: string) {
  let content = sub.rawContent || sub.fileContent || sub.pasteContent || "";
  const lines = content.split("\n");
  const startIdx = lines.findIndex(l => l.trim().startsWith(section + ":"));
  if (startIdx === -1) { content += "\n" + value; }
  else {
    const indent = lines[startIdx].search(/\S/);
    let endIdx = lines.length;
    for (let i = startIdx + 1; i < lines.length; i++) {
      const lineIndent = lines[i].search(/\S/);
      if (lineIndent !== -1 && lineIndent <= indent) { endIdx = i; break; }
    }
    lines.splice(startIdx, endIdx - startIdx, ...value.split("\n"));
  }
  const newContent = lines.join("\n");
  const isClash = sub.format === "clash";
  if (isClash) {
    sub.fileContent = newContent;
    sub.rawContent = undefined;
  } else {
    sub.rawContent = newContent;
    try {
      sub.fileContent = await convertContent(newContent, sub.format);
    } catch (e: any) {
      show(`转换失败: ${e}`, "error");
      return;
    }
  }
  show(t("subscriptions.save") + ": " + sub.name, "success");
}

async function doEditRules() {
  if (!editFileTarget.value) return;
  const serialized = serializeRules(parsedRules.value, parsedFormat.value);
  if (serialized) {
    await saveSection(editFileTarget.value, "rules", serialized);
  } else {
    show(t("common.noData") || "No rules to save", "info");
  }
  showEditRules.value = false;
}

async function doEditProxies() {
  if (!editFileTarget.value) return;
  await saveSection(editFileTarget.value, "proxies", editProxiesContent.value);
  showEditProxies.value = false;
}

async function doEditGroups() {
  if (!editFileTarget.value) return;
  await saveSection(editFileTarget.value, "proxy-groups", editGroupsContent.value);
  showEditGroups.value = false;
}

function openOverrideMerge() {
  editMergeContent.value = app.overrideMergeContent;
  showEditMerge.value = true;
}

function openOverrideScript() {
  editScriptContent.value = app.overrideScriptContent;
  showEditScript.value = true;
}

async function doEditMerge() {
  app.overrideMergeContent = editMergeContent.value;
  showEditMerge.value = false;
  show(t("subscriptions.save") + ": " + t("subscriptions.mergeConfig"), "success");
}

async function doEditScript() {
  app.overrideScriptContent = editScriptContent.value;
  showEditScript.value = false;
  show(t("subscriptions.save") + ": " + t("subscriptions.scriptConfig"), "success");
}

const editFilePlaceholder = computed(() => {
  switch (editTarget.value?.format) {
    case "clash":
      return "proxies:\n  - name: proxy1\n    type: ss\n    server: example.com\n    port: 443\n    cipher: aes-256-gcm\n    password: password\nproxy-groups:\n  - name: Proxy\n    type: select\n    proxies:\n      - proxy1\nrules:\n  - MATCH,Proxy";
    case "v2rayn":
      return "dm1lc3M6Ly9... (Base64 encoded)\nss://... \ntrojan://...";
    case "singbox":
      return '{\n  "outbounds": [\n    {\n      "type": "shadowsocks",\n      "tag": "proxy1",\n      "server": "example.com",\n      "server_port": 443,\n      "method": "aes-256-gcm",\n      "password": "password"\n    }\n  ]\n}';
    default:
      return "";
  }
});

function onEditFileImport(e: Event) {
  const input = e.target as HTMLInputElement;
  if (!input.files || !input.files[0]) return;
  const reader = new FileReader();
  reader.onload = (ev) => {
    editFileContent.value = (ev.target?.result as string) || "";
  };
  reader.readAsText(input.files[0]);
  input.value = "";
}

function onContextMenu(e: MouseEvent, sub: Subscription) {
  e.preventDefault();
  contextMenuTarget.value = sub;
  contextMenuX.value = e.clientX;
  contextMenuY.value = e.clientY;

  const isRemote = sub.type === "remote";

  const items: { label: string; danger?: boolean; divider?: boolean }[] = [];

  if (isRemote) {
    items.push({ label: t("subscriptions.ctxUse") });
    items.push({ label: t("subscriptions.ctxShareQR") });
    items.push({ label: t("subscriptions.ctxEditInfo") });
    items.push({ label: t("subscriptions.ctxEditFile") });
    items.push({ label: t("subscriptions.ctxEditRules") });
    items.push({ label: t("subscriptions.ctxEditProxies") });
    items.push({ label: t("subscriptions.ctxEditProxyGroups") });
    items.push({ divider: true, label: "" });
    items.push({ label: t("subscriptions.ctxOverrideConfig") });
    items.push({ label: t("subscriptions.ctxOverrideScript") });
    items.push({ divider: true, label: "" });
    items.push({ label: t("subscriptions.ctxOpenFile") });
    items.push({ label: t("subscriptions.ctxUpdate") });
    items.push({ label: t("subscriptions.ctxUpdateProxy") });
    items.push({ divider: true, label: "" });
    items.push({ label: t("subscriptions.ctxDelete"), danger: true });
  } else {
    items.push({ label: t("subscriptions.ctxUse") });
    items.push({ label: t("subscriptions.ctxEditInfo") });
    items.push({ label: t("subscriptions.ctxEditFile") });
    items.push({ label: t("subscriptions.ctxEditRules") });
    items.push({ label: t("subscriptions.ctxEditProxies") });
    items.push({ label: t("subscriptions.ctxEditProxyGroups") });
    items.push({ divider: true, label: "" });
    items.push({ label: t("subscriptions.ctxOverrideConfig") });
    items.push({ label: t("subscriptions.ctxOverrideScript") });
    items.push({ divider: true, label: "" });
    items.push({ label: t("subscriptions.ctxOpenFile") });
    items.push({ divider: true, label: "" });
    items.push({ label: t("subscriptions.ctxDelete"), danger: true });
  }

  contextMenuItems.value = items;
  showContextMenu.value = true;
}

function onContextMenuSelect(index: number) {
  const sub = contextMenuTarget.value;
  if (!sub) return;
  const isRemote = sub.type === "remote";

  if (isRemote) {
    switch (index) {
      case 0: applySub(sub); break;
      case 1: shareQR(sub); break;
      case 2: openEditDialog(sub); break;
      case 3: openEditFileDialog(sub); break;
      case 4: openEditSection(sub, "rules"); break;
      case 5: openEditSection(sub, "proxies"); break;
      case 6: openEditSection(sub, "proxy-groups"); break;
      case 8: openEditSection(sub, "merge"); break;
      case 9: openEditSection(sub, "script"); break;
      case 11: openAppDir(); break;
      case 12: refreshSubscription(sub.id); break;
      case 13: refreshSubscription(sub.id); break;
      case 15: confirmDelete(sub); break;
    }
  } else {
    switch (index) {
      case 0: applySub(sub); break;
      case 1: openEditDialog(sub); break;
      case 2: openEditFileDialog(sub); break;
      case 3: openEditSection(sub, "rules"); break;
      case 4: openEditSection(sub, "proxies"); break;
      case 5: openEditSection(sub, "proxy-groups"); break;
      case 7: openEditSection(sub, "merge"); break;
      case 8: openEditSection(sub, "script"); break;
      case 10: openAppDir(); break;
      case 12: confirmDelete(sub); break;
    }
  }
  showContextMenu.value = false;
}

function shareQR(sub: Subscription) {
  qrValue.value = sub.url || sub.name;
  qrTitle.value = sub.name;
  showQrViewer.value = true;
}

const showQrViewer = ref(false);
const qrValue = ref("");
const qrTitle = ref("");

function formatLabel(f: ConfigFormat): string {
  switch (f) {
    case "clash": return t("subscriptions.formatClash");
    case "v2rayn": return t("subscriptions.formatV2rayN");
    case "singbox": return t("subscriptions.formatSingbox");
  }
}

function typeIcon(type: ProfileType) {
  switch (type) {
    case "remote": return Link;
    case "local": return FolderOpen;
    case "script": return FileCode;
  }
}
</script>

<template>
  <div class="sub-page">
    <div class="sub-header">
      <h1 class="sub-title">{{ t('subscriptions.title') }} <span class="sub-count">{{ subscriptions.length }}</span></h1>
      <div class="sub-header-actions">
        <button class="sub-create-btn-top" @click="createNew">{{ t('subscriptions.create') }}</button>
        <button class="header-icon-btn" :title="t('common.refresh')" @click="refreshAll">
          <RefreshCw :size="18" />
        </button>
        <button v-if="subscriptions.length > 0" class="header-icon-btn header-icon-btn-danger" :title="t('subscriptions.clearAll')" @click="clearAllSubs">
          <Trash2 :size="16" />
        </button>
      </div>
    </div>

    <div class="sub-search-row">
      <input v-model="searchQuery" :placeholder="t('common.search') + '...'" class="sub-search-input" />
      <div class="sub-filter-dropdown">
        <button class="sub-filter-btn" @click="showFilterPanel = !showFilterPanel">
          {{ filterLabel }}
          <span v-if="filterFormat.length > 0 && !filterFormat.includes('all')" class="filter-badge">{{ filterFormat.length }}</span>
        </button>
        <div v-if="showFilterPanel" class="filter-panel">
          <label class="filter-option">
            <input type="checkbox" value="all" :checked="filterFormat.includes('all')" @change="toggleFilter('all')" />
            <span>{{ t('common.all') }}</span>
          </label>
          <label class="filter-option" v-for="opt in formatOptions" :key="opt.value">
            <input type="checkbox" :value="opt.value" :checked="filterFormat.includes(opt.value)" @change="toggleFilter(opt.value)" />
            <span>{{ opt.label }}</span>
          </label>
        </div>
      </div>
      <button class="header-icon-btn" :title="t('common.search')">
        <Search :size="16" />
      </button>
    </div>
    <div class="sub-actions-row">
    </div>

    <div class="sub-grid">
      <div
        v-for="(sub, idx) in filteredSubscriptions"
        :key="'grid-' + sub.id + '-' + idx"
        class="sub-card"
        :class="{ 'sub-card-active': selectedSub === sub.id, 'sub-card-applying': applying === sub.id }"
        @click="applySub(sub)"
        @contextmenu.prevent="onContextMenu($event, sub)"
      >
        <div class="sub-drag">
          <GripVertical :size="14" />
        </div>
        <div class="sub-content">
          <div class="sub-top-row">
            <span class="sub-name">{{ sub.name }}</span>
            <button class="sub-refresh-btn" :disabled="updating === sub.id || applying === sub.id" @click.stop="refreshSubscription(sub.id)">
              <RefreshCw :size="12" :class="{ spin: updating === sub.id || applying === sub.id }" />
            </button>
          </div>
          <div class="sub-bottom-row">
            <span class="sub-url">{{ sub.type === 'remote' ? sub.url : sub.fileName || sub.type }}</span>
            <span class="sub-time">{{ sub.timeAgo }}</span>
          </div>
          <div class="sub-meta">
            <span class="sub-format-tag">{{ formatLabel(sub.format) }}</span>
            <span class="sub-date">{{ sub.lastUpdate }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="sub-footer">
      <div class="sub-footer-card" @click="openOverrideMerge">
        <span class="footer-label">{{ t('subscriptions.mergeConfig') }}</span>
        <span class="footer-badge footer-badge-merge">Merge</span>
        <Edit :size="14" class="footer-icon" />
      </div>
      <div class="sub-footer-card" @click="openOverrideScript">
        <span class="footer-label">{{ t('subscriptions.scriptConfig') }}</span>
        <span class="footer-badge footer-badge-script">Script</span>
        <FileCode :size="14" class="footer-icon" />
      </div>
    </div>

    <ConfirmDialog
      :show="showDeleteDialog"
      :title="t('subscriptions.delete')"
      :message="t('subscriptions.confirmDeleteMsg', { name: deleteTarget?.name })"
      :confirm-text="t('common.delete')"
      type="danger"
      @confirm="doDelete"
      @cancel="showDeleteDialog = false"
    />

    <ContextMenu
      v-if="showContextMenu"
      :items="contextMenuItems"
      :x="contextMenuX"
      :y="contextMenuY"
      @select="onContextMenuSelect"
      @close="showContextMenu = false"
    />

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showEditDialog" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showEditDialog = false">
          <div class="create-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="dialog-title">{{ t('subscriptions.ctxEditInfo') }}</h3>
            <div class="dialog-body">
              <div class="field">
                <label class="field-label">{{ t('subscriptions.name') }}</label>
                <input v-model="editName" :placeholder="t('subscriptions.namePlaceholder')" class="field-input" />
              </div>
              <div class="field">
                <label class="field-label">{{ t('subscriptions.description') }}</label>
                <input v-model="editDescription" :placeholder="t('subscriptions.descriptionPlaceholder')" class="field-input" />
              </div>
              <div class="field" v-if="editTarget?.type === 'remote'">
                <label class="field-label">{{ t('subscriptions.url') }}</label>
                <input v-model="editUrl" :placeholder="t('subscriptions.urlPlaceholder')" class="field-input" />
              </div>
              <div class="field">
                <label class="field-label">{{ t('subscriptions.configFormat') }}</label>
                <select v-model="editFormat" class="field-input field-select">
                  <option value="clash">{{ t('subscriptions.formatClash') }}</option>
                  <option value="v2rayn">{{ t('subscriptions.formatV2rayN') }}</option>
                  <option value="singbox">{{ t('subscriptions.formatSingbox') }}</option>
                </select>
              </div>
            </div>
            <div class="dialog-footer">
              <button class="btn-ghost text-xs" @click="showEditDialog = false">{{ t('subscriptions.cancel') }}</button>
              <button class="btn-primary text-xs" @click="doEdit">{{ t('subscriptions.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showCreateDialog" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showCreateDialog = false">
          <div class="create-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="dialog-title">{{ t('subscriptions.createTitle') }}</h3>

            <div class="dialog-body">
              <div class="field">
                <label class="field-label">{{ t('subscriptions.type') }}</label>
                <div class="type-tabs">
                  <button class="type-tab" :class="{ active: newType === 'remote' }" @click="newType = 'remote'">
                    <Link :size="14" /> {{ t('subscriptions.typeRemote') }}
                  </button>
                  <button class="type-tab" :class="{ active: newType === 'local' }" @click="newType = 'local'">
                    <FolderOpen :size="14" /> {{ t('subscriptions.typeLocal') }}
                  </button>
                  <button class="type-tab" :class="{ active: newType === 'script' }" @click="newType = 'script'">
                    <FileCode :size="14" /> {{ t('subscriptions.typeScript') }}
                  </button>
                </div>
              </div>

              <div class="field">
                <label class="field-label">{{ t('subscriptions.name') }}</label>
                <input v-model="newName" :placeholder="t('subscriptions.namePlaceholder')" class="field-input" />
              </div>

              <div class="field">
                <label class="field-label">{{ t('subscriptions.description') }}</label>
                <input v-model="newDescription" :placeholder="t('subscriptions.descriptionPlaceholder')" class="field-input" />
              </div>

              <div class="field">
                <label class="field-label">{{ t('subscriptions.configFormat') }}</label>
                <select v-model="newFormat" class="field-input field-select">
                  <option value="clash">{{ t('subscriptions.formatClash') }}</option>
                  <option value="v2rayn">{{ t('subscriptions.formatV2rayN') }}</option>
                  <option value="singbox">{{ t('subscriptions.formatSingbox') }}</option>
                </select>
              </div>

              <template v-if="newType === 'remote'">
                <div class="field">
                  <label class="field-label">{{ t('subscriptions.url') }}</label>
                  <input v-model="newUrl" :placeholder="t('subscriptions.urlPlaceholder')" class="field-input" />
                </div>
                <div class="field">
                  <label class="field-label">{{ t('subscriptions.userAgent') }}</label>
                  <input v-model="newUserAgent" :placeholder="t('subscriptions.userAgentPlaceholder')" class="field-input" />
                </div>
              </template>

              <template v-if="newType === 'local'">
                <div class="field">
                  <label class="field-label">{{ t('subscriptions.importFile') }}</label>
                  <div
                    class="file-drop-zone"
                    :class="{ dragover: isDragOver, 'has-file': newFileName }"
                    @dragover="onDragOver"
                    @dragleave="onDragLeave"
                    @drop="onDrop"
                    @click="fileInput?.click()"
                  >
                    <input ref="fileInput" type="file" accept=".yaml,.yml,.json,.txt,.ovpn,.conf" class="hidden" @change="handleFileSelect" />
                    <template v-if="newFileName">
                      <FileText :size="18" :style="{ color: 'var(--green)' }" />
                      <span class="file-name">{{ newFileName }}</span>
                      <button class="file-clear" @click.stop="clearFile"><Trash2 :size="12" /></button>
                    </template>
                    <template v-else>
                      <Upload :size="18" :style="{ color: 'var(--text-secondary)' }" />
                      <span :style="{ color: 'var(--text-secondary)' }">{{ t('subscriptions.dragOrClick') }}</span>
                    </template>
                  </div>
                </div>
              </template>

              <template v-if="newType === 'script'">
                <div class="field">
                  <label class="field-label">{{ t('subscriptions.pasteContent') }}</label>
                  <textarea
                    v-model="newPasteContent"
                    :placeholder="t('subscriptions.pastePlaceholder')"
                    class="field-input field-textarea"
                    rows="8"
                  />
                </div>
              </template>

              <div class="field" v-if="newType === 'remote'">
                <label class="field-label">{{ t('subscriptions.httpTimeout') }}</label>
                <div class="field-with-unit">
                  <input v-model.number="newHttpTimeout" type="number" min="1" max="60" class="field-input field-number" />
                  <span class="field-unit">{{ t('subscriptions.seconds') }}</span>
                </div>
              </div>

              <div class="field" v-if="newType === 'remote'">
                <label class="field-label">{{ t('subscriptions.updateInterval') }}</label>
                <div class="field-with-unit">
                  <input v-model.number="newUpdateInterval" type="number" min="0" max="10080" class="field-input field-number" />
                  <span class="field-unit">{{ t('subscriptions.minutes') }}</span>
                </div>
              </div>

              <div class="toggles" v-if="newType === 'remote'">
                <label class="toggle-row">
                  <span>{{ t('subscriptions.useSystemProxy') }}</span>
                  <div class="toggle" :class="{ active: newUseSystemProxy }" @click="newUseSystemProxy = !newUseSystemProxy">
                    <div class="toggle-knob"></div>
                  </div>
                </label>
                <label class="toggle-row">
                  <span>{{ t('subscriptions.useCoreProxy') }}</span>
                  <div class="toggle" :class="{ active: newUseCoreProxy }" @click="newUseCoreProxy = !newUseCoreProxy">
                    <div class="toggle-knob"></div>
                  </div>
                </label>
                <label class="toggle-row">
                  <span>{{ t('subscriptions.allowInsecure') }}</span>
                  <div class="toggle" :class="{ active: newAllowInsecure }" @click="newAllowInsecure = !newAllowInsecure">
                    <div class="toggle-knob"></div>
                  </div>
                </label>
                <label class="toggle-row">
                  <span>{{ t('subscriptions.allowAutoUpdate') }}</span>
                  <div class="toggle" :class="{ active: newAllowAutoUpdate }" @click="newAllowAutoUpdate = !newAllowAutoUpdate">
                    <div class="toggle-knob"></div>
                  </div>
                </label>
              </div>
            </div>

            <div class="dialog-footer">
              <button class="btn-ghost text-xs" @click="showCreateDialog = false">{{ t('subscriptions.cancel') }}</button>
              <button class="btn-primary text-xs" @click="doCreate">{{ t('subscriptions.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showEditFileDialog" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showEditFileDialog = false">
          <div class="create-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="dialog-title">{{ t('subscriptions.ctxEditFile') }} — {{ editTarget?.name }}</h3>
            <div class="dialog-body">
              <div class="field">
                <div class="edit-file-header">
                  <span class="field-label">{{ t('subscriptions.configFormat') }}: {{ formatLabel(editTarget?.format || 'clash') }}</span>
                  <label class="btn-ghost text-xs file-import-label">
                    {{ t('subscriptions.importFile') }}
                    <input type="file" accept=".yaml,.yml,.json,.txt,.ovpn,.conf" class="hidden" @change="onEditFileImport" />
                  </label>
                </div>
                <textarea
                  v-model="editFileContent"
                  class="field-input field-textarea"
                  rows="16"
                  :placeholder="editFilePlaceholder"
                  spellcheck="false"
                />
              </div>
            </div>
            <div class="dialog-footer">
              <button class="btn-ghost text-xs" @click="showEditFileDialog = false">{{ t('subscriptions.cancel') }}</button>
              <button class="btn-primary text-xs" @click="doEditFile">{{ t('subscriptions.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showEditRules" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showEditRules = false">
          <div class="rules-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="dialog-title">{{ t('subscriptions.ctxEditRules') }} — {{ editFileTarget?.name }}</h3>
            <div class="rules-body">
              <template v-if="parsedFormat === 'v2rayn'">
                <div class="rules-empty" style="width:100%">
                  {{ 'v2rayN 格式无规则定义' }}
                </div>
              </template>
              <template v-else>
              <div class="rules-left">
                <div class="field">
                  <label class="field-label">{{ t('rules.type') || '规则类型' }}</label>
                  <select v-model="newRuleType" class="field-input field-select">
                    <option v-for="rt in RULE_TYPES" :key="rt" :value="rt">{{ rt }}</option>
                  </select>
                </div>
                <div class="field">
                  <label class="field-label">{{ t('rules.payload') || '规则内容' }}</label>
                  <input v-model="newRuleContent" :placeholder="newRuleType === 'MATCH' ? 'MATCH' : 'example.com'" class="field-input" :disabled="newRuleType === 'MATCH'" />
                </div>
                <div class="field">
                  <label class="field-label">{{ t('rules.proxy') || '代理策略' }}</label>
                  <input v-model="newRuleProxy" placeholder="DIRECT" class="field-input" />
                </div>
                <button class="rules-add-btn rules-add-prepend" @click="addRule(true)">
                  <ArrowUpFromLine :size="14" /> {{ t('subscriptions.addPrependRule') || '添加前置规则' }}
                </button>
                <button class="rules-add-btn rules-add-append" @click="addRule(false)">
                  <ArrowDownToLine :size="14" /> {{ t('subscriptions.addAppendRule') || '添加后置规则' }}
                </button>
              </div>
              <div class="rules-right">
                <div class="rules-filter-bar">
                  <input v-model="ruleFilter" class="rules-filter-input" :placeholder="t('common.search') + '...'" />
                  <button class="rules-filter-btn" :class="{ active: ruleFilterCase }" @click="ruleFilterCase = !ruleFilterCase" title="Aa">Aa</button>
                  <button class="rules-filter-btn" :class="{ active: ruleFilterRegex }" @click="ruleFilterRegex = !ruleFilterRegex" title="Regex">.*</button>
                  <button v-if="ruleFilter" class="rules-filter-clear" @click="ruleFilter = ''"><X :size="12" /></button>
                </div>
                <div class="rules-list">
                  <div v-for="(rule, idx) in filteredRules" :key="idx" class="rule-item">
                    <div class="rule-item-content">
                      <span class="rule-item-text">{{ rule.type === 'MATCH' ? 'MATCH' : rule.content }}</span>
                      <span class="rule-item-badge">{{ rule.type }}</span>
                      <span class="rule-item-proxy">{{ rule.proxy }}</span>
                    </div>
                    <button class="rule-item-delete" @click="removeRule(parsedRules.indexOf(rule))">
                      <Trash2 :size="14" />
                    </button>
                  </div>
                  <div v-if="!filteredRules.length" class="rules-empty">
                    {{ t('common.noData') }}
                  </div>
                </div>
              </div>
              </template>
            </div>
            <div class="dialog-footer">
              <button class="btn-ghost text-xs" @click="showEditRules = false">{{ t('subscriptions.cancel') }}</button>
              <button class="btn-primary text-xs" @click="doEditRules">{{ t('subscriptions.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showEditProxies" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showEditProxies = false">
          <div class="create-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="dialog-title">{{ t('subscriptions.ctxEditProxies') }} — {{ editFileTarget?.name }}</h3>
            <div class="dialog-body">
              <div class="field">
                <span class="field-label">{{ t('subscriptions.ctxEditProxies') }}</span>
                <textarea v-model="editProxiesContent" class="field-input field-textarea" rows="12" placeholder="proxies:\n  - name: proxy1\n    type: ss\n    server: example.com\n    port: 443" spellcheck="false" />
              </div>
            </div>
            <div class="dialog-footer">
              <button class="btn-ghost text-xs" @click="showEditProxies = false">{{ t('subscriptions.cancel') }}</button>
              <button class="btn-primary text-xs" @click="doEditProxies">{{ t('subscriptions.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showEditGroups" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showEditGroups = false">
          <div class="create-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="dialog-title">{{ t('subscriptions.ctxEditProxyGroups') }} — {{ editFileTarget?.name }}</h3>
            <div class="dialog-body">
              <div class="field">
                <span class="field-label">{{ t('subscriptions.ctxEditProxyGroups') }}</span>
                <textarea v-model="editGroupsContent" class="field-input field-textarea" rows="12" placeholder="proxy-groups:\n  - name: Proxy\n    type: select\n    proxies:\n      - proxy1" spellcheck="false" />
              </div>
            </div>
            <div class="dialog-footer">
              <button class="btn-ghost text-xs" @click="showEditGroups = false">{{ t('subscriptions.cancel') }}</button>
              <button class="btn-primary text-xs" @click="doEditGroups">{{ t('subscriptions.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showEditMerge" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showEditMerge = false">
          <div class="create-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="dialog-title">{{ t('subscriptions.mergeConfig') }}</h3>
            <div class="dialog-body">
              <div class="field">
                <span class="field-label">{{ t('subscriptions.ctxOverrideConfig') }}</span>
                <textarea v-model="editMergeContent" class="field-input field-textarea" rows="12" placeholder="# Global override config\n# Add your overrides here" spellcheck="false" />
              </div>
            </div>
            <div class="dialog-footer">
              <button class="btn-ghost text-xs" @click="showEditMerge = false">{{ t('subscriptions.cancel') }}</button>
              <button class="btn-primary text-xs" @click="doEditMerge">{{ t('subscriptions.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <Teleport to="body">
      <Transition name="page">
        <div v-if="showEditScript" class="fixed inset-0 flex items-center justify-center bg-black/50 z-50" @click="showEditScript = false">
          <div class="create-dialog" :style="{ backgroundColor: 'var(--bg-secondary)' }" @click.stop>
            <h3 class="dialog-title">{{ t('subscriptions.scriptConfig') }}</h3>
            <div class="dialog-body">
              <div class="field">
                <span class="field-label">{{ t('subscriptions.ctxOverrideScript') }}</span>
                <textarea v-model="editScriptContent" class="field-input field-textarea" rows="12" placeholder="// Override script\n// Add your script here" spellcheck="false" />
              </div>
            </div>
            <div class="dialog-footer">
              <button class="btn-ghost text-xs" @click="showEditScript = false">{{ t('subscriptions.cancel') }}</button>
              <button class="btn-primary text-xs" @click="doEditScript">{{ t('subscriptions.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <QrViewer :open="showQrViewer" :value="qrValue" :title="qrTitle" @close="showQrViewer = false" />
  </div>
</template>

<style scoped>
.sub-page {
  max-width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.sub-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.sub-title {
  font-size: 22px;
  font-weight: 700;
  margin: 0;
  display: flex;
  align-items: center;
  gap: 8px;
}

.sub-count {
  font-size: 12px;
  font-weight: 600;
  padding: 1px 8px;
  border-radius: 6px;
  background-color: rgba(79,142,247,0.12);
  color: var(--accent);
}

.sub-header-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.header-icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  transition: all 150ms ease;
  background: transparent;
  color: var(--text-secondary);
}
.header-icon-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}
.header-icon-btn-danger:hover {
  background-color: rgba(255,59,48,0.08);
  color: var(--red);
}

.sub-search-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  flex-shrink: 0;
}

.sub-search-input {
  max-width: 240px;
  padding: 8px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  font-size: 13px;
  color: var(--text-primary);
  outline: none;
  transition: border-color 150ms;
}
.sub-search-input:focus {
  border-color: var(--accent);
}
.sub-search-input::placeholder {
  color: var(--text-secondary);
}

.sub-filter-select {
  padding: 5px 10px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: transparent;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  outline: none;
  min-width: 100px;
  max-height: 120px;
}
.sub-filter-select:focus {
  border-color: var(--accent);
}

.sub-filter-dropdown {
  position: relative;
}

.sub-filter-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 14px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: transparent;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}
.sub-filter-btn:hover {
  border-color: var(--accent);
  color: var(--text-primary);
}

.filter-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 18px;
  height: 18px;
  padding: 0 5px;
  border-radius: 9px;
  background-color: var(--accent);
  color: #fff;
  font-size: 11px;
  font-weight: 600;
}

.filter-panel {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  min-width: 160px;
  padding: 6px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background-color: var(--bg-secondary);
  box-shadow: 0 4px 16px rgba(0,0,0,0.15);
  z-index: 100;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.filter-option {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-primary);
  transition: background 150ms;
}
.filter-option:hover {
  background-color: var(--bg-hover);
}
.filter-option input[type="checkbox"] {
  accent-color: var(--accent);
}
.sub-filter-select option {
  padding: 4px 8px;
  font-size: 12px;
}

.sub-actions-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
  flex-shrink: 0;
}

.edit-file-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.file-import-label {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
}

.sub-create-btn-top {
  padding: 6px 16px;
  border-radius: 6px;
  border: none;
  background-color: var(--accent);
  font-size: 13px;
  font-weight: 500;
  color: #fff;
  cursor: pointer;
  transition: all 150ms ease;
}
.sub-create-btn-top:hover {
  opacity: 0.9;
}

.sub-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  align-content: start;
  padding-right: 4px;
}

.sub-card {
  display: flex;
  align-items: stretch;
  padding: 12px;
  border-radius: 10px;
  border: 1px solid var(--border);
  border-left: 3px solid transparent;
  background-color: var(--card-bg);
  cursor: pointer;
  transition: all 150ms ease;
  position: relative;
  overflow: hidden;
  min-width: 0;
}
.sub-card:hover {
  border-color: var(--accent);
  border-left-color: var(--accent);
}

.sub-card-active {
  border-color: var(--accent) !important;
  border-left-color: var(--accent) !important;
  background-color: rgba(79,142,247,0.05);
}

.sub-card-applying {
  opacity: 0.7;
  pointer-events: none;
}

.sub-drag {
  display: flex;
  align-items: flex-start;
  padding-top: 2px;
  color: var(--text-secondary);
  opacity: 0.5;
  cursor: grab;
}

.sub-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.sub-top-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sub-name {
  font-size: 13px;
  font-weight: 600;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sub-desc {
  font-size: 11px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sub-refresh-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}
.sub-refresh-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}
.sub-refresh-btn:disabled {
  opacity: 0.5;
}

.sub-bottom-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sub-url {
  font-size: 11px;
  color: var(--text-secondary);
  font-family: "SF Mono", "Fira Code", monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sub-time {
  font-size: 11px;
  color: var(--text-secondary);
  white-space: nowrap;
  margin-left: 8px;
}

.sub-meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sub-format-tag {
  font-size: 10px;
  padding: 1px 6px;
  border-radius: 4px;
  background-color: rgba(79,142,247,0.12);
  color: var(--accent);
  font-weight: 500;
}

.sub-date {
  font-size: 11px;
  color: var(--text-secondary);
  text-align: right;
}

.sub-footer {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  margin-top: 16px;
  flex-shrink: 0;
}

.sub-footer-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 16px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background-color: var(--card-bg);
  cursor: pointer;
  transition: all 150ms ease;
}
.sub-footer-card:hover {
  border-color: var(--accent);
}

.footer-label {
  font-size: 13px;
  font-weight: 500;
}

.footer-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
}

.footer-badge-merge {
  background-color: rgba(52,199,89,0.12);
  color: var(--green);
}

.footer-badge-script {
  background-color: rgba(79,142,247,0.12);
  color: var(--accent);
}

.footer-icon {
  color: var(--text-secondary);
  margin-left: auto;
}

/* Dialog */
.create-dialog {
  width: 560px;
  max-width: 90vw;
  max-height: 85vh;
  border-radius: 14px;
  border: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.dialog-title {
  font-size: 16px;
  font-weight: 600;
  padding: 20px 24px 0;
  margin: 0;
}

.dialog-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 24px;
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
}

.field-input {
  width: 100%;
  padding: 8px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 150ms;
}
.field-input:focus {
  border-color: var(--accent);
}
.field-input::placeholder {
  color: var(--text-secondary);
  opacity: 0.6;
}

.field-select {
  appearance: none;
  cursor: pointer;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23999' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 10px center;
  padding-right: 32px;
}

.field-textarea {
  resize: vertical;
  min-height: 120px;
  font-family: "SF Mono", "Fira Code", monospace;
  font-size: 12px;
  line-height: 1.5;
}

.field-number {
  width: 100px;
}

.field-with-unit {
  display: flex;
  align-items: center;
  gap: 8px;
}

.field-unit {
  font-size: 12px;
  color: var(--text-secondary);
}

.type-tabs {
  display: flex;
  gap: 6px;
}

.type-tab {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: transparent;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}
.type-tab:hover {
  border-color: var(--accent);
  color: var(--text-primary);
}
.type-tab.active {
  border-color: var(--accent);
  background-color: rgba(79,142,247,0.08);
  color: var(--accent);
}

.file-drop-zone {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 24px;
  border-radius: 8px;
  border: 2px dashed var(--border);
  background: transparent;
  cursor: pointer;
  transition: all 150ms ease;
  min-height: 80px;
}
.file-drop-zone:hover,
.file-drop-zone.dragover {
  border-color: var(--accent);
  background-color: rgba(79,142,247,0.04);
}
.file-drop-zone.has-file {
  border-style: solid;
  border-color: var(--green);
  background-color: rgba(52,199,89,0.04);
}

.file-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.file-clear {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  margin-left: auto;
}
.file-clear:hover {
  color: var(--red);
}

.hidden {
  display: none;
}

.toggles {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.toggle-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
  font-size: 13px;
  cursor: pointer;
}

.toggle {
  position: relative;
  width: 36px;
  height: 20px;
  border-radius: 10px;
  background-color: var(--bg-tertiary);
  border: 1px solid var(--border);
  cursor: pointer;
  transition: all 200ms ease;
  flex-shrink: 0;
}
.toggle.active {
  background-color: var(--accent);
  border-color: var(--accent);
}
.toggle-knob {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: #fff;
  transition: transform 200ms ease;
  box-shadow: 0 1px 3px rgba(0,0,0,0.15);
}
.toggle.active .toggle-knob {
  transform: translateX(16px);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 24px 16px;
  border-top: 1px solid var(--border);
}

.spin {
  animation: spin 1s linear infinite;
}
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@media (max-width: 1200px) {
  .sub-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 900px) {
  .sub-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .sub-grid {
    grid-template-columns: 1fr;
  }
  .type-tabs {
    flex-direction: column;
  }
}

.rules-dialog {
  width: 780px;
  max-width: 92vw;
  max-height: 85vh;
  border-radius: 14px;
  border: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.rules-body {
  flex: 1;
  display: flex;
  gap: 16px;
  padding: 16px 24px;
  overflow: hidden;
}

.rules-left {
  width: 280px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.rules-right {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow: hidden;
  min-width: 0;
}

.rules-add-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 0;
  border-radius: 8px;
  border: none;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 150ms ease;
  color: #fff;
}
.rules-add-prepend {
  background-color: var(--accent);
}
.rules-add-append {
  background-color: var(--accent);
  opacity: 0.85;
}
.rules-add-btn:hover {
  opacity: 0.9;
}

.rules-filter-bar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 4px 4px 10px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--bg-tertiary);
}

.rules-filter-input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: 12px;
  color: var(--text-primary);
  outline: none;
}
.rules-filter-input::placeholder {
  color: var(--text-secondary);
}

.rules-filter-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid transparent;
  background: transparent;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}
.rules-filter-btn:hover {
  color: var(--text-primary);
}
.rules-filter-btn.active {
  border-color: var(--accent);
  color: var(--accent);
  background: rgba(79,142,247,0.08);
}

.rules-filter-clear {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
}
.rules-filter-clear:hover {
  color: var(--red);
}

.rules-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.rule-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--bg-tertiary);
  transition: all 150ms ease;
}
.rule-item:hover {
  border-color: var(--accent);
}

.rule-item-content {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
  flex: 1;
}

.rule-item-text {
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.rule-item-badge {
  font-size: 10px;
  padding: 1px 6px;
  border-radius: 4px;
  background-color: rgba(79,142,247,0.12);
  color: var(--accent);
  font-weight: 600;
  white-space: nowrap;
  flex-shrink: 0;
}

.rule-item-proxy {
  font-size: 11px;
  color: var(--text-secondary);
  margin-left: auto;
  white-space: nowrap;
  flex-shrink: 0;
}

.rule-item-delete {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  flex-shrink: 0;
  margin-left: 8px;
  transition: all 150ms ease;
}
.rule-item-delete:hover {
  color: var(--red);
  background: rgba(255,59,48,0.08);
}

.rules-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 32px;
  color: var(--text-secondary);
  font-size: 13px;
}
</style>
