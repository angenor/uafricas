<template>
  <!-- Expert Diapertise dans le fil.
       Un expert n'est pas une publication : c'est une PERSONNE qui se rend
       disponible. Sa carte n'expose donc ni like ni commentaire, seulement le
       partage et le lien vers son profil. -->
  <AfricansCartePublication
    :auteur="auteur"
    :titre="expert.expertiseInfo.domaine"
    :texte="expert.expertiseInfo.biographie"
    :etiquettes="etiquettes"
    :actions="['partager']"
    :quand="`Rejoint ${dateRelative}`"
    @partager="$emit('partager')"
  >
    <template #bandeau>
      <p class="flex items-center gap-2 bg-af-degrade px-4 py-2 text-[13px]/[1.4] font-bold text-white">
        <font-awesome-icon icon="fa-solid fa-user-tie" />
        Nouvelle expertise disponible sur Diapertise
      </p>
    </template>

    <template #actions>
      <NuxtLink
        :to="`/profil/${expert.id}`"
        class="flex items-center gap-2 text-af-chocolat transition hover:opacity-70"
      >
        <font-awesome-icon icon="fa-solid fa-circle-info" />
        Voir le profil
      </NuxtLink>
    </template>
  </AfricansCartePublication>
</template>

<script setup lang="ts">
import type { ExpertAPI } from '~/composables/useExperts'

const props = defineProps<{ expert: ExpertAPI }>()
defineEmits<{ partager: [] }>()

const auteur = computed(() => ({
  nom: `${props.expert.prenom} ${props.expert.nom}`.trim(),
  photo: urlMedia(props.expert.photoURL),
  lieu: [props.expert.ville, props.expert.pays].filter(Boolean).join(', ') || undefined,
  vers: `/profil/${props.expert.id}`,
  action: 'met son expertise à disposition',
}))

/**
 * Les trois premières spécialités seulement : au-delà, la rangée d'étiquettes
 * déborde sur deux lignes et noie l'information utile.
 */
const etiquettes = computed(() => [
  `${props.expert.expertiseInfo.nbAnneesExperience} ans d'expérience`,
  ...props.expert.expertiseInfo.specialites.slice(0, 3),
].filter(Boolean))

const dateRelative = computed(() => dateRelativeDepuis(props.expert.dateInscription))
</script>
