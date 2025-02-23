import { createApp } from "vue";
import App from "./App.vue";

import "bootstrap/dist/css/bootstrap.min.css"
import * as bootstrap from 'bootstrap/dist/js/bootstrap.bundle';

import { library, dom } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { fas } from '@fortawesome/free-solid-svg-icons'
import { far } from '@fortawesome/free-regular-svg-icons';

import Tests from "./components/Tests.vue";
import Test from "./components/Test.vue";

library.add(fas, far)
dom.watch();

const app = createApp(App);

import { createRouter, createWebHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        name: 'Tests',
        component: Tests
    },
    {
        path: '/:test_id',
        name: 'Test',
        component: Test
    }
]

const router = createRouter({ history: createWebHistory(), routes })

app
.component("font-awesome-icon", FontAwesomeIcon)
.use(router)
.provide('bootstrap', bootstrap)
.mount("#app");

