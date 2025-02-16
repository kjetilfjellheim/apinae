<script setup>
const props = defineProps(['test_id', 'tcp_listener_data'])
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
    <div class="accordion-item margin-0 padding-0" v-for="tcp_listener in tcp_listener_data" :key="tcp_listener.port">
        <div class="accordion-header">
            <button class="accordion-button" type="button" data-bs-toggle="collapse"
                :data-bs-target="'#collapse' + tcp_listener.port" aria-expanded="true"
                :aria-controls="'collapse' + tcp_listener.port">
                {{ tcp_listener.port }}
            </button>
        </div>
        <div :id="'collapse' + tcp_listener.port" class="accordion-collapse collapse" data-bs-parent="#tcpAccordion">
            <div class="accordion-body">
                <div class="container-fluid padding-0 margin-0">
                    <div class="row">
                        <div class="col-3"><small>Port</small></div>
                        <div class="col-9"><small>{{ tcp_listener.port }}</small></div>
                        <div class="col-3"><small>File</small></div>
                        <div class="col-9"><small>{{ tcp_listener.file }}</small></div>
                        <div class="col-3"><small>Data</small></div>
                        <div class="col-9"><small>{{ tcp_listener.data }}</small></div>
                        <div class="col-3"><small>Accept</small></div>
                        <div class="col-9"><small>{{ tcp_listener.accept }}</small></div>
                        <div class="col-3"><small>Delay write</small></div>
                        <div class="col-9"><small>{{ tcp_listener.delayWriteMs }}</small></div>
                        <div class="col-3"><small>Close connection</small></div>
                        <div class="col-9"><small>{{ tcp_listener.closeConnection }}</small></div>
                    </div>
                </div>
            </div>
        </div>
    </div>

</template>