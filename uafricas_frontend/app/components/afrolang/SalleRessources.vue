<script setup lang="ts">
// Ressources d'une salle publique (US6)
// Tailwind v4 pur — accepte fichiers (admin/modérateur attitré) ou liens (tout membre, modérés)
import type { RessourceSalleAPI } from '~/composables/useAfrolang'

interface Props {
  salleId: string
  estModerateurAttitre?: boolean
}

const props = defineProps<Props>()

const {
  listerRessources,
  uploaderRessourceFichier,
  soumettreLienExterne,
  supprimerRessource,
} = useAfrolang()
const userStore = useUserStore()

const ressources = ref<RessourceSalleAPI[]>([])
const chargement = ref(false)

const modaleOuvert = ref<'aucun' | 'fichier' | 'lien'>('aucun')
const formTitre = ref('')
const formDescription = ref('')
const formUrl = ref('')
const fichierSelectionne = ref<File | null>(null)
const envoiEnCours = ref(false)
const messageErreur = ref<string | null>(null)

const recharger = async () => {
  chargement.value = true
  ressources.value = await listerRessources(props.salleId)
  chargement.value = false
}

const reinitialiserForm = () => {
  formTitre.value = ''
  formDescription.value = ''
  formUrl.value = ''
  fichierSelectionne.value = null
  messageErreur.value = null
}

const ouvrirModale = (type: 'fichier' | 'lien') => {
  reinitialiserForm()
  modaleOuvert.value = type
}

const fermerModale = () => {
  modaleOuvert.value = 'aucun'
  reinitialiserForm()
}

const envoyerFichier = async () => {
  if (!fichierSelectionne.value || !formTitre.value.trim()) {
    messageErreur.value = 'Titre et fichier obligatoires'
    return
  }
  envoiEnCours.value = true
  const res = await uploaderRessourceFichier(
    props.salleId,
    fichierSelectionne.value,
    formTitre.value.trim(),
    formDescription.value.trim() || undefined,
  )
  envoiEnCours.value = false
  if (res) {
    fermerModale()
    await recharger()
  }
  else {
    messageErreur.value = "Échec de l'envoi (format/taille/autorisation)"
  }
}

const envoyerLien = async () => {
  const url = formUrl.value.trim()
  if (!formTitre.value.trim() || !url) {
    messageErreur.value = 'Titre et URL obligatoires'
    return
  }
  envoiEnCours.value = true
  const res = await soumettreLienExterne(
    props.salleId,
    formTitre.value.trim(),
    url,
    formDescription.value.trim() || undefined,
  )
  envoiEnCours.value = false
  if (res) {
    fermerModale()
    await recharger()
  }
  else {
    messageErreur.value = 'Échec de la soumission (URL invalide ?)'
  }
}

const supprimer = async (id: string) => {
  if (!confirm('Supprimer cette ressource ?')) return
  const ok = await supprimerRessource(id)
  if (ok) await recharger()
}

const libelleEtat = (etat: string) => {
  switch (etat) {
    case 'publiee': return 'Publiée'
    case 'en_attente_validation': return 'En attente de validation'
    case 'refusee': return 'Refusée'
    default: return etat
  }
}

const classEtat = (etat: string) => {
  switch (etat) {
    case 'publiee': return 'bg-green-100 text-green-800'
    case 'en_attente_validation': return 'bg-amber-100 text-amber-800'
    case 'refusee': return 'bg-red-100 text-red-800'
    default: return 'bg-gray-100 text-gray-700'
  }
}

const onFichier = (event: Event) => {
  const input = event.target as HTMLInputElement
  fichierSelectionne.value = input.files?.[0] ?? null
}

const peutSupprimer = (r: RessourceSalleAPI): boolean => {
  const uid = userStore.user?.id
  return !!uid && (uid === r.ajoute_par || props.estModerateurAttitre === true)
}

onMounted(recharger)
watch(() => props.salleId, recharger)
</script>

<template>
  <section class="rounded-lg border border-gray-200 bg-white">
    <header class="flex items-center justify-between border-b border-gray-200 px-4 py-3">
      <h3 class="text-sm font-semibold text-gray-900">Ressources</h3>
      <div class="flex gap-2">
        <button
          v-if="estModerateurAttitre"
          type="button"
          class="rounded-md bg-custom-chocolat px-3 py-1.5 text-xs font-medium text-white hover:bg-amber-800"
          @click="ouvrirModale('fichier')"
        >
          + Fichier
        </button>
        <button
          type="button"
          class="rounded-md border border-gray-300 bg-white px-3 py-1.5 text-xs font-medium text-gray-700 hover:bg-gray-50"
          @click="ouvrirModale('lien')"
        >
          + Lien externe
        </button>
      </div>
    </header>

    <div class="p-4 space-y-3">
      <p v-if="chargement" class="text-center text-sm text-gray-500">Chargement...</p>
      <p v-else-if="ressources.length === 0" class="text-center text-sm text-gray-500">
        Aucune ressource pour l'instant.
      </p>
      <ul v-else class="space-y-2">
        <li
          v-for="r in ressources"
          :key="r.id"
          class="flex items-center justify-between gap-3 rounded-md border border-gray-200 p-3"
        >
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <span :class="['rounded px-2 py-0.5 text-xs font-medium', classEtat(r.etat)]">
                {{ libelleEtat(r.etat) }}
              </span>
              <span class="text-xs text-gray-500">
                {{ r.type_ressource === 'fichier' ? 'Fichier' : 'Lien' }}
              </span>
            </div>
            <p class="mt-1 truncate text-sm font-medium text-gray-900">{{ r.titre }}</p>
            <p v-if="r.description" class="text-xs text-gray-500 line-clamp-2">
              {{ r.description }}
            </p>
            <p v-if="r.auteur_nom" class="text-xs text-gray-400 mt-1">
              Ajouté par {{ r.auteur_prenom }} {{ r.auteur_nom }}
            </p>
          </div>
          <div class="flex items-center gap-2">
            <a
              v-if="r.etat === 'publiee' && r.fichier_url"
              :href="r.fichier_url"
              target="_blank"
              rel="noopener"
              class="rounded-md border border-gray-300 px-3 py-1.5 text-xs font-medium text-gray-700 hover:bg-gray-50"
            >
              Télécharger
            </a>
            <a
              v-else-if="r.etat === 'publiee' && r.lien_url"
              :href="r.lien_url"
              target="_blank"
              rel="noopener"
              class="rounded-md border border-gray-300 px-3 py-1.5 text-xs font-medium text-gray-700 hover:bg-gray-50"
            >
              Ouvrir
            </a>
            <button
              v-if="peutSupprimer(r)"
              type="button"
              class="rounded-md border border-red-300 px-3 py-1.5 text-xs font-medium text-red-700 hover:bg-red-50"
              @click="supprimer(r.id)"
            >
              Supprimer
            </button>
          </div>
        </li>
      </ul>
    </div>

    <!-- Modale ajout -->
    <Teleport to="body">
      <div
        v-if="modaleOuvert !== 'aucun'"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 px-4"
        @click.self="fermerModale"
      >
        <div class="w-full max-w-lg rounded-lg bg-white p-6 space-y-4">
          <h4 class="text-lg font-semibold text-gray-900">
            {{ modaleOuvert === 'fichier' ? 'Ajouter un fichier' : 'Soumettre un lien externe' }}
          </h4>
          <p v-if="modaleOuvert === 'lien'" class="text-xs text-amber-700">
            Le lien sera publié après validation par un modérateur.
          </p>

          <div class="space-y-3">
            <label class="block">
              <span class="text-sm font-medium text-gray-700">Titre</span>
              <input
                v-model="formTitre"
                type="text"
                class="mt-1 w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-custom-chocolat focus:outline-none"
              />
            </label>
            <label class="block">
              <span class="text-sm font-medium text-gray-700">Description (optionnelle)</span>
              <textarea
                v-model="formDescription"
                rows="2"
                class="mt-1 w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-custom-chocolat focus:outline-none"
              />
            </label>
            <label v-if="modaleOuvert === 'fichier'" class="block">
              <span class="text-sm font-medium text-gray-700">Fichier (PDF, images, audio, vidéo — max 50 Mo)</span>
              <input
                type="file"
                accept=".pdf,.png,.jpg,.jpeg,.mp3,.mp4,.webm,.ogg,.wav"
                class="mt-1 w-full text-sm"
                @change="onFichier"
              />
            </label>
            <label v-if="modaleOuvert === 'lien'" class="block">
              <span class="text-sm font-medium text-gray-700">URL</span>
              <input
                v-model="formUrl"
                type="url"
                placeholder="https://..."
                class="mt-1 w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-custom-chocolat focus:outline-none"
              />
            </label>
          </div>

          <p v-if="messageErreur" class="text-sm text-red-700">{{ messageErreur }}</p>

          <div class="flex justify-end gap-2">
            <button
              type="button"
              class="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
              @click="fermerModale"
            >
              Annuler
            </button>
            <button
              type="button"
              :disabled="envoiEnCours"
              class="rounded-md bg-custom-chocolat px-4 py-2 text-sm font-medium text-white hover:bg-amber-800 disabled:opacity-50"
              @click="modaleOuvert === 'fichier' ? envoyerFichier() : envoyerLien()"
            >
              Envoyer
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </section>
</template>
