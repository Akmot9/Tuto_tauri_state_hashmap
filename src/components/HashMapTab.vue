<template>
    <div class="hashmap-viewer">
      <h2>HashMap State</h2>
      <div v-if="Object.keys(hashMap).length">
        <div v-for="(value, key) in hashMap" :key="key">
          {{ key }}: {{ value }}
        </div>
      </div>
      <div v-else>
        No data in HashMap.
      </div>
    </div>
  </template>
  
  <script setup>
  import { ref, onMounted, onUnmounted } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';
  
  const hashMap = ref({});
  
  const fetchHashMapState = async () => {
    try {
      const response = await invoke('get_hash_map_state');
      hashMap.value = response;
    } catch (error) {
      console.error('Error fetching HashMap state:', error);
    }
  };
  
  let intervalId;
  
  onMounted(() => {
    fetchHashMapState();
    intervalId = setInterval(fetchHashMapState, 1000);
  });
  
  onUnmounted(() => {
    clearInterval(intervalId);
  });
  </script>
  
  <style>
  .hashmap-viewer {
    text-align: center;
  }
  </style>
  