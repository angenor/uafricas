<template>
  <div class="min-h-screen flex flex-col">
    <LayoutBoutonLateralGauche />
    <LayoutNavBar />
    <main class="flex-grow">
      <slot />
    </main>
    <LayoutAppFooter />

    <!-- Messagerie temps réel : bouton flottant global, client + si connecté -->
    <ClientOnly>
      <SocialMessagerieFlottante v-if="estConnecte" />
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '~/stores/user'

// Layout par défaut avec NavBar et Footer
const userStore = useUserStore()
const estConnecte = computed(() => userStore.isAuthenticated)
</script>
