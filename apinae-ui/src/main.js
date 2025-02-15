import { createApp } from "vue";
import App from "./App.vue";
import MainContent from "./MainContent.vue";
import TestContent from "./TestContent.vue";

import "bootstrap/dist/css/bootstrap.min.css"
import "bootstrap"

import { library, dom } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { fas } from '@fortawesome/free-solid-svg-icons'
import { far } from '@fortawesome/free-regular-svg-icons';

library.add(fas, far)
dom.watch();

const app = createApp(App);

app.component('testcontent',TestContent).component('maincontent',MainContent).component("font-awesome-icon", FontAwesomeIcon).mount("#app");
