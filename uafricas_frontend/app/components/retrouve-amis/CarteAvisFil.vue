<template>
  <AfricansCartePublication
    :auteur="auteur"
    :titre="dansLeFil ? `Recherche : ${nomRecherche}` : nomRecherche"
    :texte="avis.description_physique || undefined"
    :etiquettes="etiquettes"
    :images="photoComplete ? [photoComplete] : undefined"
    :partages="avis.compteur_partages"
    :actions="['partager']"
    :quand="`Publié ${dateRelative}`"
    @partager="$emit('partager')"
  >
    <!-- Un avis de recherche n'est pas un contenu de plus : c'est un APPEL.
         Le badge en haut à droite ne se lit qu'une fois la carte regardée ;
         ce bandeau la signale pendant le défilement. -->
    <template v-if="dansLeFil" #bandeau>
      <p class="flex items-center gap-2 bg-af-degrade px-4 py-2 text-[13px]/[1.4] font-bold text-white">
        <font-awesome-icon icon="fa-solid fa-magnifying-glass" />
        Avis de recherche : aidez à retrouver cette personne
      </p>
    </template>

    <template #sous-media>
      <!-- Un avis clôturé reste en ligne : c'est l'issue heureuse du module, et
           la masquer priverait la communauté de la seule preuve que ça marche. -->
      <p
        v-if="avis.etat === 'cloture'"
        class="mx-4 mt-4 flex items-center gap-2 rounded border border-af-vert/30 bg-af-vert/[0.08] px-3 py-2 text-[12px]/[1.4] font-bold text-af-vert"
      >
        <font-awesome-icon icon="fa-solid fa-heart" />
        Cette personne a été retrouvée.
      </p>
    </template>

    <template #actions>
      <NuxtLink
        :to="`/retrouve-amis/public/${avis.slug}`"
        class="flex items-center gap-2 text-af-chocolat transition hover:opacity-70"
      >
        <font-awesome-icon icon="fa-solid fa-circle-info" />
        Voir l'avis
      </NuxtLink>
    </template>
  </AfricansCartePublication>
</template>

<script setup lang="ts">
import type { AvisPublicResume } from '~/composables/useRetrouvAmis'
import { TYPES_RELATION } from '~/composables/useRetrouvAmis'

/**
 * Avis de recherche Africonnect dans le fil de la refonte.
 *
 * Un avis ne porte NI like NI commentaire : la barre d'interactions n'expose
 * donc que le partage, seul compteur réellement tenu par le serveur
 * (`compteur_partages`).
 */
const props = withDefaults(defineProps<{
  avis: AvisPublicResume
  /**
   * Dans le fil d'actualité, la carte côtoie neuf autres sources : elle doit
   * DIRE ce qu'elle est. Sur /retrouve-amis, où chaque carte est un avis, le
   * badge et le préfixe « Recherche : » ne seraient que du bruit répété.
   */
  dansLeFil?: boolean
}>(), { dansLeFil: false })

defineEmits<{ partager: [] }>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const photoComplete = computed(() => {
  const url = props.avis.photo_url
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
})

const nomRecherche = computed(() =>
  [props.avis.prenom_recherche, props.avis.nom_recherche].filter(Boolean).join(' '))

/** L'auteur d'un avis peut choisir l'anonymat : c'est une garantie du module,
 *  pas une donnée manquante : le pseudonyme n'est alors même pas transmis. */
const auteur = computed(() => ({
  nom: props.avis.auteur_anonyme ? 'Anonyme' : (props.avis.auteur_pseudonyme ?? 'Anonyme'),
  action: props.dansLeFil ? 'recherche une personne perdue de vue' : undefined,
}))

const GENRES: Record<string, string> = { homme: 'Homme', femme: 'Femme' }

const etiquettes = computed(() => {
  const lieu = [props.avis.ville_rencontre || props.avis.ville, props.avis.pays?.nom]
    .filter(Boolean)
    .join(', ')

  return [
    props.avis.type_relation
      ? TYPES_RELATION.find(t => t.value === props.avis.type_relation)?.label ?? props.avis.type_relation
      : null,
    props.avis.genre_recherche ? GENRES[props.avis.genre_recherche] ?? props.avis.genre_recherche : null,
    props.avis.ecole_rencontre || null,
    lieu || null].filter(Boolean) as string[]
})

const dateRelative = computed(() => dateRelativeDepuis(props.avis.created_at))
</script>
