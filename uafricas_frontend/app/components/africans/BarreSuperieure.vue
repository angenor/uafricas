<template>
  <!-- Barre supérieure de la maquette : 89 px, blanche, non collante.
       Le logo est aligné sur le bord GAUCHE du conteneur (x=174 dans Figma),
       pas sur le bord du viewport. -->
  <header class="h-af-barre bg-white">
    <div class="mx-auto flex h-full max-w-af-conteneur items-center gap-6 px-6">
      <NuxtLink to="/" class="shrink-0">
        <img src="/logos/logo_uafracas.png" alt="AfricanS" class="h-[59px] w-auto" />
      </NuxtLink>

      <!-- Profil. Dans la maquette il n'est pas collé au logo : il commence au
           tiers de la barre, au droit de la colonne principale. -->
      <NuxtLink
        v-if="estConnecte"
        to="/mon-compte/profil"
        class="ml-auto flex items-center gap-3 lg:ml-[calc(var(--spacing-af-colonne)+2.5rem)] lg:mr-auto"
      >
        <img
          v-if="photo"
          :src="photo"
          :alt="nomAffiche"
          class="size-11 rounded-full object-cover"
        />
        <span v-else class="grid size-11 place-items-center rounded-full bg-af-chocolat/15 text-af-chocolat">
          <font-awesome-icon icon="fa-solid fa-user" />
        </span>
        <span class="hidden text-base font-bold sm:inline">{{ nomAffiche }}</span>
      </NuxtLink>

      <div class="ml-auto flex items-center gap-4">
        <template v-if="estConnecte">
          <button
            v-for="action in actions"
            :key="action.libelle"
            type="button"
            class="grid size-6 place-items-center text-af-chocolat transition hover:opacity-70 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-af-chocolat"
            :aria-label="action.libelle"
            :title="action.libelle"
          >
            <font-awesome-icon :icon="action.icone" class="text-xl" />
          </button>
        </template>

        <NuxtLink
          v-else
          to="/login"
          class="rounded-lg bg-af-degrade px-6 py-2.5 text-base font-bold text-white transition hover:opacity-90"
        >
          Se Connecter
        </NuxtLink>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { useUserStore } from '~/stores/user'

const userStore = useUserStore()
const estConnecte = computed(() => userStore.isAuthenticated)
// La maquette affiche le nom complet (« Wade Warren »), pas le seul prénom.
const nomAffiche = computed(() => userStore.fullName || userStore.displayName || 'Mon compte')
const photo = computed(() => userStore.user?.photo_url ?? null)

const actions = [
  { libelle: 'Notifications', icone: 'fa-solid fa-bell' },
  { libelle: 'Réglages', icone: 'fa-solid fa-gear' },
  { libelle: 'Aide', icone: 'fa-solid fa-circle-question' },
]
</script>
