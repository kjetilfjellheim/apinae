<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import type { NavigationMenuItem } from '@nuxt/ui';

const emit = defineEmits(['processReload','setupSelect', 'editSetup', 'editCollection', 'editFile']);

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

function editSetup(collectionId: string, setupId: string) {
    emit('editSetup', { collectionid: collectionId, setupid: setupId });
}

function editCollection(collectionId: string) {
    emit('editCollection', { collectionid: collectionId });
}

function editFile() {
    emit('editFile');
}

function parseTreeItems(collections: any[]): any[] {
    const treeItems: any[] = [];
    collections.forEach((collection) => {
        treeItems.push({
            label: collection.name,
            description: collection.description,
            icon: 'fa-solid fa-folder fa-fw text-neutral-400',
            value: null,
            tooltip: collection.description,
            children: collection.setups.map((setup: any) => ({
                label: setup.name,
                description: setup.description,
                icon: 'fa-solid fa-gears fa-fw text-neutral-400',
                value: { collection: collection, setup: setup },
                tooltip: setup.description
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

function handleSetupSelect(item: NavigationMenuItem | null) {
    if (item == null || item.value == null) {
        return;
    }
    console.log('Selected setup:', item.value);
    emit('setupSelect', { collectionid: item.value["collection"].id, setupid: item.value["setup"].id });
}

defineExpose({ handleDataReload });

</script>
<template class="h-full w-full">
    <div class="h-full w-full max-h-screen min-h-0 overflow-hidden flex flex-col">
        <div class="px-4 py-2 border-b border-neutral-800">
            <UButton
                size="sm"
                color="primary"
                variant="soft"
                @click="addCollection">
                <i class="fa-solid fa-plus fa-fw"></i>Add Collection
            </UButton>
            <UButton
                size="sm"
                color="primary"
                variant="soft"
                @click="editFile">
                <i class="fa-solid fa-edit fa-fw"></i>Edit description
            </UButton>            
        </div>
        <div class="flex-1 min-h-0 overflow-y-auto">
            <UNavigationMenu :items="items" orientation="vertical" highlight highlightColor="primary" class="w-full [&>div]:!px-4" @onSelect="handleSetupSelect">
                <template #item="{ item }">
                    <i :class="item.icon"></i>
                    <span class="text-neutral-300">{{ item.label }}</span>
                    <template v-if="item.collection">
                        <div class="ml-auto flex gap-1" @click.stop>                        
                            <UButton
                                size="xs"
                                color="primary"
                                variant="soft"
                                class="ml-auto"
                                @click.stop.prevent="() => addSetup(item.collection.id)">
                                <i class="fa-solid fa-plus fa-fw"></i>
                            </UButton>
                            <UButton
                                size="xs"
                                color="primary"
                                variant="soft"
                                class="ml-auto"
                                @click.stop.prevent="() => editCollection(item.collection.id)">
                                <i class="fa-solid fa-edit fa-fw"></i>
                            </UButton>                            
                        </div>
                    </template>
                    <template v-if="item.value">
                        <div class="ml-auto flex gap-1" @click.stop>        
                            <UButton 
                                size="xs"
                                color="primary"
                                variant="soft"
                                class="ml-auto"
                                @click.stop.prevent="() => startProcess(item.value.collection.id, item.value.setup.id)">
                                <i class="fa-solid fa-play fa-fw ml-auto"></i>
                            </UButton>                                                                   
                            <UButton 
                                size="xs"
                                color="primary"
                                variant="soft"
                                class="ml-auto"
                                @click.stop.prevent="() => editSetup(item.value.collection.id, item.value.setup.id)">
                                <i class="fa-solid fa-edit fa-fw ml-auto"></i>
                            </UButton>                                          
                        </div>
                    </template>
                </template>
            </UNavigationMenu>
        </div>
    </div>
</template>