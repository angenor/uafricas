<template>
  <AfricansCartePublication
    :auteur="auteur"
    :categorie="categorie"
    :texte="legende ? `« ${legende} »` : undefined"
    :actions="[]"
    :quand="quand"
  >
    <template #media>
      <!-- L'aperçu EST le média de la carte : c'est l'objet partagé, et c'est
           lui qui porte le lien. La légende, elle, appartient au partageur. -->
      <NuxtLink :to="apercu.vers" class="group block">
        <div v-if="apercu.image" class="relative aspect-[16/10] w-full overflow-hidden bg-af-bordure">
          <img :src="apercu.image" alt="" class="size-full object-cover transition duration-300 group-hover:scale-105" />
          <span
            v-if="apercu.surImage"
            class="absolute right-3 bottom-3 rounded bg-black/75 px-2 py-1 text-[12px]/[1.4] text-white"
          >{{ apercu.surImage }}</span>
        </div>

        <div class="flex items-center gap-3 border-t border-af-bordure px-4 py-3">
          <font-awesome-icon v-if="apercu.icone" :icon="apercu.icone" class="size-6 shrink-0 text-af-chocolat" />
          <div class="min-w-0 flex-1">
            <p class="truncate text-[14px]/[1.4] font-bold transition group-hover:text-af-chocolat">
              {{ apercu.titre }}
            </p>
            <p v-if="apercu.sousTitre" class="truncate text-[12px]/[1.4] text-af-corps">{{ apercu.sousTitre }}</p>
            <p v-if="apercu.meta?.length" class="mt-0.5 truncate text-[12px]/[1.4] text-af-atone">
              {{ apercu.meta.join(' · ') }}
            </p>
          </div>
          <font-awesome-icon
            icon="fa-solid fa-chevron-right"
            class="shrink-0 text-af-atone transition group-hover:translate-x-1 group-hover:text-af-chocolat"
          />
        </div>
      </NuxtLink>
    </template>
  </AfricansCartePublication>
</template>

<script setup lang="ts">
import type { AuteurPublication } from '~/components/africans/CartePublication.vue'

/**
 * Carte de PARTAGE du fil : un membre relaie un objet de la plateforme, 
 * territoire, découverte, profil, contribution, vidéo, média. Les six sources
 * partagent exactement cette anatomie ; six composants séparés la répétaient
 * six fois, avec six jeux de couleurs qui ne disaient rien de plus que le badge.
 *
 * Aucun compteur : un partage ne recueille ni like ni commentaire, la barre
 * d'interactions ne porte donc que l'horodatage.
 */
export interface ApercuPartage {
  titre: string
  sousTitre?: string
  /** Informations secondaires, jointes par « · ». */
  meta?: string[]
  image?: string | null
  /** Pastille posée sur l'image (durée d'une vidéo, région d'un territoire…). */
  surImage?: string
  icone?: string
  vers: string
}

defineProps<{
  /** `auteur.action` porte le verbe : « a partagé un territoire ». */
  auteur: AuteurPublication
  /** Badge de type : « Vidéo », « Territoire », « Profil »… */
  categorie: string
  legende?: string | null
  quand: string
  apercu: ApercuPartage
}>()
</script>
