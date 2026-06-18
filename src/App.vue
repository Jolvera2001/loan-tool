<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Calculate from "./Calculate.vue";
import { Button } from "primevue";
</script>

<template>
  <main>
    <Calculate />
  </main>
</template>