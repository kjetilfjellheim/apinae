<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { computed, onMounted, ref } from 'vue';
import type { AccordionItem } from '@nuxt/ui';

type ProcessItem = {
    processId: number;
    collectionName: string;
    setupName: string;
    params: Record<string, string>;
};

const items = ref<ProcessItem[]>([]);

type ProcessAccordionItem = AccordionItem & {
    process: ProcessItem;
    value: string;
};

function handleProcessReload() {
    invoke("get_processes", {})
        .then((message: ProcessItem[]) => {
            items.value = parseProcessItems(message);
        })
        .catch((error) => window.alert(error));
}

function parseProcessItems(processes: ProcessItem[]): ProcessItem[] {
    return processes;
}

const accordionItems = computed<ProcessAccordionItem[]>(() => {
    return items.value.map((process) => ({
        value: String(process.processId),
        label: process.collectionName,
        icon: 'i-lucide-cpu',
        process,
    }));
});

function stopProcess(processId: number) {
    invoke("stop_process", { processid: processId })
        .then(() => {
            handleProcessReload();
        })
        .catch((error) => window.alert(error));
}

const expandedValues = computed<string[]>(() => accordionItems.value.map((item) => item.value));

onMounted(() => {
    handleProcessReload();
});

defineExpose({ handleProcessReload })
</script>
<template>
    <aside class="w-72 shrink-0 border-l border-neutral-800 bg-neutral-900 p-4 overflow-y-auto h-full w-full">
        <div class="mb-3 flex items-center justify-between">
            <h3 class="text-sm font-medium text-neutral-300">Processes</h3>
            <UButton
                size="sm"
                color="primary"
                variant="soft"
                @click="handleProcessReload">
                Refresh
            </UButton>
        </div>

        <div v-if="items.length === 0" class="text-sm text-neutral-500">
            No running processes.
        </div>

        <UAccordion
            v-else
            type="multiple"
            :default-value="expandedValues"
            :items="accordionItems"
            class="w-full">
            <template #default="{ item }">
                <div class="flex flex-col leading-tight">
                    <span class="text-sm text-neutral-200">{{ item.process.collectionName }}</span>
                    <span class="text-xs text-neutral-400">{{ item.process.setupName }}</span>
                </div>
            </template>
            <template #body="{ item }">
                <div class="space-y-2 px-1 pb-2 text-xs text-neutral-300">
                    <div class="flex items-center justify-between gap-2">
                        <div class="text-neutral-400">PID: {{ item.process.processId }}</div>
                        <UButton
                            size="xs"
                            color="error"
                            variant="soft"
                            @click="stopProcess(item.process.processId)">
                            <i class="fa-solid fa-stop fa-fw ml-auto"></i>
                        </UButton>
                    </div>
                    <div v-if="Object.keys(item.process.params || {}).length === 0" class="text-neutral-500">
                        No params
                    </div>
                    <div v-else class="space-y-1">
                        <div
                            v-for="([key, value]) in Object.entries(item.process.params)"
                            :key="`${item.value}-${key}`"
                            class="rounded border border-neutral-800 bg-neutral-950 px-2 py-1">
                            <span class="text-neutral-400">{{ key }}</span>: {{ value }}
                        </div>
                    </div>
                </div>
            </template>
        </UAccordion>
    </aside>
</template>