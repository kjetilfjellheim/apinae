import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import * as bootstrap from "bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";

import { library, dom } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { fas } from "@fortawesome/free-solid-svg-icons";
import { far } from "@fortawesome/free-regular-svg-icons";

import App from "./App.vue";
import Setups from "./components/Setups.vue";
import Setup from "./components/Setup.vue";

library.add(fas, far);
dom.watch();

const app = createApp(App);

const routes: Array<{ path: string; name: string; component: any }> = [
  {
    path: "/",
    name: "Setups",
    component: Setups,
  },
  {
    path: "/:setupid",
    name: "Setup",
    component: Setup,
  },
];

const router = createRouter({ history: createWebHistory(), routes });

app
  .component("font-awesome-icon", FontAwesomeIcon)
  .use(router)
  .provide("bootstrap", bootstrap)
  .mount("#app");

