<template>
  <article class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
    <slot name="bandeau" />
    <!-- L'en-tête tombe quand il n'y a ni auteur ni catégorie : Vidafrica
         référence des vidéos dont l'auteur réel n'est pas toujours connu, et
         un avatar « anonyme » inventerait une personne. -->
    <header v-if="auteur || categorie" class="flex items-start gap-3 p-4">
      <AfricansAvatar v-if="auteur" :src="auteur.photo" :nom="auteur.nom" :taille="44" />

      <div v-if="auteur" class="min-w-0">
        <p class="flex items-center gap-2 text-[14px]/[1.4] font-bold">
          <NuxtLink v-if="auteur.vers" :to="auteur.vers" class="hover:text-af-chocolat">
            {{ auteur.nom }}
          </NuxtLink>
          <span v-else>{{ auteur.nom }}</span>
          <!-- Ce que l'auteur a FAIT, quand la publication relaie autre chose
               qu'elle-même : « … a partagé une vidéo ». En graisse normale, le
               nom reste le point d'accroche de la ligne. -->
          <span v-if="auteur.action" class="truncate font-normal text-af-atone">{{ auteur.action }}</span>
          <font-awesome-icon
            v-if="auteur.verifie"
            icon="fa-solid fa-circle-check"
            class="text-af-vert"
            title="Compte vérifié"
          />
        </p>
        <p v-if="auteur.lieu" class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-atone">
          <font-awesome-icon icon="fa-solid fa-location-dot" />
          {{ auteur.lieu }}
        </p>
      </div>

      <AfricansEtiquette v-if="categorie" ton="vert" class="ml-auto shrink-0">
        {{ categorie }}
      </AfricansEtiquette>
    </header>

    <h3 v-if="titre" class="px-4 pb-1 text-[14px]/[1.4] font-bold first:pt-4">{{ titre }}</h3>

    <p v-if="texte" class="px-4 pb-3 text-[14px]/[1.4] whitespace-pre-line text-af-corps">
      {{ texte }}
    </p>

    <div v-if="etiquettes?.length" class="flex flex-wrap gap-2 px-4 pb-3">
      <AfricansEtiquette v-for="e in etiquettes" :key="e">{{ e }}</AfricansEtiquette>
    </div>

    <!-- Le média occupe toute la largeur, sans marge : c'est ce qui donne au
         fil son rythme de bandes pleines entre deux blocs de texte encadrés. -->
    <slot name="media">
      <AfricansMosaiqueMedia v-if="images?.length" :images="images" />
    </slot>

    <slot name="sous-media" />

    <AfricansBarreInteractions
      :likes="likes"
      :commentaires="commentaires"
      :partages="partages"
      :actions="actions"
      :quand="quand"
      :jaime="jaime"
      @jaime="$emit('jaime')"
      @commenter="$emit('commenter')"
      @partager="$emit('partager')"
    >
      <slot name="actions" />
    </AfricansBarreInteractions>
  </article>
</template>

<script setup lang="ts">
/**
 * Carte de publication : l'anatomie relevée à l'identique sur quatre modules
 * (fil d'actualité, Codimoi, Africonnect, Vidafrica). Les variantes ne portent
 * que sur le média : image, mosaïque, vidéo, direct, fiche pays : d'où le slot
 * `media` qui prend le pas sur le rendu par défaut.
 */
import type { ActionInteraction } from './BarreInteractions.vue'

export interface AuteurPublication {
  nom: string
  /** Verbe d'action affiché après le nom, ex. « a partagé une vidéo ». */
  action?: string
  photo?: string | null
  lieu?: string
  verifie?: boolean
  vers?: string
}

withDefaults(defineProps<{
  /** Absent = publication sans auteur identifié ; l'en-tête n'est pas rendu. */
  auteur?: AuteurPublication
  titre?: string
  texte?: string
  categorie?: string
  etiquettes?: string[]
  images?: string[]
  likes?: number
  commentaires?: number
  /** Non renseigné = pas de compteur de partages (voir BarreInteractions). */
  partages?: number
  /** Boutons de la barre réellement rendus (voir BarreInteractions). */
  actions?: ActionInteraction[]
  quand?: string
  jaime?: boolean
}>(), { likes: 0, commentaires: 0, jaime: false })

defineEmits<{ jaime: [], commenter: [], partager: [] }>()
</script>
