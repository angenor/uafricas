<script setup lang="ts">
import type { AdminChapitre, AdminLecon, CreerChapitreForm, CreerLeconForm } from '~/types/admin'

const props = defineProps<{ moocId: string }>()

const {
  chargerChapitres, creerChapitre, modifierChapitre, supprimerChapitre, reordonnerChapitres,
  creerLecon, modifierLecon, supprimerLecon, reordonnerLecons, uploaderFichierFormation, error,
} = useAdminMooc()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const resoudreUrl = (url: string | null): string =>
  !url ? '' : (url.startsWith('http') ? url : `${apiBase}${url}`)
const estLienExterne = (url: string): boolean =>
  /youtube\.com|youtu\.be|^https?:\/\//.test(url) && !url.includes('/uploads/')

const chapitres = ref<AdminChapitre[]>([])
const chargement = ref(false)
const enregistrement = ref(false)

const recharger = async () => {
  chargement.value = true
  chapitres.value = await chargerChapitres(props.moocId)
  chargement.value = false
}
onMounted(recharger)

// ── Chapitre ───────────────────────────────────────────────
const formChapitre = reactive<CreerChapitreForm>({ titre: '', description: '' })
const chapitreEnEdition = ref<string | null>(null) // id existant, ou 'nouveau'

const ouvrirNouveauChapitre = () => {
  chapitreEnEdition.value = 'nouveau'
  formChapitre.titre = ''
  formChapitre.description = ''
}
const ouvrirEditionChapitre = (ch: AdminChapitre) => {
  chapitreEnEdition.value = ch.id
  formChapitre.titre = ch.titre
  formChapitre.description = ch.description || ''
}
const annulerChapitre = () => { chapitreEnEdition.value = null }

const enregistrerChapitre = async () => {
  if (!formChapitre.titre.trim() || enregistrement.value) return
  enregistrement.value = true
  try {
    if (chapitreEnEdition.value === 'nouveau') {
      await creerChapitre(props.moocId, { ...formChapitre })
    } else if (chapitreEnEdition.value) {
      await modifierChapitre(chapitreEnEdition.value, { ...formChapitre })
    }
    chapitreEnEdition.value = null
    await recharger()
  } finally { enregistrement.value = false }
}

const supprimerCeChapitre = async (ch: AdminChapitre) => {
  if (!confirm(`Supprimer le chapitre « ${ch.titre} » et ses ${ch.lecons.length} leçon(s) ?`)) return
  await supprimerChapitre(ch.id)
  await recharger()
}

const deplacerChapitre = async (index: number, sens: -1 | 1) => {
  const cible = index + sens
  if (cible < 0 || cible >= chapitres.value.length) return
  const arr = [...chapitres.value]
  const tmp = arr[index]!
  arr[index] = arr[cible]!
  arr[cible] = tmp
  chapitres.value = arr
  await reordonnerChapitres(props.moocId, arr.map(c => c.id))
}

// ── Leçon ──────────────────────────────────────────────────
const formLecon = reactive<CreerLeconForm>({
  titre: '', contenu: '', video_url: '', document_url: '', duree_minutes: null,
})
const leconEnEdition = ref<string | null>(null)   // id existant, ou `nouveau-<chapitreId>`
const chapitreCibleLecon = ref<string | null>(null)

const resetFormLecon = () => {
  formLecon.titre = ''
  formLecon.contenu = ''
  formLecon.video_url = ''
  formLecon.document_url = ''
  formLecon.duree_minutes = null
}
const ouvrirNouvelleLecon = (ch: AdminChapitre) => {
  leconEnEdition.value = `nouveau-${ch.id}`
  chapitreCibleLecon.value = ch.id
  resetFormLecon()
}
const ouvrirEditionLecon = (lecon: AdminLecon) => {
  leconEnEdition.value = lecon.id
  chapitreCibleLecon.value = lecon.chapitre_id
  formLecon.titre = lecon.titre
  formLecon.contenu = lecon.contenu || ''
  formLecon.video_url = lecon.video_url || ''
  formLecon.document_url = lecon.document_url || ''
  formLecon.duree_minutes = lecon.duree_minutes
}
const annulerLecon = () => {
  leconEnEdition.value = null
  chapitreCibleLecon.value = null
}

const enregistrerLecon = async () => {
  if (!formLecon.titre.trim() || !chapitreCibleLecon.value || enregistrement.value) return
  enregistrement.value = true
  // `v-model.number` sur un champ vidé laisse une chaîne vide '' (et non null) :
  // on la normalise en null, sinon serde rejette une chaîne pour Option<i32> (400).
  const d = formLecon.duree_minutes as number | string | null
  const payload: CreerLeconForm = {
    ...formLecon,
    duree_minutes: typeof d === 'number' && !Number.isNaN(d) ? d : null,
  }
  try {
    if (leconEnEdition.value && leconEnEdition.value.startsWith('nouveau-')) {
      await creerLecon(chapitreCibleLecon.value, payload)
    } else if (leconEnEdition.value) {
      await modifierLecon(leconEnEdition.value, payload)
    }
    annulerLecon()
    await recharger()
  } finally { enregistrement.value = false }
}

const supprimerCetteLecon = async (lecon: AdminLecon) => {
  if (!confirm(`Supprimer la leçon « ${lecon.titre} » ?`)) return
  await supprimerLecon(lecon.id)
  await recharger()
}

const deplacerLecon = async (ch: AdminChapitre, index: number, sens: -1 | 1) => {
  const cible = index + sens
  if (cible < 0 || cible >= ch.lecons.length) return
  const arr = [...ch.lecons]
  const tmp = arr[index]!
  arr[index] = arr[cible]!
  arr[cible] = tmp
  ch.lecons = arr
  await reordonnerLecons(ch.id, arr.map(l => l.id))
}

// ── Upload ─────────────────────────────────────────────────
const uploadVideoEnCours = ref(false)
const uploadDocEnCours = ref(false)

const onUploadVideo = async (e: Event) => {
  const fichier = (e.target as HTMLInputElement).files?.[0]
  if (!fichier) return
  uploadVideoEnCours.value = true
  const url = await uploaderFichierFormation(fichier)
  if (url) formLecon.video_url = url
  uploadVideoEnCours.value = false
  ;(e.target as HTMLInputElement).value = ''
}
const onUploadDoc = async (e: Event) => {
  const fichier = (e.target as HTMLInputElement).files?.[0]
  if (!fichier) return
  uploadDocEnCours.value = true
  const url = await uploaderFichierFormation(fichier)
  if (url) formLecon.document_url = url
  uploadDocEnCours.value = false
  ;(e.target as HTMLInputElement).value = ''
}

const formLeconVisible = (chapitreId: string): boolean => chapitreCibleLecon.value === chapitreId
</script>

<template>
  <div class="space-y-4">
    <div v-if="error" class="alert alert-error">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ error }}</span>
    </div>

    <!-- Chargement -->
    <div v-if="chargement" class="flex justify-center py-10">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else>
      <!-- En-tête -->
      <div class="flex items-center justify-between gap-3">
        <p class="text-sm text-base-content/60">
          {{ chapitres.length }} chapitre(s) ·
          {{ chapitres.reduce((n, c) => n + c.lecons.length, 0) }} leçon(s)
        </p>
        <button v-if="chapitreEnEdition !== 'nouveau'" class="btn btn-primary btn-sm" @click="ouvrirNouveauChapitre">
          <font-awesome-icon icon="plus" class="mr-1" /> Ajouter un chapitre
        </button>
      </div>

      <!-- Formulaire nouveau chapitre -->
      <div v-if="chapitreEnEdition === 'nouveau'" class="card bg-base-200">
        <div class="card-body gap-3">
          <h4 class="font-semibold">Nouveau chapitre</h4>
          <input v-model="formChapitre.titre" type="text" placeholder="Titre du chapitre *"
                 class="input input-bordered" @keydown.enter.prevent="enregistrerChapitre">
          <textarea v-model="formChapitre.description" placeholder="Description (facultative)"
                    class="textarea textarea-bordered" rows="2" />
          <div class="flex justify-end gap-2">
            <button class="btn btn-ghost btn-sm" @click="annulerChapitre">Annuler</button>
            <button class="btn btn-primary btn-sm" :disabled="!formChapitre.titre.trim() || enregistrement"
                    @click="enregistrerChapitre">Créer</button>
          </div>
        </div>
      </div>

      <!-- Aucun chapitre -->
      <div v-if="!chapitres.length && chapitreEnEdition !== 'nouveau'"
           class="text-center py-10 text-base-content/50 border border-dashed border-base-300 rounded-box">
        <font-awesome-icon icon="layer-group" class="text-3xl mb-2" />
        <p>Aucun chapitre. Structurez la formation en ajoutant un premier chapitre.</p>
      </div>

      <!-- Liste des chapitres -->
      <div v-for="(chapitre, ci) in chapitres" :key="chapitre.id" class="card bg-base-100 border border-base-300 shadow-sm">
        <div class="card-body p-4 gap-3">
          <!-- En-tête de chapitre -->
          <div v-if="chapitreEnEdition !== chapitre.id" class="flex items-start justify-between gap-2">
            <div class="min-w-0">
              <h4 class="font-semibold text-base">
                <span class="text-primary">Chapitre {{ ci + 1 }}.</span> {{ chapitre.titre }}
              </h4>
              <p v-if="chapitre.description" class="text-sm text-base-content/60 mt-0.5 whitespace-pre-line">
                {{ chapitre.description }}
              </p>
            </div>
            <div class="flex items-center gap-1 flex-shrink-0">
              <button class="btn btn-ghost btn-xs" :disabled="ci === 0" title="Monter" @click="deplacerChapitre(ci, -1)">
                <font-awesome-icon icon="arrow-up" />
              </button>
              <button class="btn btn-ghost btn-xs" :disabled="ci === chapitres.length - 1" title="Descendre" @click="deplacerChapitre(ci, 1)">
                <font-awesome-icon icon="arrow-down" />
              </button>
              <button class="btn btn-ghost btn-xs" title="Modifier" @click="ouvrirEditionChapitre(chapitre)">
                <font-awesome-icon icon="pen" />
              </button>
              <button class="btn btn-ghost btn-xs text-error" title="Supprimer" @click="supprimerCeChapitre(chapitre)">
                <font-awesome-icon icon="trash" />
              </button>
            </div>
          </div>

          <!-- Édition de chapitre -->
          <div v-else class="space-y-2 bg-base-200 rounded-box p-3">
            <input v-model="formChapitre.titre" type="text" placeholder="Titre du chapitre *" class="input input-bordered input-sm w-full">
            <textarea v-model="formChapitre.description" placeholder="Description (facultative)" class="textarea textarea-bordered textarea-sm w-full" rows="2" />
            <div class="flex justify-end gap-2">
              <button class="btn btn-ghost btn-xs" @click="annulerChapitre">Annuler</button>
              <button class="btn btn-primary btn-xs" :disabled="!formChapitre.titre.trim() || enregistrement" @click="enregistrerChapitre">Enregistrer</button>
            </div>
          </div>

          <!-- Leçons -->
          <ul class="divide-y divide-base-200 border-t border-base-200">
            <li v-for="(lecon, li) in chapitre.lecons" :key="lecon.id" class="flex items-center gap-2 py-2">
              <span class="badge badge-ghost badge-sm">{{ li + 1 }}</span>
              <div class="flex-1 min-w-0">
                <p class="font-medium truncate">{{ lecon.titre }}</p>
                <div class="flex items-center gap-2 text-xs text-base-content/50 mt-0.5">
                  <span v-if="lecon.video_url"><font-awesome-icon icon="video" /> Vidéo</span>
                  <span v-if="lecon.document_url"><font-awesome-icon icon="file" /> Doc</span>
                  <span v-if="lecon.duree_minutes">{{ lecon.duree_minutes }} min</span>
                </div>
              </div>
              <div class="flex items-center gap-1 flex-shrink-0">
                <button class="btn btn-ghost btn-xs" :disabled="li === 0" title="Monter" @click="deplacerLecon(chapitre, li, -1)">
                  <font-awesome-icon icon="arrow-up" />
                </button>
                <button class="btn btn-ghost btn-xs" :disabled="li === chapitre.lecons.length - 1" title="Descendre" @click="deplacerLecon(chapitre, li, 1)">
                  <font-awesome-icon icon="arrow-down" />
                </button>
                <button class="btn btn-ghost btn-xs" title="Modifier" @click="ouvrirEditionLecon(lecon)">
                  <font-awesome-icon icon="pen" />
                </button>
                <button class="btn btn-ghost btn-xs text-error" title="Supprimer" @click="supprimerCetteLecon(lecon)">
                  <font-awesome-icon icon="trash" />
                </button>
              </div>
            </li>
          </ul>

          <!-- Formulaire de leçon (création/édition) -->
          <div v-if="formLeconVisible(chapitre.id)" class="bg-base-200 rounded-box p-3 space-y-3">
            <h5 class="font-semibold text-sm">
              {{ leconEnEdition && leconEnEdition.startsWith('nouveau-') ? 'Nouvelle leçon' : 'Modifier la leçon' }}
            </h5>

            <div class="form-control">
              <label class="label py-1"><span class="label-text text-xs">Titre *</span></label>
              <input v-model="formLecon.titre" type="text" class="input input-bordered input-sm" placeholder="Titre de la leçon">
            </div>

            <div class="form-control">
              <label class="label py-1"><span class="label-text text-xs">Vidéo (fichier ou lien YouTube / externe)</span></label>
              <div v-if="formLecon.video_url" class="mb-2 space-y-1">
                <div v-if="!estLienExterne(formLecon.video_url)" >
                  <video :src="resoudreUrl(formLecon.video_url)" controls preload="metadata" class="w-full max-h-48 rounded bg-black" />
                </div>
                <div class="flex items-center gap-2">
                  <span class="text-xs text-base-content/60 truncate flex-1">{{ formLecon.video_url }}</span>
                  <button class="btn btn-xs btn-ghost text-error" @click="formLecon.video_url = ''">Retirer</button>
                </div>
              </div>
              <div class="flex flex-col sm:flex-row gap-2">
                <input type="file" accept="video/mp4,video/webm,video/quicktime,video/ogg,video/x-m4v"
                       class="file-input file-input-bordered file-input-sm flex-1" @change="onUploadVideo">
                <input v-model="formLecon.video_url" type="url" placeholder="ou coller un lien (YouTube, https://…)"
                       class="input input-bordered input-sm flex-1">
              </div>
              <span v-if="uploadVideoEnCours" class="text-xs text-base-content/60 mt-1">Téléversement de la vidéo…</span>
            </div>

            <div class="form-control">
              <label class="label py-1"><span class="label-text text-xs">Contenu / notes</span></label>
              <textarea v-model="formLecon.contenu" class="textarea textarea-bordered textarea-sm" rows="4"
                        placeholder="Texte de la leçon, points clés, consignes…" />
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <div class="form-control">
                <label class="label py-1"><span class="label-text text-xs">Document PDF (facultatif)</span></label>
                <div v-if="formLecon.document_url" class="flex items-center gap-2 mb-1">
                  <a :href="resoudreUrl(formLecon.document_url)" target="_blank" rel="noopener noreferrer"
                     class="text-xs link link-primary truncate flex-1">{{ formLecon.document_url }}</a>
                  <button class="btn btn-xs btn-ghost text-error" @click="formLecon.document_url = ''">Retirer</button>
                </div>
                <input type="file" accept="application/pdf" class="file-input file-input-bordered file-input-sm" @change="onUploadDoc">
                <span v-if="uploadDocEnCours" class="text-xs text-base-content/60 mt-1">Téléversement du document…</span>
              </div>
              <div class="form-control">
                <label class="label py-1"><span class="label-text text-xs">Durée (minutes)</span></label>
                <input v-model.number="formLecon.duree_minutes" type="number" min="0" class="input input-bordered input-sm" placeholder="Ex : 12">
              </div>
            </div>

            <div class="flex justify-end gap-2">
              <button class="btn btn-ghost btn-sm" @click="annulerLecon">Annuler</button>
              <button class="btn btn-primary btn-sm" :disabled="!formLecon.titre.trim() || enregistrement || uploadVideoEnCours || uploadDocEnCours"
                      @click="enregistrerLecon">Enregistrer la leçon</button>
            </div>
          </div>

          <!-- Bouton ajouter une leçon -->
          <button v-else class="btn btn-outline btn-sm self-start" @click="ouvrirNouvelleLecon(chapitre)">
            <font-awesome-icon icon="plus" class="mr-1" /> Ajouter une leçon
          </button>
        </div>
      </div>
    </template>
  </div>
</template>

