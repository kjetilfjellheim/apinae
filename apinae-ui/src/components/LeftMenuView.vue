<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import type { NavigationMenuItem } from '@nuxt/ui';

const emit = defineEmits(['processReload']);

const items = ref<NavigationMenuItem[]>([]);

function handleDataReload() {
    invoke("get_setup_tree", {})
        .then((message: any) => {
            items.value = parseTreeItems(message);
        })
        .catch((error) => window.alert(error));
}

function addCollection() {
    const name = window.prompt('Collection name', 'New Collection');
    if (name === null) {
        return;
    }

    invoke("add_collection", { name, description: '' })
        .then(() => {
            handleDataReload();
        })
        .catch((error) => window.alert(error));
}

function addSetup(collectionId: string) {
    const name = window.prompt('Setup name', 'New Setup');
    if (name === null) {
        return;
    }

    invoke("add_setup", { collectionid: collectionId, name, description: '' })
        .then(() => {
            handleDataReload();
        })
        .catch((error) => window.alert(error));
}

function parseTreeItems(collections: any[]): any[] {
    const treeItems: any[] = [];
    collections.forEach((collection) => {
        treeItems.push({
            label: collection.name,
            description: collection.description,
            icon: 'fa-solid fa-folder fa-fw text-neutral-400',
            value: null,
            children: collection.setups.map((setup: any) => ({
                label: setup.name,
                description: setup.description,
                icon: 'fa-solid fa-gears fa-fw text-neutral-400',
                value: { collection: collection, setup: setup },
            })),
            collection: collection,
        });
    });
    return treeItems;
}

function startProcess(collectionId: string, setupId: string) {
    invoke("start_process", { collectionid: collectionId, setupid: setupId, params: {} })
        .then(() => {
            handleProcessReload();
        })
        .catch((error) => window.alert(error));
}

function handleProcessReload() {
    emit('processReload');
}

defineExpose({ handleDataReload })
</script>
<template class="h-full w-full">
    <div class="h-full w-full flex flex-col">
        <div class="px-4 py-2 border-b border-neutral-800">
            <UButton
                size="xs"
                variant="ghost"
                class="text-neutral-300 hover:text-neutral-100"
                @click="addCollection"
            >
                Add Collection
            </UButton>
        </div>
        <UNavigationMenu :items="items" orientation="vertical" class="w-full h-full overflow-y-auto [&>div]:!px-4">
        <template #item="{ item }">
            <i :class="item.icon"></i>
            <span class="text-neutral-300">{{ item.label }}</span>
            <template v-if="item.collection">
                <UButton
                    size="sm"
                    variant="text"
                    class="ml-auto text-neutral-400 hover:text-neutral-200"
                    @click.stop.prevent="() => addSetup(item.collection.id)">
                    <i class="fa-solid fa-plus fa-fw text-neutral-400"></i>
                </UButton>
            </template>
            <template v-if="item.value">
                <UButton 
                    size="sm"
                    variant="text"
                    class="ml-auto text-neutral-400 hover:text-neutral-200"
                    @click.stop.prevent="() => startProcess(item.value.collection.id, item.value.setup.id)">
                    <i class="fa-solid fa-play fa-fw text-neutral-400 ml-auto" style="color: green"></i>
                </UButton>
            </template>
        </template>
        </UNavigationMenu>
    </div>
</template>