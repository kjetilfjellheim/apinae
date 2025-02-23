<script setup>
import { ref, onMounted } from "vue";
import { useRoute } from 'vue-router'
import { invoke } from "@tauri-apps/api/core";

const route = useRoute();

const test = ref([]);
const tcp_listeners = ref([]);
const http_servers = ref([]);

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

const addHttpServer = () => {
  invoke("add_test_http_server", { testid: test.value.id })
    .then((message) => {
      refresh(test.value.id);
    })
    .catch((error) => window.alert(serverId));
}

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
    .catch((error) => console.error(error));
}

const addTcpListener = () => {
  invoke("add_test_tcp_listener", { testid: test.value.id })
    .then((message) => {
      refresh(test.value.id);
    })
    .catch((error) => window.alert(error));
}

const confirmDeleteTcpListener = (port) => {
  invoke("confirm_dialog", {})
    .then((confirm) => {
      if (confirm) {
        invoke("delete_test_tcp_listener", { testid: test.value.id, port: port })
          .then((message) => {
            refresh(test.value.id);
          })
          .catch((error) => window.alert(error));
      }
    }
    )
    .catch((error) => console.error(error));
}

onMounted(() => {
  const test_id = route.params.test_id
  refresh(test_id)
});
</script>
<style>
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

.toolbar-col {
  min-width: 90px;
  max-width: 90px;
  width: 90px;
}
</style>
<template>
  <nav class="navbar navbar-expand-sm bg-body-tertiary small">
    <div class="container-fluid">
      <ol class="breadcrumb  margin-0 padding-0 align-middle">
        <li class="breadcrumb-item"><router-link to="/"><i class="fas fa-house"></i></router-link></li>
        <li class="breadcrumb-item">{{ test?.name }}</li>
      </ol>
    </div>
  </nav>
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
                  @click="editTcpListener(tcp_listener)" data-bs-toggle="modal" data-bs-target="#idEditTestModel"><i
                    class="fa-solid fa-file-pen"></i></button>
                <button class="btn btn-sm btn-outline-danger align-middle"
                  @click="confirmDeleteTcpListener(tcp_listener.port)"><i class="fa-solid fa-trash"></i></button>
              </div>
            </div>
            <dl class="row">
              <dt class="col-sm-3 small">Accept</dt>
              <dd class="col-sm-9 small">&nbsp;{{ tcp_listener.accept }}</dd>
              <dt class="col-sm-3 small">Close connection</dt>
              <dd class="col-sm-9 small">&nbsp;{{ tcp_listener.close_connection }}</dd>
              <dt class="col-sm-3 small">Delayed write response</dt>
              <dd class="col-sm-9 small">&nbsp;{{ tcp_listener.delay_write_ms }}</dd>
              <dt class="col-sm-3 small">File</dt>
              <dd class="col-sm-9 small">&nbsp;{{ tcp_listener.file }}</dd>
              <dt class="col-sm-3 small">Data</dt>
              <dd class="col-sm-9 small">&nbsp;{{ tcp_listener.data }}</dd>
            </dl>
          </div>
        </div>
        <div class="col-12">
          &nbsp;
        </div>
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
                  @click="editHttpServer(http_server)" data-bs-toggle="modal" data-bs-target="#idEditTestModel"><i
                    class="fa-solid fa-file-pen"></i></button>
                <button class="btn btn-sm btn-outline-danger align-middle"
                  @click="confirmDeleteHttpServer(http_server.id)"><i class="fa-solid fa-trash"></i></button>
              </div>
            </div>
            <div class="container-fluid padding-0 margin-0">
              <div class="row">
                <div class="col-3">
                  <dl class="row">
                    <dt class="col-sm-3 small">Id</dt>
                    <dd class="col-sm-9 small">&nbsp;{{ http_server.id }}</dd>
                    <dt class="col-sm-3 small">Name</dt>
                    <dd class="col-sm-9 small">&nbsp;{{ http_server.name }}</dd>
                    <dt class="col-sm-3 small">Description</dt>
                    <dd class="col-sm-9 small">&nbsp;{{ http_server.description }}</dd>
                    <dt class="col-sm-3 small" v-if="http_server.http_port">Http port</dt>
                    <dd class="col-sm-9 small" v-if="http_server.http_port">&nbsp;{{ http_server.http_port }}</dd>
                    <dt class="col-sm-3 small" v-if="http_server?.https_config?.https_port">Https port</dt>
                    <dd class="col-sm-9 small" v-if="http_server?.https_config">&nbsp;{{
                      http_server?.https_config.https_port }}</dd>
                    <dt class="col-sm-3 small" v-if="http_server?.https_config?.https_port">Server certificate</dt>
                    <dd class="col-sm-9 small" v-if="http_server?.https_config">&nbsp;{{
                      http_server?.https_config.server_certificate }}</dd>
                    <dt class="col-sm-3 small" v-if="http_server?.https_config">Private key</dt>
                    <dd class="col-sm-9 small" v-if="http_server?.https_config">&nbsp;{{
                      http_server?.https_config.private_key }}</dd>
                    <dt class="col-sm-3 small" v-if="http_server?.https_config">Client certificate</dt>
                    <dd class="col-sm-9 small" v-if="http_server?.https_config">&nbsp;{{
                      http_server?.https_config.client_certificate }}</dd>
                    <dt class="col-sm-3 small" v-if="http_server?.https_config">Supported TLS versions</dt>
                    <dd class="col-sm-9 small" v-if="http_server?.https_config">
                      <ul class="list-unstyled">
                        <li v-for="tls_version in http_server.https_config.supported_tls_versions">{{ tls_version }}
                        </li>
                      </ul>
                    </dd>
                  </dl>
                </div>
                <div class="col-9">
                  <div class="container-fluid padding-0 margin-0">
                    <div class="row">
                      <div class="col-12">
                        <table class="table table-sm table-striped table-bordered caption-top">
                          <caption>
                            Endpoints
                            <div class="btn-group btn-group-sm align-middle small" role="group">
                              <button type="button" class="btn btn-sm btn-outline-primary" @click="addEndpoint()"><i
                                  class="fa-solid fa-plus"></i></button>
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
                              <td class="align-middle"><label class="align-middle small">{{ endpoint.endpoint }}</label></td>
                              <td class="align-middle"><label class="align-middle small">{{ endpoint.method }}</label></td>
                              <td class="align-middle">
                                <div v-if="endpoint?.mock">
                                  <dl class="row">
                                    <dt class="col-3 small">Status</dt>
                                    <dd class="col-9 small">{{ endpoint.mock?.status }}</dd>                                    
                                    <dt class="col-3 small">Headers</dt>
                                    <dd class="col-9 small">
                                      <ul class="list-unstyled">
                                        <li v-for="(value, key) in endpoint.mock?.headers">{{ key }}: {{ value }}</li>
                                      </ul></dd>
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
                                      <button type="button"
                                        class="btn btn-sm btn-outline-primary text-decoration-none"
                                        @click="editEndpoint(endpoint)" data-bs-toggle="modal"
                                        data-bs-target="#idEditTestModel"><i class="fa-solid fa-pen-to-square"></i></button>
                                    </div>
                                    <div class="btn-group btn-group-sm align-middle small me-2" role="group">
                                      <button class="btn btn-sm btn-outline-danger text-decoration-none"
                                        @click="confirmDeleteEndpoint(endpoint)"><i class="fa-solid fa-minus"></i></button>
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
</template>