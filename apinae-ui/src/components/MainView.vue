<script setup lang="ts">

import { ref } from 'vue';
import Setup from './Setup.vue';

const selectedSetup = ref<{ collectionid: string; setupid: string } | null>(null);

const show = ref<string | null>(null);

function handleEditSetup(payload: { collectionid: string; setupid: string }) {
    show.value = "editSetup";
    selectedSetup.value = payload;
}

defineExpose({ handleEditSetup });
</script>

<template>
    <div v-if="selectedSetup" class="h-full w-full min-h-0 overflow-y-auto p-4">
        <Setup :collection-id="selectedSetup.collectionid" :setup-id="selectedSetup.setupid" />
    </div>
</template>