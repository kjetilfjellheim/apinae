<script setup>
const props = defineProps(['test_data'])
</script>

<style>
.input {
    width: 90%;
    display: inline-block;
    box-sizing: border-box;
}

.test-content {
    max-height: calc(100vh - 60px);
    overflow-y: scroll;
}

.margin-0 {
    margin: 0px 0px 0px 0px !important;
}

.padding-0 {
    padding: 0px 0px 0px 0px !important;
}
</style>
<template>
    <div class="container-fluid padding-0 margin-0 test-content">
        <div class="row margin-0 padding-0">
            <div class="col-3">
                <small>&nbsp;Id</small>
            </div>
            <div class="col-9">
                <small>{{ test_data?.id }}</small>
            </div>
            <div class="col-3">
                <small>&nbsp;Name</small>
            </div>
            <div class="col-9">
                <small type="text" placeholder="Name" class="input">{{ test_data.name }}</small>
            </div>
            <div class="col-3">
                <small>&nbsp;Description</small>
            </div>
            <div class="col-9">
                <small placeholder="Description" class="input">{{ test_data.description }}</small>
            </div>
            <div class="col-12">&nbsp;</div>
            <div class="col-12">
                <h6>&nbsp;Http servers</h6>
            </div>
            <div class="col-12">
                <div class="accordion" id="serverAccordion">
                    <httpservercontent :test_id="test_data?.id" :http_server_data="test_data.servers" />
                </div>
            </div>
            <div class="col-12">&nbsp;</div>
            <div class="col-12">
                <h6>&nbsp;Tcp listeners</h6>
            </div>
            <div class="col-12">
                <div class="accordion" id="tcpAccordion">
                    <tcplistenercontent :test_id="test_data?.id" :tcp_listener_data="test_data.listeners" />
                </div>
            </div>
        </div>
    </div>
</template>