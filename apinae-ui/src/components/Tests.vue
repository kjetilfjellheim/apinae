<script setup>
//Required for showing the test data and updating the test data.
import { ref, onMounted } from "vue";
//Required for calling the rust code.
import { invoke } from "@tauri-apps/api/core";

//Array of tests to display. 
const tests = ref([]);

//Initializes the data for editing a test. This is called when the user clicks the edit button.
//The data is copied from the tests object to the editTestData object.
const editTestData = ref({});

//Refreshes the tests array by calling the get_tests function in the backend.
//This is called when the component is mounted and when a test is added, updated, or deleted.
const refresh = () => {
    invoke("get_tests", {})
        .then((message) => {
            tests.value = message;
        })
        .catch((error) => window.alert(error));
}

//Initializes the data for editing a test. This is called when the user clicks the edit button.
//The data is copied from the tests object to the editTestData object.
const editTest = (test) => {
    editTestData.value = { ...test };
}

//Updates the test by calling the update_test function in the backend.
//This is called when the user clicks the Ok button in the edit test modal.
//The editTestData object is passed to the backend to update the test. The 
//refresh function is called to update the tests array.
const updateTest = (test) => {
    invoke("update_test", { testid: test.id, test: test })
        .then((message) => {
            refresh();
        })
        .catch((error) => window.alert(error));
}

//Deletes the test by calling the delete_test function in the backend. 
//This is called when the user clicks the delete button. A confirmation dialog is displayed
//before the test is deleted. If the user confirms the deletion, the test is deleted and the
//refresh function is called to update the tests array.
const confirmDelete = (test) => {
    invoke("confirm_dialog", {})
        .then((confirm) => {
            if (confirm) {
                invoke("delete_test", { testid: test.id })
                    .then((message) => {
                        refresh();
                    })
                    .catch((error) => window.alert(error));
            }
        })
        .catch((error) => console.error(error));
}

//Adds a test by calling the add_test function in the backend.
//This is called when the user clicks the add button. The refresh function is called.
const addTest = () => {
    invoke("add_test", {})
        .then((message) => {
            console.log("add_test", message);
            refresh();
        })
        .catch((error) => window.alert(error));
}

//Starts the test by calling the start_test function in the backend.
//This is called when the user clicks the play button. The process_id is set to the
//process_id returned by the backend.
const startTest = (test) => {
    invoke("start_test", { testid: test.id })
        .then((message) => {
            test.process_id = message.process_id
        })
        .catch((error) => window.alert(error));
}

//Stops the test by calling the stop_test function in the backend.
//This is called when the user clicks the stop button. The process_id is set to null.
const stopTest = (test) => {
    invoke("stop_test", { testid: test.id })
        .then((message) => {
            test.process_id = message.process_id;
        })
        .catch((error) => window.alert(error));
}

//Refreshes the tests array when the component is mounted.
onMounted(() => refresh());
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
</style>
<template>
    <!-- 
     Show navigation bar. 
     TODO: Move this to a separate component.
     -->
    <nav class="navbar navbar-expand-sm bg-body-tertiary small">
        <div class="container-fluid">
            <ol class="breadcrumb margin-0 padding-0 align-middle">
                <li class="breadcrumb-item"><router-link to="/"><i class="fas fa-house"></i></router-link></li>
            </ol>
        </div>
    </nav>
    <!-- 
     Show main content. 
     TODO: Move this to a separate component.
    -->
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
                            <button type="button" class="btn btn-sm btn-outline-primary margin-0" @click="addTest()"><i
                                    class="fa-solid fa-plus"></i></button>
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
                                            <button type="button"
                                                class="btn btn-sm btn-outline-primary text-decoration-none"
                                                @click="editTest(test)" data-bs-toggle="modal"
                                                data-bs-target="#idEditTestModel"><i
                                                    class="fa-solid fa-pen-to-square"></i></button>
                                                    <router-link
                                                    :to="{ name: 'Test', params: { test_id: test.id } }">
                                            <button type="button"
                                                class="btn btn-sm btn-outline-primary text-decoration-none"><i
                                                        class="fa-solid fa-server"></i></button></router-link>
                                        </div>
                                        <div class="btn-group btn-group-sm align-middle small me-2" role="group">
                                            <button class="btn btn-sm btn-outline-danger text-decoration-none"
                                                @click="confirmDelete(test)"><i class="fa-solid fa-minus"></i></button>
                                        </div>
                                    </div>
                                </span>
                            </td>
                            <td class="align-middle">
                                <span class="align-middle">
                                    <button type="button" class="btn btn-sm btn-warning" @click="startTest(test)"
                                        v-if="test.process_id === null">
                                        <i class="fa-solid fa-play"></i></button>
                                    <button type="button" class="btn btn-sm btn-success" @click="stopTest(test)" v-else>
                                        <i class="fa-solid fa-stop"></i></button>
                                </span>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
    <!-- 
     Edit test modal.
     TODO: Move this to a separate component.
    -->
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
                    <button type="button" class="btn btn-sm btn-secondary" data-bs-dismiss="modal" data-bs-target="#idEditTestModel">Cancel</button>
                    <button type="button" class="btn btn-sm btn-primary" data-bs-dismiss="modal" data-bs-target="#idEditTestModel"
                        @click="updateTest(editTestData)">Ok</button>
                </div>
            </div>
        </div>
    </div>

</template>