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

//Set the data file for the tcp listener. This is called when the user clicks the select file button.
//The file is set on the editTcpListenerData object.
const setEditTcpListenerDataFile = () => {
  invoke("open_dialog", { name: null, extension: null })
    .then((message) => {
      if (message) {
        editTcpListenerData.value.file = message;
      }
    })
    .catch((error) => window.alert(error));
}

//Clear the data file for the tcp listener. This is called when the user clicks the clear button.
const clearEditTcpListenerDataFile = () => {
  editTcpListenerData.value.file = null;
}

//Set the private key for the https config. This is called when the user clicks the select file button.
const setEditHttpsConfigPrivateKey = () => {
  invoke("open_dialog", { name: "Pem file", extension: "pem" })
    .then((message) => {
      if (message) {
        editHttpsConfig.value.privateKey = message;
      }
    })
    .catch((error) => window.alert(error));
}

//Clear the server certificate for the https config. This is called when the user clicks the clear button.
const clearEditHttpsConfigPrivateKey = () => {
  editHttpsConfig.value.privateKey = null;
}

//Set the server certificate for the https config. This is called when the user clicks the select file button.
const setEditHttpsConfigServerCertificate = () => {
  invoke("open_dialog", { name: "Pem file", extension: "pem" })
    .then((message) => {
      if (message) {
        editHttpsConfig.value.serverCertificate = message;
      }
    })
    .catch((error) => window.alert(error));
}

//Clear the server certificate for the https config. This is called when the user clicks the clear button.
const clearEditHttpsConfigServerCertificate = () => {
  editHttpsConfig.value.serverCertificate = null;
}

//Set the client certificate for the https config. This is called when the user clicks the select file button.
const setEditHttpsConfigClientCertificate = () => {
  invoke("open_dialog", { name: "Pem file", extension: "pem" })
    .then((message) => {
      if (message) {
        editHttpsConfig.value.clientCertificate = message;
      }
    })
    .catch((error) => window.alert(error));
}

//Clear the server certificate for the https config. This is called when the user clicks the clear button.
const clearEditHttpsConfigClientCertificate = () => {
  editHttpsConfig.value.clientCertificate = null;
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
  console.log("showEditMockData:" + showEditMockData.value);
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
    headers: mockData.value.headers,
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

//Verify that string input is filled out. 
const validateStringRequired = (str) => {
  if (str && str.length > 0) {
    return "is-valid";
  }
  return "is-invalid";
}

//Verify that input is a number. 
const validateNumberRequired = (str) => {
  if (str && (Number.isInteger(str) || (str.length > 0 && !isNaN(str)))) {
    return "is-valid";
  }
  return "is-invalid";
}

//Verify that input is a number or null. 
const validateNumberOptional = (str) => {
  if (!str || (Number.isInteger(str) || (str.length > 0 && !isNaN(str)))) {
    return "is-valid";
  }
  return "is-invalid";
}
</script>
<style scoped>
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

.accordion-header .accordion-button {
  box-shadow: rgba(50, 50, 93, 0.25) 0px 50px 100px -20px, rgba(0, 0, 0, 0.3) 0px 30px 60px -30px, rgba(10, 37, 64, 0.35) 0px -2px 6px 0px inset;
}

.accordion-body {
  background-color: #e0e0e0 !important;
}

.accordion-button:focus {
  box-shadow: rgba(50, 50, 93, 0.25) 0px 50px 100px -20px, rgba(0, 0, 0, 0.3) 0px 30px 60px -30px, rgba(10, 37, 64, 0.35) 0px -2px 6px 0px inset;
}

.accordion-item {
  padding: 3px;
  background: rgb(10, 0, 100);
  border: 0px;
}

.btn-accordion-buttons {
  position: absolute;
  right: 50px;
}

</style>
<template>
  <!--
     Show navigation bar. 
     TODO: Move this to a separate component.  
  -->
  <nav class="navbar navbar-expand-sm bg-primary-subtle small">
    <div class="container-fluid">
      <ol class="breadcrumb  margin-0 padding-0 align-middle">
        <li class="breadcrumb-item"><router-link to="/"><i class="fas fa-house"></i></router-link></li>
        <li class="breadcrumb-item">{{ test?.name }}</li>
      </ol>
      <div class="btn-group btn-group-sm align-middle small me-2" role="group">
        <button type="button" class="btn btn-sm btn-outline-primary" @click="addHttpServer()"><i
            class="fa-solid fa-plus">
          </i>&nbsp;Add http server</button>
        <button type="button" class="btn btn-sm btn-outline-primary " @click="addTcpListener()"><i
            class="fa-solid fa-plus"></i>&nbsp;Add tcp listener</button>
      </div>
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
        <dl class="row padding-0 margin-0">
          <dt class="col-sm-3 small padding-0 margin-0 text-light">Id</dt>
          <dd class="col-sm-9 small padding-0 margin-0 text-light">{{ test?.id }}</dd>
          <dt class="col-sm-3 small padding-0 margin-0 text-light">Name</dt>
          <dd class="col-sm-9 small padding-0 margin-0 text-light">{{ test?.name }}</dd>
          <dt class="col-sm-3 small padding-0 margin-0 text-light">Description</dt>
          <dd class="col-sm-9 small padding-0 margin-0 text-light">{{ test?.description }}</dd>
        </dl>
      </div>
      <div class="col-12">&nbsp;</div>
      <!--
        Show the tcp listeners
        TODO: Move this to a separate component.
      -->
      <div class="col-12">
        <div class="accordion accordion-flush" id="accordionTcpListener">
          <div class="accordion-item" v-for="tcpListener in tcpListeners" :key="tcpListener.id"
            v-if="tcpListeners?.length > 0">
            <h6 class="accordion-header" :id="'tcpListenerHeader' + tcpListener.id">
              <button class="accordion-button collapsed bg-primary text-light" type="button" data-bs-toggle="collapse"
                :data-bs-target="'#tcpListener' + tcpListener.id" aria-expanded="false"
                :aria-controls="'tcpListener' + tcpListener.id">
                <i class="fa-solid fa-ear-listen"></i>&nbsp;Tcp listener for port {{ tcpListener.port }}
              </button>
            </h6>
            <div :id="'tcpListener' + tcpListener.id" class="accordion-collapse collapse bg-body-secondary"
              :aria-labelledby="'tcpListener' + tcpListener.id" data-bs-parent="#accordionTcpListener">
              <div class="accordion-body">
                <div class="btn-accordion-buttons" role="toolbar"
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
                <div class="container-fluid margin-0 padding-0">
                  <div class="row">
                    <div class="col-2">
                      <dl class="row margin-0 padding-0">
                        <dt class="col-sm-8 small">Id</dt>
                        <dd class="col-sm-4 small">{{ tcpListener.id }}</dd>
                        <dt class="col-sm-8 small">Port</dt>
                        <dd class="col-sm-4 small">{{ tcpListener.port }}</dd>
                        <dt class="col-sm-8 small">Accept</dt>
                        <dd class="col-sm-4 small">{{ tcpListener.accept }}</dd>
                        <dt class="col-sm-8 small">Close</dt>
                        <dd class="col-sm-4 small">{{ tcpListener.closeConnection }}</dd>
                        <dt class="col-sm-8 small">Delayed write response</dt>
                        <dd class="col-sm-4 small">{{ tcpListener.delayWriteMs }}</dd>
                      </dl>
                    </div>
                    <div class="col-10">
                      <dl class="row margin-0 padding-0">
                        <dt class="col-sm-1 small">File</dt>
                        <dd class="col-sm-11 small">{{ tcpListener.file }}</dd>
                        <dt class="col-sm-1 small">Data</dt>
                        <dd class="col-sm-11 small"
                          style="max-height: 8pc; overflow-y: scroll; max-width: calc(100% - 0px);">
                          <pre>{{ tcpListener.data }}</pre>
                        </dd>
                      </dl>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <!--
          Show the http servers
          TODO: Move this to a separate component.
        -->
        <div class="accordion accordion-flush" id="accordionHttpServer">
          <div class="accordion-item" v-for="httpServer in httpServers" :key="httpServer.id" v-if="httpServers?.length > 0">
            <h6 class="accordion-header" :id="'httpServerHeader' + httpServer.id">
              <button class="accordion-button collapsed bg-primary text-light" type="button" data-bs-toggle="collapse"
                :data-bs-target="'#httpServer' + httpServer.id" aria-expanded="false"
                :aria-controls="'httpServer' + httpServer.id">
                <i class="fa-solid fa-cloud"></i>&nbsp;Http server: {{ httpServer.name }}
              </button>
            </h6>
            <div :id="'httpServer' + httpServer.id" class="accordion-collapse collapse"
              :aria-labelledby="'httpServer' + httpServer.id" data-bs-parent="#accordionHttpServer">
              <div class="accordion-body">
                <div class="btn-accordion-buttons" role="toolbar"
                  aria-label="Toolbar with button groups margin-0 padding-0 align-middle">
                  <div class="btn-group btn-group-sm align-middle small me-2 margin-0 padding-0 button-position-right"
                    role="group">
                    <button type="button" class="btn btn-sm btn-outline-primary align-middle"
                      @click="editHttpServer(httpServer)" data-bs-toggle="modal"
                      data-bs-target="#idEditHttpServerModel"><i class="fa-solid fa-file-pen"></i></button>
                    <button class="btn btn-sm btn-outline-danger align-middle"
                      @click="confirmDeleteHttpServer(httpServer.id)"><i class="fa-solid fa-trash"></i></button>
                  </div>
                </div>
                <div class="container-fluid margin-0 padding-0">
                  <div class="row">
                    <div class="col-12">
                      <dl class="row margin-0 padding-0">
                        <dt class="col-sm-2 small">Id</dt>
                        <dd class="col-sm-4 small">{{ httpServer.id }}</dd>
                        <dt class="col-sm-2 small">Name</dt>
                        <dd class="col-sm-4 small">{{ httpServer.name }}</dd>
                        <dt class="col-sm-2 small">Http port</dt>
                        <dd class="col-sm-4 small">{{ httpServer?.httpPort }}</dd>
                        <dt class="col-sm-2 small">Description</dt>
                        <dd class="col-sm-4 small">{{ httpServer.description }}</dd>
                      </dl>
                      <dl class="row margin-0 padding-0" v-if="httpServer?.httpsConfig">
                        <dt class="col-sm-2 small">Https port</dt>
                        <dd class="col-sm-4 small">{{ httpServer?.httpsConfig?.httpsPort }}</dd>
                        <dt class="col-sm-2 small">Server certificate</dt>
                        <dd class="col-sm-4 small">{{ httpServer?.httpsConfig?.serverCertificate }}</dd>
                        <dt class="col-sm-2 small">Private key</dt>
                        <dd class="col-sm-4 small">{{ httpServer?.httpsConfig?.privateKey }}</dd>
                        <dt class="col-sm-2 small">Client certificate</dt>
                        <dd class="col-sm-4 small">{{ httpServer?.httpsConfig?.clientCertificate }}</dd>
                      </dl>
                      <dl class="row margin-0 padding-0" v-if="httpServer?.httpsConfig">
                        <dt class="col-sm-2 small">Supported TLS versions</dt>
                        <dd class="col-sm-4 small">
                          <label v-for="tlsVersion in httpServer?.httpsConfig?.supportedTlsVersions" class="small">{{
                            tlsVersion }}&nbsp;</label>
                        </dd>
                      </dl>
                    </div>
                    <div class="col-12">
                      <table class="table table-sm table-striped table-bordered caption-top margin-0 padding-0">
                        <caption>
                          <label>Endpoints&nbsp;</label>
                          <div class="btn-group btn-group-sm align-middle small" role="group">
                            <button type="button" class="btn btn-sm btn-outline-primary"
                              @click="addEndpoint(httpServer)"><i class="fa-solid fa-plus"></i>&nbsp;Add endpoint</button>
                          </div>
                        </caption>
                        <thead>
                          <tr>
                            <th class="bg-primary-subtle small">Path</th>
                            <th class="col bg-primary-subtle small">Method</th>
                            <th class="col bg-primary-subtle small"></th>
                            <th class="col bg-primary-subtle toolbar-col small"></th>
                          </tr>
                        </thead>
                        <tbody>
                          <tr v-for="endpoint in httpServer.endpoints" :key="endpoint.id">
                            <td class="align-middle"><label class="align-middle small">{{ endpoint.pathExpression
                            }}</label>
                            </td>
                            <td class="align-middle"><label class="align-middle small">{{ endpoint.method }}</label>
                            </td>
                            <td class="align-middle">
                              <div v-if="endpoint?.mock">
                                <table class="table table-sm table-striped margin-0">
                                  <thead>
                                    <tr>
                                      <th class="bg-primary-subtle small">Status</th>
                                      <th class="bg-primary-subtle small">Delay</th>
                                      <th class="bg-primary-subtle small">Headers</th>
                                      <th class="bg-primary-subtle small">Body</th>
                                    </tr>
                                  </thead>
                                  <tbody>
                                    <tr>
                                      <td class="text-truncate small">{{ endpoint.mock?.status }}</td>
                                      <td class="text-truncate small">{{ endpoint.mock?.delay }}</td>
                                      <td class="text-truncate small">
                                        <pre class="margin-0 padding-0"
                                          style="min-width: 100%; max-height: 8pc; overflow-y: scroll; max-width: calc(100% - 150px);">{{ endpoint.mock?.headers }}</pre>
                                      </td>
                                      <td class="text-truncate small">
                                        <pre class="margin-0 padding-0"
                                          style="min-width: 100%; max-height: 8pc; overflow-y: scroll; max-width: calc(100% - 150px);">{{ endpoint.mock?.response }}</pre>
                                      </td>
                                    </tr>
                                  </tbody>
                                </table>
                              </div>
                              <div v-if="endpoint?.route">
                                <table class="table table-sm table-striped margin-0">
                                  <thead>
                                    <tr>
                                      <th class="bg-primary-subtle small">Url</th>
                                      <th class="bg-primary-subtle small">Proxy url</th>
                                      <th class="bg-primary-subtle small">Http1 only</th>
                                      <th class="bg-primary-subtle small">Accept invalid certs</th>
                                      <th class="bg-primary-subtle small">Accept invalid hostnames</th>
                                      <th class="bg-primary-subtle small">Min TLS version</th>
                                      <th class="bg-primary-subtle small">Max TLS version</th>
                                      <th class="bg-primary-subtle small">Read timeout</th>
                                      <th class="bg-primary-subtle small">Connect timeout</th>
                                    </tr>
                                  </thead>
                                  <tbody>
                                    <tr>
                                      <td class="text-truncate small">{{ endpoint.route?.url }}</td>
                                      <td class="text-truncate small">{{ endpoint.route?.proxyUrl }}</td>
                                      <td class="text-truncate small">{{ endpoint.route?.http1Only }}</td>
                                      <td class="text-truncate small">{{ endpoint.route?.acceptInvalidCerts }}</td>
                                      <td class="text-truncate small">{{ endpoint.route?.acceptInvalidHostnames }}</td>
                                      <td class="text-truncate small">{{ endpoint.route?.minTlsVersion }}</td>
                                      <td class="text-truncate small">{{ endpoint.route?.maxTlsVersion }}</td>
                                      <td class="text-truncate small">{{ endpoint.route?.readTimeout }}</td>
                                      <td class="text-truncate small">{{ endpoint.route?.connectTimeout }}</td>
                                    </tr>
                                  </tbody>
                                </table>
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
  <!--
    Edit modals for editing the tcp listener.
    TODO: Move this to a separate component.
  -->
  <div class="modal fade" id="idEditTcpListenerModel" tabindex="-1" aria-labelledby="editTcpListenerLabel"
    aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header bg-primary">
          <h6 class="modal-title fs-5 small" id="editTcpListenerLabel"><i class="fa-solid fa-pen-to-square"></i>Edit Tcp listener</h6>
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
              <input type="text" class="form-control form-control-sm" id="idEditPort" v-model="editTcpListenerData.port"
                :class="validateNumberRequired(editTcpListenerData.port)">
            </div>
            <div class="col-md-4">
              <label for="idEditAccept" class="form-label small">Accept</label>
              <div class="form-check">
                <input type="checkbox" class="form-check-input is-valid" id="idEditAccept"
                  v-model="editTcpListenerData.accept" />
              </div>
            </div>
            <div class="col-md-6">
              <label for="idEditDelayedWrite" class="form-label small">Delayed write ms</label>
              <input type="text" class="form-control form-control-sm" id="idEditDelayedWrite"
                v-model="editTcpListenerData.delayWriteMs"
                :class="validateNumberOptional(editTcpListenerData.delayWriteMs)">
            </div>
            <div class="col-md-6">
              <label for="idEditCloseConnection" class="form-label small">Close connection&nbsp;</label>
              <select id="idEditCloseConnection" class="form-select form-select-sm"
                :class="validateStringRequired(editTcpListenerData.closeConnection)"
                v-model="editTcpListenerData.closeConnection">
                <option value="AfterRead">AfterRead</option>
                <option value="AfterResponse">AfterResponse</option>
                <option value="BeforeRead">BeforeRead</option>>
                <option value="Never">Never</option>
              </select>
            </div>
            <div class="col-md-12">
              <label for="idEditFile" class="form-label small">File</label>
              <div class="input-group mb-3">
                <input type="text" readonly class="form-control form-control-sm is-valid" id="idEditFile"
                  v-model="editTcpListenerData.file">
                <button class="btn btn-sm btn-outline-primary" type="button" id="idOpenTcpListenerFile"
                  @click="setEditTcpListenerDataFile()"><i class="fa-solid fa-check"></i>Select file</button>
                <button class="btn btn-sm btn-outline-danger" type="button" id="idOpenTcpListenerFile"
                  @click="clearEditTcpListenerDataFile()"><i class="fa-solid fa-broom"></i>&nbsp;Clear</button>
              </div>
            </div>
            <div class="col-md-12">
              <label for="idEditData" class="form-label small">Response</label>
              <textarea class="form-control form-control-sm is-valid" id="idEditData" rows="6"
                v-model="editTcpListenerData.data"></textarea>
            </div>
          </form>
        </div>
        <div class="modal-footer bg-primary-subtle">
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
        <div class="modal-header bg-primary">
          <h6 class="modal-title fs-5 small" id="editHttpServerLabel"><i class="fa-solid fa-pen-to-square"></i>Edit Http server</h6>
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
              <input type="text" class="form-control form-control-sm" id="idEditName" v-model="editHttpServerData.name"
                :class="validateStringRequired(editHttpServerData.name)">
            </div>
            <div class="col-md-12">
              <label for="idEditHttpPort" class="form-label small">Http port</label>
              <input type="text" class="form-control form-control-sm" id="idEditHttpPort"
                v-model="editHttpServerData.httpPort" :class="validateNumberOptional(editHttpServerData.httpPort)">
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
                  v-model="editHttpsConfig.httpsPort" :class="validateNumberRequired(editHttpsConfig.httpsPort)">
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditServerCertificate" class="form-label small">Server certificate</label>
                <div class="input-group mb-3">
                  <input type="text" readonly class="form-control form-control-sm" id="idEditServerCertificate"
                    v-model="editHttpsConfig.serverCertificate"
                    :class="validateStringRequired(editHttpsConfig.serverCertificate)">
                  <button class="btn btn-sm btn-outline-primary" type="button" id="idOpenServerCertificateFile"
                    @click="setEditHttpsConfigServerCertificate()"><i class="fa-solid fa-file"></i>&nbsp;Select file</button>
                  <button class="btn btn-sm btn-outline-danger" type="button" id="idClearServerCertificateFile"
                    @click="clearEditHttpsConfigServerCertificate()"><i class="fa-solid fa-broom"></i>&nbsp;Clear</button>
                </div>
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditPrivateKey" class="form-label small">Private key</label>
                <div class="input-group mb-3">
                  <input type="text" readonly class="form-control form-control-sm" id="idEditPrivateKey"
                    v-model="editHttpsConfig.privateKey" :class="validateStringRequired(editHttpsConfig.privateKey)">
                  <button class="btn btn-sm btn-outline-primary" type="button" id="idOpenPrivateKeyFile"
                    @click="setEditHttpsConfigPrivateKey()"><i class="fa-solid fa-file"></i>&nbsp;Select file</button>
                  <button class="btn btn-sm btn-outline-danger" type="button" id="idClearPrivateKeyFile"
                    @click="clearEditHttpsConfigPrivateKey()"><i class="fa-solid fa-broom"></i>&nbsp;Clear</button>
                </div>
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditClientCertificate" class="form-label small">Client certificate</label>
                <div class="input-group mb-3">
                  <input type="text" readonly class="form-control form-control-sm is-valid" id="idEditClientCertificate"
                    v-model="editHttpsConfig.clientCertificate">
                  <button class="btn btn-sm btn-outline-primary" type="button" id="idOpenPrivateKeyFile"
                    @click="setEditHttpsConfigClientCertificate()"><i class="fa-solid fa-file"></i>&nbsp;Select file</button>
                  <button class="btn btn-sm btn-outline-danger" type="button" id="idClearPrivateKeyFile"
                    @click="clearEditHttpsConfigClientCertificate()"><i class="fa-solid fa-broom"></i>&nbsp;Clear</button>
                </div>
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check form-check-inline">
                <input type="checkbox" id="idTLSv1_0" value="TLSv1.0" v-model="editSupportedTlsVersions"
                  class="is-valid form-check-input" />
                <label for="idTLSv1_0" class="form-check-label">&nbsp;TLSv1_0</label>
              </div>
              <div class="form-check form-check-inline">
                <input type="checkbox" id="idTLSv1_1" value="TLSv1.1" v-model="editSupportedTlsVersions"
                  class="is-valid form-check-input" />
                <label for="idTLSv1_1" class="form-check-label">&nbsp;TLSv1_1</label>
              </div>
              <div class="form-check form-check-inline">
                <input type="checkbox" id="idTLSv1_2" value="TLSv1.2" v-model="editSupportedTlsVersions"
                  class="is-valid form-check-input" />
                <label for="idTLSv1_2" class="form-check-label">&nbsp;TLSv1_2</label>
              </div>
              <div class="form-check form-check-inline">
                <input type="checkbox" id="idTLSv1_3" value="TLSv1.3" v-model="editSupportedTlsVersions"
                  class="is-valid form-check-input" />
                <label for="idTLSv1_3" class="form-check-label">&nbsp;TLSv1_3</label>
              </div>
            </div>
          </form>
        </div>
        <div class="modal-footer bg-primary-subtle">
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
        <div class="modal-header bg-primary">
          <h6 class="modal-title fs-5 small" id="editEndpointLabel"><i class="fa-solid fa-pen-to-square"></i>Edit endpoint</h6>
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
                :class="validateStringRequired(editEndpointData.method)" v-model="editEndpointData.method">
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
                v-model="editEndpointData.pathExpression"
                :class="validateStringRequired(editEndpointData.pathExpression)">
            </div>
            <div class="col-md-6" v-if="showEditMockData">
              <label for="idEditStatus" class="form-label small">Status</label>
              <input type="text" class="form-control form-control-sm" id="idEditStatus" v-model="editMockData.status"
                :class="validateNumberRequired(editMockData.status)">
            </div>
            <div class="col-md-6" v-if="showEditMockData">
              <label for="idEditDelay" class="form-label small">Delay</label>
              <input type="text" class="form-control form-control-sm" id="idEditDelay" v-model="editMockData.delay"
                :class="validateNumberRequired(editMockData.delay)">
            </div>
            <div class="col-md-12" v-if="showEditMockData">
              <label for="idEditHeaders" class="form-label small">Headers</label>
              <textarea type="text" class="form-control form-control-sm is-valid" id="idEditHeaders"
                v-model="editMockData.headers" rows="6"></textarea>
            </div>
            <div class="col-md-12" v-if="showEditMockData">
              <label for="idEditResponse" class="form-label small">Response</label>
              <textarea type="text" class="form-control form-control-sm is-valid" id="idEditResponse"
                v-model="editMockData.response" rows="6"></textarea>
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <label for="idEditUrl" class="form-label small">Url</label>
              <input type="text" class="form-control form-control-sm" id="idEditUrl" v-model="editRouteData.url"
                :class="validateStringRequired(editRouteData.url)">
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <label for="idEditProxyUrl" class="form-label small">Proxy url</label>
              <input type="text" class="form-control form-control-sm is-valid" id="idEditProxyUrl"
                v-model="editRouteData.proxyUrl">
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <div class="form-check">
                <input class="form-check-input is-valid" type="checkbox" id="idEditHttp1Only"
                  v-model="editRouteData.http1Only">
                <label class="form-check-label" for="idEditHttp1Only">Http1 only</label><br>
              </div>
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <div class="form-check">
                <input class="form-check-input is-valid" type="checkbox" id="idEditAcceptInvalidCerts"
                  v-model="editRouteData.acceptInvalidCerts">
                <label class="form-check-label" for="idEditAcceptInvalidCerts">Accept invalid certs</label>
              </div>
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <div class="form-check">
                <input class="form-check-input is-valid" type="checkbox" id="idEditAcceptInvalidHostnames"
                  v-model="editRouteData.acceptInvalidHostnames">
                <label class="form-check-label" for="idEditAcceptInvalidHostnames">Accept invalid hostnames</label>
              </div>
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <label for="idEditMinTlsVersion" class="form-label small">Min Tls version&nbsp;</label>
              <select id="idEditMinTlsVersion" class="form-select form-select-sm is-valid"
                v-model="editRouteData.minTlsVersion">
                <option value="TLSv1.2">TLSv1.2</option>
                <option value="TLSv1.3">TLSv1.3</option>
              </select>
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <label for="idEditMaxTlsVersion" class="form-label small">Max Tls version&nbsp;</label>
              <select id="idEditMaxTlsVersion" class="form-select form-select-sm is-valid"
                v-model="editRouteData.maxTlsVersion">
                <option value="TLSv1.2">TLSv1.2</option>
                <option value="TLSv1.3">TLSv1.3</option>
              </select>
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <label class="form-label small" for="idEditReadTimeout">Read timeout</label>
              <input class="form-control form-control-sm" type="text" id="idEditReadTimeout"
                v-model="editRouteData.readTimeout" :class="validateNumberOptional(editRouteData.readTimeout)">
            </div>
            <div class="col-md-6" v-if="!showEditMockData">
              <label class="form-label small" for="idEditConnectTimeout">Connect timeout</label>
              <input class="form-control form-control-sm" type="test" id="idEditConnectTimeout"
                v-model="editRouteData.connectTimeout" :class="validateNumberOptional(editRouteData.connectTimeout)">
            </div>
          </form>
        </div>
        <div class="modal-footer bg-primary-subtle">
          <button type="button" class="btn btn-sm btn-secondary" data-bs-toggle="modal"
            data-bs-target="#idEditEndpointModel">Cancel</button>
          <button type="button" class="btn btn-sm btn-primary" @click="updateEndpoint(editEndpointData)">Ok</button>
        </div>
      </div>
    </div>
  </div>
</template>