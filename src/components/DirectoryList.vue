<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

type DirItem = {
  name: string;
  is_directory: boolean;
  is_hidden: boolean;
};

let items: Ref<DirItem[]> = ref(Array<DirItem>());
let itemsPromise: Promise<DirItem[]> = invoke("folder_items", {
  // context: "/home/xithrius/Repositories/twitch-tui",
});
itemsPromise.then((val) => (items.value = val));

import { useMagicKeys, whenever } from "@vueuse/core";
const { Ctrl_h } = useMagicKeys();

let showHidden = ref(false);
whenever(Ctrl_h, () => {
  showHidden.value = !showHidden.value;
});

import File from "./File.vue";
</script>

<template>
  <div class="card flex w-screen flex-wrap content-start mx-8">
    <div v-for="item of items">
      <!-- https://en.wikipedia.org/wiki/Material_conditional -->
      <div v-if="item.is_hidden <= showHidden" class="h-32 w-48">
        <div v-if="item.is_hidden" class="opacity-50">
          <File :item="item" />
        </div>
        <div v-else>
          <File :item="item" />
        </div>
      </div>
    </div>
  </div>
</template>
