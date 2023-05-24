<script setup lang="ts">
import TitleBar from "./components/TitleBar.vue";
import Files from "./components/Files.vue";
</script>

<template>
  <div class="h-screen max-h-screen w-screen bg-stone-800 overflow-hidden">
    <TitleBar />

    <!-- <div class="h-full w-full">
      <div class="h-full w-48 bg-stone-800" />
    </div> -->
    <Files />
  </div>
</template>
