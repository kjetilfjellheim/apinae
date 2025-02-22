<script setup>
const props = defineProps({
  test_id: String
});

</script>
<style>

</style>
<template>
    <nav class="navbar navbar-expand-sm bg-body-tertiary small">
      <div class="container-fluid">        
        <ol class="breadcrumb  margin-0 padding-0 align-middle">
          <li class="breadcrumb-item"><router-link to="/"><i class="fas fa-house"></i></router-link></li>
          <li class="breadcrumb-item">{{ $route.params.test_id }}</li>
          <li class="breadcrumb-item"><router-link to="/{{ test.id }}/servers">Servers</router-link></li>
        </ol>
      </div>
    </nav>     
    {{ $route.params.test_id }}
</template>