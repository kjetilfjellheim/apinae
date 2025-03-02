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

//Add the required icons to the library. This allows you to 
//use them in your components without having to import them
//in each component.
library.add(fas, far)
dom.watch();

// Create the app instance
const app = createApp(App);

import { createRouter, createWebHistory } from 'vue-router'

//Define the routes for the application.
const routes = [
    {
        path: '/',
        name: 'Tests',
        component: Tests
    },
    {
        path: '/:testid',
        name: 'Test',
        component: Test
    }
]

//Create the router instance and pass the routes to it.
const router = createRouter({ history: createWebHistory(), routes })

//Mount the app to the #app element in the DOM.
//This is defined in the index.html file.
app
.component("font-awesome-icon", FontAwesomeIcon)
.use(router)
.provide('bootstrap', bootstrap)
.mount("#app");

