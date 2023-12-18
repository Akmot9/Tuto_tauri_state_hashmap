<template>
    <div class="task-creator">
      <h2>Create Async Tasks</h2>
      <form @submit.prevent="createTasks">
        <input type="number" v-model.number="numTasks" placeholder="Number of tasks" min="1" />
        <button type="submit">Create Tasks</button>
      </form>
    </div>
  </template>
  
  <script setup>
  import { ref } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';
  
  const numTasks = ref(0);
  
  async function createTasks() {
    if (numTasks.value > 0) {
      await invoke('create_threads', { num_threads: numTasks.value });
    }
  }
  </script>
  
  <style>
  .task-creator {
    text-align: center;
    margin-top: 10px;
  }
  
  .task-creator input {
    margin-right: 10px;
  }
  </style>
  