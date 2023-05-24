<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

let items = ref(Array<string>());
let itemsPromise: Promise<string[]> = invoke("home_folder_items");
itemsPromise.then((val) => (items.value = val));

import Folder from "../folder.svg";
</script>

<template>
  <div class="card flex w-screen flex-wrap content-start mx-8">
    <div v-for="item of items">
      <div class="h-32 w-48">
        <div class="px-8">
          <Folder class="h-24 w-24 mx-4 mt-4" />
        </div>
        <div class="text-white font-mono justify-center text-center">
          {{ item }}
        </div>
      </div>
    </div>
  </div>
</template>
