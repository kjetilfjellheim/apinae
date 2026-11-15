<script setup lang="ts">
import { ref } from 'vue';

import HeaderMenuView from './components/HeaderMenuView.vue';
import BannerView from './components/BannerView.vue';
import LeftMenuView from './components/LeftMenuView.vue';
import Main from './components/MainView.vue';
import RightView from './components/RightView.vue';
import SettingsView from './components/Settings.vue';
import { SplitterItem } from '@nuxt/ui';

const leftMenuView = ref<InstanceType<typeof import('./components/LeftMenuView.vue').default>>();
const rightView = ref<InstanceType<typeof import('./components/RightView.vue').default>>();


function handleDataReload() {
  leftMenuView?.value?.handleDataReload();
}

function handleProcessReload() {
  rightView?.value?.handleProcessReload();
}

const card = 'bg-elevated/50 border border-default rounded-xl items-center justify-center text-muted font-medium';

const items: SplitterItem[] = [
  { slot: 'left', minSize: 15, defaultSize: 25, class: card },
  { slot: 'main', minSize: 30, defaultSize: 50, class: card },
  { slot: 'right', minSize: 15, defaultSize: 15, class: card }
];

</script>

<template>
  <UApp class="flex flex-col">
    <UHeader class="[&>div]:max-w-none [&>div]:mx-0 [&>div]:!px-4">
      <template #title>
        <BannerView />
      </template>
      <template #right>
        <HeaderMenuView @dataReload="handleDataReload" />
      </template>
    </UHeader>
    <UMain class="flex-1 [&>div]:max-w-none [&>div]:mx-0 [&>div]:!px-4 min-h-[calc(98vh-var(--ui-header-height))] overflow-hidden">
      <USplitter id="splitter-example" :items="items" class="min-h-[calc(98vh-var(--ui-header-height))] max-h-[calc(98vh-var(--ui-header-height))] w-full overflow-hidden">
        <template #left>
          <LeftMenuView ref="leftMenuView" @processReload="handleProcessReload"/>
        </template>
        <template #main>
          <Main />
        </template>
        <template #right>
          <RightView ref="rightView" />
        </template>
      </USplitter>
    </UMain>
    <SettingsView />
  </UApp>
</template>
