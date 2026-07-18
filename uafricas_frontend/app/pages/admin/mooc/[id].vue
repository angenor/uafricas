<script setup lang="ts">
import type { TableColumn } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const {
  moocDetail, inscriptions, inscriptionStats,
  chargerDetail, modifier, changerEtat, chargerInscriptions, chargerStatsInscriptions,
  loading, error,
} = useAdminMooc()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const ongletActif = ref<'infos' | 'programme' | 'inscriptions'>('infos')

const form = reactive({
  titre: '',
  description: '',
  type_formation: '',
  format: '',
  langue: '',
  date_heure_debut: '',
  date_heure_fin: '',
  nombre_places: null as number | null,
  pays_id: '',
  ville: '',
  lien_en_ligne: '',
  prerequis: '',
  objectif: '',
  presentation: '',
  a_evaluation: false,
  est_certifiante: false,
  image_couverture_url: '',
})

const formats = [
  { label: 'Presentiel', value: 'presentiel' },
  { label: 'En ligne', value: 'en_ligne' },
  { label: 'Hybride', value: 'hybride' },
]

const typesFormation = [
  { label: 'Cours', value: 'cours' },
  { label: 'Atelier', value: 'atelier' },
  { label: 'Seminaire', value: 'seminaire' },
  { label: 'Conference', value: 'conference' },
  { label: 'Certification', value: 'certification' },
  { label: 'Formations grand public', value: 'grand_public' },
]

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'badge-info',
    publie: 'badge-success',
    suspendu: 'badge-warning',
    annule: 'badge-error',
  }
  return map[etat] || 'badge-neutral'
}

const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'Brouillon',
    publie: 'Publie',
    suspendu: 'Suspendu',
    annule: 'Annule',
  }
  return map[etat] || etat
}

const statutBadge = (statut: string) => {
  const map: Record<string, string> = {
    inscrit: 'badge-info',
    en_cours: 'badge-warning',
    complete: 'badge-success',
    abandonne: 'badge-error',
  }
  return map[statut] || 'badge-neutral'
}

const statutLabel = (statut: string) => {
  const map: Record<string, string> = {
    inscrit: 'Inscrit',
    en_cours: 'En cours',
    complete: 'Complete',
    abandonne: 'Abandonne',
  }
  return map[statut] || statut
}

const inscriptionColonnes: TableColumn[] = [
  { key: 'nom_complet', label: 'Participant' },
  { key: 'email', label: 'Email' },
  { key: 'statut', label: 'Statut', width: 'w-24', align: 'center' },
  { key: 'progression', label: 'Progression', width: 'w-32', align: 'center' },
  { key: 'created_at', label: 'Date', width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

// Etat modal
const showEtatModal = ref(false)
const nouvelEtat = ref('')
const etatLoading = ref(false)

const charger = async () => {
  await chargerDetail(id)
  if (moocDetail.value) {
    const m = moocDetail.value
    form.titre = m.titre
    form.description = m.description || ''
    form.type_formation = m.type_formation || ''
    form.format = m.format || ''
    form.langue = m.langue || ''
    form.date_heure_debut = m.date_heure_debut || ''
    form.date_heure_fin = m.date_heure_fin || ''
    form.nombre_places = m.nombre_places
    form.pays_id = m.pays_id || ''
    form.ville = m.ville || ''
    form.lien_en_ligne = m.lien_en_ligne || ''
    form.prerequis = m.prerequis || ''
    form.objectif = m.objectif || ''
    form.presentation = m.presentation || ''
    form.a_evaluation = m.a_evaluation ?? false
    form.est_certifiante = m.est_certifiante ?? false
    form.image_couverture_url = m.image_couverture_url || ''
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = { ...form }
    if (!body.pays_id) delete body.pays_id
    if (body.nombre_places === null) delete body.nombre_places
    await modifier(id, body)
    successMsg.value = 'MOOC mis a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally { saving.value = false }
}

const ouvrirEtatModal = () => {
  if (moocDetail.value) {
    nouvelEtat.value = moocDetail.value.etat
    showEtatModal.value = true
  }
}

const executerChangerEtat = async () => {
  if (!moocDetail.value || nouvelEtat.value === moocDetail.value.etat) return
  etatLoading.value = true
  try {
    await changerEtat(id, nouvelEtat.value)
    showEtatModal.value = false
    await chargerDetail(id)
  } catch {} finally { etatLoading.value = false }
}

const chargerOngletInscriptions = async () => {
  await chargerInscriptions(id)
  await chargerStatsInscriptions(id)
}

onMounted(async () => {
  await charger()
  await chargerOngletInscriptions()
})
</script>

<template>
  <div>
    <AdminPageHeader :titre="moocDetail?.titre || 'Chargement...'" sous-titre="Modifier le MOOC">
      <template #actions>
        <NuxtLink to="/admin/mooc" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !moocDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="moocDetail">
      <!-- Etat actuel -->
      <div class="flex items-center gap-3 mb-4">
        <span :class="['badge', etatBadge(moocDetail.etat)]">
          {{ etatLabel(moocDetail.etat) }}
        </span>
        <button class="btn btn-outline btn-xs" @click="ouvrirEtatModal">
          <font-awesome-icon icon="arrows-rotate" class="mr-1" /> Changer etat
        </button>
        <span v-if="moocDetail.nombre_inscriptions > 0" class="text-sm text-base-content/70">
          {{ moocDetail.nombre_inscriptions }} inscription(s)
        </span>
      </div>

      <!-- Onglets -->
      <div class="tabs tabs-bordered mb-4">
        <button :class="['tab', { 'tab-active': ongletActif === 'infos' }]" @click="ongletActif = 'infos'">
          <font-awesome-icon icon="info-circle" class="mr-1" /> Informations
        </button>
        <button :class="['tab', { 'tab-active': ongletActif === 'programme' }]" @click="ongletActif = 'programme'">
          <font-awesome-icon icon="layer-group" class="mr-1" /> Programme
        </button>
        <button :class="['tab', { 'tab-active': ongletActif === 'inscriptions' }]" @click="ongletActif = 'inscriptions'">
          <font-awesome-icon icon="users" class="mr-1" /> Inscriptions
          <span v-if="moocDetail.nombre_inscriptions" class="badge badge-sm ml-1">{{ moocDetail.nombre_inscriptions }}</span>
        </button>
      </div>

      <!-- Onglet Informations -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div v-if="erreurLocale || error" class="alert alert-error mb-4">
            <font-awesome-icon icon="circle-exclamation" />
            <span>{{ erreurLocale || error }}</span>
          </div>
          <div v-if="successMsg" class="alert alert-success mb-4">
            <font-awesome-icon icon="circle-check" />
            <span>{{ successMsg }}</span>
          </div>

          <form @submit.prevent="sauvegarder" class="space-y-6">
            <!-- Infos de base -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Titre *</span></label>
                <input v-model="form.titre" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="form.description" class="textarea textarea-bordered h-32" />
              </div>
            </div>

            <!-- Type & format -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Type & format</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Type de formation</span></label>
                  <select v-model="form.type_formation" class="select select-bordered">
                    <option value="">Non specifie</option>
                    <option v-for="t in typesFormation" :key="t.value" :value="t.value">{{ t.label }}</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Format</span></label>
                  <select v-model="form.format" class="select select-bordered">
                    <option value="">Non specifie</option>
                    <option v-for="f in formats" :key="f.value" :value="f.value">{{ f.label }}</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Langue</span></label>
                  <input v-model="form.langue" type="text" class="input input-bordered" placeholder="Ex: Francais">
                </div>
              </div>
            </div>

            <!-- Dates -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Dates & capacite</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Date de debut *</span></label>
                  <input v-model="form.date_heure_debut" type="datetime-local" class="input input-bordered" required>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Date de fin</span></label>
                  <input v-model="form.date_heure_fin" type="datetime-local" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Nombre de places</span></label>
                  <input v-model.number="form.nombre_places" type="number" min="1" class="input input-bordered">
                </div>
              </div>
            </div>

            <!-- Localisation -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Localisation</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Territoire</span></label>
                  <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="UUID du territoire">
                  <label v-if="moocDetail.pays_nom" class="label"><span class="label-text-alt text-success">{{ moocDetail.pays_nom }}</span></label>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Ville</span></label>
                  <input v-model="form.ville" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Lien en ligne</span></label>
                  <input v-model="form.lien_en_ligne" type="url" class="input input-bordered" placeholder="https://...">
                </div>
              </div>
            </div>

            <!-- Contenu pedagogique -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Contenu pedagogique</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Objectif</span></label>
                <textarea v-model="form.objectif" class="textarea textarea-bordered h-24" placeholder="Objectifs pedagogiques : ce que l'apprenant saura faire a l'issue de la formation..." />
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Presentation</span></label>
                <textarea v-model="form.presentation" class="textarea textarea-bordered h-32" placeholder="Presentation detaillee de la formation (affichee sur la page publique)..." />
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Prerequis</span></label>
                <textarea v-model="form.prerequis" class="textarea textarea-bordered" />
              </div>
            </div>

            <!-- Evaluation & certification -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Evaluation & certification</h3>
              <div class="flex flex-col gap-3">
                <label class="label cursor-pointer justify-start gap-3">
                  <input v-model="form.a_evaluation" type="checkbox" class="checkbox checkbox-primary">
                  <span class="label-text">La formation se termine par une evaluation</span>
                </label>
                <label class="label cursor-pointer justify-start gap-3">
                  <input v-model="form.est_certifiante" type="checkbox" class="checkbox checkbox-primary">
                  <span class="label-text">La formation est certifiante</span>
                </label>
              </div>
            </div>

            <!-- Medias -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Medias</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Image de couverture</span></label>
                <OpportuniteAfriqueImageUploadField v-model="form.image_couverture_url" label="" />
              </div>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="moocDetail.cree_par_nom">Cree par {{ moocDetail.cree_par_nom }}</span>
                <span v-if="moocDetail.progression_moyenne !== undefined"> &middot; Progression moyenne : {{ moocDetail.progression_moyenne }}%</span>
                <br>ID: {{ moocDetail.id.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Programme -->
      <div v-else-if="ongletActif === 'programme'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="text-lg font-semibold border-b pb-2 mb-2">Programme — chapitres & leçons</h3>
          <p class="text-sm text-base-content/60 mb-4">
            Structurez la formation en chapitres et leçons. Le contenu des leçons (vidéo, support, document)
            n'est accessible qu'aux membres inscrits ; sa structure reste visible publiquement.
          </p>
          <AdminFormationCurriculum :mooc-id="id" />
        </div>
      </div>

      <!-- Onglet Inscriptions -->
      <div v-else-if="ongletActif === 'inscriptions'">
        <!-- Stats cards -->
        <div v-if="inscriptionStats" class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-3 mb-4">
          <div class="stat bg-base-100 shadow-sm rounded-box p-4">
            <div class="stat-title text-xs">Total</div>
            <div class="stat-value text-xl">{{ inscriptionStats.total }}</div>
          </div>
          <div class="stat bg-base-100 shadow-sm rounded-box p-4">
            <div class="stat-title text-xs">Inscrits</div>
            <div class="stat-value text-xl text-info">{{ inscriptionStats.inscrits }}</div>
          </div>
          <div class="stat bg-base-100 shadow-sm rounded-box p-4">
            <div class="stat-title text-xs">En cours</div>
            <div class="stat-value text-xl text-warning">{{ inscriptionStats.en_cours }}</div>
          </div>
          <div class="stat bg-base-100 shadow-sm rounded-box p-4">
            <div class="stat-title text-xs">Completes</div>
            <div class="stat-value text-xl text-success">{{ inscriptionStats.completes }}</div>
          </div>
          <div class="stat bg-base-100 shadow-sm rounded-box p-4">
            <div class="stat-title text-xs">Abandonnes</div>
            <div class="stat-value text-xl text-error">{{ inscriptionStats.abandonnes }}</div>
          </div>
          <div class="stat bg-base-100 shadow-sm rounded-box p-4">
            <div class="stat-title text-xs">Progression moy.</div>
            <div class="stat-value text-xl">{{ inscriptionStats.progression_moyenne }}%</div>
          </div>
        </div>

        <!-- Table inscriptions -->
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!inscriptions.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" />
              <p>Aucune inscription pour ce MOOC</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead>
                  <tr>
                    <th v-for="col in inscriptionColonnes" :key="col.key" :class="col.width">{{ col.label }}</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="insc in inscriptions" :key="insc.id">
                    <td>{{ insc.nom }} {{ insc.prenom }}</td>
                    <td>{{ insc.email }}</td>
                    <td class="text-center">
                      <span :class="['badge badge-sm', statutBadge(insc.statut)]">
                        {{ statutLabel(insc.statut) }}
                      </span>
                    </td>
                    <td>
                      <div class="flex items-center gap-2">
                        <progress class="progress progress-primary w-20" :value="insc.progression" max="100" />
                        <span class="text-xs text-base-content/70">{{ insc.progression }}%</span>
                      </div>
                    </td>
                    <td>{{ new Date(insc.created_at).toLocaleDateString('fr-FR') }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- Modal changement etat -->
      <div v-if="showEtatModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">Changer l'etat du MOOC</h3>
          <div class="form-control">
            <select v-model="nouvelEtat" class="select select-bordered">
              <option value="brouillon">Brouillon</option>
              <option value="publie">Publie</option>
              <option value="suspendu">Suspendu</option>
              <option value="annule">Annule</option>
            </select>
          </div>
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showEtatModal = false">Annuler</button>
            <button class="btn btn-primary" :class="{ loading: etatLoading }" :disabled="etatLoading || nouvelEtat === moocDetail.etat" @click="executerChangerEtat">
              Confirmer
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showEtatModal = false" />
      </div>
    </template>
  </div>
</template>
