<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const type = (route.query.type as string) || 'stations'

const {
  stationDetail, chaineDetail, programmeDetail,
  chargerStation, chargerChaine, chargerProgramme,
  modifierStation, modifierChaine, modifierProgramme,
  loading, error,
} = useAdminRadioTele()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

// Station Radio form
const stationForm = reactive({
  nom: '',
  description: '',
  stream_url: '',
  image_couverture_url: '',
  genre: '',
  genres_liste: [] as string[],
  pays_id: '',
  ville: '',
  type_station: 'nationale',
})

const genreInput = ref('')

const ajouterGenre = () => {
  const g = genreInput.value.trim()
  if (g && !stationForm.genres_liste.includes(g)) {
    stationForm.genres_liste.push(g)
    genreInput.value = ''
  }
}

const retirerGenre = (index: number) => {
  stationForm.genres_liste.splice(index, 1)
}

// Chaine TV form
const chaineForm = reactive({
  nom: '',
  description: '',
  stream_url: '',
  image_couverture_url: '',
  categorie: 'generaliste',
  pays_id: '',
  langue: '',
  est_en_direct: false,
})

// Programme Media form
const programmeForm = reactive({
  nom_emission: '',
  type_programme: 'radio',
  description: '',
  image_couverture_url: '',
  video_url: '',
  info_animateur: '',
  info_producteur: '',
  pays_id: '',
  est_international: false,
  langue: '',
  categorie_radio: '',
})

// Titre dynamique
const titreDetail = computed(() => {
  if (type === 'stations') return stationDetail.value?.nom || 'Chargement...'
  if (type === 'chaines') return chaineDetail.value?.nom || 'Chargement...'
  if (type === 'programmes') return programmeDetail.value?.nom_emission || 'Chargement...'
  return 'Chargement...'
})

const sousTitreMap: Record<string, string> = {
  stations: 'Modifier la station radio',
  chaines: 'Modifier la chaine TV',
  programmes: 'Modifier le programme media',
}

// Etat courant
const etatCourant = computed(() => {
  if (type === 'stations') return stationDetail.value?.etat || ''
  if (type === 'chaines') return chaineDetail.value?.etat || ''
  if (type === 'programmes') return programmeDetail.value?.etat || ''
  return ''
})

const detailCharge = computed(() => {
  if (type === 'stations') return !!stationDetail.value
  if (type === 'chaines') return !!chaineDetail.value
  if (type === 'programmes') return !!programmeDetail.value
  return false
})

// Badge etat
const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'badge-warning',
    publie: 'badge-success',
    suspendu: 'badge-error',
    supprime: 'badge-ghost',
  }
  return map[etat] || 'badge-info'
}

const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'Brouillon',
    publie: 'Publie',
    suspendu: 'Suspendu',
    supprime: 'Supprime',
  }
  return map[etat] || etat
}

// Modal changement d'etat
const showEtatModal = ref(false)
const nouvelEtat = ref('')
const etatLoading = ref(false)

const ouvrirEtatModal = () => {
  nouvelEtat.value = etatCourant.value
  showEtatModal.value = true
}

const executerChangerEtat = async () => {
  if (nouvelEtat.value === etatCourant.value) return
  etatLoading.value = true
  try {
    if (type === 'stations') await modifierStation(id, { etat: nouvelEtat.value } as any)
    else if (type === 'chaines') await modifierChaine(id, { etat: nouvelEtat.value } as any)
    else if (type === 'programmes') await modifierProgramme(id, { etat: nouvelEtat.value } as any)

    showEtatModal.value = false
    successMsg.value = `Etat change en "${etatLabel(nouvelEtat.value)}"`
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
  finally { etatLoading.value = false }
}

// Chargement initial
const charger = async () => {
  if (type === 'stations') {
    await chargerStation(id)
    if (stationDetail.value) {
      const s = stationDetail.value
      stationForm.nom = s.nom
      stationForm.description = s.description || ''
      stationForm.stream_url = s.stream_url || ''
      stationForm.image_couverture_url = s.image_couverture_url || ''
      stationForm.genre = s.genre || ''
      stationForm.genres_liste = s.genres_liste || []
      stationForm.pays_id = s.pays_id || ''
      stationForm.ville = s.ville || ''
      stationForm.type_station = s.type_station || 'nationale'
    }
  }
  else if (type === 'chaines') {
    await chargerChaine(id)
    if (chaineDetail.value) {
      const c = chaineDetail.value
      chaineForm.nom = c.nom
      chaineForm.description = c.description || ''
      chaineForm.stream_url = c.stream_url || ''
      chaineForm.image_couverture_url = c.image_couverture_url || ''
      chaineForm.categorie = c.categorie || 'generaliste'
      chaineForm.pays_id = c.pays_id || ''
      chaineForm.langue = c.langue || ''
      chaineForm.est_en_direct = c.est_en_direct || false
    }
  }
  else if (type === 'programmes') {
    await chargerProgramme(id)
    if (programmeDetail.value) {
      const p = programmeDetail.value
      programmeForm.nom_emission = p.nom_emission
      programmeForm.type_programme = p.type_programme || 'radio'
      programmeForm.description = p.description || ''
      programmeForm.image_couverture_url = p.image_couverture_url || ''
      programmeForm.video_url = p.video_url || ''
      programmeForm.info_animateur = p.info_animateur || ''
      programmeForm.info_producteur = p.info_producteur || ''
      programmeForm.pays_id = p.pays_id || ''
      programmeForm.est_international = p.est_international || false
      programmeForm.langue = p.langue || ''
      programmeForm.categorie_radio = p.categorie_radio || ''
    }
  }
}

// Sauvegarde
const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null

  try {
    if (type === 'stations') {
      const body: any = {
        nom: stationForm.nom.trim(),
        stream_url: stationForm.stream_url.trim(),
        type_station: stationForm.type_station,
      }
      if (stationForm.description.trim()) body.description = stationForm.description.trim()
      if (stationForm.image_couverture_url.trim()) body.image_couverture_url = stationForm.image_couverture_url.trim()
      if (stationForm.genre.trim()) body.genre = stationForm.genre.trim()
      if (stationForm.genres_liste.length) body.genres_liste = stationForm.genres_liste
      if (stationForm.pays_id) body.pays_id = stationForm.pays_id
      if (stationForm.ville.trim()) body.ville = stationForm.ville.trim()

      await modifierStation(id, body)
    }
    else if (type === 'chaines') {
      const body: any = {
        nom: chaineForm.nom.trim(),
        stream_url: chaineForm.stream_url.trim(),
        categorie: chaineForm.categorie,
        est_en_direct: chaineForm.est_en_direct,
      }
      if (chaineForm.description.trim()) body.description = chaineForm.description.trim()
      if (chaineForm.image_couverture_url.trim()) body.image_couverture_url = chaineForm.image_couverture_url.trim()
      if (chaineForm.pays_id) body.pays_id = chaineForm.pays_id
      if (chaineForm.langue.trim()) body.langue = chaineForm.langue.trim()

      await modifierChaine(id, body)
    }
    else if (type === 'programmes') {
      const body: any = {
        nom_emission: programmeForm.nom_emission.trim(),
        type_programme: programmeForm.type_programme,
        est_international: programmeForm.est_international,
      }
      if (programmeForm.description.trim()) body.description = programmeForm.description.trim()
      if (programmeForm.image_couverture_url.trim()) body.image_couverture_url = programmeForm.image_couverture_url.trim()
      if (programmeForm.video_url.trim()) body.video_url = programmeForm.video_url.trim()
      if (programmeForm.info_animateur.trim()) body.info_animateur = programmeForm.info_animateur.trim()
      if (programmeForm.info_producteur.trim()) body.info_producteur = programmeForm.info_producteur.trim()
      if (programmeForm.pays_id) body.pays_id = programmeForm.pays_id
      if (programmeForm.langue.trim()) body.langue = programmeForm.langue.trim()
      if (programmeForm.categorie_radio) body.categorie_radio = programmeForm.categorie_radio

      await modifierProgramme(id, body)
    }

    successMsg.value = 'Mis a jour avec succes'
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
  finally { saving.value = false }
}

// Metadonnees
const metaCreeParNom = computed(() => {
  if (type === 'stations') return stationDetail.value?.cree_par_nom
  if (type === 'chaines') return chaineDetail.value?.cree_par_nom
  if (type === 'programmes') return programmeDetail.value?.cree_par_nom
  return null
})

const metaId = computed(() => {
  if (type === 'stations') return stationDetail.value?.id
  if (type === 'chaines') return chaineDetail.value?.id
  if (type === 'programmes') return programmeDetail.value?.id
  return null
})

const metaPaysNom = computed(() => {
  if (type === 'stations') return stationDetail.value?.pays_nom
  if (type === 'chaines') return chaineDetail.value?.pays_nom
  if (type === 'programmes') return programmeDetail.value?.pays_nom
  return null
})

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="titreDetail" :sous-titre="sousTitreMap[type] || ''">
      <template #actions>
        <NuxtLink to="/admin/radio-tele" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !detailCharge" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="detailCharge">
      <!-- Barre d'etat + moderation -->
      <div class="flex items-center gap-3 mb-4">
        <span :class="['badge', etatBadge(etatCourant)]">
          {{ etatLabel(etatCourant) }}
        </span>
        <button class="btn btn-outline btn-xs" @click="ouvrirEtatModal">
          <font-awesome-icon icon="arrows-rotate" class="mr-1" /> Changer etat
        </button>
      </div>

      <!-- Alertes -->
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">x</button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Formulaire Station Radio -->
      <div v-if="type === 'stations'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-6">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Nom de la station *</span></label>
                <input v-model="stationForm.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL de stream *</span></label>
                <input v-model="stationForm.stream_url" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="stationForm.description" class="textarea textarea-bordered h-32" />
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL image de couverture</span></label>
                <input v-model="stationForm.image_couverture_url" type="text" class="input input-bordered">
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Classification</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Genre principal</span></label>
                  <input v-model="stationForm.genre" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Type de station</span></label>
                  <select v-model="stationForm.type_station" class="select select-bordered">
                    <option value="nationale">Nationale</option>
                    <option value="locale">Locale</option>
                    <option value="internationale">Internationale</option>
                  </select>
                </div>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Genres (tags)</span></label>
                <div class="flex gap-2">
                  <input v-model="genreInput" type="text" class="input input-bordered flex-1" placeholder="Ex: Afrobeat" @keydown.enter.prevent="ajouterGenre">
                  <button type="button" class="btn btn-outline" @click="ajouterGenre">Ajouter</button>
                </div>
                <div v-if="stationForm.genres_liste.length" class="flex flex-wrap gap-2 mt-2">
                  <span v-for="(g, i) in stationForm.genres_liste" :key="i" class="badge badge-primary gap-1">
                    {{ g }}
                    <button type="button" class="btn btn-ghost btn-xs p-0" @click="retirerGenre(i)">&times;</button>
                  </span>
                </div>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Localisation</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Pays</span></label>
                  <input v-model="stationForm.pays_id" type="text" class="input input-bordered" placeholder="UUID du pays">
                  <label v-if="metaPaysNom" class="label"><span class="label-text-alt text-success">{{ metaPaysNom }}</span></label>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Ville</span></label>
                  <input v-model="stationForm.ville" type="text" class="input input-bordered">
                </div>
              </div>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="metaCreeParNom">Cree par {{ metaCreeParNom }}</span>
                <br>ID: {{ metaId?.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Formulaire Chaine TV -->
      <div v-else-if="type === 'chaines'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-6">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Nom de la chaine *</span></label>
                <input v-model="chaineForm.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL de stream *</span></label>
                <input v-model="chaineForm.stream_url" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="chaineForm.description" class="textarea textarea-bordered h-32" />
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL image de couverture</span></label>
                <input v-model="chaineForm.image_couverture_url" type="text" class="input input-bordered">
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Classification</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Categorie</span></label>
                  <select v-model="chaineForm.categorie" class="select select-bordered">
                    <option value="generaliste">Generaliste</option>
                    <option value="info">Info</option>
                    <option value="sport">Sport</option>
                    <option value="culture">Culture</option>
                    <option value="divertissement">Divertissement</option>
                    <option value="education">Education</option>
                    <option value="musique">Musique</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Langue</span></label>
                  <input v-model="chaineForm.langue" type="text" class="input input-bordered">
                </div>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Localisation & diffusion</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Pays</span></label>
                <input v-model="chaineForm.pays_id" type="text" class="input input-bordered" placeholder="UUID du pays">
                <label v-if="metaPaysNom" class="label"><span class="label-text-alt text-success">{{ metaPaysNom }}</span></label>
              </div>
              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input v-model="chaineForm.est_en_direct" type="checkbox" class="checkbox checkbox-primary" />
                  <span class="label-text">En direct actuellement</span>
                </label>
              </div>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="metaCreeParNom">Cree par {{ metaCreeParNom }}</span>
                <br>ID: {{ metaId?.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Formulaire Programme Media -->
      <div v-else-if="type === 'programmes'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-6">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Nom de l'emission *</span></label>
                <input v-model="programmeForm.nom_emission" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Type de programme</span></label>
                <select v-model="programmeForm.type_programme" class="select select-bordered">
                  <option value="radio">Radio</option>
                  <option value="tele">Tele</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="programmeForm.description" class="textarea textarea-bordered h-32" />
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Medias</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">URL image de couverture</span></label>
                  <input v-model="programmeForm.image_couverture_url" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">URL video</span></label>
                  <input v-model="programmeForm.video_url" type="text" class="input input-bordered">
                </div>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Equipe & production</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Animateur</span></label>
                  <input v-model="programmeForm.info_animateur" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Producteur</span></label>
                  <input v-model="programmeForm.info_producteur" type="text" class="input input-bordered">
                </div>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Classification & localisation</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Pays</span></label>
                  <input v-model="programmeForm.pays_id" type="text" class="input input-bordered" placeholder="UUID du pays">
                  <label v-if="metaPaysNom" class="label"><span class="label-text-alt text-success">{{ metaPaysNom }}</span></label>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Langue</span></label>
                  <input v-model="programmeForm.langue" type="text" class="input input-bordered">
                </div>
              </div>
              <div v-if="programmeForm.type_programme === 'radio'" class="form-control">
                <label class="label"><span class="label-text">Categorie radio</span></label>
                <select v-model="programmeForm.categorie_radio" class="select select-bordered">
                  <option value="">Non specifie</option>
                  <option value="information">Information</option>
                  <option value="divertissement">Divertissement</option>
                  <option value="musique">Musique</option>
                  <option value="culture">Culture</option>
                  <option value="sport">Sport</option>
                  <option value="education">Education</option>
                  <option value="debat">Debat</option>
                  <option value="religieux">Religieux</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input v-model="programmeForm.est_international" type="checkbox" class="checkbox checkbox-primary" />
                  <span class="label-text">Programme international</span>
                </label>
              </div>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="metaCreeParNom">Cree par {{ metaCreeParNom }}</span>
                <br>ID: {{ metaId?.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Modal changement d'etat -->
      <div v-if="showEtatModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">Changer l'etat</h3>
          <div class="form-control">
            <select v-model="nouvelEtat" class="select select-bordered">
              <option value="brouillon">Brouillon</option>
              <option value="publie">Publie</option>
              <option value="suspendu">Suspendu</option>
            </select>
          </div>
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showEtatModal = false">Annuler</button>
            <button class="btn btn-primary" :class="{ loading: etatLoading }" :disabled="etatLoading || nouvelEtat === etatCourant" @click="executerChangerEtat">
              Confirmer
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showEtatModal = false" />
      </div>
    </template>
  </div>
</template>
