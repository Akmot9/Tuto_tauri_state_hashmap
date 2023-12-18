<template>
    <div class="save-to-csv">
      <button @click="saveHashMapToCsv">Save HashMap to CSV</button>
      <p v-if="saveStatus">{{ saveStatus }}</p>
    </div>
  </template>
  
  <script setup>
  import { ref } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';
  
  const saveStatus = ref('');
  
  async function saveHashMapToCsv() {
    try {
      const response = await invoke('save_hash_map_to_csv');
      saveStatus.value = response;
    } catch (error) {
      console.error('Error saving HashMap to CSV:', error);
      saveStatus.value = 'Failed to save HashMap to CSV.';
    }
  }
  </script>
  
  <style>
  .save-to-csv {
    text-align: center;
    margin-top: 20px;
  }
  </style>
  