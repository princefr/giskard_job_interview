<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import TheHome from './components/TheHome.vue';


</script>

<template>
  <header>
    <div>
      <TheHome />

      <!-- <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/about">About</RouterLink>
      </nav> -->
    </div>
  </header>

  <!-- <RouterView /> -->
</template>

