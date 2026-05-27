<script setup lang="ts">
// Carte d'une ressource contribuée (feature 001-ressources-fermeture-session, US1).
// Rendu adapté au `type` (document / video_youtube / lien_web / accompagnateur).
// Tailwind v4 pur — Principe VI.
import { computed } from 'vue'
import type { RessourceContribueeAPI } from '~/composables/useAfrolangRessources'
import { useAfrolangRessources } from '~/composables/useAfrolangRessources'

const props = defineProps<{
  ressource: RessourceContribueeAPI
  /** ID de l'utilisateur courant pour décider l'affichage du bouton supprimer. */
  utilisateurId?: string | null
  /** Si l'utilisateur courant est admin plateforme. */
  estAdmin?: boolean
}>()

const emit = defineEmits<{
  (e: 'supprime', id: string): void
}>()

const { resoudreUrl } = useAfrolangRessources()

const peutSupprimer = computed(
  () =>
    props.estAdmin === true
    || (props.utilisateurId != null && props.utilisateurId === props.ressource.auteur.id),
)

const iconeType = computed(() => {
  switch (props.ressource.type) {
    case 'document': return 'fa-solid fa-file-lines'
    case 'video_youtube': return 'fa-brands fa-youtube'
    case 'lien_web': return 'fa-solid fa-link'
    case 'accompagnateur': return 'fa-solid fa-user-graduate'
    default: return 'fa-solid fa-file'
  }
})

const libelleType = computed(() => {
  switch (props.ressource.type) {
    case 'document': return 'Document'
    case 'video_youtube': return 'Vidéo'
    case 'lien_web': return 'Lien'
    case 'accompagnateur': return 'Accompagnateur'
    default: return 'Ressource'
  }
})

const fichierUrlAbsolu = computed(() => resoudreUrl(props.ressource.fichier_url))

const tailleFichierFormatee = computed(() => {
  const t = props.ressource.fichier_taille_octets
  if (!t) return null
  if (t < 1024) return `${t} o`
  if (t < 1024 * 1024) return `${(t / 1024).toFixed(1)} Ko`
  return `${(t / (1024 * 1024)).toFixed(1)} Mo`
})

const dateFormatee = computed(() => {
  const d = new Date(props.ressource.created_at)
  return d.toLocaleDateString('fr-FR', { day: '2-digit', month: 'short', year: 'numeric' })
})

const embedYoutubeUrl = computed(() =>
  props.ressource.video_id_youtube
    ? `https://www.youtube.com/embed/${props.ressource.video_id_youtube}`
    : null,
)

const statutAccompagnateurBadge = computed(() => {
  const a = props.ressource.accompagnateur
  if (!a) return null
  switch (a.statut) {
    case 'acceptee': return { libelle: 'Accepté', couleurs: 'bg-green-50 text-green-700 border-green-200' }
    case 'en_attente': return { libelle: 'En attente', couleurs: 'bg-amber-50 text-amber-700 border-amber-200' }
    case 'refusee': return { libelle: 'Refusé', couleurs: 'bg-red-50 text-red-700 border-red-200' }
    case 'retiree': return { libelle: 'Retiré', couleurs: 'bg-gray-100 text-gray-700 border-gray-200' }
    default: return null
  }
})

const onSupprimer = () => {
  if (confirm(`Supprimer définitivement « ${props.ressource.titre} » ?`)) {
    emit('supprime', props.ressource.id)
  }
}
</script>

<template>
  <article class="relative bg-white rounded-xl border border-gray-200 p-4 hover:shadow-md transition-all">
    <!-- En-tête : icône + type + actions -->
    <div class="flex items-start gap-3 mb-3">
      <div class="flex items-center justify-center w-10 h-10 rounded-lg bg-custom-chocolat/10 text-custom-chocolat shrink-0">
        <font-awesome-icon :icon="iconeType" class="text-lg" />
      </div>
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-2 mb-1">
          <span class="text-xs font-semibold uppercase tracking-wide text-custom-chocolat">{{ libelleType }}</span>
          <span v-if="statutAccompagnateurBadge"
                class="text-[10px] px-2 py-0.5 rounded-full border font-medium"
                :class="statutAccompagnateurBadge.couleurs">
            {{ statutAccompagnateurBadge.libelle }}
          </span>
        </div>
        <h4 class="font-medium text-gray-900 text-sm line-clamp-2">{{ ressource.titre }}</h4>
      </div>
      <button v-if="peutSupprimer"
              type="button"
              class="text-gray-400 hover:text-red-600 transition-colors p-1.5 rounded-md hover:bg-red-50"
              title="Supprimer"
              @click="onSupprimer">
        <font-awesome-icon icon="fa-solid fa-trash" class="text-sm" />
      </button>
    </div>

    <!-- Corps : rendu spécifique au type -->
    <div class="mb-3">
      <!-- Document -->
      <div v-if="ressource.type === 'document' && fichierUrlAbsolu" class="flex items-center gap-3">
        <a :href="fichierUrlAbsolu"
           target="_blank"
           rel="noopener"
           class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-custom-chocolat text-white text-xs font-medium hover:bg-custom-chocolat/90 transition-colors">
          <font-awesome-icon icon="fa-solid fa-download" class="text-xs" />
          Télécharger
        </a>
        <span v-if="tailleFichierFormatee" class="text-xs text-gray-500">{{ tailleFichierFormatee }}</span>
      </div>

      <!-- Vidéo YouTube : iframe embed -->
      <div v-else-if="ressource.type === 'video_youtube' && embedYoutubeUrl" class="aspect-video rounded-lg overflow-hidden bg-gray-900">
        <iframe :src="embedYoutubeUrl"
                class="w-full h-full"
                frameborder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                allowfullscreen />
      </div>

      <!-- Lien web -->
      <a v-else-if="ressource.type === 'lien_web' && ressource.lien_url"
         :href="ressource.lien_url"
         target="_blank"
         rel="noopener noreferrer"
         class="inline-flex items-center gap-2 text-sm text-custom-chocolat hover:underline break-all">
        <font-awesome-icon icon="fa-solid fa-external-link-alt" class="text-xs shrink-0" />
        <span class="line-clamp-2">{{ ressource.lien_url }}</span>
      </a>

      <!-- Accompagnateur : mini-profil + motif -->
      <div v-else-if="ressource.type === 'accompagnateur' && ressource.accompagnateur" class="flex gap-3 p-3 rounded-lg bg-amber-50/50 border border-amber-100">
        <div class="w-10 h-10 rounded-full bg-amber-200 text-amber-900 flex items-center justify-center font-semibold text-sm shrink-0">
          {{ ressource.accompagnateur.membre.prenom.charAt(0) }}{{ ressource.accompagnateur.membre.nom.charAt(0) }}
        </div>
        <div class="flex-1 min-w-0">
          <div class="text-sm font-medium text-gray-900">
            {{ ressource.accompagnateur.membre.prenom }} {{ ressource.accompagnateur.membre.nom }}
          </div>
          <p class="text-xs text-gray-700 mt-1 line-clamp-3 italic">« {{ ressource.accompagnateur.motif }} »</p>
        </div>
      </div>
    </div>

    <!-- Description (optionnelle) -->
    <p v-if="ressource.description" class="text-xs text-gray-600 mb-3 line-clamp-2">{{ ressource.description }}</p>

    <!-- Pied : auteur + date -->
    <div class="flex items-center justify-between gap-2 pt-2 border-t border-gray-100 text-xs text-gray-500">
      <div class="flex items-center gap-1.5 min-w-0">
        <font-awesome-icon icon="fa-solid fa-user" class="text-[10px] shrink-0" />
        <span class="truncate">{{ ressource.auteur.prenom }} {{ ressource.auteur.nom }}</span>
      </div>
      <time>{{ dateFormatee }}</time>
    </div>
  </article>
</template>
