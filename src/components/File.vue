<script lang="ts">
export default {
  name: "File",
  props: ["item"],
};
</script>

<template>
  <div
    class="hover:outline hover:outline-1 hover:outline-blue-500 mt-4"
  >
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
</template>
