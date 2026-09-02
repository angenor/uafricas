<template>
  <!-- Annonce Afromarket dans le fil. La carte de la vitrine
       (`MarcheAnnonceCard`) est un `NuxtLink` qui enveloppe tout : elle ne
       peut pas porter la barre d'interactions du fil, dont les boutons
       seraient des liens imbriqués. D'où cette carte-ci, sur la coque
       partagée, comme Africonnect fait déjà pour ses avis. -->
  <AfricansCartePublication
    :auteur="auteur"
    :titre="annonce.titre"
    :texte="annonce.description"
    :etiquettes="etiquettes"
    :images="photo ? [photo] : undefined"
    :actions="['partager']"
    :quand="`Publié ${dateRelative}`"
    @partager="$emit('partager')"
  >
    <template #bandeau>
      <p class="flex items-center gap-2 px-4 py-2 text-[13px]/[1.4] font-bold text-white" :class="tonBandeau">
        <font-awesome-icon icon="fa-solid fa-store" />
        {{ annonce.type_echange }} sur Afromarket
      </p>
    </template>

    <template #actions>
      <NuxtLink
        :to="`/marche-africain/${annonce.id}`"
        class="flex items-center gap-2 text-af-chocolat transition hover:opacity-70"
      >
        <font-awesome-icon icon="fa-solid fa-circle-info" />
        Voir l'annonce
      </NuxtLink>
    </template>
  </AfricansCartePublication>
</template>

<script setup lang="ts">
import { formatPrix, type AnnonceAPI } from '~/composables/useMarcheAfricain'

const props = defineProps<{ annonce: AnnonceAPI }>()
defineEmits<{ partager: [] }>()

const photo = computed(() => urlMedia(props.annonce.photo_url))

const auteur = computed(() => ({
  nom: `${props.annonce.user.prenom ?? ''} ${props.annonce.user.nom ?? ''}`.trim() || 'Un membre',
  action: 'a publié une annonce',
}))

/**
 * Le prix ouvre la liste : c'est la première chose qu'on cherche sur une
 * annonce. Un don ou un troc n'en a pas, la mention le dit plutôt que
 * d'afficher « 0 ».
 */
const etiquettes = computed(() => [
  props.annonce.type_echange === 'Vente' || props.annonce.type_echange === "Opportunité d'investissement"
    ? formatPrix(props.annonce.prix, props.annonce.devise)
    : 'Sans contrepartie financière',
  props.annonce.categorie,
  [props.annonce.ville, props.annonce.pays].filter(Boolean).join(', ') || null,
].filter(Boolean) as string[])

// Plein pour ce qui engage de l'argent, vert pour le reste : la même règle
// que la pastille de type d'échange des cartes de la vitrine.
const tonBandeau = computed(() =>
  props.annonce.type_echange === 'Don' || props.annonce.type_echange === 'Troc'
    ? 'bg-af-vert'
    : 'bg-af-degrade')

const dateRelative = computed(() => dateRelativeDepuis(props.annonce.created_at))
</script>
