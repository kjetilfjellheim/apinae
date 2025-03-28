<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

import settings from "./components/Settings.vue";

const current_file_path = ref("");

const render_route_view = ref(true);

async function load() {
  render_route_view.value = false;
  await invoke("load", {})
    .then((file_path) => {
      current_file_path.value = file_path;
      render_route_view.value = true;
    })
    .catch((error) => window.alert(error));
}

async function save() {
  await invoke("save", {})
    .then((message) => console.log(message))
    .catch((error) => window.alert(error));
}

async function save_as() {
  await invoke("save_as", {})
    .then((message) => console.log(message))
    .catch((error) => window.alert(error));
}

async function clean() {
  render_route_view.value = false;
  let data = await invoke("clean", {})
    .then((message) => {render_route_view.value = true;})
    .catch((error) => window.alert(error));
} 

function show_settings() {
  const modal = new bootstrap.Modal(document.getElementById("idSettingsModal"));
  modal.show();
}

</script>
<style>

main {
  background-color: rgb(10, 0, 100);
  height: 100vh;
}

.container-fluid {
  padding: 0px 0px 0px 0px;
}

li.dropdown:last-child .dropdown-menu {
  right: 0;
  left: auto;
}

.navbar {
  border-bottom: 1px solid #4b4b4b;
  height: 30px;
}

.footer {
  position: absolute !important;
  bottom: 0 !important;
  width: 100%;
  height: 32px;
  text-align: left;
  border-top: 1px solid #4b4b4b;
}

</style>
<template>
  <main class="m-0 p-0">
    <nav class="navbar navbar-expand-lg navbar-light bg-primary">
      <div class="container-fluid">
        <ul class="navbar-nav">
          <li class="nav-item">
            <small class="nav-link active" aria-current="page">Apinae</small>
          </li>
        </ul>
        <form class="d-flex">
          <ul class="navbar-nav">
            <li class="nav-item dropdown">
              <a class="nav-link dropdown-toggle" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                Actions
              </a>
              <ul class="dropdown-menu">
                <li><a class="dropdown-item" v-on:click="load()"><i class="fas fa-upload"></i>&nbsp;Open</a></li>
                <li><a class="dropdown-item" v-on:click="save()"><i class="fas fa-download"></i>&nbsp;Save</a></li>
                <li><a class="dropdown-item" v-on:click="save_as()"><i class="fas fa-download"></i>&nbsp;Save as</a>
                </li>
                <li>
                  <hr class="dropdown-divider">
                </li>
                <li><a class="dropdown-item" v-on:click="clean()"><i
                      class="fa-regular fa-square-caret-down"></i>&nbsp;New</a></li>
                <li>
                  <hr class="dropdown-divider">
                </li>
                <li><a class="dropdown-item" data-bs-toggle="modal" data-bs-target="#idSettingsModal"><i class="fas fa-gear"></i>&nbsp;Settings</a></li>
              </ul>
            </li>
          </ul>
        </form>
      </div>
    </nav>
    <router-view v-if="render_route_view" />
    <footer class="footer bg-info-subtle">
      <div class="container-fluid small">
        <div class="row">
          <div class="col-12">
            <span class="text-muted" style="margin: auto;">File: {{ current_file_path }}</span>
          </div>
        </div>
      </div>
    </footer>
  </main>
  <settings></settings>
</template>