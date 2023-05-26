<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

type DirItem = {
  name: string;
  is_directory: boolean;
  is_hidden: boolean;
};

let items: Ref<DirItem[]> = ref(Array<DirItem>());
let itemsPromise: Promise<DirItem[]> = invoke("folder_items");
itemsPromise.then((val) => (items.value = val));

let showHidden = ref(true);
const toggleHidden = () => {
  showHidden.value = !showHidden.value;
  console.log("Toggled hidden items");
};
</script>

<template>
  <div
    @click="toggleHidden"
    class="card flex w-screen flex-wrap content-start mx-8"
  >
    <div v-for="item of items">
      <!-- https://en.wikipedia.org/wiki/Material_conditional -->
      <div v-if="item.is_hidden <= showHidden" class="h-32 w-48">
        <div class="px-8">
          <v-icon
            v-if="item.is_directory"
            name="vi-default-folder"
            class="h-24 w-24 mx-4 mt-4"
          />
          <v-icon v-else name="vi-file-type-text" class="h-24 w-24 mx-4 mt-4" />
        </div>
        <div class="text-white font-mono justify-center text-center">
          {{ item.name }}
        </div>
      </div>
    </div>
  </div>
</template>
