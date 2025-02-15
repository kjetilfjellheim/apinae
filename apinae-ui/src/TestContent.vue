<script setup>
const props = defineProps(['test_data'])
</script>

<style>
    .input {
        width: 90%;
        display: inline-block;
        box-sizing: border-box;
    }
</style>
<template>
    <div class="container-fluid padding-0 margin-0">
        <div class="row">
            <div class="col-3">
                <small>&nbsp;Id</small>
            </div>
            <div class="col-9">
                <label>{{ test_data?.id }}</label>
            </div>            
            <div class="col-3">
                <small>&nbsp;Name</small>
            </div>
            <div class="col-9">
                <input type="text" placeholder="Name" class="input" v-model="test_data.name"/>
            </div>            
            <div class="col-3">
                <small>&nbsp;Description</small>                
            </div>
            <div class="col-9">
                <textarea placeholder="Description" class="input" v-model="test_data.description"></textarea>
            </div>            
        </div>
    </div>
</template>