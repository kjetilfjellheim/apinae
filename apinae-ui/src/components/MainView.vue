<script setup lang="ts">
import { ref } from 'vue';
import Setup from './Setup.vue';

const setupComponent = ref<InstanceType<typeof Setup> | null>(null);
const show = ref<string | null>(null);

function handleEditSetup(payload: { collectionid: string; setupid: string }) {
    show.value = "editSetup";
    setupComponent?.value?.refresh(payload.collectionid, payload.setupid);
}

defineExpose({ handleEditSetup });
</script>

<template>
    <div v-if="show === 'editSetup'" class="h-full w-full min-h-0 overflow-y-auto p-4">
        <Setup ref="setupComponent" />
    </div>
</template>