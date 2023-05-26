<script setup lang="ts">
import TitleBar from "./components/TitleBar.vue";
import DirectoryList from "./components/DirectoryList.vue";
</script>

<template>
  <div class="h-screen max-h-screen w-screen bg-stone-800 overflow-x-hidden">
    <TitleBar />

    <div class="bg-stone-800">
      <DirectoryList />
    </div>
  </div>
</template>
