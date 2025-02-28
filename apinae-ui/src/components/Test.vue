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
const tcp_listeners = ref([]);
// Array of all http servers for the test
const http_servers = ref([]);
// Data for editing a tcp listener. Data is copied from the tcp_listener 
// object to this object when the user clicks the edit button
const editTcpListenerData = ref({});
// Data for editing a http server. Data is copied from the http_server
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
const refresh = (test_id) => {
  invoke("get_test", { testid: test_id })
    .then((message) => {
      test.value = message;
    })
    .catch((error) => window.alert(error));

  invoke("get_test_http_servers", { testid: test_id })
    .then((message) => {
      http_servers.value = message;
    })
    .catch((error) => window.alert(error));

  invoke("get_test_tcp_listeners", { testid: test_id })
    .then((message) => {
      tcp_listeners.value = message;
    })
    .catch((error) => window.alert(error));
}

//Add an http server to the test. This is called when the user clicks the add button.
//The server is added to the test and the data is refreshed.
const addHttpServer = () => {
  invoke("add_test_http_server", { testid: test.value.id })
    .then((message) => {
      refresh(test.value.id);
    })
    .catch((error) => window.alert(serverId));
}

//Delete an http server from the test. This is called when the user clicks the delete button.
//The server is deleted from the test and the data is refreshed. The user is asked to confirm
//the deletion first. This uses the rust confirm_dialog function to display a dialog to the user.
const confirmDeleteHttpServer = (serverId) => {
  invoke("confirm_dialog", {})
    .then((confirm) => {
      if (confirm) {
        invoke("delete_test_http_server", { testid: test.value.id, serverid: serverId })
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
  invoke("add_test_tcp_listener", { testid: test.value.id })
    .then((message) => {
      refresh(test.value.id);
    })
    .catch((error) => window.alert(error));
}

//Add an endpoint to the http server. This is called when the user clicks the add button.
//The endpoint is added to the http server and the data is refreshed.
const addEndpoint = (http_server) => {
  invoke("add_endpoint", { testid: test.value.id, serverid: http_server.id })
    .then((message) => {
      refresh(test.value.id);
    })
    .catch((error) => window.alert(error));
}

//Delete an endpoint from the http server. This is called when the user clicks the delete button.
//The endpoint is deleted from the htp server and the data is refreshed. The user is asked to confirm
//the deletion first. This uses the rust confirm_dialog function to display a dialog to the user.
const confirmDeleteEndpoint = (http_server, endpoint) => {
  invoke("confirm_dialog", {})
    .then((confirm) => {
      if (confirm) {
        invoke("delete_endpoint", { testid: test.value.id, serverid: http_server.id, endpointid: endpoint.id })
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
        invoke("delete_test_tcp_listener", { testid: test.value.id, listenerid: listenerid })
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
//The data is copied from the tcp_listener object to the editTcpListenerData object.
const editTcpListener = (tcp_listener) => {
  editTcpListenerData.value = { ...tcp_listener };
}

//Initializes the data for editing an endpoint. This is called when the user clicks the edit button.
//The data is copied from the endpoint object to the editEndpointData object.
const editEndpoint = (http_server, endpoint) => {
  if (endpoint.mock) {
    editMockData.value = { ...endpoint.mock };
    editRouteData.value = {};
    showEditMockData.value = true;
  } else {
    editMockData.value = {};
    editRouteData.value = { ...endpoint.route };
    showEditMockData.value = false;
  }
  serverIdEditEndpoint.value = http_server.id;
  editEndpointData.value = { ...endpoint };
}

//Initializes the data for editing a http server. This is called when the user clicks the edit button.
//The data is copied from the http_server object to the editHttpServerData object. If the http server have
//an https config, the data is copied from the https_config object to the editHttpsConfig object it also copies
//the supported tls versions to the editSupportedTlsVersions object.
const editHttpServer = (http_server) => {
  if (http_server?.https_config) {
    editHttpsConfig.value = { ...http_server?.https_config };
    editSupportedTlsVersions.value = http_server?.https_config?.supported_tls_versions ? [...http_server?.https_config?.supported_tls_versions] : [];
  } else {
    editHttpsConfig.value = {};
    editSupportedTlsVersions.value = [];
  }
  editHttpServerData.value = { ...http_server };
  showEditHttpsConfig.value = http_server.https_config ? true : false;
}

//Updates the tcp listener. This is called when the user clicks the Ok button in the edit modal.
//The data on the editTcpListenerData object is sent to the rust code to update the tcp listener.
//If successful the modal is hidden and the data is refreshed.
const updateTcpListener = (tcp_listener) => {
  invoke("update_test_tcp_listener", { testid: test.value.id, listenerid: tcp_listener.id, tcplistener: convertTcpListenerToRequestObject(tcp_listener) })
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
const updateHttpServer = (http_server, https_config, supported_tls_versions) => {
  invoke("update_test_http_server", { testid: test.value.id, serverid: http_server.id, httpserver: convertHttpServerToRequestObject(http_server, https_config, supported_tls_versions) })
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
    endpoint: editEndpointData.value.endpoint,
    method: editEndpointData.value.method,
    mock: showEditMockData.value ? convertMockToRequestObject(editMockData) : null,
    route: !showEditMockData.value ? convertRouteToRequestObject(editRouteData) : null,
  }
}

const convertMockToRequestObject = (mock_data) => {
  return {
    status: mock_data.value.status ? parseInt(mock_data.value.status) : null,
    headers: mock_data.value.headers,
    delay: parseInt(mock_data.value.delay),
    response: mock_data.value.response
  }
}

const convertRouteToRequestObject = (route_data) => {
  return {
    endpoint: route_data.value.endpoint,
    proxy_url: route_data.value.proxy_url,
    verbose: route_data.value.verbose ? true : false,
    http1_only: route_data.value.http1_only ? true : false,
    accept_invalid_certs: route_data.value.accept_invalid_certs ? true : false,
    accept_invalid_hostnames: route_data.value.accept_invalid_hostnames ? true : false,
    min_tls_version: route_data.value.min_tls_version,
    max_tls_version: route_data.value.max_tls_version,
    read_timeout: route_data.value.read_timeout ? parseInt(route_data.value.read_timeout) : null,
    connect_timeout: route_data.value.connect_timeout ? parseInt(route_data.value.connect_timeout) : null
  }
}

//Converts the http server object to a request object that can be sent to the rust code.
//This is used when updating the http server.
//TODO: Implement validation if the objects are not valid.
const convertHttpServerToRequestObject = (http_server, https_config, supported_tls_versions) => {
  return {
    id: http_server.id,
    name: http_server.name,
    description: http_server.description,
    http_port: http_server.http_port ? parseInt(http_server.http_port) : null,
    https_config: showEditHttpsConfig.value ? convertHttpsConfigToRequestObject(https_config, supported_tls_versions) : null,
    endpoints: []
  }
}

//Converts the https config object to a request object that can be sent to the rust code.
//This is used when updating the http server.
//TODO: Implement validation if the objects are not valid.
const convertHttpsConfigToRequestObject = (https_config, supported_tls_versions) => {
  return {
    https_port: https_config.https_port ? parseInt(https_config.https_port) : null,
    server_certificate: https_config.server_certificate,
    private_key: https_config.private_key,
    client_certificate: https_config.client_certificate,
    supported_tls_versions: supported_tls_versions
  }
}

//Converts the tcp listener object to a request object that can be sent to the rust code.
//This is used when updating the tcp listener.
//TODO: Implement validation if the objects are not valid.
const convertTcpListenerToRequestObject = (tcp_listener) => {
  return {
    id: tcp_listener.id,
    port: tcp_listener.port ? parseInt(tcp_listener.port) : null,
    accept: tcp_listener.accept,
    close_connection: tcp_listener.close_connection,
    delay_write_ms: tcp_listener.delay_write_ms ? parseInt(tcp_listener.delay_write_ms) : null,
    file: tcp_listener.file,
    data: tcp_listener.data
  }
}

//Initializes the data when the page is loaded.
//Initializes the httpserver and tcplistener modals so that we can show and hide them 
//in code rather than just in the button.
onMounted(() => {
  editEndpointModal.value = new Modal('#idEditEndpointModel', { keyboard: false });
  editHttpServerModal.value = new Modal('#idEditHttpServerModel', { keyboard: false });
  editTcpListenerModal.value = new Modal('#idEditTcpListenerModel', { keyboard: false });
  const test_id = route.params.test_id
  refresh(test_id)
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
        <div class="card" v-for="tcp_listener in tcp_listeners" :key="tcp_listener.port"
          v-if="tcp_listeners?.length > 0">
          <div class="card-header">
            Tcp listener for port {{ tcp_listener.port }}
          </div>
          <div class="card-body">
            <div class="btn-toolbar" role="toolbar"
              aria-label="Toolbar with button groups margin-0 padding-0 align-middle">
              <div class="btn-group btn-group-sm align-middle small me-2 margin-0 padding-0 button-position-right"
                role="group">
                <button type="button" class="btn btn-sm btn-outline-primary align-middle"
                  @click="editTcpListener(tcp_listener)" data-bs-toggle="modal"
                  data-bs-target="#idEditTcpListenerModel"><i class="fa-solid fa-file-pen"></i></button>
                <button class="btn btn-sm btn-outline-danger align-middle"
                  @click="confirmDeleteTcpListener(tcp_listener.id)"><i class="fa-solid fa-trash"></i></button>
              </div>
            </div>
            <div class="container-fluid">
              <div class="row">
                <div class="col-4">
                  <div class="mb-0 row">
                    <label for="idLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Id</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="idLabel"
                        :value="tcp_listener.id">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="portLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Port</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="portLabel"
                        :value="tcp_listener.port">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="acceptLabel"
                      class="col-sm-6 col-form-label small text-truncate padding-0">Accept</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="acceptLabel"
                        :value="tcp_listener.accept">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="acceptLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Close
                      connection</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="acceptLabel"
                        :value="tcp_listener.close_connection">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="acceptLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Delayed write
                      response</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="acceptLabel"
                        :value="tcp_listener.delay_write_ms">
                    </div>
                  </div>
                </div>
                <div class="col-8" v-if="tcp_listener.file">
                  <div class="mb-0 row">
                    <label for="fileLabel" class="col-sm-3 col-form-label small text-truncate padding-0">File</label>
                    <div class="col-sm-9">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="fileLabel"
                        :value="tcp_listener.data">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="tcp_listener.data">
                    <label for="dataLabel" class="col-sm-3 col-form-label small text-truncate padding-0">Data</label>
                    <div class="col-sm-9 small">{{ tcp_listener.file }}</div>
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
        <div class="card" v-for="http_server in http_servers" :key="http_server.port">
          <div class="card-header">
            Http server: {{ http_server.name }}
          </div>
          <div class="card-body">
            <div class="btn-toolbar" role="toolbar"
              aria-label="Toolbar with button groups margin-0 padding-0 align-middle">
              <div class="btn-group btn-group-sm align-middle small me-2 margin-0 padding-0 button-position-right"
                role="group">
                <button type="button" class="btn btn-sm btn-outline-primary align-middle"
                  @click="editHttpServer(http_server)" data-bs-toggle="modal" data-bs-target="#idEditHttpServerModel"><i
                    class="fa-solid fa-file-pen"></i></button>
                <button class="btn btn-sm btn-outline-danger align-middle"
                  @click="confirmDeleteHttpServer(http_server.id)"><i class="fa-solid fa-trash"></i></button>
              </div>
            </div>
            <div class="container-fluid">
              <div class="row">
                <div class="col-4">
                  <div class="mb-0 row">
                    <label for="idLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Id</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="idLabel"
                        :value="http_server.id">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="nameLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Name</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="nameLabel"
                        :value="http_server.name">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="http_server.http_port">
                    <label for="httpPortLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Http
                      port</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="httpPortLabel"
                        :value="http_server.http_port">
                    </div>
                  </div>
                  <div class="mb-0 row">
                    <label for="descriptionLabel"
                      class="col-sm-6 col-form-label small text-truncate padding-0">Description</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="descriptionLabel"
                        :value="http_server.description">
                    </div>
                  </div>
                </div>
                <div class="col-4">
                  <div class="mb-0 row" v-if="http_server?.https_config?.https_port">
                    <label for="httpsPortLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Https
                      port</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="httpsPortLabel"
                        :value="http_server?.https_config.https_port">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="http_server?.https_config?.server_certificate">
                    <label for="serverCertLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Server
                      certificate</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="serverCertLabel"
                        :value="http_server?.https_config?.server_certificate">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="http_server?.https_config?.private_key">
                    <label for="privateKeyLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Private
                      key</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="privateKeyLabel"
                        :value="http_server?.https_config?.private_key">
                    </div>
                  </div>
                  <div class="mb-0 row" v-if="http_server?.https_config?.client_certificate">
                    <label for="clientCertLabel" class="col-sm-6 col-form-label small text-truncate padding-0">Client
                      certificate</label>
                    <div class="col-sm-6">
                      <input type="text" readonly class="form-control-plaintext small padding-0" id="clientCertLabel"
                        :value="http_server?.https_config?.client_certificate">
                    </div>
                  </div>
                </div>
                <div class="col-4">
                  <div class="mb-0" v-if="http_server?.https_config?.supported_tls_versions">
                    <label for="tlsVersionsLabel"
                      class="col-sm-6 col-form-label small text-truncate padding-0">Supported TLS versions</label>
                    <div class="col-sm-6">
                      <ul class="list-unstyled">
                        <li v-for="tls_version in http_server?.https_config?.supported_tls_versions" class="small">{{
                          tls_version }}</li>
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
                                @click="addEndpoint(http_server)"><i class="fa-solid fa-plus"></i></button>
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
                            <tr v-for="endpoint in http_server.endpoints" :key="endpoint.id">
                              <td class="align-middle"><label class="align-middle small">{{ endpoint.endpoint }}</label>
                              </td>
                              <td class="align-middle"><label class="align-middle small">{{ endpoint.method }}</label>
                              </td>
                              <td class="align-middle">
                                <div v-if="endpoint?.mock">
                                  <dl class="row">
                                    <dt class="col-3 small">Status</dt>
                                    <dd class="col-9 small">{{ endpoint.mock?.status }}</dd>
                                    <dt class="col-3 small">Headers</dt>
                                    <dd class="col-9 small">
                                      <ul class="list-unstyled">
                                        <li v-for="(value, key) in endpoint.mock?.headers">{{ key }}: {{ value }}</li>
                                      </ul>
                                    </dd>
                                    <dt class="col-3 small">Delay</dt>
                                    <dd class="col-9 small">{{ endpoint.mock?.delay }}</dd>
                                    <dt class="col-3 small">Body</dt>
                                    <dd class="col-9 small">{{ endpoint.mock?.response }}</dd>
                                  </dl>
                                </div>
                                <div v-if="endpoint?.route">
                                  <dl class="row">
                                    <dt class="col-3 small">&nbsp;Endpoint</dt>
                                    <dd class="col-9 small">{{ endpoint.route?.endpoint }}</dd>
                                    <dt class="col-3 small">Proxy url</dt>
                                    <dd class="col-9 small">&nbsp;{{ endpoint.route?.proxy_url }}</dd>
                                    <dt class="col-3 small">Verbose</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.verbose }}</dd>
                                    <dt class="col-3 small">Http1 only</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.http1_only }}</dd>
                                    <dt class="col-3 small">Accept invalid certs</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.accept_invalid_certs }}</dd>
                                    <dt class="col-3 small">Accept invalid hostnames</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.accept_invalid_hostnames }}</dd>
                                    <dt class="col-3 small">Min TLS version</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.min_tls_version }}</dd>
                                    <dt class="col-3 small">Max TLS version</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.max_tls_version }}</dd>
                                    <dt class="col-3 small">Read timeout</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.read_timeout }}</dd>
                                    <dt class="col-3 small">Connect timeout</dt>
                                    <dd class="col-3 small">&nbsp;{{ endpoint.route?.connect_timeout }}</dd>
                                  </dl>
                                </div>
                              </td>
                              <td class="align-middle toolbar-col">
                                <span class="align-middle">
                                  <div class="btn-toolbar" role="toolbar" aria-label="Toolbar with button groups">
                                    <div class="btn-group btn-group-sm align-middle small me-2" role="group">
                                      <button type="button" class="btn btn-sm btn-outline-primary text-decoration-none"
                                        @click="editEndpoint(http_server, endpoint)" data-bs-toggle="modal"
                                        data-bs-target="#idEditEndpointModel"><i
                                          class="fa-solid fa-pen-to-square"></i></button>
                                    </div>
                                    <div class="btn-group btn-group-sm align-middle small me-2" role="group">
                                      <button class="btn btn-sm btn-outline-danger text-decoration-none"
                                        @click="confirmDeleteEndpoint(http_server, endpoint)"><i
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
                v-model="editTcpListenerData.delay_write_ms">
            </div>
            <div class="col-md-6">
              <label for="idEditCloseConnection" class="form-label small">Close connection</label>
              <input type="text" class="form-control form-control-sm" id="idEditCloseConnection"
                v-model="editTcpListenerData.close_connection">
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
                v-model="editHttpServerData.http_port">
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
                  v-model="editHttpsConfig.https_port">
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditServerCertificate" class="form-label small">Server certificate</label>
                <input type="text" class="form-control form-control-sm" id="idEditServerCertificate"
                  v-model="editHttpsConfig.server_certificate">
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditPrivateKey" class="form-label small">Private key</label>
                <input type="text" class="form-control form-control-sm" id="idEditPrivateKey"
                  v-model="editHttpsConfig.private_key">
              </div>
            </div>
            <div class="col-md-12" v-if="showEditHttpsConfig">
              <div class="form-check">
                <label for="idEditClientCertificate" class="form-label small">Private key</label>
                <input type="text" class="form-control form-control-sm" id="idEditClientCertificate"
                  v-model="editHttpsConfig.client_certificate">
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
            <div class="col-md-6">
              <div class="form-check">
                <input class="form-check-input" type="checkbox" id="idEditShowEditMockData" v-model="showEditMockData">
                <label class="form-check-label" for="idEditShowEditMockData">Mock</label>
              </div>              
            </div>
            <div class="col-md-6">
              <label for="idEditEndpoint" class="form-label small">Endpoint</label>
              <input type="text" class="form-control form-control-sm" id="idEditEndpoint"
                v-model="editEndpointData.endpoint">
            </div>
            <div class="col-md-6">
              <label for="idEditMethod" class="form-label small">Method</label>
              <input type="text" class="form-control form-control-sm" id="idEditMethod"
                v-model="editEndpointData.method">
            </div>    
            <div class="col-md-6" v-if="showEditMockData">
              <label for="idEditResponse" class="form-label small">Response</label>
              <input type="text" class="form-control form-control-sm" id="idEditResponse"
                v-model="editMockData.response">
            </div>  
            <div class="col-md-6" v-if="showEditMockData">
              <label for="idEditStatus" class="form-label small">Status</label>
              <input type="text" class="form-control form-control-sm" id="idEditStatus"
                v-model="editMockData.status">
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
            <div class="col-md-6" v-if="!showEditMockData">              
              <label for="idEditEndpoint" class="form-label small">Path</label>
                <input type="text" class="form-control form-control-sm" id="idEditEndpoint"
                  v-model="editRouteData.endpoint">
            </div>
            <div class="col-md-6" v-if="!showEditMockData">              
              <label for="idEditProxyUrl" class="form-label small">Proxy url</label>
                <input type="text" class="form-control form-control-sm" id="idEditProxyUrl"
                  v-model="editRouteData.proxy_url">
            </div>
            <div class="col-md-6" v-if="!showEditMockData">  
                <label class="form-label small" for="idEditVerbose">Verbose</label><br>
                <input class="form-check-input" type="checkbox" id="idEditVerbose" v-model="editRouteData.verbose">
            </div>             
            <div class="col-md-6" v-if="!showEditMockData">  
                <label class="form-label small" for="idEditHttp1Only">Http1 only</label><br>
                <input class="form-check-input" type="checkbox" id="idEditHttp1Only" v-model="editRouteData.http1_only">
            </div>   
            <div class="col-md-6" v-if="!showEditMockData">  
              <div class="form-check">
                <input class="form-check-input" type="checkbox" id="idEditAcceptInvalidCerts" v-model="editRouteData.accept_invalid_certs">
                <label class="form-check-label" for="idEditAcceptInvalidCerts">Accept invalid certs</label>
              </div>
            </div>   
            <div class="col-md-6" v-if="!showEditMockData">  
              <div class="form-check">
                <input class="form-check-input" type="checkbox" id="idEditAcceptInvalidHostnames" v-model="editRouteData.accept_invalid_hostnames">
                <label class="form-check-label" for="idEditAcceptInvalidHostnames">Accept invalid hostnames</label>
              </div>
            </div>  
            <div class="col-md-6" v-if="!showEditMockData">                
              <label class="form-label small" for="idEditMinTlsVersion">Min Tls version</label>             
              <input class="form-control form-control-sm" type="text" id="idEditMinTlsVersion" v-model="editRouteData.min_tls_version">
            </div> 
            <div class="col-md-6" v-if="!showEditMockData">                
              <label class="form-label small" for="idEditMaxTlsVersion">Max Tls version</label>              
              <input class="form-control form-control-sm" type="text" id="idEditMaxTlsVersion" v-model="editRouteData.max_tls_version">
            </div>  
            <div class="col-md-6" v-if="!showEditMockData">  
              <label class="form-label small" for="idEditReadTimeout">Read timeout</label>
              <input class="form-control form-control-sm" type="text" id="idEditReadTimeout" v-model="editRouteData.read_timeout">
            </div>  
            <div class="col-md-6" v-if="!showEditMockData">  
              <label class="form-label small" for="idEditConnectTimeout">Connect timeout</label>
              <input class="form-control form-control-sm" type="test" id="idEditConnectTimeout" v-model="editRouteData.connect_timeout">
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