<template>
  <div class="save-to-csv">
    <button @click="saveHashMapToCsv">Save HashMap to CSV</button>
    <p v-if="saveStatus">{{ saveStatus }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { save } from '@tauri-apps/api/dialog';

const saveStatus = ref('');

async function saveHashMapToCsv() {
  try {
    const filePath = await save({
      title: 'Save HashMap to CSV',
      defaultPath: 'Relever.csv',
      filters: [{
        name: 'CSV',
        extensions: ['csv']
      }]
    });
    if (filePath) {
      const response = await invoke('save_hash_map_to_csv', { filePath: filePath });
      saveStatus.value = response;
    } else {
      saveStatus.value = 'Save operation cancelled.';
    }
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
