<script setup lang="ts">
import {exit} from '@tauri-apps/api/process';
import {appWindow} from '@tauri-apps/api/window';</script>

<template>
  <div data-tauri-drag-region class="flex w-full h-8 bg-stone-900">
    <div class="flex flex-row h-full w-fit p-4 -mx-2 items-center gap-2 group">

      <div class="w-4 h-4 rounded-full bg-red-600 hover:cursor-pointer" @click="() => exit(0)">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
             class="w-4 h-4 hidden group-hover:block">
          <path fill-rule="evenodd"
                d="M5.47 5.47a.75.75 0 011.06 0L12 10.94l5.47-5.47a.75.75 0 111.06 1.06L13.06 12l5.47 5.47a.75.75 0 11-1.06 1.06L12 13.06l-5.47 5.47a.75.75 0 01-1.06-1.06L10.94 12 5.47 6.53a.75.75 0 010-1.06z"
                clip-rule="evenodd"/>
        </svg>
      </div>

      <div class="w-4 h-4 rounded-full bg-yellow-600 hover:cursor-pointer" @click="appWindow.minimize()">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
             class="w-4 h-4 hidden group-hover:block">
          <path fill-rule="evenodd" d="M3.75 12a.75.75 0 01.75-.75h15a.75.75 0 010 1.5h-15a.75.75 0 01-.75-.75z"
                clip-rule="evenodd"/>
        </svg>
      </div>

      <div class="w-4 h-4 rounded-full bg-green-600 hover:cursor-pointer" @click="appWindow.toggleMaximize()">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
             class="w-4 h-4 hidden group-hover:block">
          <path fill-rule="evenodd"
                d="M11.47 4.72a.75.75 0 011.06 0l3.75 3.75a.75.75 0 01-1.06 1.06L12 6.31 8.78 9.53a.75.75 0 01-1.06-1.06l3.75-3.75zm-3.75 9.75a.75.75 0 011.06 0L12 17.69l3.22-3.22a.75.75 0 111.06 1.06l-3.75 3.75a.75.75 0 01-1.06 0l-3.75-3.75a.75.75 0 010-1.06z"
                clip-rule="evenodd"/>
        </svg>
      </div>
    </div>
  </div>
</template>
