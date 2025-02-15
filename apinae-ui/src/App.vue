<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const current_file_path = ref("");
const data = ref("");

async function load() {
  await invoke("load", {})
    .then((message) => {
      current_file_path.value = message[0];
      data.value = message[1];
    })
    .catch((error) => console.error(error));
}

async function save() {
  await invoke("save", {})
    .then((message) => console.log(message))
    .catch((error) => console.error(error));
}

async function save_as() {
  await invoke("save_as", {})
    .then((message) => console.log(message))
    .catch((error) => console.error(error));
}

async function clean() {
  let data = await invoke("clean", {})
    .then((message) => console.log(message))
    .catch((error) => console.error(error));
} 
</script>

<style>

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
  position: absolute;
  bottom: 0;
  width: 100%;
  height: 30px;
  line-height: 30px;
  text-align: left;
  border-top: 1px solid #4b4b4b;
}

.margin-0 {
  margin: 0;
}

.padding-0 {
  padding: 0;
}
</style>

<template>
  <main class="margin-0 padding-0">
    <nav class="navbar navbar-expand-lg navbar-light bg-body-tertiary">
      <div class="container-fluid">
        <ul class="navbar-nav">
          <li class="nav-item">
            <small class="nav-link active" aria-current="page">Apinae</small>
          </li>
        </ul>
        <form class="d-flex">
          <ul class="navbar-nav">
            <li class="nav-item dropdown">
              <a class="nav-link dropdown-toggle" role="button" data-bs-toggle="dropdown"
                aria-expanded="false">
                Actions
              </a>
              <ul class="dropdown-menu">
                <li><a class="dropdown-item" v-on:click="load()"><i class="fas fa-upload"></i>&nbsp;Open</a></li>
                <li><a class="dropdown-item" v-on:click="save()"><i class="fas fa-download"></i>&nbsp;Save</a></li>
                <li><a class="dropdown-item" v-on:click="save_as()"><i class="fas fa-download"></i>&nbsp;Save as</a></li>
                <li><hr class="dropdown-divider"></li>
                <li><a class="dropdown-item" v-on:click="clean()"><i class="fa-regular fa-square-caret-down"></i>&nbsp;New</a></li>
                <li><hr class="dropdown-divider"></li>
                <li><a class="dropdown-item" v-on:click=""><i class="fa-solid fa-power-off"></i>&nbsp;Exit</a></li>
              </ul>
            </li>
          </ul>
        </form>
      </div>
    </nav>
    <maincontent :tests="data.tests"/>
    <footer class="footer navbar-light bg-body-tertiary">
      <div class="container">File: {{ current_file_path }}</div>
    </footer>      
  </main>
</template>
