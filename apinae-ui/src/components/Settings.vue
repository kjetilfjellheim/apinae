<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onBeforeUnmount, onMounted, ref } from 'vue';

type CommandType = 'installed' | 'specific';

type SettingsData = {
    apinaeCommand: string;
    commandType: CommandType;
    bodyHeight: string;
};

const isOpen = ref(false);

const settingsData = ref<SettingsData>({
    apinaeCommand: '',
    commandType: 'installed',
    bodyHeight: '8pc',
});

const commandTypeItems = [
    { label: 'Installed', value: 'installed' },
    { label: 'Specific', value: 'specific' },
];

function convertToSettingsRequestObject(data: SettingsData) {
    return {
        apinaePath: data.commandType === 'specific' ? data.apinaeCommand : null,
        bodyHeight: data.bodyHeight,
    };
}

async function loadSettings() {
    await invoke('load_settings', {})
        .then((settings: any) => {
            settingsData.value.apinaeCommand = settings.apinaePath ?? '';
            settingsData.value.commandType = settings.apinaePath ? 'specific' : 'installed';
            settingsData.value.bodyHeight = settings.bodyHeight ?? '8pc';
        })
        .catch((error) => window.alert(error));
}

async function saveSettings() {
    await invoke('save_settings', { settings: convertToSettingsRequestObject(settingsData.value) })
        .then(() => {
            isOpen.value = false;
        })
        .catch((error) => window.alert(error));
}

function openSettingsModal() {
    isOpen.value = true;
}

onMounted(() => {
    loadSettings();
    window.addEventListener('open-settings-modal', openSettingsModal);
});

onBeforeUnmount(() => {
    window.removeEventListener('open-settings-modal', openSettingsModal);
});
</script>

<template>
    <UModal v-model:open="isOpen" title="Settings" :ui="{ content: 'sm:max-w-2xl' }">
        <template #body>
            <div class="space-y-4">
                <UFormField label="Command Type" description="Which apinae command should be used.">
                    <URadioGroup
                        v-model="settingsData.commandType"
                        :items="commandTypeItems"
                        orientation="horizontal"
                        value-key="value"
                        label-key="label"
                    />
                </UFormField>

                <UFormField label="Path" description="Used only when Command Type is Specific.">
                    <UInput
                        v-model="settingsData.apinaeCommand"
                        placeholder="/usr/local/bin/apinae"
                        :disabled="settingsData.commandType === 'installed'"
                    />
                </UFormField>

                <UFormField label="Multiline Field Height" description="Height used for multiline fields in the UI.">
                    <UInput v-model="settingsData.bodyHeight" placeholder="8pc" />
                </UFormField>
            </div>
        </template>

        <template #footer>
            <div class="flex w-full justify-end gap-2">
                <UButton color="neutral" variant="ghost" @click="isOpen = false">Close</UButton>
                <UButton color="primary" variant="solid" @click="saveSettings">Save changes</UButton>
            </div>
        </template>
    </UModal>
</template>