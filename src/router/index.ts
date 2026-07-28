import { createRouter, createWebHistory } from "vue-router";
import Dashboard from "@/pages/Dashboard.vue";
import Proxies from "@/pages/Proxies.vue";
import Connections from "@/pages/Connections.vue";
import Rules from "@/pages/Rules.vue";
import Logs from "@/pages/Logs.vue";
import Subscriptions from "@/pages/Subscriptions.vue";
import Test from "@/pages/Test.vue";
import Settings from "@/pages/Settings.vue";

const routes = [
  { path: "/", redirect: "/dashboard" },
  { path: "/dashboard", name: "Dashboard", component: Dashboard },
  { path: "/proxies", name: "Proxies", component: Proxies },
  { path: "/subscriptions", name: "Subscriptions", component: Subscriptions },
  { path: "/connections", name: "Connections", component: Connections },
  { path: "/rules", name: "Rules", component: Rules },
  { path: "/logs", name: "Logs", component: Logs },
  { path: "/test", name: "Test", component: Test },
  { path: "/settings", name: "Settings", component: Settings },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
