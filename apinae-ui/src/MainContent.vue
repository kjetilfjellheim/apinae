<script setup>
import { ref } from "vue";

const props = defineProps(['tests'])

const selected_test = ref("");

function set_selected_test(test_data) {
    selected_test.value = test_data;
}

</script>
<style>
.sidebar {
    top: 0;
    left: 0;
    background-color: #333333;
    border-left: 1px solid #4b4b4b;
    min-height: calc(100vh - 60px);
    border-right: 1px solid #4b4b4b;
    border-bottom: 1px solid #4b4b4b;
    padding: 0px 0px 0px 0px;
}

.sidebar-content {
    border-right: 1px solid #4b4b4b;
}

.main-content {
    min-height: calc(100vh - 60px);
}

.margin-0 {
    margin: 0px 0px 0px 0px !important;
}

.padding-0 {
    padding: 0px 0px 0px 0px !important;
}

.selected {
    background-color: #0f0064 !important;
}
</style>
<template>
    <div class="container-fluid main-content padding-0 margin-0">
        <div class="row padding-0 margin-0">
            <div class="sidebar col-3 padding-0 margin-0">
                <div class="sidebar-header bg-body-secondary padding-0 margin-0"><small>&nbsp;&nbsp;Tests</small></div>
                <div class="sidebar-content">
                    <div class="list-group">
                        <div v-for="test in tests" :key="test.id">
                            <div class="list-group-item list-group-item-action" v-on:click="set_selected_test(test)"
                                :class="{ active: selected_test.id === test.id }">
                                <small :class="{ active: selected_test.id === test.id }">{{ test.name }}</small>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-9 padding-0 margin-0">
                <div class="container-fluid padding-0 margin-0">
                    <div class="row padding-0 margin-0">
                        <div class="col-12">
                            <testcontent :test_data="selected_test" v-if="selected_test != ''" />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>