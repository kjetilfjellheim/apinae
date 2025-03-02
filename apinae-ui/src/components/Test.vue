<script setup>
//Required for showing editing modals.
import { Modal } from 'bootstrap/dist/js/bootstrap.bundle';
//Required for showing the test data and updating the test data.
import { ref, onMounted } from "vue";
//Required for getting the route parameters.
import { useRoute } from 'vue-router'
//Required for calling the rust code.
import { invoke } from "@tauri-apps/api/core";

// Required to get the route parameters
const route = useRoute();

// The test being displayed
const test = ref([]);
// Array of all tcp listeners for the test
const tcpListeners = ref([]);
// Array of all http servers for the test
const httpServers = ref([]);
// Data for editing a tcp listener. Data is copied from the tcpListener 
// object to this object when the user clicks the edit button
const editTcpListenerData = ref({});
// Data for editing a http server. Data is copied from the httpServer
// object to this object when the user clicks the edit button
const editHttpServerData = ref({});
// Data for editing an endpoint. Data is copied from the endpoint object
// to this object when the user clicks the edit button
const editEndpointData = ref({});
// Show or hide the https config for the http server. If the http server
// has an https config, this is set to true, otherwise false
const showEditHttpsConfig = ref({});
// Show or hide the mock data for the endpoint when editing. If the endpoint
// has mock data, this is set to true, otherwise false.
const showEditMockData = ref(true);
// Data for editing the https config for the http server. Data is copied
// from the https_config object to this object when the user clicks the edit button.
// Null if the http server does not have an https config
const editHttpsConfig = ref({});
// Data for editing the endpoint mock data. Data is copied from the endpoint mock object
// to this object when the user clicks the edit button. Empty if the endpoint does not have
// mock data.
const editMockData = ref({});
// Data for editing the endpoint route data. Data is copied from the endpoint route object
// to this object when the user clicks the edit button. Empty if the endpoint does not have
// route data.
const editRouteData = ref({});
// Supported tls versions for the https config. This is an array of strings
// with the supported tls versions and is updated in during editing of the https config.
const editSupportedTlsVersions = ref([]);
// Reference to the modal for editing the http server. Implemented as a ref rather than using
// the modal directly in the button so that we can implement validation when
// the user clicks the Ok button
const editHttpServerModal = ref(null);
// Reference to the modal for editing the tcp listener. Implemented as a ref rather than using
// the modal directly in the button so that we can implement validation when
// the user clicks the Ok button
const editTcpListenerModal = ref(null);
// Reference to the modal for editing the endpoint. Implemented as a ref rather than using
// the modal directly in the button so that we can implement validation when
// the user clicks the Ok button
const editEndpointModal = ref(null);
// Reference to the server id for the endpoint being edited. This is used to send the server id
// to the rust code when updating the endpoint.
const serverIdEditEndpoint = ref(null);

//Refreshes the test data, tcp listeners and http servers. This is called when the page is loaded
//and when the user updates the data. This is because when updating the data we only update the
//remote rust data, not the local data. This is so that we only have one source of truth for the data.
//TODO: Only refresh required data, not all data.
const refresh = (testId) => {
  invoke("get_test", { testid: testId })
    .then((message) => {
      test.value = message;
    })
    .catch((error) => window.alert(error));

  invoke("get_servers", { testid: testId })
    .then((message) => {
      httpServers.value = message;
    })
    .catch((error) => window.alert(error));

  invoke("get_listeners", { testid: testId })
    .then((message) => {
      tcpListeners.value = message;
    })
    .catch((error) => window.alert(error));
}

//Add an http server to the test. This is called when the user clicks the add button.
//The server is added to the test and the data is refreshed.
const addHttpServer = () => {
  invoke("add_server", { testid: test.value.id })
    .then((message) => {
      refresh(test.value.id);
    })
    .catch((error) => window.alert(error));
}

//Delete an http server from the test. This is called when the user clicks the delete button.
//The server is deleted from the test and the data is refreshed. The user is asked to confirm
//the deletion first. This uses the rust confirm_dialog function to display a dialog to the user.
const confirmDeleteHttpServer = (serverId) => {
  invoke("confirm_dialog", {})
    .then((confirm) => {
      if (confirm) {
        invoke("delete_server", { testid: test.value.id, serverid: serverId })
          .then((message) => {
            refresh(test.value.id);
          })
          .catch((error) => window.alert(error));
      }
    }
    )
    .catch((error) => window.error(error));
}

//Add a tcp listener to the test. This is called when the user clicks the add button.
//The listener is added to the test and the data is refreshed.
const addTcpListener = () => {
  invoke("add_listener", { testid: test.value.id })
    .then((message) => {
      refresh(test.value.id);
    })
    .catch((error) => window.alert(error));
}

//Add an endpoint to the http server. This is called when the user clicks the add button.
//The endpoint is added to the http server and the data is refreshed.
const addEndpoint = (httpServer) => {
  invoke("add_endpoint", { testid: test.value.id, serverid: httpServer.id })
    .then((message) => {
      refresh(test.value.id);
    })
    .catch((error) => window.alert(error));
}

//Delete an endpoint from the http server. This is called when the user clicks the delete button.
//The endpoint is deleted from the htp server and the data is refreshed. The user is asked to confirm
//the deletion first. This uses the rust confirm_dialog function to display a dialog to the user.
const confirmDeleteEndpoint = (httpServer, endpoint) => {
  invoke("confirm_dialog", {})
    .then((confirm) => {
      if (confirm) {
        invoke("delete_endpoint", { testid: test.value.id, serverid: httpServer.id, endpointid: endpoint.id })
          .then((message) => {
            refresh(test.value.id);
          })
          .catch((error) => window.alert(error));
      }
    }
    )
    .catch((error) => console.error(error));
}

//Delete a tcp listener from the test. This is called when the user clicks the delete button.
//The listener is deleted from the test and the data is refreshed. The user is asked to confirm
//the deletion first. This uses the rust confirm_dialog function to display a dialog to the user.
const confirmDeleteTcpListener = (listenerid) => {
  invoke("confirm_dialog", {})
    .then((confirm) => {
      if (confirm) {
        invoke("delete_listener", { testid: test.value.id, listenerid: listenerid })
          .then((message) => {
            refresh(test.value.id);
          })
          .catch((error) => window.alert(error));
      }
    }
    )
    .catch((error) => console.error(error));
}

//Initializes the data for editing a tcp listener. This is called when the user clicks the edit button.
//The data is copied from the tcpListener object to the editTcpListenerData object.
const editTcpListener = (tcpListener) => {
  editTcpListenerData.value = { ...tcpListener };
}

//Initializes the data for editing an endpoint. This is called when the user clicks the edit button.
//The data is copied from the endpoint object to the editEndpointData object.
const editEndpoint = (httpServer, endpoint) => {
  if (endpoint.mock) {
    editMockData.value = { ...endpoint.mock };
    editRouteData.value = {};
    showEditMockData.value = true;
  } else {
    editMockData.value = {};
    editRouteData.value = { ...endpoint.route };
    showEditMockData.value = false;
  }
  serverIdEditEndpoint.value = httpServer.id;
  editEndpointData.value = { ...endpoint };
}

//Initializes the data for editing a http server. This is called when the user clicks the edit button.
//The data is copied from the httpServer object to the editHttpServerData object. If the http server have
//an https config, the data is copied from the httpsConfig object to the editHttpsConfig object it also copies
//the supported tls versions to the editSupportedTlsVersions object.
const editHttpServer = (httpServer) => {
  if (httpServer?.httpsConfig) {
    editHttpsConfig.value = { ...httpServer?.httpsConfig };
    editSupportedTlsVersions.value = httpServer?.httpsConfig?.supportedTlsVersions ? [...httpServer?.httpsConfig?.supportedTlsVersions] : [];
  } else {
    editHttpsConfig.value = {};
    editSupportedTlsVersions.value = [];
  }
  editHttpServerData.value = { ...httpServer };
  showEditHttpsConfig.value = httpServer.httpsConfig ? true : false;
}

//Updates the tcp listener. This is called when the user clicks the Ok button in the edit modal.
//The data on the editTcpListenerData object is sent to the rust code to update the tcp listener.
//If successful the modal is hidden and the data is refreshed.
const updateTcpListener = (tcpListener) => {
  invoke("update_listener", { testid: test.value.id, listenerid: tcpListener.id, tcplistener: convertTcpListenerToRequestObject(tcpListener) })
    .then((message) => {
      editTcpListenerModal.value.hide();
      refresh(test.value.id);
    })
    .catch((error) => window.alert(error));
}

//Updates the http server. This is called when the user clicks the Ok button in the edit modal.
//The data on the editHttpServerData, editHttpsConfig and editSupportedTlsVersions objects are sent 
//to the rust code to update the http server.
//If successful the modal is hidden and the data is refreshed.
const updateHttpServer = (httpServer, httpsConfig, supportedTlsVersions) => {
  invoke("update_server", { testid: test.value.id, serverid: httpServer.id, httpserver: convertHttpServerToRequestObject(httpServer, httpsConfig, supportedTlsVersions) })
    .then((message) => {
      editHttpServerModal.value.hide();
      refresh(test.value.id);
    })
    .catch((error) => window.alert(error));
}

//Updates the endpoint. This is called when the user clicks the Ok button in the edit modal.
//The data on the editEndpointData object is sent to the rust code to update the endpoint.
//If successful the modal is hidden and the data is refreshed.
const updateEndpoint = (endpoint) => {
  invoke("update_endpoint", { testid: test.value.id, serverid: serverIdEditEndpoint.value, endpointid: endpoint.id, endpoint: convertEndpointToRequestObject(editEndpointData, editMockData, editRouteData) })
    .then((message) => {
      editEndpointModal.value.hide();
      refresh(test.value.id);
    })
    .catch((error) => window.alert(error));
}

const convertEndpointToRequestObject = (editEndpointData, editMockData, editRouteData) => {
  console.log("showEditMockData:" +showEditMockData.value);
  return {
    id: editEndpointData.value.id,
    pathExpression: editEndpointData.value.pathExpression,
    method: editEndpointData.value.method,
    mock: showEditMockData.value ? convertMockToRequestObject(editMockData) : null,
    route: !showEditMockData.value ? convertRouteToRequestObject(editRouteData) : null,
  }
}

const convertMockToRequestObject = (mockData) => {
  return {
    status: mockData.value.status ? parseInt(mockData.value.status) : null,
    headers: {},
    delay: parseInt(mockData.value.delay),
    response: mockData.value.response
  }
}

const convertRouteToRequestObject = (routeData) => {
  return {
    url: routeData.value.url,
    proxyUrl: routeData.value.proxyUrl,
    http1Only: routeData.value.http1Only ? true : false,
    acceptInvalidCerts: routeData.value.acceptInvalidCerts ? true : false,
    acceptInvalidHostnames: routeData.value.acceptInvalidHostnames ? true : false,
    minTlsVersion: routeData.value.minTlsVersion,
    maxTlsVersion: routeData.value.maxTlsVersion,
    readTimeout: routeData.value.readTimeout ? parseInt(routeData.value.readTimeout) : null,
    connectTimeout: routeData.value.connectTimeout ? parseInt(routeData.value.connectTimeout) : null
  }
}

//Converts the http server object to a request object that can be sent to the rust code.
//This is used when updating the http server.
//TODO: Implement validation if the objects are not valid.
const convertHttpServerToRequestObject = (httpServer, httpsConfig, supportedTlsVersions) => {
  return {
    id: httpServer.id,
    name: httpServer.name,
    description: httpServer.description,
    httpPort: httpServer.httpPort ? parseInt(httpServer.httpPort) : null,
    httpsConfig: showEditHttpsConfig.value ? convertHttpsConfigToRequestObject(httpsConfig, supportedTlsVersions) : null,
    endpoints: []
  }
}

//Converts the https config object to a request object that can be sent to the rust code.
//This is used when updating the http server.
//TODO: Implement validation if the objects are not valid.
const convertHttpsConfigToRequestObject = (httpsConfig, supportedTlsVersions) => {
  return {
    httpsPort: httpsConfig.httpsPort ? parseInt(httpsConfig.httpsPort) : null,
    serverCertificate: httpsConfig.serverCertificate,
    privateKey: httpsConfig.privateKey,
    clientCertificate: httpsConfig.clientCertificate,
    supportedTlsVersions: supportedTlsVersions
  }
}

//Converts the tcp listener object to a request object that can be sent to the rust code.
//This is used when updating the tcp listener.
//TODO: Implement validation if the objects are not valid.
const convertTcpListenerToRequestObject = (tcpListener) => {
  return {
    id: tcpListener.id,
    port: tcpListener.port ? parseInt(tcpListener.port) : null,
    accept: tcpListener.accept,
    closeConnection: tcpListener.closeConnection,
    delayWriteMs: tcpListener.delayWriteMs ? parseInt(tcpListener.delayWriteMs) : null,
    file: tcpListener.file,
    data: tcpListener.data
  }
}

//Initializes the data when the page is loaded.
//Initializes the httpserver and tcplistener modals so that we can show and hide them 
//in code rather than just in the button.
onMounted(() => {
  editEndpointModal.value = new Modal('#idEditEndpointModel', { keyboard: false });
  editHttpServerModal.value = new Modal('#idEditHttpServerModel', { keyboard: false });
  editTcpListenerModal.value = new Modal('#idEditTcpListenerModel', { keyboard: false });
  const testId = route.params.testid
  refresh(testId)
});
</script>
<style>
/* The max height is full view height minus (top bar, menu bar and status bar + margins) */
.main-content {
  max-height: calc(100vh - 93px);
  overflow-y: scroll;
}

.margin-0 {
  margin: 0px 0px 0px 0px !important;
}

.padding-0 {
  padding: 0px 0px 0px 0px !important;
}

.card>.card-header {
  background-color: #525252;
}

.card>.card-body {
  background-color: #242424;
}

.button-position-right {
  position: absolute !important;
  right: 15px !important;
}

/* Keep the toolbar height always 90px */
.toolbar-col {
  min-width: 90px;
  max-width: 90px;
  width: 90px;
}
</style>
<template>
  <!--
     Show navigation bar. 
     TODO: Move this to a separate component.  
  -->
  <nav class="navbar navbar-expand-sm bg-body-tertiary small">
    <div class="container-fluid">
      <ol class="breadcrumb  margin-0 padding-0 align-middle">
        <li class="breadcrumb-item"><router-link to="/"><i class="fas fa-house"></i></router-link></li>
        <li class="breadcrumb-item">{{ test?.name }}</li>
      </ol>
    </div>
  </nav>
  <!--
    Show the test data.
  -->
  <div class="container-fluid main-content padding-0 margin-0">
    <div class="row padding-0 margin-0">
      <div class="col-12">
        &nbsp;
      </div>
      <div class="col-12">
        <dl class="row">
          <dt class="col-sm-3 small">Id</dt>
          <dd class="col-sm-9 small">{{ test?.id }}</dd>
          <dt class="col-sm-3 small">Name</dt>
          <dd class="col-sm-9 small">{{ test?.name }}</dd>
          <dt class="col-sm-3 small">Description</dt>
          <dd class="col-sm-9 small">{{ test?.description }}</dd>
        </dl>
      </div>
      <!--
        Show the tcp listeners
        TODO: Move this to a separate component.
      -->
      <div class="col-12">
        <h5>Tcp listeners
          <div class="btn-group btn-group-sm align-middle small me-2" role="group">
            <button type="button" class="btn btn-sm btn-outline-primary " @click="addTcpListener()"><i
                class="fa-solid fa-plus"></i></button>
          </div>
        </h5>
      </div>
      <div class="col-12">
        <div class="card" v-for="tcpListener in tcpListeners" :key="tcpListener.port"
          v-if="tcpListeners?.length > 0">
          <div class="card-header">
            Tcp listener for port {{ tcpListener.port }}
          </div>
          <div class="card-body">
            <div class="btn-toolbar" role="toolbar"
              aria-label="Toolbar with button groups margin-0 padding-0 align-middle">
              <div class="btn-group btn-group-sm align-middle small me-2 margin-0 padding-0 button-position-right"
                role="group">
                <button type="button" class="btn btn-sm btn-outline-primary align-middle"
                  @click="editTcpListener(tcpListener)" data-bs-toggle="modal"
                  data-bs-target="#idEditTcpListenerModel"><i class="fa-solid fa-file-pen"></i></button>
                <button class="btn btn-sm btn-outline-danger align-middle"
                  @click="confirmDeleteTcpListener(tcpListener.id)"><i class="fa-solid fa-trash"></i></button>
              </div>
            </div>
            <div class="container-fluid">
              <div class="row">
                <div class="col-4">
                  <div class="mb-0 row">
                    <label for="idLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Id</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="idLabel"
                        :value="tcpListener.id">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="portLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Port</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="portLabel"
                        :value="tcpListener.port">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="acceptLabel"
                      class="col-sm-6 col-form-label small text-truncate padding-0">Accept</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="acceptLabel"
                        :value="tcpListener.accept">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="acceptLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Close
                      connection</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="acceptLabel"
                        :value="tcpListener.closeConnection">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="acceptLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Delayed write
                      response</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="acceptLabel"
                        :value="tcpListener.delayWriteMs">
                    </div>
                  </div>
                </div>
                <div class="col-8" v-if="tcpListener.file">
                  <div class="mb-0 row">
                    <label for="fileLabel" class="col-sm-3 col-form-label small text-truncate padding-0">File</label>
                    <div class="col-sm-9">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="fileLabel"
                        :value="tcpListener.data">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="tcpListener.data">
                    <label for="dataLabel" class="col-sm-3 col-form-label small text-truncate padding-0">Data</label>
                    <div class="col-sm-9 small">{{ tcpListener.file }}</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="col-12">
          &nbsp;
        </div>
        <!--
          Show the http servers
          TODO: Move this to a separate component.
        -->
        <div class="col-12">
          <h5>Http servers
            <div class="btn-group btn-group-sm align-middle small me-2" role="group">
              <button type="button" class="btn btn-sm btn-outline-primary" @click="addHttpServer()"><i
                  class="fa-solid fa-plus"></i></button>
            </div>
          </h5>
        </div>
        <div class="card" v-for="httpServer in httpServers" :key="httpServer.port">
          <div class="card-header">
            Http server: {{ httpServer.name }}
          </div>
          <div class="card-body">
            <div class="btn-toolbar" role="toolbar"
              aria-label="Toolbar with button groups margin-0 padding-0 align-middle">
              <div class="btn-group btn-group-sm align-middle small me-2 margin-0 padding-0 button-position-right"
                role="group">
                <button type="button" class="btn btn-sm btn-outline-primary align-middle"
                  @click="editHttpServer(httpServer)" data-bs-toggle="modal" data-bs-target="#idEditHttpServerModel"><i
                    class="fa-solid fa-file-pen"></i></button>
                <button class="btn btn-sm btn-outline-danger align-middle"
                  @click="confirmDeleteHttpServer(httpServer.id)"><i class="fa-solid fa-trash"></i></button>
              </div>
            </div>
            <div class="container-fluid">
              <div class="row">
                <div class="col-4">
                  <div class="mb-0 row">
                    <label for="idLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Id</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="idLabel"
                        :value="httpServer.id">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="nameLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Name</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="nameLabel"
                        :value="httpServer.name">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="httpServer.httpPort">
                    <label for="httpPortLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Http
                      port</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="httpPortLabel"
                        :value="httpServer.httpPort">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="descriptionLabel"
                      class="col-sm-6 col-form-label small text-truncate padding-0">Description</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="descriptionLabel"
                        :value="httpServer.description">
                    </div>
                  </div>
                </div>
                <div class="col-4">
                  <div class="mb-0 row" v-if="httpServer?.httpsConfig?.httpsPort">
                    <label for="httpsPortLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Https
                      port</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="httpsPortLabel"
                        :value="httpServer?.httpsConfig?.httpsPort">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="httpServer?.httpsConfig?.serverCertificate">
                    <label for="serverCertLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Server
                      certificate</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="serverCertLabel"
                        :value="httpServer?.httpsConfig?.serverCertificate">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="httpServer?.httpsConfig?.privateKey">
                    <label for="privateKeyLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Private
                      key</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="privateKeyLabel"
                        :value="httpServer?.httpsConfig?.privateKey">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="httpServer?.httpsConfig?.clientCertificate">
                    <label for="clientCertLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Client
                      certificate</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="clientCertLabel"
                        :value="httpServer?.httpsConfig?.clientCertificate">
                    </div>
                  </div>
                </div>
                <div class="col-4">
                  <div class="mb-0" v-if="httpServer?.httpsConfig?.supportedTlsVersions">
                    <label for="tlsVersionsLabel"
                      class="col-sm-6 col-form-label small text-truncate padding-0">Supported TLS versions</label>
                    <div class="col-sm-6">
                      <ul class="list-unstyled">
                        <li v-for="tlsVersion in httpServer?.httpsConfig?.supportedTlsVersions" class="small">{{
                          tlsVersion }}</li>
                      </ul>
                    </div>
                  </div>
                </div>
                <!--
                Show the endpoints for the http server
                TODO: Move this to a separate component.
                -->
                <div class="col-12">
                  <div class="container-fluid padding-0 margin-0">
                    <div class="row">
                      <div class="col-12">
                        <table class="table table-sm table-striped table-bordered caption-top">
                          <caption>
                            Endpoints
                            <div class="btn-group btn-group-sm align-middle small" role="group">
                              <button type="button" class="btn btn-sm btn-outline-primary"
                                @click="addEndpoint(httpServer)"><i class="fa-solid fa-plus"></i></button>
                            </div>
                          </caption>
                          <thead>
                            <tr>
                              <th scope="col">Path</th>
                              <th scope="col">Method</th>
                              <th scope="col"></th>
                              <th scope="col toolbar-col"></th>
                            </tr>
                          </thead>
                          <tbody>
                            <tr v-for="endpoint in httpServer.endpoints" :key="endpoint.id">
                              <td class="align-middle"><label class="align-middle small">{{ endpoint.pathExpression }}</label>
                              </td>
                              <td class="align-middle"><label class="align-middle small">{{ endpoint.method }}</label>
                              </td>
                              <td class="align-middle">
                                <div v-if="endpoint?.mock">
                                  <dl class="row">
                                    <dt class="col-3 small">Status</dt>
                                    <dd class="col-3 small">{{ endpoint.mock?.status }}</dd>
                                    <dt class="col-3 small">Delay</dt>
                                    <dd class="col-3 small">{{ endpoint.mock?.delay }}</dd>
                                    <dt class="col-3 small">Headers</dt>
                                    <dd class="col-3 small">
                                      <ul class="list-unstyled">
                                        <li v-for="(value, key) in endpoint.mock?.headers">{{ key }}: {{ value }}</li>
                                      </ul>
                                    </dd>                                    
                                    <dt class="col-3 small">Body</dt>
                                    <dd class="col-3 small">{{ endpoint.mock?.response }}</dd>
                                  </dl>
                                </div>
                                <div v-if="endpoint?.route">
                                  <dl class="row">
                                    <dt class="col-3 small">&nbsp;Url</dt>
                                    <dd class="col-9 small">{{ endpoint.route?.url }}</dd>
                                    <dt class="col-3 small">Proxy url</dt>
                                    <dd class="col-9 small">&nbsp;{{ endpoint.route?.proxyUrl }}</dd>
                                    <dt class="col-3 small">Http1 only</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.http1Only }}</dd>
                                    <dt class="col-3 small">Accept invalid certs</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.acceptInvalidCerts }}</dd>
                                    <dt class="col-3 small">Accept invalid hostnames</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.acceptInvalidHostnames }}</dd>
                                    <dt class="col-3 small"></dt>
                                    <dd class="col-3 small"></dd>
                                    <dt class="col-3 small">Min TLS version</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.minTlsVersion }}</dd>
                                    <dt class="col-3 small">Max TLS version</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.maxTlsVersion }}</dd>
                                    <dt class="col-3 small">Read timeout</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.readTimeout }}</dd>
                                    <dt class="col-3 small">Connect timeout</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.connectTimeout }}</dd>
                                  </dl>
                                </div>
                              </td>
                              <td class="align-middle toolbar-col">
                                <span class="align-middle">
                                  <div class="btn-toolbar" role="toolbar" aria-label="Toolbar with button groups">
                                    <div class="btn-group btn-group-sm align-middle small me-2" role="group">
                                      <button type="button" class="btn btn-sm btn-outline-primary text-decoration-none"
                                        @click="editEndpoint(httpServer, endpoint)" data-bs-toggle="modal"
                                        data-bs-target="#idEditEndpointModel"><i
                                          class="fa-solid fa-pen-to-square"></i></button>
                                    </div>
                                    <div class="btn-group btn-group-sm align-middle small me-2" role="group">
                                      <button class="btn btn-sm btn-outline-danger text-decoration-none"
                                        @click="confirmDeleteEndpoint(httpServer, endpoint)"><i
                                          class="fa-solid fa-minus"></i></button>
                                    </div>
                                  </div>
                                </span>
                              </td>
                            </tr>
                          </tbody>
                        </table>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <!--
    Edit modals for editing the tcp listener.
    TODO: Move this to a separate component.
  -->  
  <div class="modal fade" id="idEditTcpListenerModel" tabindex="-1" aria-labelledby="editTcpListenerLabel"
    aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h6 class="modal-title fs-5 small" id="editTcpListenerLabel">Edit Tcp listener</h6>
        </div>
        <div class="modal-body">
          <form class="row g-3">
            <div class="col-md-4">
              <label for="idEditTcpListenerId" class="form-label small">Id</label>
              <label class="form-control form-control-sm" id="idEditTcpListenerId" readonly>{{ editTcpListenerData.id
                }}</label>
            </div>
            <div class="col-md-4">
              <label for="idEditPort" class="form-label small">Port</label>
              <input type="text" class="form-control form-control-sm" id="idEditPort"
                v-model="editTcpListenerData.port">
            </div>
            <div class="col-md-4">
              <label for="idEditAccept" class="form-label small">Accept</label>
              <div class="form-check">
                <input type="checkbox" class="form-check-input" id="idEditAccept"
                  v-model="editTcpListenerData.accept" />
              </div>
            </div>
            <div class="col-md-6">
              <label for="idEditDelayedWrite" class="form-label small">Delayed write ms</label>
              <input type="text" class="form-control form-control-sm" id="idEditDelayedWrite"
                v-model="editTcpListenerData.delayWriteMs">
            </div>
            <div class="col-md-6">
              <label for="idEditCloseConnection" class="form-label small">Close connection&nbsp;</label>              
              <select id="idEditCloseConnection" class="form-select form-select-sm"
                v-model="editTcpListenerData.closeConnection">
                <option value="AfterRead">AfterRead</option>
                <option value="AfterResponse">AfterResponse</option>               
                <option value="BeforeRead">BeforeRead</option>>
                <option value="Never">Never</option>
              </select>                            
            </div>
            <div class="col-md-12">
              <label for="idEditFile" class="form-label small">File</label>
              <input type="text" class="form-control form-control-sm" id="idEditFile"
                v-model="editTcpListenerData.file">
            </div>
            <div class="col-md-12">
              <label for="idEditData" class="form-label small">Response</label>
              <textarea class="form-control form-control-sm" id="idEditData" rows="6"
                v-model="editTcpListenerData.data"></textarea>
            </div>
          </form>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-sm btn-secondary" data-bs-dismiss="modal"
            data-bs-target="#idEditTcpListenerModel">Cancel</button>
          <button type="button" class="btn btn-sm btn-primary"
            @click="updateTcpListener(editTcpListenerData)">Ok</button>
        </div>
      </div>
    </div>
  </div>
  <!--
    Edit modals for editing the http server.
    TODO: Move this to a separate component.
  -->  
  <div class="modal fade" id="idEditHttpServerModel" tabindex="-1" aria-labelledby="editHttpServerLabel"
    aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h6 class="modal-title fs-5 small" id="editHttpServerLabel">Edit Http server</h6>
        </div>
        <div class="modal-body">
          <form class="row g-3">
            <div class="col-md-6">
              <label for="idEditHttpServerId" class="form-label small">Id</label>
              <label class="form-control form-control-sm" id="idEditHttpServerId" readonly>{{ editHttpServerData.id
                }}</label>
            </div>
            <div class="col-md-6">
              <label for="idEditName" class="form-label small">Name</label>
              <input type="text" class="form-control form-control-sm" id="idEditName" v-model="editHttpServerData.name">
            </div>
            <div class="col-md-12">
              <label for="idEditHttpPort" class="form-label small">Http port</label>
              <input type="text" class="form-control form-control-sm" id="idEditHttpPort"
                v-model="editHttpServerData.httpPort">
            </div>
            <div class="col-md-6">
              <div class="form-check">
                <input type="checkbox" class="form-check-input" id="idEditHttpsConfig" v-model="showEditHttpsConfig" />
                <label for="idEditHttpsConfig" class="form-check-label small">Https config</label>
              </div>
            </div>
            <div class="col-md-6" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditHttpsPort" class="form-label small">Https port</label>
                <input type="text" class="form-control form-control-sm" id="idEditHttpsPort"
                  v-model="editHttpsConfig.httpsPort">
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditServerCertificate" class="form-label small">Server certificate</label>
                <input type="text" class="form-control form-control-sm" id="idEditServerCertificate"
                  v-model="editHttpsConfig.serverCertificate">
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditPrivateKey" class="form-label small">Private key</label>
                <input type="text" class="form-control form-control-sm" id="idEditPrivateKey"
                  v-model="editHttpsConfig.privateKey">
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditClientCertificate" class="form-label small">Client certificate</label>
                <input type="text" class="form-control form-control-sm" id="idEditClientCertificate"
                  v-model="editHttpsConfig.clientCertificate">
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <input type="checkbox" id="idTLSv1_0" value="TLSv1.0" v-model="editSupportedTlsVersions" />
                <label for="idTLSv1_0">&nbsp;TLSv1_0</label>
                <input type="checkbox" id="idTLSv1_1" value="TLSv1.1" v-model="editSupportedTlsVersions" />
                <label for="idTLSv1_1">&nbsp;TLSv1_1</label>
                <input type="checkbox" id="idTLSv1_2" value="TLSv1.2" v-model="editSupportedTlsVersions" />
                <label for="idTLSv1_2">&nbsp;TLSv1_2</label>
                <input type="checkbox" id="idTLSv1_3" value="TLSv1.3" v-model="editSupportedTlsVersions" />
                <label for="idTLSv1_3">&nbsp;TLSv1_3</label>
              </div>
            </div>
          </form>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-sm btn-secondary" data-bs-toggle="modal"
            data-bs-target="#idEditHttpServerModel">Cancel</button>
          <button type="button" class="btn btn-sm btn-primary"
            @click="updateHttpServer(editHttpServerData, editHttpsConfig, editSupportedTlsVersions)">Ok</button>
        </div>
      </div>
    </div>
  </div>
  <!--
    Edit modals for editing the endpoint.
    TODO: Move this to a separate component.
  -->  
  <div class="modal modal-lg fade" id="idEditEndpointModel" tabindex="-1" aria-labelledby="editEndpointLabel"
    aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h6 class="modal-title fs-5 small" id="editEndpointLabel">Edit endpoint</h6>
        </div>
        <div class="modal-body">
          <form class="row g-3">
            <div class="col-md-2">
              <div class="form-check">
                <input class="form-check-input" type="checkbox" id="idEditShowEditMockData" v-model="showEditMockData">
                <label class="form-check-label" for="idEditShowEditMockData">Mock</label>
              </div>              
            </div>
            <div class="col-md-4">
              <label for="idEditMethod" class="form-label small">Method&nbsp;</label>              
              <select id="idEditMethod" class="form-select form-select-sm"
                v-model="editEndpointData.method">
                <option value="DELETE">Delete</option>
                <option value="GET">Get</option>
                <option value="HEAD">Head</option>
                <option value="POST">Post</option>
                <option value="PUT">Put</option>
                <option value="OPTION">Option</option>
              </select>                            
            </div>             
            <div class="col-md-6">
              <label for="idEditPathExpression" class="form-label small">Path expression</label>
              <input type="text" class="form-control form-control-sm" id="idEditPathExpression"
                v-model="editEndpointData.pathExpression">
            </div>
            <div class="col-md-6" v-if="showEditMockData">
              <label for="idEditStatus" class="form-label small">Status</label>
              <input type="text" class="form-control form-control-sm" id="idEditStatus"
                v-model="editMockData.status">
            </div>                                                     
            <div class="col-md-6" v-if="showEditMockData">              
              <label for="idEditDelay" class="form-label small">Delay</label>
                <input type="text" class="form-control form-control-sm" id="idEditDelay"
                  v-model="editMockData.delay">
            </div>
            <div class="col-md-12" v-if="showEditMockData">
              <label for="idEditResponse" class="form-label small">Response</label>
              <textarea type="text" class="form-control form-control-sm" id="idEditResponse"
                v-model="editMockData.response"></textarea>
            </div>  
            <div class="col-md-6" v-if="!showEditMockData">              
              <label for="idEditUrl" class="form-label small">Url</label>
                <input type="text" class="form-control form-control-sm" id="idEditUrl"
                  v-model="editRouteData.url">
            </div>
            <div class="col-md-6" v-if="!showEditMockData">              
              <label for="idEditProxyUrl" class="form-label small">Proxy url</label>
                <input type="text" class="form-control form-control-sm" id="idEditProxyUrl"
                  v-model="editRouteData.proxyUrl">
            </div>           
            <div class="col-md-6" v-if="!showEditMockData">  
              <div class="form-check">
                <input class="form-check-input" type="checkbox" id="idEditHttp1Only" v-model="editRouteData.http1Only">
                <label class="form-check-label" for="idEditHttp1Only">Http1 only</label><br>                
              </div>
            </div>   
            <div class="col-md-6" v-if="!showEditMockData">  
              <div class="form-check">
                <input class="form-check-input" type="checkbox" id="idEditAcceptInvalidCerts" v-model="editRouteData.acceptInvalidCerts">
                <label class="form-check-label" for="idEditAcceptInvalidCerts">Accept invalid certs</label>
              </div>
            </div>   
            <div class="col-md-6" v-if="!showEditMockData">  
              <div class="form-check">
                <input class="form-check-input" type="checkbox" id="idEditAcceptInvalidHostnames" v-model="editRouteData.acceptInvalidHostnames">
                <label class="form-check-label" for="idEditAcceptInvalidHostnames">Accept invalid hostnames</label>
              </div>
            </div>  
            <div class="col-md-6" v-if="!showEditMockData">  
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <label for="idEditMinTlsVersion" class="form-label small">Min Tls version&nbsp;</label>              
              <select id="idEditMinTlsVersion" class="form-select form-select-sm"
                v-model="editRouteData.minTlsVersion">
                <option value="TLSv1.0">TLSv1.0</option>
                <option value="TLSv1.1">TLSv1.1</option>            
                <option value="TLSv1.2">TLSv1.2</option>
                <option value="TLSv1.3">TLSv1.3</option>
              </select>                            
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <label for="idEditMaxTlsVersion" class="form-label small">Max Tls version&nbsp;</label>              
              <select id="idEditMaxTlsVersion" class="form-select form-select-sm"
                v-model="editRouteData.maxTlsVersion">
                <option value="TLSv1.0">TLSv1.0</option>
                <option value="TLSv1.1">TLSv1.1</option>            
                <option value="TLSv1.2">TLSv1.2</option>
                <option value="TLSv1.3">TLSv1.3</option>
              </select>                            
            </div>                           
            <div class="col-md-6" v-if="!showEditMockData">  
              <label class="form-label small" for="idEditReadTimeout">Read timeout</label>
              <input class="form-control form-control-sm" type="text" id="idEditReadTimeout" v-model="editRouteData.readTimeout">
            </div>  
            <div class="col-md-6" v-if="!showEditMockData">  
              <label class="form-label small" for="idEditConnectTimeout">Connect timeout</label>
              <input class="form-control form-control-sm" type="test" id="idEditConnectTimeout" v-model="editRouteData.connectTimeout">
            </div>                                                
          </form>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-sm btn-secondary" data-bs-toggle="modal"
            data-bs-target="#idEditEndpointModel">Cancel</button>
          <button type="button" class="btn btn-sm btn-primary"
            @click="updateEndpoint(editEndpointData)">Ok</button>
        </div>
      </div>
    </div>
  </div>
</template>