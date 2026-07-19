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

    <!-- Lecteur audio persistant. Monté ICI, hors du <slot/> : c'est ce
         placement qui fait survivre l'écoute au changement de page (FR-017).
         Dans une page, il serait démonté à la navigation et le son se couperait. -->
    <ClientOnly>
      <MediaBarreLecturePersistante />
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '~/stores/user'

// Layout par défaut avec NavBar et Footer
const userStore = useUserStore()
const estConnecte = computed(() => userStore.isAuthenticated)
</script>
