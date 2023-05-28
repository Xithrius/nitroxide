<script lang="ts">
export default {
  name: "Navigation",
  props: ["previous"],
};
</script>

<template>
    <div>
        {{ previous.value }}
    </div>
</template>
