<script setup lang="ts">
import { ref } from 'vue';

import HeaderMenuView from './components/HeaderMenuView.vue';
import BannerView from './components/BannerView.vue';
import LeftMenuView from './components/LeftMenuView.vue';
import Main from './components/MainView.vue';
import FooterView from './components/FooterView.vue';
import RightView from './components/RightView.vue';

const leftMenuView = ref<InstanceType<typeof import('./components/LeftMenuView.vue').default>>();

function handleDataReload() {
  leftMenuView?.value?.handleDataReload();
}

</script>

<template>
  <div class="dark h-screen flex flex-col bg-neutral-950 text-neutral-200">
    <UHeader
      class="w-full shrink-0 border-b border-neutral-800 bg-neutral-900/90 backdrop-blur supports-[backdrop-filter]:bg-neutral-900/75"
      :ui="{ container: 'max-w-none mx-0 px-2 sm:px-3 lg:px-4' }">
      <template #left>
        <BannerView />
      </template>
      <template #right>
        <HeaderMenuView @dataReload="handleDataReload" />
      </template>
    </UHeader>

    <div class="flex flex-1 min-h-0 overflow-hidden bg-neutral-950">
      <LeftMenuView ref="leftMenuView" />
      <Main />
      <RightView /> 
    </div>

    <FooterView />
  </div>
</template>
