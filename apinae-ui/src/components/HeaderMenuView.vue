<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits(['dataReload']);

async function load() {
  await invoke("load", {})
    .then(() => {
      emit('dataReload');
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
  await invoke("clean", {})
    .then(() => {
      emit('dataReload');
    })
    .catch((error) => window.alert(error));
}

function show_settings() {
  const modalElement = document.getElementById("idSettingsModal");
  if (!modalElement) return;
}

const navigationItems = ref([
  {
    label: 'New',
    icon: 'fas fa-file fa-fw',
    func: () => {
      clean();
    }
  },
  {
    label: 'Open',
    icon: 'fas fa-home fa-fw',
    func: () => {
      load();
    }
  },
  {
    label: 'Save',
    icon: 'fas fa-file fa-fw',
    func: () => {
      save();
    }
  },
  {
    label: 'Save as',
    icon: 'fas fa-file fa-fw',
    func: () => {
      save_as();
    }
  },
  {
    label: 'Settings',
    icon: 'fas fa-cog fa-fw',
    func: () => {
      show_settings();
    }
  }
]);
</script>
<template @>
    <UNavigationMenu :items="navigationItems">
        <template #item="{ item }">
        <UButton
            @click="item.func"
            size="xs"
            color="primary"
            variant="soft"
            class="ml-auto">
            <i :class="item.icon"></i>
            <span>{{ item.label }}</span>
        </UButton>
        </template>
    </UNavigationMenu>
</template>