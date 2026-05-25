<template>
  <section class="bg-gradient-to-br from-custom-chocolat/5 via-white to-custom-green/5 pt-16 pb-10">
    <div class="max-w-7xl mx-auto px-4 flex flex-col md:flex-row items-center gap-12">
      <!-- Contenu texte -->
      <div class="flex-1 text-center md:text-left">
        <h1 class="font-display text-4xl md:text-5xl text-custom-chocolat font-bold mb-4">
          Retrouvez vos amis d'antan
        </h1>
        <p class="text-lg text-gray-600 mb-8 max-w-xl mx-auto md:mx-0">
          La diaspora africaine se reconnecte. Retrouvez vos amis d'école, vos voisins
          d'enfance, vos compagnons de route.
        </p>

        <!-- CTAs utilisateur connecté -->
        <div v-if="estConnecte" class="flex flex-col sm:flex-row gap-4 justify-center md:justify-start">
          <button
            @click="$emit('creer-avis')"
            class="bg-custom-chocolat text-white hover:bg-custom-chocolat/90 rounded-xl px-6 py-3 font-semibold shadow-lg transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'search']" class="mr-2" />
            Créer un avis de recherche
          </button>
          <button
            v-if="!estTrouvable"
            @click="$emit('activer-trouvable')"
            class="border-2 border-custom-green text-custom-green hover:bg-custom-green/10 rounded-xl px-6 py-3 font-semibold transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'users']" class="mr-2" />
            Activer mon profil trouvable
          </button>
        </div>

        <!-- CTA utilisateur non connecté -->
        <div v-else class="flex justify-center md:justify-start">
          <NuxtLink
            to="/login"
            class="bg-custom-chocolat text-white hover:bg-custom-chocolat/90 rounded-xl px-6 py-3 font-semibold shadow-lg transition-colors inline-block"
          >
            Se connecter pour commencer
          </NuxtLink>
        </div>
      </div>

      <!-- Illustration décorative (desktop) -->
      <div class="hidden md:flex flex-1 items-center justify-center">
        <div class="relative w-72 h-72">
          <!-- Cercle de fond -->
          <div class="absolute inset-0 rounded-full bg-custom-chocolat/5"></div>

          <!-- Icones disposées autour du cercle -->
          <div class="absolute top-6 left-1/2 -translate-x-1/2 text-custom-chocolat">
            <font-awesome-icon :icon="['fas', 'users']" class="text-5xl opacity-80" />
          </div>
          <div class="absolute bottom-6 left-1/2 -translate-x-1/2 text-custom-green">
            <font-awesome-icon :icon="['fas', 'handshake']" class="text-5xl opacity-80" />
          </div>
          <div class="absolute top-1/2 -translate-y-1/2 left-6 text-custom-chocolat/60">
            <font-awesome-icon :icon="['fas', 'search']" class="text-4xl" />
          </div>
          <div class="absolute top-1/2 -translate-y-1/2 right-6 text-custom-green/60">
            <font-awesome-icon :icon="['fas', 'heart']" class="text-4xl" />
          </div>

          <!-- Lignes de connexion (SVG) -->
          <svg class="absolute inset-0 w-full h-full" viewBox="0 0 288 288" fill="none">
            <path
              d="M144 48 L48 144"
              stroke="#A54A1C"
              stroke-width="1.5"
              stroke-dasharray="6 4"
              opacity="0.3"
            />
            <path
              d="M144 48 L240 144"
              stroke="#228B22"
              stroke-width="1.5"
              stroke-dasharray="6 4"
              opacity="0.3"
            />
            <path
              d="M48 144 L144 240"
              stroke="#228B22"
              stroke-width="1.5"
              stroke-dasharray="6 4"
              opacity="0.3"
            />
            <path
              d="M240 144 L144 240"
              stroke="#A54A1C"
              stroke-width="1.5"
              stroke-dasharray="6 4"
              opacity="0.3"
            />
          </svg>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
defineProps<{
  estConnecte: boolean
  estTrouvable: boolean
}>()

defineEmits<{
  'creer-avis': []
  'activer-trouvable': []
}>()
</script>
