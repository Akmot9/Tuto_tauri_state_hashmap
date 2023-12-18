<script setup>

import Greet from "./components/Greet.vue";
import CreatTask from "./components/CreatTask.vue";
import HashMapTab from "./components/HashMapTab.vue";
import SaveToCsv from "./components/SaveToCsv.vue";
</script>

<template>
  <div >
    <CreatTask />
    <Greet />
    <HashMapTab />
    <SaveToCsv />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
