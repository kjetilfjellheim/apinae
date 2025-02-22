<script setup>
import { ref, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

const tests = ref([]);

const editTestData = ref({});

const refresh = () => {
    invoke("get_tests", {})
        .then((message) => {
            tests.value = message;
        })
        .catch((error) => console.error(error));       
}

const editTest = (test) => {
    editTestData.value = { ...test };
}

const updateTest = (test) => {
    invoke("update_test_data", { test: test })
        .then((message) => {
            refresh();
        })
        .catch((error) => console.error(error));
}

const confirmDelete = (test) => {
    invoke("confirm_dialog", {})
        .then((confirm) => {
            if (confirm) {
                invoke("delete_test", { testid: test.id })
                    .then((message) => {
                        refresh();
                    })
                    .catch((error) => console.error(error));
            }
        })
        .catch((error) => console.error(error));
}

const addTest = () => {
    invoke("add_test", {})
        .then((message) => {
            console.log("add_test", message);
            refresh();
        })
        .catch((error) => console.error(error));
}

const startTest = (test) => {
    invoke("start_test", { testid: test.id })
        .then((message) => {
           test.process_id = message.process_id
        })
        .catch((error) => console.error(error));
}

const stopTest = (test) => {
    invoke("stop_test", { testid: test.id })
        .then((message) => {
           test.process_id = message.process_id;
        })
        .catch((error) => console.error(error));
}

onMounted(() => refresh());
</script>
<style>
.sidebar {
    top: 0;
    left: 0;
    background-color: #333333;
    border-left: 1px solid #4b4b4b;
    min-height: calc(100vh - 50px);
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
    <nav class="navbar navbar-expand-sm bg-body-tertiary small">
        <div class="container-fluid">
            <ol class="breadcrumb margin-0 padding-0 align-middle">
                <li class="breadcrumb-item"><router-link to="/"><i class="fas fa-house"></i></router-link></li>
            </ol>
        </div>
    </nav>
    <div class="container-fluid main-content padding-0 margin-0">
        <div class="row padding-0 margin-0">
            <div class="col-12">&nbsp;</div>
            <div class="col-12 bg-body-tertiary">
                <h5>File information</h5>
            </div>
            <div class="col-12">
                <dl class="row">
                    <dt class="col-sm-3">Name</dt>
                    <dd class="col-sm-9"></dd>
                    <dt class="col-sm-3">Description</dt>
                    <dd class="col-sm-9"></dd>
                </dl>
            </div>
            <div class="col-12">&nbsp;</div>
            <div class="col-12 bg-body-tertiary">
                <table class="table table-sm table-striped table-bordered caption-top">
                    <caption>
                        Tests
                        <div class="btn-group btn-group-sm align-middle small" role="group">
                            <Popper content="Add test" hover="true" placement="right">
                                <button type="button" class="btn btn-sm btn-outline-primary margin-0"
                                    @click="addTest()"><i class="fa-solid fa-plus"></i></button>
                            </Popper>
                        </div>
                    </caption>
                    <thead>
                        <tr>
                            <th scope="col">Id</th>
                            <th scope="col">Name</th>
                            <th scope="col">Description</th>
                            <th scope="col"></th>
                            <th scope="col"></th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="test in tests" :key="test.id">
                            <td class="align-middle"><label class="align-middle small">{{ test.id }}</label></td>
                            <td class="align-middle"><label class="align-middle small">{{ test.name }}</label></td>
                            <td class="align-middle"><label class="align-middle small">{{ test.description }}</label>
                            </td>
                            <td class="align-middle">
                                <span class="align-middle">
                                    <div class="btn-toolbar" role="toolbar" aria-label="Toolbar with button groups">
                                        <div class="btn-group btn-group-sm align-middle small me-2" role="group">
                                            <Popper content="Edit test" hover="true" placement="right">
                                                <button type="button"
                                                    class="btn btn-sm btn-outline-primary text-decoration-none"
                                                    @click="editTest(test)" data-bs-toggle="modal"
                                                    data-bs-target="#idEditTestModel"><i
                                                        class="fa-solid fa-pen-to-square"></i></button>
                                            </Popper>
                                            <Popper content="Configure test" hover="true" placement="right">
                                                <button type="button"
                                                    class="btn btn-sm btn-outline-primary text-decoration-none"><router-link
                                                        :to="{ name: 'Servers', params: { test_id: test.id } }"><i
                                                            class="fa-solid fa-server"></i></router-link></button>
                                            </Popper>
                                        </div>
                                        <div class="btn-group btn-group-sm align-middle small me-2" role="group">
                                            <Popper content="Delete test" hover="true" placement="right">
                                                <button class="btn btn-sm btn-outline-danger text-decoration-none"
                                                    @click="confirmDelete(test)"><i
                                                        class="fa-solid fa-minus"></i></button>
                                            </Popper>
                                        </div>
                                    </div>
                                </span>
                            </td>
                            <td class="align-middle">
                                <span class="align-middle">
                                    <Popper content="Start/Stop test" hover="true" placement="right">
                                        <button type="button" class="btn btn-sm btn-warning" @click="startTest(test)" v-if="test.process_id === null">
                                            <i class="fa-solid fa-play"></i></button>
                                        <button type="button" class="btn btn-sm btn-success" @click="stopTest(test)" v-else>
                                            <i class="fa-solid fa-stop"></i></button>                                            
                                    </Popper>
                                </span>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
    <div class="modal fade" id="idEditTestModel" tabindex="-1" aria-labelledby="editTestLabel" aria-hidden="true">
        <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                    <h6 class="modal-title fs-5 small" id="editTestLabel">Edit test</h6>
                </div>
                <div class="modal-body">
                    <div class="mb-3">
                        <label for="idEditId" class="form-label small">Id</label>
                        <label class="form-control form-control-sm" id="idEditId" readonly>{{ editTestData.id }}</label>
                    </div>
                    <div class="mb-3">
                        <label for="idEditName" class="form-label small">Name</label>
                        <input type="text" class="form-control form-control-sm" id="idEditName"
                            v-model="editTestData.name">
                    </div>
                    <div class="mb-3">
                        <label for="idEditDescription" class="form-label small">Description</label>
                        <textarea class="form-control form-control-sm" id="idEditDescription" rows="3"
                            v-model="editTestData.description"></textarea>
                    </div>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-sm btn-secondary" data-bs-dismiss="modal">Cancel</button>
                    <button type="button" class="btn btn-sm btn-primary" data-bs-dismiss="modal"
                        @click="updateTest(editTestData)">Ok</button>
                </div>
            </div>
        </div>
    </div>

</template>