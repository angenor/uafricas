<template>
  <!-- Programme Sabbafrica dans le fil. `SabbatiqueCard` est une vignette de
       320 px de haut, taillée pour une grille à deux colonnes : elle ne tient
       pas le format pleine largeur du fil. -->
  <AfricansCartePublication
    :auteur="auteur"
    :titre="programme.titre"
    :texte="resume"
    :etiquettes="etiquettes"
    :images="couverture ? [couverture] : undefined"
    :actions="['partager']"
    :quand="`Publié ${dateRelative}`"
    @partager="$emit('partager')"
  >
    <template #bandeau>
      <p class="flex items-center gap-2 bg-af-vert px-4 py-2 text-[13px]/[1.4] font-bold text-white">
        <font-awesome-icon icon="fa-solid fa-plane" />
        {{ programme.interafricain ? 'Échange interafricain' : 'Échange vers l\'Afrique' }} sur Sabbafrica
      </p>
    </template>

    <template #actions>
      <NuxtLink
        :to="`/echanges-sabbatiques/${programme.id}`"
        class="flex items-center gap-2 text-af-chocolat transition hover:opacity-70"
      >
        <font-awesome-icon icon="fa-solid fa-circle-info" />
        Voir le programme
      </NuxtLink>
    </template>
  </AfricansCartePublication>
</template>

<script setup lang="ts">
import type { SabbatiqueAPI } from '~/composables/useSabbatiques'

const props = defineProps<{ programme: SabbatiqueAPI }>()
defineEmits<{ partager: [] }>()

const couverture = computed(() => urlMedia(props.programme.couverture_url))

// La description vient de l'éditeur riche : c'est du HTML, que la carte du
// fil rend échappé. Seul le texte est retenu, et borné à un aperçu.
const resume = computed(() => texteBrut(props.programme.description))

const auteur = computed(() => ({
  nom: `${props.programme.user.prenom ?? ''} ${props.programme.user.nom ?? ''}`.trim() || 'Un organisateur',
  photo: urlMedia(props.programme.user.photo_url),
  action: 'propose un échange sabbatique',
}))

/**
 * La date de début et la durée passent devant : ce sont les deux critères qui
 * décident si l'on peut candidater. Le domaine et le lieu suivent.
 */
const etiquettes = computed(() => [
  `Dès le ${new Date(props.programme.date_debut).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })}`,
  props.programme.duree_label,
  props.programme.domaine,
  [props.programme.ville, props.programme.pays].filter(Boolean).join(', ') || null,
].filter(Boolean) as string[])

const dateRelative = computed(() => dateRelativeDepuis(props.programme.created_at))
</script>
