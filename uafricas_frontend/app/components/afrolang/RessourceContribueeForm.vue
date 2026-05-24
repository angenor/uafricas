<script setup lang="ts">
// Modal d'ajout d'une ressource contribuée (feature 001-ressources-fermeture-session, US1).
// 4 onglets : Document / Vidéo / Lien / Accompagnateur. Tailwind v4 pur.
import { computed, ref, watch } from 'vue'
import type { TypeRessourceContribuee } from '~/composables/useAfrolangRessources'
import { useAfrolangRessources } from '~/composables/useAfrolangRessources'

const props = defineProps<{
  ouvert: boolean
  salleId: string
  sessionId?: string | null
}>()

const emit = defineEmits<{
  (e: 'fermer'): void
  (e: 'ajoutee'): void
}>()

const {
  ajouterDocument,
  ajouterVideoYoutube,
  ajouterLienWeb,
  recommanderAccompagnateur,
  chargement,
  erreur,
} = useAfrolangRessources()

const ongletActif = ref<TypeRessourceContribuee>('document')
const titre = ref('')
const description = ref('')

// État spécifique par onglet
const fichier = ref<File | null>(null)
const videoUrl = ref('')
const lienUrl = ref('')
const membreRecommandeId = ref('')
const motifRecommandation = ref('')

const erreurLocale = ref<string | null>(null)

const REGEX_YT = /^(https?:\/\/)?(www\.)?(youtube\.com|youtu\.be|m\.youtube\.com)\//i

const reinitialiser = () => {
  titre.value = ''
  description.value = ''
  fichier.value = null
  videoUrl.value = ''
  lienUrl.value = ''
  membreRecommandeId.value = ''
  motifRecommandation.value = ''
  erreurLocale.value = null
}

watch(
  () => props.ouvert,
  (v) => { if (v) reinitialiser() },
)

const titreValide = computed(() => {
  const t = titre.value.trim()
  return t.length >= 1 && t.length <= 120
})

const motifLongueur = computed(() => motifRecommandation.value.trim().length)

const peutSoumettre = computed(() => {
  if (!titreValide.value) return false
  if (chargement.value) return false
  switch (ongletActif.value) {
    case 'document':
      return fichier.value !== null && (fichier.value?.size ?? 0) <= 20 * 1024 * 1024
    case 'video_youtube':
      return REGEX_YT.test(videoUrl.value.trim())
    case 'lien_web':
      return lienUrl.value.trim().startsWith('https://')
    case 'accompagnateur':
      return membreRecommandeId.value.length === 36 && motifLongueur.value >= 20 && motifLongueur.value <= 2000
  }
  return false
})

const onFichierChange = (e: Event) => {
  const target = e.target as HTMLInputElement
  fichier.value = target.files?.[0] ?? null
  if (fichier.value) {
    const taille = fichier.value.size
    if (taille > 20 * 1024 * 1024) {
      erreurLocale.value = 'Le fichier dépasse 20 Mo.'
      fichier.value = null
      target.value = ''
    }
    else {
      erreurLocale.value = null
    }
  }
}

const soumettre = async () => {
  erreurLocale.value = null
  let ok = false
  switch (ongletActif.value) {
    case 'document': {
      if (!fichier.value) return
      const fd = new FormData()
      fd.append('type', 'document')
      fd.append('titre', titre.value.trim())
      if (description.value.trim()) fd.append('description', description.value.trim())
      if (props.sessionId) fd.append('session_origine_id', props.sessionId)
      fd.append('fichier', fichier.value)
      ok = (await ajouterDocument(props.salleId, fd)) !== null
      break
    }
    case 'video_youtube': {
      const res = await ajouterVideoYoutube(props.salleId, {
        type: 'video_youtube',
        titre: titre.value.trim(),
        description: description.value.trim() || null,
        session_origine_id: props.sessionId ?? null,
        video_url: videoUrl.value.trim(),
      })
      ok = res !== null
      break
    }
    case 'lien_web': {
      const res = await ajouterLienWeb(props.salleId, {
        type: 'lien_web',
        titre: titre.value.trim(),
        description: description.value.trim() || null,
        session_origine_id: props.sessionId ?? null,
        lien_url: lienUrl.value.trim(),
      })
      ok = res !== null
      break
    }
    case 'accompagnateur': {
      const res = await recommanderAccompagnateur(props.salleId, {
        type: 'accompagnateur',
        titre: titre.value.trim(),
        description: description.value.trim() || null,
        session_origine_id: props.sessionId ?? null,
        membre_recommande_id: membreRecommandeId.value,
        motif_recommandation: motifRecommandation.value.trim(),
      })
      ok = res !== null
      break
    }
  }
  if (ok) {
    emit('ajoutee')
    emit('fermer')
  }
}

const onglets: { code: TypeRessourceContribuee; libelle: string; icone: string }[] = [
  { code: 'document', libelle: 'Document', icone: 'fa-solid fa-file-lines' },
  { code: 'video_youtube', libelle: 'Vidéo', icone: 'fa-brands fa-youtube' },
  { code: 'lien_web', libelle: 'Lien', icone: 'fa-solid fa-link' },
  { code: 'accompagnateur', libelle: 'Accompagnateur', icone: 'fa-solid fa-user-graduate' },
]
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity duration-200"
      leave-active-class="transition-opacity duration-150"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0">
      <div v-if="ouvert"
           class="fixed inset-0 z-10000 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4"
           @click.self="emit('fermer')">
        <div class="w-full max-w-2xl bg-white rounded-xl shadow-2xl overflow-hidden flex flex-col max-h-[90vh]">
          <!-- Header -->
          <header class="flex items-center justify-between px-5 py-4 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-900">Contribuer une ressource</h2>
            <button type="button" class="text-gray-400 hover:text-gray-700 p-1.5 rounded-md hover:bg-gray-100" @click="emit('fermer')">
              <font-awesome-icon icon="fa-solid fa-xmark" />
            </button>
          </header>

          <!-- Onglets -->
          <nav class="flex border-b border-gray-100 bg-gray-50">
            <button v-for="o in onglets"
                    :key="o.code"
                    type="button"
                    class="flex-1 flex items-center justify-center gap-2 px-4 py-3 text-xs font-medium transition-colors"
                    :class="ongletActif === o.code
                      ? 'text-custom-chocolat border-b-2 border-custom-chocolat bg-white'
                      : 'text-gray-600 hover:text-gray-900 border-b-2 border-transparent'"
                    @click="ongletActif = o.code">
              <font-awesome-icon :icon="o.icone" class="text-sm" />
              <span class="hidden sm:inline">{{ o.libelle }}</span>
            </button>
          </nav>

          <!-- Corps formulaire -->
          <form class="flex-1 overflow-y-auto px-5 py-4 space-y-4" @submit.prevent="soumettre">
            <!-- Titre (commun) -->
            <div>
              <label class="block text-xs font-medium text-gray-700 mb-1">Titre <span class="text-red-500">*</span></label>
              <input v-model="titre"
                     type="text"
                     maxlength="120"
                     required
                     class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat focus:border-transparent">
              <p class="text-xs text-gray-400 mt-1">{{ titre.length }}/120</p>
            </div>

            <!-- Description (commun, optionnel) -->
            <div>
              <label class="block text-xs font-medium text-gray-700 mb-1">Description (optionnelle)</label>
              <textarea v-model="description"
                        rows="2"
                        maxlength="500"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat focus:border-transparent" />
              <p class="text-xs text-gray-400 mt-1">{{ description.length }}/500</p>
            </div>

            <!-- Variant : document -->
            <div v-if="ongletActif === 'document'">
              <label class="block text-xs font-medium text-gray-700 mb-1">Fichier (PDF, DOC, DOCX, ODT — max 20 Mo) <span class="text-red-500">*</span></label>
              <input type="file"
                     accept=".pdf,.doc,.docx,.odt,application/pdf,application/msword,application/vnd.openxmlformats-officedocument.wordprocessingml.document,application/vnd.oasis.opendocument.text"
                     class="block w-full text-sm text-gray-700 file:mr-3 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-medium file:bg-custom-chocolat file:text-white hover:file:bg-custom-chocolat/90 file:cursor-pointer"
                     @change="onFichierChange">
              <p v-if="fichier" class="text-xs text-gray-500 mt-1">{{ fichier.name }} ({{ (fichier.size / (1024 * 1024)).toFixed(2) }} Mo)</p>
            </div>

            <!-- Variant : video_youtube -->
            <div v-else-if="ongletActif === 'video_youtube'">
              <label class="block text-xs font-medium text-gray-700 mb-1">URL YouTube <span class="text-red-500">*</span></label>
              <input v-model="videoUrl"
                     type="url"
                     placeholder="https://www.youtube.com/watch?v=..."
                     class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat focus:border-transparent">
              <p v-if="videoUrl && !REGEX_YT.test(videoUrl)" class="text-xs text-red-600 mt-1">URL YouTube invalide</p>
            </div>

            <!-- Variant : lien_web -->
            <div v-else-if="ongletActif === 'lien_web'">
              <label class="block text-xs font-medium text-gray-700 mb-1">URL HTTPS <span class="text-red-500">*</span></label>
              <input v-model="lienUrl"
                     type="url"
                     placeholder="https://..."
                     maxlength="1000"
                     class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat focus:border-transparent">
              <p v-if="lienUrl && !lienUrl.startsWith('https://')" class="text-xs text-red-600 mt-1">L'URL doit commencer par https://</p>
            </div>

            <!-- Variant : accompagnateur -->
            <div v-else-if="ongletActif === 'accompagnateur'" class="space-y-3">
              <div>
                <label class="block text-xs font-medium text-gray-700 mb-1">ID membre recommandé (UUID) <span class="text-red-500">*</span></label>
                <input v-model="membreRecommandeId"
                       type="text"
                       maxlength="36"
                       placeholder="00000000-0000-0000-0000-000000000000"
                       class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm font-mono focus:outline-none focus:ring-2 focus:ring-custom-chocolat focus:border-transparent">
                <p class="text-xs text-gray-400 mt-1">Recherche utilisateur — à brancher dans une version ultérieure.</p>
              </div>
              <div>
                <label class="block text-xs font-medium text-gray-700 mb-1">Motif de recommandation (≥ 20 caractères) <span class="text-red-500">*</span></label>
                <textarea v-model="motifRecommandation"
                          rows="3"
                          maxlength="2000"
                          class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat focus:border-transparent" />
                <p class="text-xs mt-1" :class="motifLongueur < 20 ? 'text-red-600' : 'text-gray-400'">{{ motifLongueur }}/2000 (min 20)</p>
              </div>
            </div>

            <!-- Erreur -->
            <div v-if="erreurLocale || erreur" class="text-sm text-red-700 bg-red-50 border border-red-200 rounded-md px-3 py-2">
              {{ erreurLocale || erreur }}
            </div>
          </form>

          <!-- Footer -->
          <footer class="flex justify-end gap-2 px-5 py-3 border-t border-gray-100 bg-gray-50">
            <button type="button" class="px-4 py-2 text-sm text-gray-700 hover:text-gray-900 rounded-md hover:bg-white" @click="emit('fermer')">
              Annuler
            </button>
            <button type="button"
                    :disabled="!peutSoumettre"
                    class="px-4 py-2 text-sm font-medium text-white rounded-md transition-colors"
                    :class="peutSoumettre
                      ? 'bg-custom-chocolat hover:bg-custom-chocolat/90'
                      : 'bg-gray-300 cursor-not-allowed'"
                    @click="soumettre">
              <span v-if="chargement"><font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin mr-1" />Envoi…</span>
              <span v-else>Contribuer</span>
            </button>
          </footer>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
