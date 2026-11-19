<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { end } from "@popperjs/core";

const collectionIdRef = ref<string>('');
const setupIdRef = ref<string>('');

// The setup being displayed
const setup = ref<any>(null);

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
const editSelectedPredefinedSet = ref<any>({});
const editSupportedTlsVersions = ref<string[]>([]);
const editEndpointData = ref<any>({});

const editServerIdForNewEndpoint = ref<string>('');
const selectedMock = ref<boolean>(true);

const showEditHttpsConfig = ref(false);

const get_file_name = (element: any) => {
  const filePath = element?.value || '';
  invoke("open_dialog", { filePath })
    .then((message: string) => {
      return message;
    })
    .catch((error) => window.alert(error));
};

const refresh = (collectionId: string, setupId: string) => {
  if (setupId == null) {
    setup.value = null;
    return;
  }
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

const editTcpListener = (listener: any) => {
  editTcpListenerData.value = { ...listener };
  isEditTcpListenerOpen.value = true;
}

const editHttpServer = (server: any) => {
  editHttpServerData.value = { ...server };
  showEditHttpsConfig.value = editHttpServerData.value?.httpsConfig != null;
  isEditHttpServerOpen.value = true;
}

const editEndpoint = (serverId: string, endpoint: any) => {
  editServerIdForNewEndpoint.value = serverId;
  editEndpointData.value = { ...endpoint };
  isEditEndpointOpen.value = true;
}

function addParameter() {
  if (!editAddParameter.value) {
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

function deleteParameter(param: string) {
  if (!editSetupData.value.params) {
    return;
  }
  editSetupData.value.params = editSetupData.value.params.filter((p: string) => p !== param);
}

function addTcpListener() {
    invoke("add_tcp_listener", { collectionid: collectionIdRef.value, setupid: setupIdRef.value})
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
    })
    .catch((error) => window.alert(error));
}

function updateTcpListener() {
  invoke("update_tcp_listener", { collectionid: collectionIdRef.value, setupid: setupIdRef.value, listenerid: editTcpListenerData.value.id, tcplistener: editTcpListenerData.value })
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
      isEditTcpListenerOpen.value = false;
    })
    .catch((error) => window.alert(error));
}

function addHttpServer() {
    invoke("add_http_server", { collectionid: collectionIdRef.value, setupid: setupIdRef.value })
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
    })
    .catch((error) => window.alert(error));
}

function deleteTcpListener(listenerId: string) {
    invoke("delete_tcp_listener", { collectionid: collectionIdRef.value, setupid: setupIdRef.value, listenerid: listenerId })
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
    })
    .catch((error) => window.alert(error));
}

function deleteHttpServer(serverId: string) {
    invoke("delete_http_server", { collectionid: collectionIdRef.value, setupid: setupIdRef.value, serverid: serverId })
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
    })
    .catch((error) => window.alert(error));
}

function updateHttpServer() {
  invoke("update_http_server", { collectionid: collectionIdRef.value, setupid: setupIdRef.value, serverid: editHttpServerData.value.id, httpServer: editHttpServerData.value })
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
      isEditHttpServerOpen.value = false;
    })
    .catch((error) => window.alert(error));
}

function updateEndpoint() {
  invoke("update_endpoint", { collectionid: collectionIdRef.value, setupid: setupIdRef.value, serverid: editServerIdForNewEndpoint.value, endpointid: editEndpointData.value.id, endpoint: editEndpointData.value })
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
      isEditEndpointOpen.value = false;
    })
    .catch((error) => window.alert(error));
}

function addEndpoint(serverId: string) {
    invoke("add_endpoint", { collectionid: collectionIdRef.value, setupid: setupIdRef.value, serverid: serverId })
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
    })
    .catch((error) => window.alert(error));  
}

function deleteEndpoint(serverid: string, endpointid: string) {
    invoke("delete_endpoint", { collectionid: collectionIdRef.value, setupid: setupIdRef.value, serverid: serverid, endpointid: endpointid })
    .then(() => {
      refresh(collectionIdRef.value, setupIdRef.value);
    })
    .catch((error) => window.alert(error));  
}

defineExpose({ refresh });
</script>

<template class="h-full w-full min-h-0 overflow-y-auto p-4">
  <div class="h-full w-full min-h-0 overflow-y-auto p-4" v-if="setup">
    <div class="mx-auto max-w-7xl space-y-4">
      <UCard>
        <div class="flex flex-wrap gap-2">
          <UButton :variant="activeTab === 'general' ? 'solid' : 'soft'" color="primary" size="sm"
            @click="activeTab = 'general'">General</UButton>
          <UButton :variant="activeTab === 'tcp' ? 'solid' : 'soft'" color="primary" size="sm"
            @click="activeTab = 'tcp'">Tcp listeners</UButton>
          <UButton :variant="activeTab === 'http' ? 'solid' : 'soft'" color="primary" size="sm"
            @click="activeTab = 'http'">Http servers</UButton>
          <UButton v-if="setup?.params" :variant="activeTab === 'predefined' ? 'solid' : 'soft'" color="primary"
            size="sm" @click="activeTab = 'predefined'">Predefined sets</UButton>
        </div>
      </UCard>

      <div v-if="activeTab === 'general'" class="space-y-4">
        <UCard>
          <template #header>
            <div class="flex items-center justify-between gap-2">
              <div class="text-sm text-neutral-200">General</div>
              <UButton size="xs" color="primary" variant="soft" @click="editSetup">Edit</UButton>
            </div>
          </template>
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
                <UBadge v-for="param in (setup?.params || [])" :key="param" color="secondary" variant="soft">{{ param }}
                </UBadge>
                <span v-if="!setup?.params || setup.params.length === 0" class="text-sm text-neutral-500">No
                  parameters</span>
              </div>
            </div>
          </div>
        </UCard>
      </div>

      <div v-if="activeTab === 'tcp'" class="space-y-3">
        <UCard>
          <template #header>
            <div class="flex items-center justify-between gap-3">
              <div class="text-sm text-neutral-200">Tcp listeners</div>
              <UButton size="xs" color="primary" variant="soft" @click="addTcpListener()"><i class="fa-solid fa-plus" />Add tcp listener
              </UButton>
            </div>
          </template>
        </UCard>
        <UAccordion :items="setup?.listeners" class="w-full"
          :ui="{
            item: 'overflow-hidden rounded-xl border border-neutral-700 bg-neutral-900',
            trigger: 'px-3 py-3 text-left hover:bg-neutral-800/80 data-[state=open]:bg-neutral-800',
            content: 'border-t border-neutral-700 bg-neutral-950 p-3'
          }">
          <template #default="{item: listenerItem}">
            <div class="flex items-center justify-between gap-2">
              <div class="text-sm text-neutral-200">
                <i class="fa-solid fa-ear-listen mr-2" />Tcp listener on port {{ listenerItem.port }}
              </div>
            </div>
          </template>
          <template #body="{item: listenerItem}">
            <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
              <p><span class="text-neutral-500 md:col-span-1">Accept:</span> {{ listenerItem.accept }}</p>
              <div class="flex gap-1 justify-end md:col-span-2">
                <UButton size="xs" color="primary" variant="soft" @click="editTcpListener(listenerItem)">Edit</UButton>
                <UButton size="xs" color="error" variant="soft" @click="deleteTcpListener(listenerItem.id)">Delete</UButton>
              </div>                  
              <p><span class="text-neutral-500 md:col-span-1">Close:</span> {{ listenerItem.closeConnection }}</p>
              <p><span class="text-neutral-500 md:col-span-1">Delay write:</span> {{ listenerItem.delayWriteMs }}</p>             
              <p class="break-all"><span class="text-neutral-500">File:</span> {{ listenerItem.file }}</p>
              <UFormField label="Data" class="w-full md:col-span-3">
                <UTextarea :model-value="listenerItem.data" :rows="8" autoresize readonly class="w-full" variant="none text-neutral-500" />
              </UFormField>
            </div>
          </template>
        </UAccordion>
        <UCard v-if="setup?.tcpListeners?.length === 0">
          <p class="text-sm text-neutral-500">No TCP listeners configured.</p>
        </UCard>
      </div>

      <div v-if="activeTab === 'http'" class="space-y-3">
        <UCard>
          <template #header>
            <div class="flex items-center justify-between gap-3">
              <div class="text-sm text-neutral-200">Http servers</div>
              <UButton size="xs" color="primary" variant="soft" @click="addHttpServer()"><i class="fa-solid fa-plus" />Add http server
              </UButton>
            </div>
          </template>
        </UCard>
        <UAccordion
          :items="setup.servers"
          class="w-full"
          :ui="{
            item: 'overflow-hidden rounded-xl border border-neutral-700 bg-neutral-900',
            trigger: 'px-3 py-3 text-left hover:bg-neutral-800/80 data-[state=open]:bg-neutral-800',
            content: 'border-t border-neutral-700 bg-neutral-950 p-3'
          }"
        >
          <template #default="{ item: serverItem }">
            <div class="flex items-center justify-between gap-2">
              <div class="flex items-center gap-2 text-sm font-medium text-neutral-200">
                Http server: {{ serverItem.name || 'Unnamed' }}
              </div>
            </div>
          </template>

          <template #body="{ item: serverItem }">
            
            <div class="grid grid-cols-1 gap-1 md:grid-cols-4 text-sm">
              <p><span class="text-neutral-500">Http port:</span> {{ serverItem.httpPort }}</p>
              <p class="md:col-span-2"><span class="text-neutral-500">Description:</span> {{ serverItem.description }}</p>
              <div class="flex gap-1 justify-end md:col-span-1">
                <UButton size="xs" color="primary" variant="soft" @click="editHttpServer(serverItem)">Edit</UButton>
                <UButton size="xs" color="error" variant="soft" @click="deleteHttpServer(serverItem.id)">Delete</UButton>
              </div>
            </div>

            <div v-if="serverItem?.httpsConfig" class="mt-4 rounded border border-neutral-800 p-3 text-sm">
              <p class="mb-2 text-neutral-300">HTTPS config</p>
              <div class="grid grid-cols-1 gap-2 md:grid-cols-2">
                <p><span class="text-neutral-500">Https port:</span> {{ serverItem.httpsConfig.httpsPort }}</p>
                <p><span class="text-neutral-500">Server cert:</span> {{ serverItem.httpsConfig.serverCertificate }}</p>
                <p><span class="text-neutral-500">Private key:</span> {{ serverItem.httpsConfig.privateKey }}</p>
                <p><span class="text-neutral-500">Client cert:</span> {{ serverItem.httpsConfig.clientCertificate }}</p>
              </div>
            </div>

            <div class="mt-4 space-y-3">
              <div class="flex justify-end">
                <UButton size="sm" color="primary" variant="soft" @click="addEndpoint(serverItem.id)">Add endpoint</UButton>
              </div>
              <UAccordion
                :items="serverItem?.endpoints || []"
                class="w-full"
                :ui="{
                  item: 'overflow-hidden rounded-lg border border-neutral-700 bg-neutral-900',
                  trigger: 'px-3 py-2 text-left hover:bg-neutral-800/80 data-[state=open]:bg-neutral-800',
                  content: 'border-t border-neutral-700 bg-neutral-950 p-3'
                }"
              >
                <template #default="{ item: endpointItem }">
                  <div class="flex items-center justify-between gap-2">
                    <div class="flex items-center gap-2 text-sm font-medium text-neutral-200">
                      <span class="inline-flex h-5 w-10 items-center justify-center rounded border border-neutral-600 bg-neutral-800 text-[10px] text-primary-300">
                        {{ endpointItem.method }}
                      </span>
                      <span class="truncate">{{ endpointItem.pathExpression }}</span>
                    </div>
                  </div>
                </template>
                <template #body="{ item: endpointItem }">
                  <div class="space-y-2 text-sm md:grid md:grid-cols-3 md:gap-1">
                    <div class="flex gap-1 justify-end md:col-span-3">
                      <UButton size="xs" color="primary" variant="soft" @click="editEndpoint(serverItem.id, endpointItem)">Edit</UButton>
                      <UButton size="xs" color="error" variant="soft" @click="deleteEndpoint(serverItem.id, endpointItem.id)">Delete</UButton>
                    </div>                    
                  </div>
                  <div v-if="endpointItem?.mock" class="space-y-1 text-sm md:grid md:grid-cols-2 md:gap-1">
                    <p><span class="text-neutral-500">Status:</span> {{ endpointItem.mock.status }}</p>
                    <p><span class="text-neutral-500">Delay:</span> {{ endpointItem.mock.delay }}</p>
                    <p><span class="text-neutral-500">Headers:</span></p>
                    <p><span class="text-neutral-500">Response:</span></p>
                    <UTextarea :model-value="endpointItem.mock.headers" readonly variant="none" autoresize resize-none
                      class="w-full" color="neutral" />
                    <UTextarea :model-value="endpointItem.mock.response" readonly variant="none" autoresize resize-none
                      class="w-full" color="neutral" />
                  </div>
                  <div v-if="endpointItem?.route" class="grid grid-cols-1 gap-2 text-sm md:grid-cols-2">
                    <p class="md:col-span-2"><span class="text-neutral-500">Url:</span> {{ endpointItem.route.url }}</p>
                    <p class="md:col-span-2"><span class="text-neutral-500">Proxy url:</span> {{ endpointItem.route.proxyUrl
                      }}</p>
                    <p><span class="text-neutral-500">Http1 only:</span> {{ endpointItem.route.http1Only }}</p>
                    <p><span class="text-neutral-500">Accept invalid certs:</span> {{ endpointItem.route.acceptInvalidCerts }}
                    </p>
                    <p><span class="text-neutral-500">Accept invalid hostnames:</span> {{
                      endpointItem.route.acceptInvalidHostnames }}</p>
                    <p><span class="text-neutral-500">Min TLS:</span> {{ endpointItem.route.minTlsVersion }}</p>
                    <p><span class="text-neutral-500">Max TLS:</span> {{ endpointItem.route.maxTlsVersion }}</p>
                    <p><span class="text-neutral-500">Read timeout:</span> {{ endpointItem.route.readTimeout }}</p>
                    <p><span class="text-neutral-500">Connect timeout:</span> {{ endpointItem.route.connectTimeout }}</p>
                    <p><span class="text-neutral-500">Delay before:</span> {{ endpointItem.route.delayBefore }}</p>
                    <p><span class="text-neutral-500">Delay after:</span> {{ endpointItem.route.delayAfter }}</p>
                  </div>
                </template>
              </UAccordion>
            </div>
          </template>          
        </UAccordion>
        <UCard v-if="setup?.servers?.length === 0">
          <p class="text-sm text-neutral-500">No HTTP servers configured.</p>
        </UCard>
      </div>

      <div v-if="activeTab === 'predefined' && setup?.params">
        <UCard>
          <template #header>
            <div class="flex items-center justify-between gap-1">
              <p class="text-sm text-neutral-200">Predefined sets</p>
              <UButton size="xs" color="primary" variant="soft" @click="">Add</UButton>
            </div>
          </template>
          <div class="space-y-2">
            <div v-for="predefSet in setup?.predefinedSets" :key="predefSet.name"
              class="flex items-center justify-between gap-2">
              <UButton size="xs" :variant="predefSet.name === setup?.selectedPredefinedSet?.name ? 'solid' : 'ghost'"
                color="primary" class="justify-start" @click="">
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
                <tr v-for="predefKey in Object.keys(setup?.selectedPredefinedSet.values || {})" :key="predefKey"
                  class="border-t border-neutral-800">
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
          <UFormField label="Id" class="w-full">
            <UInput class="w-full" :model-value="editSetupData.id" disabled />
          </UFormField>
          <UFormField label="Name" class="w-full">
            <UInput class="w-full" v-model="editSetupData.name" />
          </UFormField>
          <UFormField label="Description" class="w-full">
            <UTextarea class="w-full" v-model="editSetupData.description" :rows="4" />
          </UFormField>
          <UFormField label="Parameters" class="w-full">
            <div class="flex gap-2">
              <UInput v-model="editAddParameter" placeholder="Parameter" class="flex-1 w-full" />
              <UButton color="primary" variant="soft" @click="addParameter()">Add</UButton>
            </div>
            <div class="mt-2 flex flex-wrap gap-2">
              <UButton v-for="param in (editSetupData.params || [])" :key="param" size="xs" color="error"
                variant="soft" @click="deleteParameter(param)">{{ param }}</UButton>
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
        <div class="space-y-4 grid grid-cols-1 md:grid-cols-4">
          <UFormField label="Port">
            <UInput type="number" v-model="editTcpListenerData.port" maxlength="5" />
          </UFormField>
          <UFormField label="Delayed write ms">
            <UInput type="number" v-model="editTcpListenerData.delayWriteMs" maxlength="5" />
          </UFormField>
          <UFormField label="Close connection" class="w-full">
            <USelect v-model="editTcpListenerData.closeConnection"
              :items="['AfterRead', 'AfterResponse', 'BeforeRead', 'Never']" class="w-full" />
          </UFormField>
          <div class="flex gap-4">
            <UCheckbox v-model="editTcpListenerData.accept" label="Accept" />
          </div>
          <UFormField label="File" class="md:col-span-4">
            <div class="flex gap-2">
              <UInput v-model="editTcpListenerData.file" readonly class="flex-1" />
              <UButton size="sm" color="primary" variant="soft" @click="">Select</UButton>
              <UButton size="sm" color="error" variant="soft" @click="">Clear</UButton>
            </div>
          </UFormField>
          <UFormField label="Response" class="md:col-span-4 w-full">
            <UTextarea v-model="editTcpListenerData.data" :rows="8" class="w-full" />
          </UFormField>
        </div>
      </template>
      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton color="neutral" variant="ghost" @click="isEditTcpListenerOpen = false">Cancel</UButton>
          <UButton color="primary" variant="solid" @click="updateTcpListener()">Save</UButton>
        </div>
      </template>
    </UModal>

    <UModal v-model:open="isEditHttpServerOpen" title="Edit HTTP server" :ui="{ content: 'sm:max-w-2xl' }">
      <template #body>
        <div class="space-y-4 grid grid-cols-1 md:grid-cols-4">
          <UFormField label="Name" class="md:col-span-2">
            <UInput v-model="editHttpServerData.name" />
          </UFormField>
          <UFormField label="Http port" class="md:col-span-2">
            <UInput type="number" v-model="editHttpServerData.httpPort" />
          </UFormField>
          <UCheckbox v-model="showEditHttpsConfig" @change="editHttpServerData.httpsConfig = showEditHttpsConfig ? {} : null" label="Https config" class="md:col-span-4" />
          <div v-if="editHttpServerData.httpsConfig" class="space-y-4 rounded border border-neutral-800 p-3 md:col-span-4">
            <UFormField label="Https port" class="md:col-span-2">
              <UInput type="number" v-model="editHttpServerData.httpsConfig.httpsPort" />
            </UFormField>
            <UFormField label="Server certificate" class="md:col-span-2">
              <div class="flex gap-2">
                <UInput v-model="editHttpServerData.httpsConfig.serverCertificate" readonly class="flex-1" />
                <UButton size="sm" color="primary" variant="soft" @click="">Select</UButton>
                <UButton size="sm" color="error" variant="soft" @click="">Clear</UButton>
              </div>
            </UFormField>
            <UFormField label="Private key" class="md:col-span-2">
              <div class="flex gap-2">
                <UInput v-model="editHttpServerData.httpsConfig.privateKey" readonly class="flex-1" />
                <UButton size="sm" color="primary" variant="soft" @click="">Select</UButton>
                <UButton size="sm" color="error" variant="soft" @click="">Clear</UButton>
              </div>
            </UFormField>
            <UFormField label="Client certificate" class="md:col-span-2">
              <div class="flex gap-2">
                <UInput v-model="editHttpServerData.httpsConfig.clientCertificate" readonly class="flex-1" />
                <UButton size="sm" color="primary" variant="soft" @click="">Select</UButton>
                <UButton size="sm" color="error" variant="soft" @click="">Clear</UButton>
              </div>
            </UFormField>
            <UFormField label="Supported TLS versions" class="md:col-span-4">
              <UCheckboxGroup v-model="editSupportedTlsVersions"
                :items="['TLSv1.0', 'TLSv1.1', 'TLSv1.2', 'TLSv1.3']" />
            </UFormField>
          </div>
        </div>
      </template>
      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton color="neutral" variant="ghost" @click="isEditHttpServerOpen = false">Cancel</UButton>
          <UButton color="primary" variant="solid" @click="updateHttpServer()">Save</UButton>
        </div>
      </template>
    </UModal>

    <UModal v-model:open="isEditEndpointOpen" title="Edit endpoint" :ui="{ content: 'sm:max-w-4xl' }">
      <template #body>
        <div class="grid grid-cols-1 gap-4 md:grid-cols-4">
          <UFormField label="Method" class="md:col-span-2">
            <USelect v-model="editEndpointData.method" :items="['DELETE', 'GET', 'HEAD', 'POST', 'PUT', 'OPTION']" class="w-full" />
          </UFormField>
          <UFormField label="Path expression " class="md:col-span-4 w-full">
            <UInput v-model="editEndpointData.pathExpression" class="w-full" />
          </UFormField>
          <UFormField label="Body expression" class="md:col-span-4 w-full">
            <UInput v-model="editEndpointData.bodyExpression" class="w-full" />
          </UFormField>
          <div class="md:col-span-4">
            <UCheckbox v-model="selectedMock" @change="editEndpointData.mock = selectedMock ? {} : null; editEndpointData.route = selectedMock ? null : {}" label="Mock/Route" />
          </div>          
          <template v-if="editEndpointData.mock">
            <div class="md:grid md:grid-cols-4 gap-4 md:col-span-4">
              <UFormField label="Status" class="md:col-span-1">
                <UInput v-model="editEndpointData.mock.status" />
              </UFormField>
              <UFormField label="Delay" class="md:col-span-1">
                <UInput type="number" v-model="editEndpointData.mock.delay" />
              </UFormField>
              <UFormField label="Headers" class="md:col-span-4 w-full">
                <UTextarea v-model="editEndpointData.mock.headers" :rows="6" class="w-full" />
              </UFormField>
              <UFormField label="Response" class="md:col-span-4 w-full">
                <UTextarea v-model="editEndpointData.mock.response" :rows="8" class="w-full" />
              </UFormField>
            </div>
          </template>
          <template v-if="!editEndpointData.mock">
            <div class="md:grid md:grid-cols-4 gap-4 md:col-span-4">
              <UFormField label="Url" class="md:col-span-2">
                <UInput v-model="editEndpointData.route.url" class="w-full" />
              </UFormField>
              <UFormField label="Proxy url" class="md:col-span-2">
                <UInput v-model="editEndpointData.route.proxyUrl" class="w-full" />
              </UFormField>
              <UCheckbox v-model="editEndpointData.route.http1Only" label="Http1 only" class="md:col-span-1" />
              <UCheckbox v-model="editEndpointData.route.acceptInvalidCerts" label="Accept invalid certs" class="md:col-span-1" />
              <UCheckbox v-model="editEndpointData.route.acceptInvalidHostnames" label="Accept invalid hostnames" class="md:col-span-1" />
              <div class="md:col-span-1"></div>
              <UFormField label="Min TLS" class="md:col-span-1 w-full">
                <USelect v-model="editEndpointData.route.minTlsVersion" :items="['TLSv1.2', 'TLSv1.3']" class="w-full" />
              </UFormField>
              <UFormField label="Max TLS" class="md:col-span-1 w-full">
                <USelect v-model="editEndpointData.route.maxTlsVersion" :items="['TLSv1.2', 'TLSv1.3']" class="w-full" />
              </UFormField>
              <div class="md:col-span-1"></div>
              <div class="md:col-span-1"></div>
              <UFormField label="Read timeout" class="md:col-span-1 w-full">
                <UInput type="number" v-model="editEndpointData.route.readTimeout" class="w-full" />
              </UFormField>
              <UFormField label="Connect timeout" class="md:col-span-1 w-full">
                <UInput type="number" v-model="editEndpointData.route.connectTimeout" class="w-full" />
              </UFormField>
              <div class="md:col-span-1"></div>
              <div class="md:col-span-1"></div>              
              <UFormField label="Delay before request" class="md:col-span-1 w-full">
                <UInput type="number" v-model="editEndpointData.route.delayBefore" class="w-full" />
              </UFormField>
              <UFormField label="Delay after request" class="md:col-span-1 w-full">
                <UInput type="number" v-model="editEndpointData.route.delayAfter" class="w-full" />
              </UFormField>
            </div>
          </template>
        </div>
      </template>
      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton color="neutral" variant="ghost" @click="isEditEndpointOpen = false">Cancel</UButton>
          <UButton color="primary" variant="solid" @click="updateEndpoint()">Save</UButton>
        </div>
      </template>
    </UModal>

    <UModal v-model:open="isEditPredefinedSetOpen" title="Edit predefined set" :ui="{ content: 'sm:max-w-2xl' }">
      <template #body>
        <div class="space-y-4">
          <UFormField label="Name">
            <UInput v-model="editSelectedPredefinedSet.name" />
          </UFormField>
          <div class="space-y-2">
            <div v-for="predefKey in Object.keys(editSelectedPredefinedSet.values || {})" :key="predefKey"
              class="grid grid-cols-3 gap-2">
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