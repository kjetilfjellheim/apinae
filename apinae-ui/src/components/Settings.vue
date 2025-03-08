<script setup>
//Required for showing editing modals.
import { Modal } from 'bootstrap/dist/js/bootstrap.bundle';
//Required for calling the rust code.
import { invoke } from "@tauri-apps/api/core";

//Importing the ref function from vue.
import { onMounted, ref } from "vue";

//The modal for editing settings. Initialized in the onMounted function.
const editSettingsModal = ref(null);

//The settings data.
const settingsData = ref({
    apinaeCommand: "",
    commandType: ""
});

// Saves the settings. Calls the rust code to save the settings.
const saveSettings = () => {
    invoke("save_settings", { settings: convertToSettingsRequestObject(settingsData.value) })
        .then((message) => {
            editSettingsModal.value.hide();
        })
    .catch((error) => window.alert(error));
}

// Converts the settings data to a request object. The request object is used to call the rust code.
const convertToSettingsRequestObject = (settingsData) => {
    return {
        apinaePath: settingsData.commandType == 'specific' ? settingsData.apinaeCommand : null,
    };
}

//Called when the component is mounted. Initializes the editSettingsModal and loads the settings.
onMounted(() => {
    editSettingsModal.value = new Modal(document.getElementById("idSettingsModal"));
    invoke("load_settings", {})
        .then((settings) => {
            settingsData.value.apinaeCommand = settings.apinaePath;
            settingsData.value.commandType = settings.apinaePath ? 'specific' : 'installed';
        })
        .catch((error) => window.alert(error));
});

</script>
<style></style>
<template>
    <div id="idSettingsModal" class="modal" tabindex="-1">
        <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title">Settings</h5>
                </div>
                <div class="modal-body">
                    <form class="row g-3">
                        <div class="col-md-12">
                            <div class="form-check form-check-inline">
                                <input class="form-check-input" type="radio" name="commandType" id="idInstalledRadio" value="installed" v-model="settingsData.commandType">
                                <label class="form-check-label" for="installedRadio">Installed</label>
                            </div>
                            <div class="form-check form-check-inline">
                                <input class="form-check-input" type="radio" name="commandType" id="idSpecificRadio" value="specific" v-model="settingsData.commandType">
                                <label class="form-check-label" for="specificRadio">Specific</label>
                            </div>
                        </div>
                        <div class="col-md-12">
                            <div class="mb-3 row">
                                <label for="idEditApinaePath" class="col-sm-2 form-label small">Path</label>
                                <div class="col-sm-10">
                                    <input type="text" class="form-control form-control-sm" id="idEditApinaePath" v-model="settingsData.apinaeCommand" :disabled="settingsData.commandType === 'installed'">
                                </div>
                            </div>                            
                        </div>
                    </form>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-sm btn-secondary" data-bs-dismiss="modal">Close</button>
                    <button type="button" class="btn btn-sm btn-primary" @click="saveSettings()">Save changes</button>
                </div>
            </div>
        </div>
    </div>
</template>