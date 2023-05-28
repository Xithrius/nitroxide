<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import File from "./File.vue";
import Navigation from "./Navigation.vue";
import { useMagicKeys, whenever } from "@vueuse/core";

type DirItem = {
  path: string;
  name: string;
  is_directory: boolean;
  is_hidden: boolean;
};

let dirContext = ref("");
let previousContext = ref("");
let items: Ref<DirItem[]> = ref(Array<DirItem>());

const getItems = () => {
  let itemsPromise: Promise<DirItem[]> = invoke("folder_items", {
    context: dirContext.value.length == 0 ? null : dirContext.value,
  });
  itemsPromise.then((val) => (items.value = val));
};

getItems();

const changeContext = (item: DirItem) => {
  if (item.is_directory) {
    previousContext.value = dirContext.value;
    dirContext.value = item.path;

    getItems();
  }
};

const { Ctrl_h } = useMagicKeys();

let showHidden: Ref<boolean> = ref(false);

whenever(Ctrl_h, () => {
  showHidden.value = !showHidden.value;
});
</script>

<template>
  <div class="flex w-screen flex-wrap content-start mx-8">
    <Navigation :previous="previousContext" />

    <div v-for="item of items">
      <!-- https://en.wikipedia.org/wiki/Material_conditional -->
      <div
        v-if="item.is_hidden <= showHidden"
        class="h-32 w-48"
        @click="() => changeContext(item)"
      >
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
