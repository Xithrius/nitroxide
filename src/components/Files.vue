<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

let items = ref(Array<string>());
// Scuffed, but works
// https://stackoverflow.com/a/67525124
(async () => {
  const res: string[] = await invoke("home_folder_items");
  items.value = res;
})();

</script>

<template>
  <div class="card">
    <li v-for="item in items">
      {{ item }}
    </li>
  </div>
</template>
