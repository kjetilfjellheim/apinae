<script setup>
const props = defineProps(['test_id', 'http_server_data'])
</script>

<style>
.input {
    width: 90%;
    display: inline-block;
    box-sizing: border-box;
}

.margin-0 {
    margin: 0px 0px 0px 0px !important;
}

.padding-0 {
    padding: 0px 0px 0px 0px !important;
}
</style>
<template>
    <div class="accordion-item margin-0 padding-0" v-for="http_server in http_server_data" :key="http_server.id">
        <div class="accordion-header">
            <button class="accordion-button" type="button" data-bs-toggle="collapse"
                :data-bs-target="'#collapse' + http_server.id" aria-expanded="true"
                :aria-controls="'collapse' + http_server.id">
                {{ http_server.name }}
            </button>
        </div>
        <div :id="'collapse' + http_server.id" class="accordion-collapse collapse" data-bs-parent="#serverAccordion">
            <div class="accordion-body">
                <div class="container-fluid padding-0 margin-0">
                    <div class="row">
                        <div class="col-3">
                            <h6>Http config</h6>
                        </div>
                        <div class="col-3"><small>Port</small></div>
                        <div class="col-6"><small>{{ http_server.http_port }}</small></div>
                    </div>
                    <div class="row">
                        <div class="col-3">
                            <h6>Https config</h6>
                        </div>
                        <div class="col-3"><small>Server certificate</small></div>
                        <div class="col-6"><small>{{ http_server.httpsConfig?.serverCertificate }}</small></div>
                        <div class="col-3"></div>
                        <div class="col-3"><small>Private key</small></div>
                        <div class="col-6"><small>{{ http_server.httpsConfig?.privateKey }}</small></div>
                        <div class="col-3"></div>
                        <div class="col-3"><small>Port</small></div>
                        <div class="col-6"><small>{{ http_server.httpsConfig?.httpsPort }}</small></div>
                        <div class="col-3"></div>
                        <div class="col-3"><small>Client certificate</small></div>
                        <div class="col-6"><small>{{ http_server.httpsConfig?.clientCertificate }}</small></div>
                    </div>
                    <div class="row">
                        <div class="col-12">
                            <div class="card" v-for="endpoint in http_server?.endpoints"
                                :key="endpoint.id">
                                <div class="card-body">
                                    <table class="table">
                                        <thead>
                                            <tr>
                                                <th scope="col">Endpoint</th>
                                                <th scope="col">Method</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            <tr v-for="endpoint in http_server?.endpoints" :key="endpoint.id">
                                                <td><small>{{ endpoint.endpoint }}</small></td>
                                                <td><small>{{ endpoint.method }}</small></td>
                                            </tr>
                                        </tbody>
                                    </table>
                                    <div class="row" v-if="endpoint.mock">
                                        <div class="col-12">
                                            <h6>Mock Response</h6>
                                        </div>
                                        <div class="col-3"><small>Status</small></div>
                                        <div class="col-9"><small>{{ endpoint.mock?.status }}</small></div>
                                        <div class="col-3"><small>Response</small></div>
                                        <div class="col-9"><small>{{ endpoint.mock?.response }}</small></div>
                                        <div class="col-3"><small>Headers</small></div>
                                        <div class="col-9"><small>{{ endpoint.mock?.headers }}</small></div>
                                        <div class="col-3"><small>Delay</small></div>
                                        <div class="col-9"><small>{{ endpoint.mock?.delay }}</small></div>
                                    </div>
                                    <div class="row" v-if="endpoint.route">
                                        <div class="col-12">
                                            <h6>Route Response</h6>
                                        </div>
                                        <div class="col-3"><small>Endpoint</small></div>
                                        <div class="col-9"><small>{{ endpoint.route?.endpoint }}</small></div>
                                        <div class="col-3"><small>Proxy</small></div>
                                        <div class="col-9"><small>{{ endpoint.route?.proxyUrl }}</small></div>
                                        <div class="col-3"><small>Http1 only</small></div>
                                        <div class="col-9"><small>{{ endpoint.route?.http1Only }}</small></div>
                                        <div class="col-3"><small>Accept invalid certs</small></div>
                                        <div class="col-9"><small>{{ endpoint.route?.acceptInvalidCerts }}</small></div>
                                        <div class="col-3"><small>Accept invalid hostnames</small></div>
                                        <div class="col-9"><small>{{ endpoint.route?.acceptInvalidHostnames }}</small></div> 
                                        <div class="col-3"><small>Min TLS version</small></div>
                                        <div class="col-9"><small>{{ endpoint.route?.minTlsVersion }}</small></div>  
                                        <div class="col-3"><small>Max TLS version</small></div>
                                        <div class="col-9"><small>{{ endpoint.route?.maxTlsVersion }}</small></div>  
                                        <div class="col-3"><small>Read timeout</small></div>
                                        <div class="col-9"><small>{{ endpoint.route?.readTimeout }}</small></div>  
                                        <div class="col-3"><small>Conect timeout</small></div>
                                        <div class="col-9"><small>{{ endpoint.route?.connectTimeout }}</small></div>                                                                                                                                                                                                          
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