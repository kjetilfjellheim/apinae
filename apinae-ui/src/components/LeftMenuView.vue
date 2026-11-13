<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { TreeItem } from '@nuxt/ui';

const items = ref([
]);

function handleDataReload() {
    invoke("get_setup_tree", {})
        .then((message: any) => {
            items.value = parseTreeItems(message);
        })
        .catch((error) => window.alert(error));
}

function parseTreeItems(collections: any[]): any[] {
    const treeItems: any[] = [];

    collections.forEach((collection) => {
        treeItems.push({
            label: collection.name,
            description: collection.description,
            icon: 'fas fa-folder fa-fw text-neutral-400',
            modelValue: collection.id,
            children: collection.setups.map((setup: any) => ({
                label: setup.name,
                description: setup.description,
                icon: 'fas fa-cogs fa-fw text-neutral-400',
                modelValue: setup.id,
            })),
        });
    });
    return treeItems;
}

defineExpose({ handleDataReload })
</script>
<template>
      <aside class="w-72 shrink-0 border-r border-neutral-800 bg-neutral-900 p-4 overflow-y-auto">
        <UTree :items="items">
          <template #item="{ item }">
            <i :class="item.icon" class="text-neutral-400"></i>
            <span class="text-neutral-400">{{ item.label }}</span>
          </template>
        </UTree>
      </aside>
</template>