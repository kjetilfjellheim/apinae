<script setup lang="ts">
import { ref} from "vue";
import { invoke } from "@tauri-apps/api/core";

const collectionIdRef = ref<string>('');
const setupIdRef = ref<string>('');

// The setup being displayed
const setup = ref<any>({});

const activeTab = ref<'general' | 'tcp' | 'http' | 'predefined'>('general');
const isEditSetupOpen = ref(false);
const isEditTcpListenerOpen = ref(false);
const isEditHttpServerOpen = ref(false);
const isEditPredefinedSetOpen = ref(false);
const isEditEndpointOpen = ref(false);

const editSetupData = ref<Record<string, any>>({});
const editAddParameter = ref<string>('');
const editTcpListenerData = ref<any>({});
const editHttpServerData = ref<any>({});
const editHttpsConfig = ref<any>({});
const editSelectedPredefinedSet = ref<any>({});
const editSupportedTlsVersions = ref<string[]>([]);
const editEndpointData = ref<any>({});
const editMockData = ref<any>({});
const editRouteData = ref<any>({});

const showEditHttpsConfig = ref(false);
const showEditMockData = ref(false);

const refresh = (collectionId: string, setupId: string) => {
  collectionIdRef.value = collectionId;
  setupIdRef.value = setupId;
  invoke("get_setup", { collectionid: collectionId, setupid: setupId })
    .then((message: any) => {
      setup.value = message;
    })
    .catch((error) => window.alert(error));
};

const editSetup = () => {
  editSetupData.value = { ...setup.value };
  isEditSetupOpen.value = true;
}

function addParameter() {
  if (!editAddParameter.value ) {
    return;
  }
  editSetupData.value.params = editSetupData.value.params || [];
  editSetupData.value.params.push(editAddParameter.value);
  editAddParameter.value = '';
}

function saveEditSetup() {
  if (!editSetupData.value.name) {
    window.alert('Name is required');
    return;
  }
  if (!editSetupData.value.params) {
    editSetupData.value.params = [];
  }  
  invoke("update_setup", { collectionid: collectionIdRef.value, setupid: setupIdRef.value, name: editSetupData.value.name, description: editSetupData.value.description, parameters: editSetupData.value.params })
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
      isEditSetupOpen.value = false;
    })
    .catch((error) => window.alert(error));
}

defineExpose({ refresh });
</script>

<template class="h-full w-full min-h-0 overflow-y-auto p-4">
  <div class="h-full w-full min-h-0 overflow-y-auto p-4">
    <div class="mx-auto max-w-7xl space-y-4">
      <UCard>
        <template #header>
          <div class="flex flex-wrap items-center justify-between gap-3">
            <div class="flex flex-wrap gap-2">
              <UButton size="sm" color="primary" variant="soft" @click="editSetup">
                <i class="fa-solid fa-pen-to-square" />
                Edit General
              </UButton>
              <UButton size="sm" color="primary" variant="soft" @click="">
                <i class="fa-solid fa-plus" />
                Add http server
              </UButton>
              <UButton size="sm" color="primary" variant="soft" @click="">
                <i class="fa-solid fa-plus" />
                Add tcp listener
              </UButton>
            </div>
          </div>
        </template>

        <div class="flex flex-wrap gap-2">
          <UButton :variant="activeTab === 'general' ? 'solid' : 'soft'" color="primary" size="sm" @click="activeTab = 'general'">General</UButton>
          <UButton :variant="activeTab === 'tcp' ? 'solid' : 'soft'" color="primary" size="sm" @click="activeTab = 'tcp'">Tcp listener</UButton>
          <UButton :variant="activeTab === 'http' ? 'solid' : 'soft'" color="primary" size="sm" @click="activeTab = 'http'">Http server</UButton>
          <UButton v-if="setup?.params" :variant="activeTab === 'predefined' ? 'solid' : 'soft'" color="primary" size="sm" @click="activeTab = 'predefined'">Predefined sets</UButton>
        </div>
      </UCard>

      <div v-if="activeTab === 'general'" class="space-y-4">
        <UCard>
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div>
              <p class="text-xs text-neutral-500">Id</p>
              <p class="text-sm text-neutral-200">{{ setup?.id }}</p>
            </div>
            <div>
              <p class="text-xs text-neutral-500">Name</p>
              <p class="text-sm text-neutral-200">{{ setup?.name }}</p>
            </div>
            <div class="md:col-span-2">
              <p class="text-xs text-neutral-500">Description</p>
              <p class="text-sm text-neutral-200">{{ setup?.description }}</p>
            </div>
            <div class="md:col-span-2">
              <p class="text-xs text-neutral-500">Parameters</p>
              <div class="mt-2 flex flex-wrap gap-2">
                <UBadge v-for="param in (setup?.params || [])" :key="param" color="primary" variant="soft">{{ param }}</UBadge>
                <span v-if="!setup?.params || setup.params.length === 0" class="text-sm text-neutral-500">No parameters</span>
              </div>
            </div>
          </div>
        </UCard>
      </div>

      <div v-if="activeTab === 'tcp'" class="space-y-3">
        <UCard v-for="tcpListener in setup?.listeners" :key="tcpListener.id">
          <template #header>
            <div class="flex items-center justify-between gap-2">
              <div class="text-sm text-neutral-200">
                <i class="fa-solid fa-ear-listen mr-2" />Tcp listener on port {{ tcpListener.port }}
              </div>
              <div class="flex gap-2">
                <UButton size="xs" color="primary" variant="soft" @click="">Edit</UButton>
                <UButton size="xs" color="error" variant="soft" @click="">Delete</UButton>
              </div>
            </div>
          </template>
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div class="space-y-2 text-sm">
              <p><span class="text-neutral-500">Id:</span> {{ tcpListener.id }}</p>
              <p><span class="text-neutral-500">Accept:</span> {{ tcpListener.accept }}</p>
              <p><span class="text-neutral-500">Close:</span> {{ tcpListener.closeConnection }}</p>
              <p><span class="text-neutral-500">Delay write:</span> {{ tcpListener.delayWriteMs }}</p>
              <p class="break-all"><span class="text-neutral-500">File:</span> {{ tcpListener.file }}</p>
            </div>
            <UFormField label="Data">
              <UTextarea :model-value="tcpListener.data" :rows="8" autoresize readonly />
            </UFormField>
          </div>
        </UCard>
        <UCard v-if="setup?.tcpListeners?.length === 0">
          <p class="text-sm text-neutral-500">No TCP listeners configured.</p>
        </UCard>
      </div>

      <div v-if="activeTab === 'http'" class="space-y-3">
        <UCard v-for="httpServer in setup?.servers" :key="httpServer.id">
          <template #header>
            <div class="flex items-center justify-between gap-1">
              <div class="text-sm text-neutral-200">
                <i class="fa-solid fa-cloud mr-1" />Http server: {{ httpServer.name }}
              </div>
              <div class="flex gap-1">
                <UButton size="xs" color="primary" variant="soft" @click="(httpServer)">Edit</UButton>
                <UButton size="xs" color="error" variant="soft" @click="">Delete</UButton>
              </div>
            </div>
          </template>

          <div class="grid grid-cols-1 gap-4 md:grid-cols-2 text-sm">
            <p><span class="text-neutral-500">Http port:</span> {{ httpServer.httpPort }}</p>
            <p class="md:col-span-2"><span class="text-neutral-500">Description:</span> {{ httpServer.description }}</p>
          </div>

          <div v-if="httpServer?.httpsConfig" class="mt-4 rounded border border-neutral-800 p-3 text-sm">
            <p class="mb-2 text-neutral-300">HTTPS config</p>
            <div class="grid grid-cols-1 gap-2 md:grid-cols-2">
              <p><span class="text-neutral-500">Https port:</span> {{ httpServer.httpsConfig.httpsPort }}</p>
              <p><span class="text-neutral-500">Server cert:</span> {{ httpServer.httpsConfig.serverCertificate }}</p>
              <p><span class="text-neutral-500">Private key:</span> {{ httpServer.httpsConfig.privateKey }}</p>
              <p><span class="text-neutral-500">Client cert:</span> {{ httpServer.httpsConfig.clientCertificate }}</p>
            </div>
          </div>

          <div class="mt-4 space-y-3">
            <div class="flex justify-end">
              <UButton size="sm" color="primary" variant="soft" @click="">Add endpoint</UButton>
            </div>
            <UCard v-for="endpoint in (httpServer.endpoints || [])" :key="endpoint.id" class="border border-neutral-800">
              <template #header>
                <div class="flex items-center justify-between gap-1">
                  <div class="text-xs text-neutral-300">Method: {{ endpoint.method }} | Path: {{ endpoint.pathExpression }}</div>
                  <div class="flex gap-1">
                    <UButton size="xs" color="primary" variant="soft" @click="">Edit</UButton>
                    <UButton size="xs" color="error" variant="soft" @click="">Delete</UButton>
                  </div>
                </div>
              </template>
              <div v-if="endpoint?.mock" class="space-y-1 text-sm md:grid md:grid-cols-4 md:gap-1">
                <p><span class="text-neutral-500">Status:</span> {{ endpoint.mock.status }}</p>
                <p><span class="text-neutral-500">Delay:</span> {{ endpoint.mock.delay }}</p>
                <UFormField label="Headers"><UTextarea :model-value="endpoint.mock.headers" :rows="6" readonly class="w-full"/></UFormField>
                <UFormField label="Response"><UTextarea :model-value="endpoint.mock.response" :rows="6" readonly class="w-full"/></UFormField>
              </div>
              <div v-if="endpoint?.route" class="grid grid-cols-1 gap-2 text-sm md:grid-cols-2">
                <p class="md:col-span-2"><span class="text-neutral-500">Url:</span> {{ endpoint.route.url }}</p>
                <p class="md:col-span-2"><span class="text-neutral-500">Proxy url:</span> {{ endpoint.route.proxyUrl }}</p>
                <p><span class="text-neutral-500">Http1 only:</span> {{ endpoint.route.http1Only }}</p>
                <p><span class="text-neutral-500">Accept invalid certs:</span> {{ endpoint.route.acceptInvalidCerts }}</p>
                <p><span class="text-neutral-500">Accept invalid hostnames:</span> {{ endpoint.route.acceptInvalidHostnames }}</p>
                <p><span class="text-neutral-500">Min TLS:</span> {{ endpoint.route.minTlsVersion }}</p>
                <p><span class="text-neutral-500">Max TLS:</span> {{ endpoint.route.maxTlsVersion }}</p>
                <p><span class="text-neutral-500">Read timeout:</span> {{ endpoint.route.readTimeout }}</p>
                <p><span class="text-neutral-500">Connect timeout:</span> {{ endpoint.route.connectTimeout }}</p>
                <p><span class="text-neutral-500">Delay before:</span> {{ endpoint.route.delayBefore }}</p>
                <p><span class="text-neutral-500">Delay after:</span> {{ endpoint.route.delayAfter }}</p>
              </div>
            </UCard>
          </div>
        </UCard>
        <UCard v-if="setup?.httpServers?.length === 0">
          <p class="text-sm text-neutral-500">No HTTP servers configured.</p>
        </UCard>
      </div>

      <div v-if="activeTab === 'predefined' && setup?.params" class="grid grid-cols-1 gap-4 md:grid-cols-3">
        <UCard class="md:col-span-1">
          <template #header>
            <div class="flex items-center justify-between">
              <p class="text-sm text-neutral-200">Predefined sets</p>
              <UButton size="xs" color="primary" variant="soft" @click="">Add</UButton>
            </div>
          </template>
          <div class="space-y-2">
            <div v-for="predefSet in setup?.predefinedSets" :key="predefSet.name" class="flex items-center justify-between gap-2">
              <UButton size="xs" :variant="predefSet.name === setup?.selectedPredefinedSet?.name ? 'solid' : 'ghost'" color="primary" class="justify-start" @click="">
                {{ predefSet.name }}
              </UButton>
              <UButton size="xs" color="error" variant="ghost" @click="">
                <i class="fa-solid fa-trash" />
              </UButton>
            </div>
          </div>
        </UCard>

        <UCard class="md:col-span-2" v-if="setup?.selectedPredefinedSet">
          <template #header>
            <div class="flex items-center justify-between">
              <p class="text-sm text-neutral-200">{{ setup?.selectedPredefinedSet.name }}</p>
              <UButton size="xs" color="primary" variant="soft" @click="">Edit</UButton>
            </div>
          </template>
          <div class="overflow-x-auto">
            <table class="w-full text-sm">
              <thead>
                <tr class="text-left text-neutral-500">
                  <th class="py-1">Key</th>
                  <th class="py-1">Value</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="predefKey in Object.keys(setup?.selectedPredefinedSet.values || {})" :key="predefKey" class="border-t border-neutral-800">
                  <td class="py-1 text-neutral-300">{{ predefKey }}</td>
                  <td class="py-1 text-neutral-300">{{ setup?.selectedPredefinedSet.values[predefKey] }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </UCard>
      </div>
    </div>

    <UModal v-model:open="isEditSetupOpen" title="Edit setup" :ui="{ content: 'sm:max-w-2xl' }">
      <template #body>
        <div class="space-y-4 w-full">
          <UFormField label="Id" class="w-full"><UInput class="w-full" :model-value="editSetupData.id" disabled /></UFormField>
          <UFormField label="Name" class="w-full"><UInput class="w-full" v-model="editSetupData.name" /></UFormField>
          <UFormField label="Description" class="w-full"><UTextarea class="w-full" v-model="editSetupData.description" :rows="4" /></UFormField>
          <UFormField label="Parameters" class="w-full">
            <div class="flex gap-2">
              <UInput v-model="editAddParameter" placeholder="Parameter" class="flex-1 w-full" />
              <UButton color="primary" variant="soft" @click="addParameter()">Add</UButton>
            </div>
            <div class="mt-2 flex flex-wrap gap-2">
              <UButton v-for="param in (editSetupData.params || [])" :key="param" size="xs" color="primary" variant="soft" @click="">{{ param }}</UButton>
            </div>
          </UFormField>
        </div>
      </template>
      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton color="neutral" variant="ghost" @click="isEditSetupOpen = false">Cancel</UButton>
          <UButton color="primary" variant="solid" @click="saveEditSetup">Save</UButton>
        </div>
      </template>
    </UModal>

    <UModal v-model:open="isEditTcpListenerOpen" title="Edit TCP listener" :ui="{ content: 'sm:max-w-2xl' }">
      <template #body>
        <div class="space-y-4">
          <UFormField label="Id"><UInput :model-value="editTcpListenerData.id" disabled /></UFormField>
          <UFormField label="Port"><UInput v-model="editTcpListenerData.port" /></UFormField>
          <UFormField label="Delayed write ms"><UInput v-model="editTcpListenerData.delayWriteMs" /></UFormField>
          <UFormField label="Close connection"><USelect v-model="editTcpListenerData.closeConnection" :items="['AfterRead','AfterResponse','BeforeRead','Never']" /></UFormField>
          <div class="flex gap-4">
            <UCheckbox v-model="editTcpListenerData.accept" label="Accept" />
          </div>
          <UFormField label="File">
            <div class="flex gap-2">
              <UInput v-model="editTcpListenerData.file" readonly class="flex-1" />
              <UButton size="sm" color="primary" variant="soft" @click="">Select</UButton>
              <UButton size="sm" color="error" variant="soft" @click="">Clear</UButton>
            </div>
          </UFormField>
          <UFormField label="Response"><UTextarea v-model="editTcpListenerData.data" :rows="8" /></UFormField>
        </div>
      </template>
      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton color="neutral" variant="ghost" @click="isEditTcpListenerOpen = false">Cancel</UButton>
          <UButton color="primary" variant="solid" @click="(editTcpListenerData)">Save</UButton>
        </div>
      </template>
    </UModal>

    <UModal v-model:open="isEditHttpServerOpen" title="Edit HTTP server" :ui="{ content: 'sm:max-w-2xl' }">
      <template #body>
        <div class="space-y-4">
          <UFormField label="Id"><UInput :model-value="editHttpServerData.id" disabled /></UFormField>
          <UFormField label="Name"><UInput v-model="editHttpServerData.name" /></UFormField>
          <UFormField label="Http port"><UInput v-model="editHttpServerData.httpPort" /></UFormField>
          <UCheckbox v-model="showEditHttpsConfig" label="Https config" />
          <div v-if="showEditHttpsConfig" class="space-y-4 rounded border border-neutral-800 p-3">
            <UFormField label="Https port"><UInput v-model="editHttpsConfig.httpsPort" /></UFormField>
            <UFormField label="Server certificate">
              <div class="flex gap-2">
                <UInput v-model="editHttpsConfig.serverCertificate" readonly class="flex-1" />
                <UButton size="sm" color="primary" variant="soft" @click="">Select</UButton>
                <UButton size="sm" color="error" variant="soft" @click="">Clear</UButton>
              </div>
            </UFormField>
            <UFormField label="Private key">
              <div class="flex gap-2">
                <UInput v-model="editHttpsConfig.privateKey" readonly class="flex-1" />
                <UButton size="sm" color="primary" variant="soft" @click="">Select</UButton>
                <UButton size="sm" color="error" variant="soft" @click="">Clear</UButton>
              </div>
            </UFormField>
            <UFormField label="Client certificate">
              <div class="flex gap-2">
                <UInput v-model="editHttpsConfig.clientCertificate" readonly class="flex-1" />
                <UButton size="sm" color="primary" variant="soft" @click="">Select</UButton>
                <UButton size="sm" color="error" variant="soft" @click="">Clear</UButton>
              </div>
            </UFormField>
            <UFormField label="Supported TLS versions">
              <UCheckboxGroup v-model="editSupportedTlsVersions" :items="['TLSv1.0','TLSv1.1','TLSv1.2','TLSv1.3']" />
            </UFormField>
          </div>
        </div>
      </template>
      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton color="neutral" variant="ghost" @click="isEditHttpServerOpen = false">Cancel</UButton>
          <UButton color="primary" variant="solid" @click="">Save</UButton>
        </div>
      </template>
    </UModal>

    <UModal v-model:open="isEditEndpointOpen" title="Edit endpoint" :ui="{ content: 'sm:max-w-4xl' }">
      <template #body>
        <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
          <div class="md:col-span-2"><UCheckbox v-model="showEditMockData" label="Mock" /></div>
          <UFormField label="Method"><USelect v-model="editEndpointData.method" :items="['DELETE','GET','HEAD','POST','PUT','OPTION']" /></UFormField>
          <UFormField label="Path expression"><UInput v-model="editEndpointData.pathExpression" /></UFormField>
          <UFormField label="Body expression" class="md:col-span-2"><UInput v-model="editEndpointData.bodyExpression" /></UFormField>

          <template v-if="showEditMockData">
            <UFormField label="Status"><UInput v-model="editMockData.status" /></UFormField>
            <UFormField label="Delay"><UInput v-model="editMockData.delay" /></UFormField>
            <UFormField label="Headers" class="md:col-span-2"><UTextarea v-model="editMockData.headers" :rows="6" /></UFormField>
            <UFormField label="Response" class="md:col-span-2"><UTextarea v-model="editMockData.response" :rows="8" /></UFormField>
          </template>

          <template v-else>
            <UFormField label="Url"><UInput v-model="editRouteData.url" /></UFormField>
            <UFormField label="Proxy url"><UInput v-model="editRouteData.proxyUrl" /></UFormField>
            <UCheckbox v-model="editRouteData.http1Only" label="Http1 only" />
            <UCheckbox v-model="editRouteData.acceptInvalidCerts" label="Accept invalid certs" />
            <UCheckbox v-model="editRouteData.acceptInvalidHostnames" label="Accept invalid hostnames" />
            <div></div>
            <UFormField label="Min TLS"><USelect v-model="editRouteData.minTlsVersion" :items="['TLSv1.2','TLSv1.3']" /></UFormField>
            <UFormField label="Max TLS"><USelect v-model="editRouteData.maxTlsVersion" :items="['TLSv1.2','TLSv1.3']" /></UFormField>
            <UFormField label="Read timeout"><UInput v-model="editRouteData.readTimeout" /></UFormField>
            <UFormField label="Connect timeout"><UInput v-model="editRouteData.connectTimeout" /></UFormField>
            <UFormField label="Delay before request"><UInput v-model="editRouteData.delayBefore" /></UFormField>
            <UFormField label="Delay after request"><UInput v-model="editRouteData.delayAfter" /></UFormField>
          </template>
        </div>
      </template>
      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton color="neutral" variant="ghost" @click="isEditEndpointOpen = false">Cancel</UButton>
          <UButton color="primary" variant="solid" @click="">Save</UButton>
        </div>
      </template>
    </UModal>

    <UModal v-model:open="isEditPredefinedSetOpen" title="Edit predefined set" :ui="{ content: 'sm:max-w-2xl' }">
      <template #body>
        <div class="space-y-4">
          <UFormField label="Name"><UInput v-model="editSelectedPredefinedSet.name" /></UFormField>
          <div class="space-y-2">
            <div v-for="predefKey in Object.keys(editSelectedPredefinedSet.values || {})" :key="predefKey" class="grid grid-cols-3 gap-2">
              <div class="col-span-1 text-sm text-neutral-400">{{ predefKey }}</div>
              <UInput class="col-span-2" v-model="editSelectedPredefinedSet.values[predefKey]" />
            </div>
          </div>
        </div>
      </template>
      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton color="neutral" variant="ghost" @click="isEditPredefinedSetOpen = false">Cancel</UButton>
          <UButton color="primary" variant="solid" @click="">Save</UButton>
        </div>
      </template>
    </UModal>
  </div>
</template>