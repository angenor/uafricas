<template>
  <div class="flex items-start gap-5 overflow-x-auto pb-2 scrollbar-none">
    <!-- Entrée d'action : la maquette pose un « + » en tête de rangée. Il
         ouvre le composeur, il n'ouvre pas un format éphémère, la plateforme
         n'a rien qui ressemble à une story. -->
    <button
      type="button"
      class="flex w-20 shrink-0 flex-col items-center gap-2"
      @click="$emit('publier')"
    >
      <span class="grid size-16 place-items-center rounded-full border-2 border-dashed border-af-chocolat text-af-chocolat transition hover:bg-af-chocolat/[0.07]">
        <font-awesome-icon icon="fa-solid fa-plus" class="text-xl" />
      </span>
      <span class="truncate text-[12px]/[1.4] text-af-corps">Publier</span>
    </button>

    <NuxtLink
      v-for="ami in amis"
      :key="ami.id"
      :to="`/profil/${ami.id}`"
      class="flex w-20 shrink-0 flex-col items-center gap-2 group"
    >
      <span class="rounded-full p-0.5 ring-2 ring-af-chocolat/40 transition group-hover:ring-af-chocolat">
        <AfricansAvatar :nom="`${ami.prenom} ${ami.nom}`" :src="photo(ami.photoUrl)" :taille="60" />
      </span>
      <span class="w-full truncate text-center text-[12px]/[1.4] text-af-corps">
        {{ ami.prenom }} {{ (ami.nom ?? '').charAt(0) }}.
      </span>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
import type { MembreLightAPI } from '~/composables/useAmis'

/**
 * Rangée d'ami(e)s en tête du fil. La maquette y dessine des pastilles rondes
 * à la manière des stories ; ce n'en sont pas, aucun contenu éphémère
 * n'existe sur la plateforme. Chaque pastille mène au PROFIL, ce que l'anneau
 * discret annonce sans promettre autre chose.
 */
defineProps<{ amis: MembreLightAPI[] }>()
defineEmits<{ publier: [] }>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const photo = (url: string | null): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}
</script>
