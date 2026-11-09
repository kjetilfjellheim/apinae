<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
import appIcon from '../app-icon.png'

const items: NavigationMenuItem[][] = [
  [
    {
      label: 'General',
      icon: 'i-lucide-user',
      active: true
    },
    {
      label: 'Members',
      icon: 'i-lucide-users'
    },
    {
      label: 'Notifications',
      icon: 'i-lucide-bell'
    }
  ],
  [
    {
      label: 'Documentation',
      icon: 'i-lucide-book-open',
      target: '_blank'
    },
    {
      label: 'Help & Feedback',
      icon: 'i-lucide-help-circle',
      target: '_blank'
    }
  ]
]
</script>

<template>
  <div class="dark h-screen flex flex-col bg-neutral-950 text-neutral-200">
    <UHeader
      class="w-full shrink-0 border-b border-neutral-800 bg-neutral-900/90 backdrop-blur supports-[backdrop-filter]:bg-neutral-900/75"
      :ui="{ container: 'max-w-none mx-0 px-2 sm:px-3 lg:px-4' }">
      <template #left>
        <div class="flex items-center gap-3">
          <img :src="appIcon" alt="Apinae icon" class="h-8 w-8 rounded-md ring-0 ring-neutral-700" />
          <div class="leading-tight">
            <div class="text-base font-light tracking-wide text-neutral-200 text-lg">Apinae</div>
          </div>
        </div>
      </template>
    </UHeader>

    <div class="flex flex-1 min-h-0 overflow-hidden bg-neutral-950">
      <aside class="w-72 shrink-0 border-r border-neutral-800 bg-neutral-900 p-4 overflow-y-auto">
        <h3 class="text-sm font-medium text-neutral-300 mb-3">Left Panel</h3>
        <UNavigationMenu :items="items" highlight orientation="vertical" />
      </aside>

      <main class="flex-1 p-6 overflow-y-auto bg-neutral-950">
        <h1 class="text-2xl font-bold mb-4 text-neutral-50">Main Content</h1>
        <p class="text-neutral-300">Put your central application view here.</p>
      </main>

      <aside class="w-72 shrink-0 border-l border-neutral-800 bg-neutral-900 p-4 overflow-y-auto">
        <h3 class="text-sm font-medium text-neutral-300 mb-3">Processes</h3>
        <p class="text-neutral-300"></p>
      </aside>
    </div>

    <UFooter
      class="w-full h-14 shrink-0 border-t border-neutral-800 bg-neutral-900/80 backdrop-blur"
      :ui="{ container: 'h-full max-w-none mx-0 px-2 sm:px-3 lg:px-4', left: 'h-full flex items-center' }">
      <template #left>
        <span class="text-base font-light tracking-wide text-neutral-400 text-lg leading-none">Apinae Footer</span>
      </template>
    </UFooter>
  </div>
</template>
