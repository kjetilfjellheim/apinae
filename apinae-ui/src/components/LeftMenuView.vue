<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import type { NavigationMenuItem } from '@nuxt/ui';
import type { DropdownMenuItem } from '@nuxt/ui'

const emit = defineEmits(['processReload', 'setupSelect', 'editSetup', 'editCollection', 'editFile']);

const items = ref<NavigationMenuItem[]>([]);

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
                    startProcess(collectionId, setupId);
                }
            },
            {
                label: 'Edit',
                icon: 'fa-pen-square',
                onSelect() {
                    editSetup(collectionId, setupId);
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

function editSetup(collectionId: string, setupId: string) {
    emit('editSetup', { collectionid: collectionId, setupid: setupId });
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
    if (item == null) {
        return;
    }

    const node = item as Partial<SetupNode>;
    if (node.nodeType !== 'setup' || !node.collection || !node.setup) {
        return;
    }

    emit('setupSelect', { collectionid: node.collection.id, setupid: node.setup.id });
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
                class="w-full [&>div]:!px-4" @onSelect="handleSetupSelect">
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
</template>