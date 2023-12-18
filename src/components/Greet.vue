<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const hashMapData = ref({});
const name = ref("");

async function updateHashMap() {
  try {
    const response = await invoke("push_to_hash_map", { word: name.value });
    hashMapData.value = response;
  } catch (error) {
    console.error('Error invoking Tauri command:', error);
  }
}
</script>

<template>
  <div class="container">
    <form class="input-form" @submit.prevent="updateHashMap">
      <input id="word-input" v-model="name" placeholder="Enter a word..." />
      <button type="submit">Update HashMap</button>
    </form>
  </div>
</template>

<style>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 10vh;
}

.input-form {
  margin-bottom: 20px;
}

.hashmap-display {
  text-align: center;
}
</style>
