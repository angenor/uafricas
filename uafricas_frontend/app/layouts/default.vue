<template>
  <div class="min-h-screen flex flex-col">
    <LayoutNavBar />
    <main class="flex-grow">
      <slot />
    </main>
    <LayoutAppFooter />
  </div>
</template>

<script setup lang="ts">
// Layout par défaut avec NavBar et Footer
</script>
