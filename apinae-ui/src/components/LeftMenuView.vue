<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import type { NavigationMenuItem } from '@nuxt/ui';
import type { DropdownMenuItem } from '@nuxt/ui'

const emit = defineEmits(['processReload', 'setupSelect', 'editCollection', 'editFile']);

const items = ref<NavigationMenuItem[]>([]);

const editRunData = ref<{
    params: Map<string, string>;
    predefinedSets: string[];
    collectionId: string;
    setupId: string;
    selectedPredefinedSetName: string;
} | null>(null);

const isRunParamsOpen = ref(false);

type CollectionNode = NavigationMenuItem & {
    nodeType: 'collection';
    collection: any;
    children: SetupNode[];
};

type SetupNode = NavigationMenuItem & {
    nodeType: 'setup';
    collection: any;
    setup: any;
};

function getCollectionMenuItems(collectionId: string, collectionName: string): DropdownMenuItem[][] {
    return [
        [
            {
                label: 'Add setup',
                icon: 'fa-plus',
                onSelect() {
                    addSetup(collectionId);
                }
            },
            {
                label: 'Rename',
                icon: 'fa-pen',
                onSelect() {
                    renameCollection(collectionId, collectionName);
                }
            }
        ],
        [
            {
                label: 'Delete',
                color: 'error',
                icon: 'fa-trash',
                onSelect() {
                    deleteCollection(collectionId);
                }
            }
        ]
    ];
}

function getSetupMenuItems(collectionId: string, setupId: string, setupName: string): DropdownMenuItem[][] {
    return [
        [
            {
                label: 'Run',
                icon: 'fa-play',
                onSelect() {
                    initWithParams(collectionId, setupId);
                }
            },
            {
                label: 'Edit',
                icon: 'fa-pen-square',
                onSelect() {
                    selectSetup(collectionId, setupId);
                }
            },
            {
                label: 'Rename',
                icon: 'fa-pen',
                onSelect() {
                    renameSetup(collectionId, setupId, setupName);
                }
            }
        ],
        [
            {
                label: 'Delete',
                color: 'error',
                icon: 'fa-trash',
                onSelect() {
                    deleteSetup(collectionId, setupId);
                }
            }
        ]
    ];
}


function handleDataReload() {
    invoke("get_setup_tree", {})
        .then((message: any) => {
            items.value = parseTreeItems(message);
            selectSetup(null, null);
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

function renameSetup(collectionId: string, setupId: string, currentName: string) {
    const newName = window.prompt('Setup name', currentName);
    if (newName === null) {
        return;
    }

    invoke("rename_setup", { collectionid: collectionId, setupid: setupId, name: newName })
        .then(() => {
            handleDataReload();
        })
        .catch((error) => window.alert(error));
}

function deleteSetup(collectionId: string, setupId: string) {
    invoke("delete_setup", { collectionid: collectionId, setupid: setupId })
        .then(() => {
            handleDataReload();
        })
        .catch((error) => window.alert(error));
}

function renameCollection(collectionId: string, currentName: string) {
    const newName = window.prompt('Collection name', currentName);
    if (newName === null) {
        return;
    }

    invoke("rename_collection", { collectionid: collectionId, name: newName })
        .then(() => {
            handleDataReload();
        })
        .catch((error) => window.alert(error));
}

function deleteCollection(collectionId: string) {
    invoke("delete_collection", { collectionid: collectionId })
        .then(() => {
            handleDataReload();
        })
        .catch((error) => window.alert(error));
}

function parseTreeItems(collections: any[]): CollectionNode[] {
    const treeItems: CollectionNode[] = [];
    collections.forEach((collection) => {
        treeItems.push({
            nodeType: 'collection',
            label: collection.name,
            icon: 'fa-solid fa-folder fa-fw text-neutral-400',
            tooltip: collection.description,
            children: collection.setups.map((setup: any): SetupNode => ({
                nodeType: 'setup',
                label: setup.name,
                icon: 'fa-solid fa-gears fa-fw text-neutral-400',
                tooltip: setup.description,
                collection,
                setup
            })),
            collection,
        });
    });
    return treeItems;
}

function initWithParams(collectionId: string, setupId: string) {
    invoke("get_setup", { collectionid: collectionId, setupid: setupId })
        .then((setup: any) => {
            editRunData.value = {
                params: new Map(setup?.params?.map((str: string) => [str, '']) || []),
                predefinedSets: Object.keys(setup?.predefinedSets || {}),
                collectionId: collectionId,
                setupId: setupId,
                selectedPredefinedSetName: null
            };                            
            if (editRunData.value.params.size > 0) {
                isRunParamsOpen.value = true;
            } else {
                runWithParams();
            }
        })
        .catch((error) => window.alert(error));
}

function handleProcessReload() {
    emit('processReload');
}

function selectSetup(collectionId: string, setupId: string) {
    emit('setupSelect', { collectionid: collectionId, setupid: setupId });
}

function runWithParams() {
    invoke("start_process", { 
        collectionid: editRunData.value.collectionId, 
        setupid: editRunData.value.setupId, 
        params: editRunData.value.params, 
        predefinedSetName: editRunData.value.selectedPredefinedSetName 
    })
    .then(() => {
        handleProcessReload();
        isRunParamsOpen.value = false;
    })
    .catch((error) => window.alert(error));
}

defineExpose({ handleDataReload });

</script>
<template class="h-full w-full">
    <div class="h-full w-full max-h-screen min-h-0 overflow-hidden flex flex-col">
        <div class="px-4 py-2 border-b border-neutral-800">
            <UButton size="sm" color="primary" variant="soft" @click="addCollection">
                <i class="fa-solid fa-plus fa-fw"></i>Add Collection
            </UButton>
        </div>
        <div class="flex-1 min-h-0 overflow-y-auto">
            <UNavigationMenu :items="items" orientation="vertical" highlight highlightColor="primary"
                class="w-full [&>div]:!px-4">
                <template #item="{ item }">
                    <i :class="item.icon"></i>
                    <span class="text-neutral-300">{{ item.label }}</span>
                    <template v-if="item.nodeType === 'collection'">
                        <div class="ml-auto flex gap-1" @click.stop>
                            <UDropdownMenu :items="getCollectionMenuItems(item.collection.id, item.collection.name)" :content="{ align: 'start' }"
                                :ui="{ content: 'w-48' }">
                                <UButton color="primary" variant="soft" size="xs" class="ml-auto">
                                    <i class="fa-solid fa-list-ul fa-fw"></i>
                                </UButton>
                                <template #item="{ item }">
                                    <i :class="`fa-solid ${item.icon} fa-fw`"></i> {{ item.label }}
                                </template>
                            </UDropdownMenu>
                        </div>
                    </template>
                    <template v-if="item.nodeType === 'setup'">
                        <div class="ml-auto flex gap-1" @click.stop>
                            <UDropdownMenu :items="getSetupMenuItems(item.collection.id, item.setup.id, item.setup.name)" :content="{ align: 'start' }"
                                :ui="{ content: 'w-48' }">
                                <UButton color="primary" variant="soft" size="xs" class="ml-auto">
                                    <i class="fa-solid fa-list-ul fa-fw"></i>
                                </UButton>
                                <template #item="{ item }">
                                    <i :class="`fa-solid ${item.icon} fa-fw`"></i> {{ item.label }}
                                </template>
                            </UDropdownMenu>
                        </div>
                    </template>
                </template>
            </UNavigationMenu>
        </div>
    </div>
    <UModal v-model:open="isRunParamsOpen" title="Run parameters">
      <template #body>
        <div class="space-y-4">
          <div class="grid grid-cols-1 gap-1 md:grid-cols-4 text-sm">
            <p class="md:col-span-1"><span class="text-neutral-500">Predefined set:</span></p>
            <USelectMenu clear v-model="editRunData.selectedPredefinedSetName" :items="editRunData?.predefinedSets || []" class="md:col-span-3" />
            <p class="md:col-span-4"><span class="text-neutral-300">Parameters</span></p>
            <template v-for="[key, value] in editRunData?.params" :key="key">
              <span class="text-neutral-500 md:col-span-1">{{ key }}:</span>
              <UInput :value="editRunData?.params.get(key)" @input="editRunData?.params.set(key, $event.target.value)" class="md:col-span-3" />
            </template>
          </div>
        </div>
      </template>
      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton color="neutral" variant="ghost" @click="isRunParamsOpen = false">Cancel</UButton>
          <UButton color="primary" variant="solid" @click="runWithParams()">Run</UButton>
        </div>
      </template>
    </UModal>    
</template>