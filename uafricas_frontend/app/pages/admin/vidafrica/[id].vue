<script setup lang="ts">
import { LANGUES_LABELS, formaterDuree, formaterTimestamp } from '~/mocks/vidafrica'
import type { PisteSousTitre, SegmentSousTitre, TimingMot } from '~/mocks/vidafrica'
import { PAYS_AFRICAINS } from '~/composables/useEvenements'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const router = useRouter()
const id = route.params.id as string

const {
  videoDetail, loading, error,
  chargerDetail, modifier, changerEtat, supprimer,
  chargerPistes, creerPiste, changerEtatPiste, supprimerPiste,
  chargerSegments, creerSegment, modifierSegment, supprimerSegment,
  enregistrerTimingsMot, supprimerTimingsMot,
} = useAdminVidafrica()

// ═══════════════════════════════════════════════════════════════
// ÉTAT LOCAL
// ═══════════════════════════════════════════════════════════════

const ongletActif = ref('infos')
const erreurLocale = ref('')
const soumettreLoading = ref(false)
const etatLoading = ref(false)

// Formulaire modification
const form = reactive({
  titre: '',
  description: '',
  auteur_reel: '',
})
const territoires = ref<string[]>([])
const territoireAAjouter = ref('')
const dechargeDroits = ref(false)
const nouvelleVignette = ref<File | null>(null)

const territoiresDisponibles = computed(() =>
  PAYS_AFRICAINS.filter(p => !territoires.value.includes(p)),
)

const ajouterTerritoire = () => {
  const t = territoireAAjouter.value
  if (t && !territoires.value.includes(t)) territoires.value.push(t)
  territoireAAjouter.value = ''
}

const retirerTerritoire = (t: string) => {
  territoires.value = territoires.value.filter(x => x !== t)
}

// Pistes
const pistes = ref<PisteSousTitre[]>([])
const pisteSelectionnee = ref<PisteSousTitre | null>(null)
const nouvelleLangue = ref('')
const pisteLoading = ref(false)

// Segments
const segments = ref<SegmentSousTitre[]>([])
const segmentLoading = ref(false)
const showAjoutSegment = ref(false)
const nouveauSegment = reactive({ texte: '', debut_ms: 0, fin_ms: 0 })
const segmentEdite = ref<string | null>(null)
const segmentEditeData = reactive({ texte: '', debut_ms: 0, fin_ms: 0 })

// Tap-to-mark
const showTapToMark = ref(false)
const tapToMarkSegment = ref<SegmentSousTitre | null>(null)

// Prévisualisation
const showPrevisualisation = ref(false)

// ═══════════════════════════════════════════════════════════════
// CHARGEMENT
// ═══════════════════════════════════════════════════════════════

const charger = async () => {
  const data = await chargerDetail(id)
  if (data) {
    form.titre = data.titre
    form.description = data.description || ''
    form.auteur_reel = data.auteur_reel || ''
    territoires.value = [...(data.territoires || [])]
    dechargeDroits.value = data.decharge_droits ?? false
    pistes.value = data.pistes || []
  }
}

const rechargerPistes = async () => {
  pistes.value = await chargerPistes(id)
  if (pisteSelectionnee.value) {
    const maj = pistes.value.find(p => p.id === pisteSelectionnee.value!.id)
    if (maj) pisteSelectionnee.value = maj
  }
}

const selectionnerPiste = async (piste: PisteSousTitre) => {
  pisteSelectionnee.value = piste
  segmentLoading.value = true
  try {
    segments.value = await chargerSegments(piste.id)
  } finally {
    segmentLoading.value = false
  }
}

// ═══════════════════════════════════════════════════════════════
// MODIFICATION VIDÉO
// ═══════════════════════════════════════════════════════════════

const sauvegarder = async () => {
  erreurLocale.value = ''
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre est requis'
    return
  }

  soumettreLoading.value = true
  try {
    const formData = new FormData()
    formData.append('titre', form.titre.trim())
    formData.append('description', form.description.trim())
    formData.append('auteur_reel', form.auteur_reel.trim())
    // Marqueur de présence : permet de vider entièrement la liste côté backend.
    formData.append('territoires_modifies', '1')
    territoires.value.forEach(t => formData.append('territoires', t))
    formData.append('decharge_droits', dechargeDroits.value ? 'true' : 'false')
    if (nouvelleVignette.value) {
      formData.append('vignette', nouvelleVignette.value)
    }
    await modifier(id, formData)
    await charger()
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la modification'
  } finally {
    soumettreLoading.value = false
  }
}

const changerEtatVideo = async (etat: string) => {
  etatLoading.value = true
  try {
    await changerEtat(id, etat)
    await charger()
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    etatLoading.value = false
  }
}

const supprimerVideo = async () => {
  if (!confirm('Supprimer cette vidéo ?')) return
  await supprimer(id)
  router.push('/admin/vidafrica')
}

const onVignetteChange = (e: Event) => {
  const input = e.target as HTMLInputElement
  if (input.files?.[0]) nouvelleVignette.value = input.files[0]
}

// ═══════════════════════════════════════════════════════════════
// PISTES
// ═══════════════════════════════════════════════════════════════

const ajouterPiste = async () => {
  if (!nouvelleLangue.value) return
  pisteLoading.value = true
  try {
    await creerPiste(id, nouvelleLangue.value)
    nouvelleLangue.value = ''
    await rechargerPistes()
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    pisteLoading.value = false
  }
}

const changerEtatPisteAction = async (pisteId: string, etat: string) => {
  try {
    await changerEtatPiste(pisteId, etat)
    await rechargerPistes()
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const supprimerPisteAction = async (pisteId: string) => {
  if (!confirm('Supprimer cette piste de sous-titres ?')) return
  await supprimerPiste(pisteId)
  if (pisteSelectionnee.value?.id === pisteId) {
    pisteSelectionnee.value = null
    segments.value = []
  }
  await rechargerPistes()
}

const languesDisponibles = computed(() => {
  const utilisees = new Set(pistes.value.map(p => p.langue))
  return Object.entries(LANGUES_LABELS)
    .filter(([code]) => !utilisees.has(code as any))
    .map(([code, label]) => ({ code, label }))
})

// ═══════════════════════════════════════════════════════════════
// SEGMENTS
// ═══════════════════════════════════════════════════════════════

const ajouterSegment = async () => {
  if (!pisteSelectionnee.value || !nouveauSegment.texte.trim()) return
  if (nouveauSegment.debut_ms >= nouveauSegment.fin_ms) {
    erreurLocale.value = 'Le début doit être inférieur à la fin'
    return
  }
  segmentLoading.value = true
  try {
    await creerSegment(pisteSelectionnee.value.id, {
      texte: nouveauSegment.texte.trim(),
      debut_ms: nouveauSegment.debut_ms,
      fin_ms: nouveauSegment.fin_ms,
    })
    nouveauSegment.texte = ''
    nouveauSegment.debut_ms = 0
    nouveauSegment.fin_ms = 0
    showAjoutSegment.value = false
    segments.value = await chargerSegments(pisteSelectionnee.value.id)
    await rechargerPistes()
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    segmentLoading.value = false
  }
}

const debutEdition = (seg: SegmentSousTitre) => {
  segmentEdite.value = seg.id
  segmentEditeData.texte = seg.texte
  segmentEditeData.debut_ms = seg.debut_ms
  segmentEditeData.fin_ms = seg.fin_ms
}

const sauvegarderSegment = async () => {
  if (!segmentEdite.value || !pisteSelectionnee.value) return
  segmentLoading.value = true
  try {
    await modifierSegment(segmentEdite.value, {
      texte: segmentEditeData.texte.trim(),
      debut_ms: segmentEditeData.debut_ms,
      fin_ms: segmentEditeData.fin_ms,
    })
    segmentEdite.value = null
    segments.value = await chargerSegments(pisteSelectionnee.value.id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    segmentLoading.value = false
  }
}

const supprimerSegmentAction = async (segId: string) => {
  if (!confirm('Supprimer ce segment ?') || !pisteSelectionnee.value) return
  await supprimerSegment(segId)
  segments.value = await chargerSegments(pisteSelectionnee.value.id)
  await rechargerPistes()
}

// ═══════════════════════════════════════════════════════════════
// TAP-TO-MARK
// ═══════════════════════════════════════════════════════════════

const ouvrirTapToMark = (seg: SegmentSousTitre) => {
  tapToMarkSegment.value = seg
  showTapToMark.value = true
}

const onTimingsEnregistres = async (timings: TimingMot[]) => {
  if (!tapToMarkSegment.value || !pisteSelectionnee.value) return
  await enregistrerTimingsMot(tapToMarkSegment.value.id, timings)
  showTapToMark.value = false
  tapToMarkSegment.value = null
  segments.value = await chargerSegments(pisteSelectionnee.value.id)
  await rechargerPistes()
}

const supprimerTimingsAction = async (segId: string) => {
  if (!confirm('Supprimer les timings mot de ce segment ?') || !pisteSelectionnee.value) return
  await supprimerTimingsMot(segId)
  segments.value = await chargerSegments(pisteSelectionnee.value.id)
  await rechargerPistes()
}

// ═══════════════════════════════════════════════════════════════
// PRÉVISUALISATION
// ═══════════════════════════════════════════════════════════════

const segmentsPrevisualisation = computed(() => {
  return segments.value.map(s => ({
    position: s.position,
    texte: s.texte,
    debut_ms: s.debut_ms,
    fin_ms: s.fin_ms,
    mots: s.timings_mot.map(t => ({
      position: t.position,
      mot: t.mot,
      debut_ms: t.debut_ms,
      fin_ms: t.fin_ms,
    })),
  }))
})

// ═══════════════════════════════════════════════════════════════
// INIT
// ═══════════════════════════════════════════════════════════════

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="videoDetail?.titre || 'Chargement...'"
      sous-titre="Éditer la vidéo et gérer les sous-titres"
    >
      <template #actions>
        <NuxtLink to="/admin/vidafrica" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="erreurLocale || error" class="alert alert-error mb-4">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ erreurLocale || error }}</span>
      <button class="btn btn-ghost btn-xs" @click="erreurLocale = ''">✕</button>
    </div>

    <!-- Onglets -->
    <div role="tablist" class="tabs tabs-bordered mb-4">
      <a role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'infos' }" @click="ongletActif = 'infos'">
        <font-awesome-icon icon="circle-info" class="mr-1" /> Informations
      </a>
      <a role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'sous-titres' }" @click="ongletActif = 'sous-titres'">
        <font-awesome-icon icon="closed-captioning" class="mr-1" /> Sous-titres
        <span v-if="pistes.length" class="badge badge-sm ml-1">{{ pistes.length }}</span>
      </a>
    </div>

    <!-- ═══ ONGLET INFORMATIONS ═══ -->
    <div v-if="ongletActif === 'infos' && videoDetail" class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <!-- Aperçu vidéo -->
        <div v-if="videoDetail.fichier_video_url" class="mb-4">
          <video
            :src="videoDetail.fichier_video_url"
            controls
            class="w-full max-w-lg rounded-lg"
            preload="metadata"
          />
        </div>

        <!-- État + actions rapides -->
        <div class="flex items-center gap-2 mb-4">
          <span class="badge" :class="{
            'badge-success': videoDetail.etat === 'publie',
            'badge-info': videoDetail.etat === 'brouillon',
            'badge-error': videoDetail.etat === 'suspendu',
          }">
            {{ videoDetail.etat }}
          </span>
          <span v-if="videoDetail.duree_secondes" class="text-sm text-base-content/60">
            {{ formaterDuree(videoDetail.duree_secondes) }}
          </span>
          <span v-if="videoDetail.format_video" class="badge badge-outline badge-sm">{{ videoDetail.format_video }}</span>

          <div class="ml-auto flex gap-1">
            <button
              v-if="videoDetail.etat !== 'publie'"
              class="btn btn-success btn-sm"
              :disabled="etatLoading"
              @click="changerEtatVideo('publie')"
            >
              <font-awesome-icon icon="eye" class="mr-1" /> Publier
            </button>
            <button
              v-if="videoDetail.etat === 'publie'"
              class="btn btn-warning btn-sm"
              :disabled="etatLoading"
              @click="changerEtatVideo('suspendu')"
            >
              Suspendre
            </button>
            <button
              v-if="videoDetail.etat !== 'brouillon'"
              class="btn btn-ghost btn-sm"
              :disabled="etatLoading"
              @click="changerEtatVideo('brouillon')"
            >
              Brouillon
            </button>
            <button class="btn btn-error btn-sm" @click="supprimerVideo">
              <font-awesome-icon icon="trash" />
            </button>
          </div>
        </div>

        <div class="divider">Modifier</div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Titre</span></label>
          <input v-model="form.titre" type="text" class="input input-bordered" maxlength="300" />
        </div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Description</span></label>
          <textarea v-model="form.description" class="textarea textarea-bordered h-24" />
        </div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Territoires</span></label>
          <select v-model="territoireAAjouter" class="select select-bordered" @change="ajouterTerritoire">
            <option value="">Ajouter un territoire…</option>
            <option v-for="p in territoiresDisponibles" :key="p" :value="p">{{ p }}</option>
          </select>
          <div v-if="territoires.length" class="flex flex-wrap gap-2 mt-2">
            <span v-for="t in territoires" :key="t" class="badge badge-primary gap-1">
              {{ t }}
              <button type="button" class="hover:text-error" @click="retirerTerritoire(t)">
                <font-awesome-icon icon="xmark" />
              </button>
            </span>
          </div>
        </div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Auteur réel</span></label>
          <input
            v-model="form.auteur_reel"
            type="text"
            class="input input-bordered"
            placeholder="Nom de l'auteur réel de la chanson (le cas échéant)"
            maxlength="300"
          />
        </div>

        <div class="form-control mb-4">
          <label class="label cursor-pointer justify-start gap-3">
            <input v-model="dechargeDroits" type="checkbox" class="checkbox checkbox-primary" />
            <span class="label-text">Le contributeur ne revendique aucun droit sur cette œuvre (décharge de droits)</span>
          </label>
        </div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Nouvelle vignette</span></label>
          <input type="file" class="file-input file-input-bordered file-input-sm" accept=".jpg,.jpeg,.png,.webp" @change="onVignetteChange" />
        </div>

        <div class="card-actions justify-end">
          <button class="btn btn-primary" :class="{ loading: soumettreLoading }" :disabled="soumettreLoading" @click="sauvegarder">
            Enregistrer les modifications
          </button>
        </div>
      </div>
    </div>

    <!-- ═══ ONGLET SOUS-TITRES ═══ -->
    <div v-if="ongletActif === 'sous-titres'" class="space-y-4">
      <!-- Liste des pistes -->
      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="card-title text-base">Pistes de sous-titres</h3>

          <div v-if="pistes.length === 0" class="text-sm text-base-content/60 py-2">
            Aucune piste de sous-titres. Ajoutez-en une pour commencer.
          </div>

          <div class="space-y-2">
            <div
              v-for="piste in pistes" :key="piste.id"
              class="flex items-center gap-3 p-2 rounded-lg cursor-pointer hover:bg-base-200 transition-colors"
              :class="{ 'bg-primary/10 border border-primary/30': pisteSelectionnee?.id === piste.id }"
              @click="selectionnerPiste(piste)"
            >
              <span class="font-medium">{{ LANGUES_LABELS[piste.langue] || piste.langue }}</span>
              <span class="badge badge-sm" :class="{
                'badge-success': piste.etat === 'publie',
                'badge-warning': piste.etat === 'brouillon',
                'badge-ghost': piste.etat === 'masque',
              }">
                {{ piste.etat === 'publie' ? 'Publiée' : piste.etat === 'masque' ? 'Masquée' : 'En attente' }}
              </span>
              <span class="badge badge-sm" :class="piste.est_complete ? 'badge-success badge-outline' : 'badge-warning badge-outline'">
                {{ piste.est_complete ? 'Complet' : 'Incomplet' }}
              </span>
              <span class="text-xs text-base-content/50">{{ piste.nombre_segments }} segments</span>
              <span v-if="piste.cree_par_nom" class="text-xs text-base-content/50" title="Auteur">
                <font-awesome-icon icon="user" class="mr-0.5" />{{ piste.cree_par_nom }}
              </span>
              <div class="ml-auto flex gap-1" @click.stop>
                <button
                  v-if="piste.etat !== 'publie'"
                  class="btn btn-success btn-xs"
                  title="Valider et publier la piste"
                  @click="changerEtatPisteAction(piste.id, 'publie')"
                >
                  <font-awesome-icon icon="eye" class="mr-1" /> Publier
                </button>
                <button
                  v-if="piste.etat === 'publie'"
                  class="btn btn-ghost btn-xs"
                  title="Masquer la piste"
                  @click="changerEtatPisteAction(piste.id, 'masque')"
                >
                  Masquer
                </button>
                <AdminMiseEnAvantBouton
                  v-if="piste.etat === 'publie'"
                  type-objet="video"
                  :objet-id="piste.id"
                />
                <button class="btn btn-ghost btn-xs text-error" @click="supprimerPisteAction(piste.id)">
                  <font-awesome-icon icon="trash" />
                </button>
              </div>
            </div>
          </div>

          <!-- Ajouter une piste -->
          <div v-if="languesDisponibles.length > 0" class="flex items-end gap-2 mt-3">
            <div class="form-control flex-1">
              <label class="label"><span class="label-text text-sm">Ajouter une piste</span></label>
              <select v-model="nouvelleLangue" class="select select-bordered select-sm">
                <option value="">Choisir une langue</option>
                <option v-for="l in languesDisponibles" :key="l.code" :value="l.code">{{ l.label }}</option>
              </select>
            </div>
            <button class="btn btn-primary btn-sm" :disabled="!nouvelleLangue || pisteLoading" @click="ajouterPiste">
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>
        </div>
      </div>

      <!-- Segments de la piste sélectionnée -->
      <div v-if="pisteSelectionnee" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div class="flex items-center justify-between">
            <h3 class="card-title text-base">
              Segments — {{ LANGUES_LABELS[pisteSelectionnee.langue] || pisteSelectionnee.langue }}
            </h3>
            <div class="flex gap-2">
              <button
                v-if="segments.length > 0 && segments.some(s => s.timings_mot.length > 0)"
                class="btn btn-info btn-sm"
                @click="showPrevisualisation = true"
              >
                <font-awesome-icon icon="play" class="mr-1" /> Prévisualiser
              </button>
              <button class="btn btn-primary btn-sm" @click="showAjoutSegment = !showAjoutSegment">
                <font-awesome-icon icon="plus" class="mr-1" /> Nouveau segment
              </button>
            </div>
          </div>

          <!-- Formulaire ajout segment -->
          <div v-if="showAjoutSegment" class="bg-base-200 rounded-lg p-3 mt-3">
            <div class="form-control mb-2">
              <label class="label"><span class="label-text text-sm">Texte du segment</span></label>
              <textarea v-model="nouveauSegment.texte" class="textarea textarea-bordered textarea-sm" placeholder="Ex: Bienvenue dans ce documentaire" />
            </div>
            <div class="flex gap-3">
              <div class="form-control flex-1">
                <label class="label"><span class="label-text text-sm">Début (ms)</span></label>
                <input v-model.number="nouveauSegment.debut_ms" type="number" min="0" class="input input-bordered input-sm" />
              </div>
              <div class="form-control flex-1">
                <label class="label"><span class="label-text text-sm">Fin (ms)</span></label>
                <input v-model.number="nouveauSegment.fin_ms" type="number" min="0" class="input input-bordered input-sm" />
              </div>
            </div>
            <div class="flex justify-end gap-2 mt-3">
              <button class="btn btn-ghost btn-sm" @click="showAjoutSegment = false">Annuler</button>
              <button class="btn btn-primary btn-sm" :disabled="segmentLoading" @click="ajouterSegment">Ajouter</button>
            </div>
          </div>

          <!-- Liste des segments -->
          <div v-if="segmentLoading" class="flex justify-center py-4">
            <span class="loading loading-spinner" />
          </div>

          <div v-else-if="segments.length === 0" class="text-sm text-base-content/60 py-4 text-center">
            Aucun segment. Ajoutez des segments avec leur texte et timestamps.
          </div>

          <div v-else class="space-y-2 mt-3">
            <div v-for="seg in segments" :key="seg.id" class="border border-base-300 rounded-lg p-3">
              <!-- Mode affichage -->
              <div v-if="segmentEdite !== seg.id">
                <div class="flex items-start justify-between">
                  <div>
                    <span class="badge badge-outline badge-sm mr-2">#{{ seg.position }}</span>
                    <span class="text-sm font-medium">{{ seg.texte }}</span>
                  </div>
                  <div class="flex gap-1">
                    <button class="btn btn-ghost btn-xs" title="Marquer les timings" @click="ouvrirTapToMark(seg)">
                      <font-awesome-icon icon="stopwatch" />
                    </button>
                    <button class="btn btn-ghost btn-xs" title="Modifier" @click="debutEdition(seg)">
                      <font-awesome-icon icon="pen" />
                    </button>
                    <button class="btn btn-ghost btn-xs text-error" @click="supprimerSegmentAction(seg.id)">
                      <font-awesome-icon icon="trash" />
                    </button>
                  </div>
                </div>
                <div class="text-xs text-base-content/50 mt-1">
                  {{ formaterTimestamp(seg.debut_ms) }} → {{ formaterTimestamp(seg.fin_ms) }}
                </div>

                <!-- Timings mot -->
                <div v-if="seg.timings_mot.length > 0" class="mt-2 flex flex-wrap gap-1">
                  <span
                    v-for="t in seg.timings_mot" :key="t.position"
                    class="badge badge-sm badge-outline"
                    :title="`${formaterTimestamp(t.debut_ms)} → ${formaterTimestamp(t.fin_ms)}`"
                  >
                    {{ t.mot }}
                  </span>
                  <button class="btn btn-ghost btn-xs text-warning" title="Remarquer" @click="ouvrirTapToMark(seg)">
                    <font-awesome-icon icon="redo" size="xs" />
                  </button>
                  <button class="btn btn-ghost btn-xs text-error" title="Supprimer timings" @click="supprimerTimingsAction(seg.id)">
                    <font-awesome-icon icon="xmark" size="xs" />
                  </button>
                </div>
                <div v-else class="text-xs text-warning mt-1">
                  <font-awesome-icon icon="exclamation-triangle" /> Pas de timings mot — cliquez sur <font-awesome-icon icon="stopwatch" /> pour marquer
                </div>
              </div>

              <!-- Mode édition -->
              <div v-else>
                <div class="form-control mb-2">
                  <textarea v-model="segmentEditeData.texte" class="textarea textarea-bordered textarea-sm" />
                </div>
                <div class="flex gap-3 mb-2">
                  <div class="form-control flex-1">
                    <input v-model.number="segmentEditeData.debut_ms" type="number" min="0" class="input input-bordered input-sm" placeholder="Début (ms)" />
                  </div>
                  <div class="form-control flex-1">
                    <input v-model.number="segmentEditeData.fin_ms" type="number" min="0" class="input input-bordered input-sm" placeholder="Fin (ms)" />
                  </div>
                </div>
                <div class="flex justify-end gap-2">
                  <button class="btn btn-ghost btn-xs" @click="segmentEdite = null">Annuler</button>
                  <button class="btn btn-primary btn-xs" @click="sauvegarderSegment">Enregistrer</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Tap-to-mark modal -->
      <div v-if="showTapToMark && tapToMarkSegment && videoDetail" class="modal modal-open">
        <div class="modal-box max-w-3xl">
          <h3 class="font-bold text-lg mb-3">Marquer les timings mot par mot</h3>
          <VidafricaTapToMark
            :video-url="videoDetail.fichier_video_url"
            :mots="tapToMarkSegment.texte.split(/\s+/)"
            :debut-ms="tapToMarkSegment.debut_ms"
            :fin-ms="tapToMarkSegment.fin_ms"
            @timings-enregistres="onTimingsEnregistres"
            @annuler="showTapToMark = false"
          />
        </div>
        <div class="modal-backdrop" @click="showTapToMark = false" />
      </div>

      <!-- Prévisualisation modal -->
      <div v-if="showPrevisualisation && videoDetail" class="modal modal-open">
        <div class="modal-box max-w-4xl">
          <h3 class="font-bold text-lg mb-3">Prévisualisation karaoké</h3>
          <VidafricaLecteur
            :video-url="videoDetail.fichier_video_url"
            :segments="segmentsPrevisualisation"
          />
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showPrevisualisation = false">Fermer</button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showPrevisualisation = false" />
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading && !videoDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>
  </div>
</template>
