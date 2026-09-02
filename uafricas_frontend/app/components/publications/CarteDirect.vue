<template>
  <AfricansCartePublication
    :auteur="auteur"
    :titre="salle.titre"
    :texte="salle.description || undefined"
    categorie="En direct"
    :actions="[]"
    quand="En direct"
  >
    <template #media>
      <NuxtLink :to="`/afrolang/session/${salle.id}`" class="group relative block aspect-video overflow-hidden bg-af-bordure">
        <img
          v-if="salle.image_couverture_url"
          :src="salle.image_couverture_url"
          alt=""
          class="size-full object-cover transition duration-300 group-hover:scale-105"
        />

        <!-- Pastille de direct : rouge, clignotante, en haut à gauche comme sur
             la maquette. Le compteur qui la suit est le nombre de SESSIONS
             ouvertes dans la salle : la liste des salles ne porte pas de
             décompte de spectateurs, et en afficher un serait inventé. -->
        <span class="absolute top-3 left-3 flex items-center gap-2 rounded bg-af-live px-2 py-1 text-[12px]/[1.4] font-bold text-white">
          <span class="size-2 animate-pulse rounded-full bg-white" />
          Live
        </span>
        <span
          v-if="salle.langue_cible"
          class="absolute top-3 right-3 rounded-full bg-black/65 px-3 py-1 text-[12px]/[1.4] font-bold text-white"
        >{{ salle.langue_cible }}</span>

        <span class="absolute inset-x-0 bottom-0 flex items-center gap-2 bg-black/65 px-3 py-2 text-[12px]/[1.4] text-white">
          <font-awesome-icon icon="fa-solid fa-video" />
          {{ salle.sessions_en_cours }} session{{ salle.sessions_en_cours > 1 ? 's' : '' }} ouverte{{ salle.sessions_en_cours > 1 ? 's' : '' }}
        </span>
      </NuxtLink>
    </template>

    <template #sous-media>
      <div class="flex justify-end px-4 pt-4">
        <AfricansBouton
          variante="secondaire"
          icone="fa-solid fa-video"
          :vers="`/afrolang/session/${salle.id}`"
        >
          Suivre le live
        </AfricansBouton>
      </div>
    </template>
  </AfricansCartePublication>
</template>

<script setup lang="ts">
import type { SalleAPI } from '~/composables/useAfrolang'

/**
 * Salle Afrolang en direct dans le fil.
 *
 * Aucun endpoint public ne liste les sessions actives toutes salles
 * confondues : la carte est construite depuis la LISTE DES SALLES, dont chaque
 * entrée porte `sessions_en_cours`. C'est pourquoi elle annonce des sessions
 * ouvertes et non des spectateurs, que rien ne compte à ce niveau.
 */
const props = defineProps<{ salle: SalleAPI }>()

/** Les administrateurs de la salle en tiennent lieu d'auteur ; sans eux, la
 *  carte n'affiche pas d'en-tête plutôt que d'inventer quelqu'un. */
const auteur = computed(() => {
  const admin = props.salle.administrateurs?.[0]
  if (!admin) return undefined
  return {
    nom: `${admin.prenom} ${admin.nom}`.trim(),
    photo: admin.photo_url,
    action: 'anime une salle en direct',
  }
})
</script>
