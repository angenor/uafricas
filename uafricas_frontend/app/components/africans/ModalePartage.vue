<script setup lang="ts">
import { usePartageExterne, construireReseaux, ouvrirPartage, type ReseauExterne } from '~/composables/usePartageExterne'

/**
 * Modale de partage — la coquille commune des six modales de la plateforme.
 *
 * Elles existaient en six exemplaires, dont DEUX byte à byte identiques à
 * quatre phrases près (profil et contribution de gouvernance). Les quatre
 * autres rejouaient en plus, chacune pour soi, les six URL de réseaux
 * sociaux : un défaut d'encodage n'aurait été corrigé qu'à un seul endroit.
 *
 * Le contrat des appelantes est CONSERVÉ : `isOpen` / `close` / `submit`, et
 * les trois méthodes exposées que les pages pilotent par `ref`
 * (`setLoading`, `setError`, `setSuccess`). Aucune page à modifier.
 */
const props = withDefaults(defineProps<{
  isOpen: boolean
  titre: string
  /** Message de la confirmation, une fois le partage passé. */
  succesTexte?: string
  /** Texte accompagnant le lien sur les réseaux. Absent = pas de réseaux. */
  textePartage?: string
  /** Couple traçable, pour créditer l'auteur du contenu partagé. */
  typeObjet?: string
  objetId?: string
  /**
   * URL à partager. Par défaut la page courante — mais une modale ouverte
   * depuis une LISTE doit partager la fiche du contenu, pas la liste : c'est
   * ce que l'aperçu social des réseaux ira lire.
   */
  url?: string
  /**
   * Le partage sur le MUR est réservé aux membres ; les réseaux externes, non.
   * Un visiteur voit donc les boutons de partage et une invitation à se
   * connecter, à la place de la légende et du bouton d'envoi.
   */
  estConnecte?: boolean
}>(), {
  succesTexte: 'Votre partage apparaît désormais sur la page Publications.',
  estConnecte: true,
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', legende: string): void
}>()

const MAX = 500

const legende = ref('')
const enCours = ref(false)
const erreur = ref('')
const succes = ref(false)
const urlPage = ref('')
const lienCopie = ref(false)

const restant = computed(() => MAX - legende.value.length)

// L'URL n'est résolue qu'à l'OUVERTURE, côté client : `window` n'existe pas au
// rendu serveur, et la page peut avoir changé depuis le montage.
watch(() => props.isOpen, (ouvert) => {
  if (!ouvert) return
  legende.value = ''
  erreur.value = ''
  succes.value = false
  enCours.value = false
  lienCopie.value = false
  urlPage.value = props.url ? new URL(props.url, window.location.origin).href : window.location.href
})

const reseaux = computed(() =>
  props.textePartage ? construireReseaux(props.textePartage, urlPage.value) : [],
)

const { tracerPartage } = usePartageExterne()

const partagerReseau = (r: { url: string, reseau?: ReseauExterne }) => {
  ouvrirPartage(r as never)
  // Traçage best-effort, APRÈS l'ouverture : une erreur de comptage ne doit
  // pas empêcher le partage lui-même.
  if (r.reseau && props.typeObjet && props.objetId) {
    tracerPartage(props.typeObjet, props.objetId, r.reseau)
  }
}

const copierLien = async () => {
  try {
    await navigator.clipboard.writeText(urlPage.value)
    lienCopie.value = true
    setTimeout(() => { lienCopie.value = false }, 2000)
  }
  catch {
    erreur.value = 'Impossible de copier le lien.'
  }
}

const fermer = () => {
  if (enCours.value) return
  emit('close')
}

const soumettre = () => {
  if (enCours.value) return
  if (legende.value.length > MAX) {
    erreur.value = `La légende ne doit pas dépasser ${MAX} caractères.`
    return
  }
  erreur.value = ''
  emit('submit', legende.value.trim())
}

// Les pages pilotent l'état d'envoi par `ref` : le contrat est repris tel quel.
defineExpose({
  setLoading: (v: boolean) => { enCours.value = v },
  setError: (m: string) => { erreur.value = m; enCours.value = false },
  setSuccess: () => { succes.value = true; enCours.value = false },
})
</script>

<template>
  <AfricansModale
    :model-value="isOpen"
    :titre="titre"
    icone="fa-solid fa-share-nodes"
    @update:model-value="!$event && fermer()"
  >
    <div v-if="succes" class="flex flex-col items-center gap-3 py-8 text-center">
      <font-awesome-icon icon="fa-solid fa-circle-check" class="text-4xl text-af-vert" />
      <p class="text-[16px]/[1.4] font-bold text-af-encre">C'est partagé</p>
      <p class="max-w-sm text-[14px]/[1.4] text-af-corps">{{ succesTexte }}</p>
    </div>

    <div v-else class="flex flex-col gap-5">
      <!-- L'appelante décrit ce qui est partagé : elle seule le connaît. -->
      <p class="text-[14px]/[1.5] text-af-corps"><slot /></p>

      <!-- Réseaux externes. Absents quand aucun texte de partage n'est fourni :
           une modale qui ne partage que sur le mur n'a rien à y mettre. -->
      <div v-if="reseaux.length" class="flex flex-col gap-3">
        <span class="text-[12px]/[1.4] font-bold tracking-wide text-af-atone uppercase">Partager ailleurs</span>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="r in reseaux"
            :key="r.nom"
            type="button"
            class="inline-flex items-center gap-2 rounded-lg px-3.5 py-2.5 text-[14px]/[1.4] font-bold text-white transition hover:opacity-90"
            :class="r.couleur"
            @click="partagerReseau(r)"
          >
            <font-awesome-icon :icon="r.icon" />
            {{ r.nom }}
          </button>

          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-lg border border-af-bordure px-3.5 py-2.5 text-[14px]/[1.4] font-bold text-af-corps transition hover:border-af-chocolat"
            @click="copierLien"
          >
            <font-awesome-icon :icon="lienCopie ? 'fa-solid fa-check' : 'fa-solid fa-link'" />
            {{ lienCopie ? 'Lien copié' : 'Copier le lien' }}
          </button>
        </div>
      </div>

      <!-- Invitation à se connecter, à la place de la légende. -->
      <p v-if="!estConnecte" class="text-center text-[14px]/[1.4] text-af-corps">
        <NuxtLink to="/login" class="font-bold text-af-chocolat hover:underline">Connectez-vous</NuxtLink>
        pour partager aussi ce contenu sur le mur communautaire.
      </p>

      <div v-else class="flex flex-col gap-2">
        <label for="legende-partage" class="text-[14px]/[1.4] text-af-atone italic">
          Légende (facultative)
        </label>
        <textarea
          id="legende-partage"
          v-model="legende"
          rows="4"
          :maxlength="MAX"
          :disabled="enCours"
          placeholder="Ajoutez un mot pour accompagner ce partage…"
          class="rounded-md border border-af-bordure bg-white px-4 py-3 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none disabled:opacity-50"
        />
        <div class="flex items-start justify-between gap-3">
          <p v-if="erreur" role="alert" class="text-[12px]/[1.4] text-af-live">{{ erreur }}</p>
          <span v-else />
          <span class="shrink-0 text-[12px]/[1.4]" :class="restant < 0 ? 'text-af-live' : 'text-af-atone'">
            {{ restant }}
          </span>
        </div>
      </div>
    </div>

    <template v-if="!succes && estConnecte" #actions>
      <button
        type="button"
        :disabled="enCours"
        class="text-base font-bold text-af-corps transition hover:opacity-70 disabled:opacity-50"
        @click="fermer"
      >
        Annuler
      </button>
      <AfricansBouton
        :desactive="enCours"
        :tourne="enCours"
        :icone="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-share-nodes'"
        @click="soumettre"
      >
        {{ enCours ? 'Partage en cours…' : 'Partager' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
