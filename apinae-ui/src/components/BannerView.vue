<script setup lang="ts">
    import appIcon from '../../app-icon.png';
</script>
<template #left>
    <div class="flex items-center gap-3">
        <img :src="appIcon" alt="Apinae icon" class="h-8 w-8 rounded-md ring-0 ring-neutral-700" />
        <div class="leading-tight">
        <div class="text-base font-light tracking-wide text-neutral-200 text-lg">Apinae</div>
        </div>
    </div>
</template>