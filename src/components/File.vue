<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
</script>

<script lang="ts">
export default {
  name: "File",
  props: ["item"],
};
</script>

<template>
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
</template>
