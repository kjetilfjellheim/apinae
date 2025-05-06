<script setup>
//Required for showing editing modals.
import { Modal } from 'bootstrap/dist/js/bootstrap.bundle';
//Required for showing the test data and updating the test data.
import { ref, onMounted } from "vue";
//Required for calling the rust code.
import { invoke } from "@tauri-apps/api/core";

//Array of tests to display. 
const tests = ref([]);

//Initializes the data for editing a test. This is called when the user clicks the edit button.
//The data is copied from the tests object to the editTestData object.
const editTestData = ref({});

// Parameter data for adding a new parameter to the test.
//This is called when the user clicks the add button in the edit test modal.
const editAddParameter = ref("");

//Modal for editing the run parameters of the test.
const editRunParameterModal = ref(null);

// Parameter data for the test.
//This is called when the user clicks the play button and the test requires parameters.
const editRunParameterData = ref({});

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
            refresh();
        })
        .catch((error) => window.alert(error));
}

// Shows the parameter dialog if the test has parameters else starts the test.
//This is called when the user clicks the play button. The processId is set to the
//processId returned by the backend. The test is started with the parameters passed to it.
const initTest = (test) => {
    if (test.params) {
        showParameterDialog(test);
    } 
    else {
        startTest(test, {});
    }
}

//Starts the test by calling the start_test function in the backend.
//This is called when the user clicks the play button. The processId is set to the
//processId returned by the backend.
const startTest = (test, params) => {
    invoke("start_test", { testid: test.id, params: params })
        .then((message) => {
            test.processId = message.processId
        })
        .catch((error) => window.alert(error));
}

//Stops the test by calling the stop_test function in the backend.
//This is called when the user clicks the stop button. The process_id is set to null.
const stopTest = (test) => {
    invoke("stop_test", { testid: test.id })
        .then((message) => {
            test.processId = message.processId;
        })
        .catch((error) => window.alert(error));
}

//Shows the parameter dialog so that the user can select the parameters for the test.
//This is called when the user clicks the play button. The parameters are passed to the start_test function.
const showParameterDialog = (test) => {
    editRunParameterData.value.test = test;
    editRunParameterData.value.params = {};
    for (const param of test.params) {        
        editRunParameterData.value.params[param] = "";
    }  
    editRunParameterModal.value.show();
}

//Refreshes the tests array when the component is mounted.
onMounted(() => {
    editRunParameterModal.value = new Modal('#idRunParameterModel', { keyboard: false });
    refresh()
});

//Verify that string input is filled out. 
const validateStringRequired = (str) => {
    if (str && str.length > 0) {
        return "is-valid";
    }
    return "is-invalid";
}

// Adds a parameter to the test.
//This is called when the user clicks the add button in the edit test modal.
//The parameter is added to the params array of the test. The editAddParameter object is cleared.
//If the parameter is empty, it is not added to the params array.
const addParameter = () => {
    if (editAddParameter.value && editAddParameter.value.length > 0) {
        if (!editTestData.value.params) {
            editTestData.value.params = [];
        }
        editTestData.value.params.push(editAddParameter.value);
        editAddParameter.value = "";        
    }
}

//Removes a parameter from the test.
//This is called when the user clicks the remove parameter button in the edit test modal.
const removeParameter = (param) => {
    if (editTestData.value.params) {
        const index = editTestData.value.params.indexOf(param);
        if (index > -1) {
            editTestData.value.params.splice(index, 1);
        }
        if (editTestData.value.params.length === 0) {
            editTestData.value.params = null;
        }
    }
}

</script>
<style>
/* The max height is full view height minus (top bar, menu bar and status bar + margins) */
.main-content {
    max-height: calc(100vh - 93px);
    overflow-y: scroll;
}

</style>
<template>
    <!-- 
     Show navigation bar. 
     TODO: Move this to a separate component.
     -->
    <nav class="navbar navbar-expand-sm bg-primary-subtle small">
        <div class="container-fluid">
            <ol class="breadcrumb m-0 p-0 align-middle">
                <li class="breadcrumb-item"><router-link to="/"><i class="fas fa-house"></i></router-link></li>
            </ol>
        </div>
    </nav>
    <!-- 
     Show main content. 
     TODO: Move this to a separate component.
    -->
    <div class="container-fluid main-content p-0 m-0">
        <div class="row p-0 m-0">
            <div class="col-12">&nbsp;</div>
            <div class="col-12">
                <div class="card">
                    <div class="card-header bg-primary">
                        &nbsp;Tests
                    </div>
                    <div class="card-body m-0 p-0">
                        <table class="table table-sm table-striped table-bordered table-primary caption-top m-0 p-0">
                            <caption class="text-start small bg-body-secondary">
                                &nbsp;
                                <div class="btn-group btn-group-sm align-middle small" role="group">
                                    <button type="button" class="btn btn-sm btn-outline-primary" @click="addTest()"><i
                                            class="fa-solid fa-plus"></i>&nbsp;Add test</button>
                                </div>
                            </caption>
                            <thead>
                                <tr>
                                    <th scope="col">Id</th>
                                    <th scope="col">Name</th>
                                    <th scope="col">Description</th>
                                    <th scope="col">Parameters</th>
                                    <th scope="col"></th>
                                    <th scope="col"></th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr v-for="test in tests" :key="test.id">
                                    <td class="align-middle"><label class="align-middle small">{{ test.id }}</label>
                                    </td>
                                    <td class="align-middle"><label class="align-middle small">{{ test.name }}</label>
                                    </td>
                                    <td class="align-middle"><label class="align-middle small">{{ test.description }}</label>
                                    </td>
                                    <td class="align-middle">
                                        <template v-if="test.params">
                                            <template v-for="param in test.params" :key="param">
                                                <span class="badge bg-info-subtle text-primary small">{{ param }}</span>&nbsp;
                                            </template>
                                        </template>
                                    </td>                                    
                                    <td class="align-middle">
                                        <span class="align-middle">
                                            <div class="btn-toolbar" role="toolbar"
                                                aria-label="Toolbar with button groups">
                                                <div class="btn-group btn-group-sm align-middle small me-2"
                                                    role="group">
                                                    <button type="button"
                                                        class="btn btn-sm btn-outline-primary text-decoration-none"
                                                        @click="editTest(test)" data-bs-toggle="modal"
                                                        data-bs-target="#idEditTestModel"><i
                                                            class="fa-solid fa-pen-to-square"></i></button>
                                                    <router-link :to="{ name: 'Test', params: { testid: test.id } }">
                                                        <button type="button"
                                                            class="btn btn-sm btn-outline-primary text-decoration-none"><i
                                                                class="fa-solid fa-server"></i></button></router-link>
                                                </div>
                                                <div class="btn-group btn-group-sm align-middle small me-2"
                                                    role="group">
                                                    <button class="btn btn-sm btn-outline-danger text-decoration-none"
                                                        @click="confirmDelete(test)"><i
                                                            class="fa-solid fa-trash"></i></button>
                                                </div>
                                            </div>
                                        </span>
                                    </td>
                                    <td class="align-middle">
                                        <span class="align-middle">
                                            <button type="button" class="btn btn-sm btn-warning"
                                                @click="initTest(test)" v-if="test.processId === null">
                                                <i class="fa-solid fa-play"></i></button>
                                            <button type="button" class="btn btn-sm btn-success" @click="stopTest(test)"
                                                v-else>
                                                <i class="fa-solid fa-stop"></i></button>
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
    <!-- 
     Edit test modal.
     TODO: Move this to a separate component.
    -->
    <div class="modal fade" id="idEditTestModel" tabindex="-1" aria-labelledby="editTestLabel" aria-hidden="true">
        <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header bg-primary">
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
                            v-model="editTestData.name" :class="validateStringRequired(editTestData.name)">
                    </div>
                    <div class="mb-3">
                        <label for="idEditDescription" class="form-label small">Description</label>
                        <textarea class="form-control form-control-sm is-valid" id="idEditDescription" rows="3"
                            v-model="editTestData.description"></textarea>
                    </div>
                    <div class="mb-3">                        
                        <label for="idEditParams" class="form-label small">Parameters</label>
                        <div class="input-group mb-3">
                            <input type="text" class="form-control is-valid" placeholder="Parameter" aria-label="Parameter" v-model="editAddParameter">
                            <button class="btn btn-outline-primary" type="button" id="addParameter" @click="addParameter()">Add</button>
                        </div>                                                
                        <template v-if="editTestData.params">
                            <template v-for="param in editTestData.params" :key="param">
                                <button type="button" class="btn btn-sm btn-outline-primary small" @click="removeParameter(param)">{{ param }}</button>
                            </template>
                        </template>
                    </div>
                </div>
                <div class="modal-footer bg-primary-subtle">
                    <button type="button" class="btn btn-sm btn-secondary" data-bs-dismiss="modal"
                        data-bs-target="#idEditTestModel">Cancel</button>
                    <button type="button" class="btn btn-sm btn-primary" data-bs-dismiss="modal"
                        data-bs-target="#idEditTestModel" @click="updateTest(editTestData)">Ok</button>
                </div>
            </div>
        </div>
    </div>
    <div class="modal fade" id="idRunParameterModel" tabindex="-1" aria-labelledby="parameterModel" aria-hidden="true">
        <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header bg-primary">
                    <h6 class="modal-title fs-5 small" id="editTestLabel">Run test</h6>
                </div>
                <div class="modal-body">
                    <table class="table table-sm table-striped table-bordered table-primary m-0 p-0">
                        <thead>
                            <tr>
                                <th scope="col">Parameter</th>
                                <th scope="col">Value</th>
                            </tr>
                        </thead>
                        <tbody>
                            <template v-if="editRunParameterData.params">
                                <tr v-for="paramKey in Object.keys(editRunParameterData.params)" :key="paramKey">
                                    <td class="align-middle"><label class="align-middle small">{{ paramKey }}</label></td>
                                    <td class="align-middle">
                                        <input type="text" class="form-control form-control-sm" v-model="editRunParameterData.params[paramKey]"
                                            :class="validateStringRequired(editRunParameterData.params[paramKey])">
                                    </td>
                                </tr>
                            </template>
                        </tbody>
                    </table>
                </div>
                <div class="modal-footer bg-primary-subtle">
                    <button type="button" class="btn btn-sm btn-secondary" data-bs-dismiss="modal"
                        data-bs-target="#idRunParameterModel">Cancel</button>
                    <button type="button" class="btn btn-sm btn-primary" @click="startTest(editRunParameterData.test, editRunParameterData.params)">Run</button>
                </div>
            </div>
        </div>
    </div>
</template>