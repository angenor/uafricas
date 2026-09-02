<script setup lang="ts">
/**
 * Édition d'une **station de radio**.
 *
 * Les émissions ne s'éditent plus ici : depuis 09q ce sont des `emission_*`,
 * gérées sur `/admin/medias/emissions`. Le tableau du bas les liste seulement.
 *
 * Thématiques et couverture : exigées à l'état `publie` (FR-029, FR-035), pas
 * sur un brouillon : sinon une fiche en cours de saisie ne s'enregistrerait plus.
 */
import type { AdminEmission } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  stationDetail, chargerStation, modifierStation,
  loading, error,
  ORIGINES_PUBLICATION_RADIO, ROLES_PARTIE_PRENANTE_RADIO,
} = useAdminRadio()
const { listerPays } = useCentresCulturels()
const {
  listerReferentielsEdition, obtenirThematiques, definirThematiques,
  obtenirCouverture, definirCouverture,
} = useMediaSupport()
const { chargerEmissions, filtres: filtresEmissions, emissions, ETATS_EMISSION, libelleCadence } = useAdminMediaEmissions()

const paysDisponibles = ref<{ id: string; nom: string }[]>([])
const thematiquesRef = ref<{ id: string; nom: string }[]>([])
const territoiresRef = ref<{ id: string; nom: string }[]>([])

const thematiquesChoisies = ref<string[]>([])
const couvertureContinentale = ref(false)
const territoiresChoisis = ref<string[]>([])

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const stationForm = reactive({
  nom: '', description: '', stream_url: '', audio_url: '', image_couverture_url: '',
  genre: '', genres_liste: [] as string[], pays_id: '', ville: '', type_station: 'nationale', a_la_une: false,
  origine_publication: 'territoire', role_partie_prenante: '', role_partie_prenante_autre: '',
  contact_email: '', contact_telephone: '', contact_whatsapp: '',
  contact_site_web: '', contact_adresse: '',
})

const aideOrigine = computed(() => ORIGINES_PUBLICATION_RADIO.find(o => o.valeur === stationForm.origine_publication)?.aide || '')

const genreInput = ref('')
const ajouterGenre = () => {
  const g = genreInput.value.trim()
  if (g && !stationForm.genres_liste.includes(g)) { stationForm.genres_liste.push(g); genreInput.value = '' }
}
const retirerGenre = (i: number) => stationForm.genres_liste.splice(i, 1)

const titreDetail = computed(() => stationDetail.value?.nom || 'Chargement...')

const etatCourant = computed(() => stationDetail.value?.etat || '')
const detailCharge = computed(() => !!stationDetail.value)
/** Une station publiée doit porter thématique ET couverture. */
const exigeFiche = computed(() => etatCourant.value === 'publie')
const ficheComplete = computed(() =>
  thematiquesChoisies.value.length > 0
  && (couvertureContinentale.value || territoiresChoisis.value.length > 0),
)

const etatBadge = (etat: string) => ({ brouillon: 'badge-warning', publie: 'badge-success', suspendu: 'badge-error', supprime: 'badge-ghost' } as Record<string, string>)[etat] || 'badge-info'
const etatLabel = (etat: string) => ({ brouillon: 'Brouillon', publie: 'Publie', suspendu: 'Suspendu', supprime: 'Supprime' } as Record<string, string>)[etat] || etat

const showEtatModal = ref(false)
const nouvelEtat = ref('')
const etatLoading = ref(false)
const ouvrirEtatModal = () => { nouvelEtat.value = etatCourant.value; showEtatModal.value = true }
/**
 * Publier exige la fiche complète. Le refus est prononcé avant l'appel : le
 * serveur refuserait l'écriture des thématiques vides, mais la station serait
 * déjà passée « publiée » entre-temps.
 */
const executerChangerEtat = async () => {
  if (nouvelEtat.value === etatCourant.value) return
  if (nouvelEtat.value === 'publie' && !ficheComplete.value) {
    erreurLocale.value = 'Publier une station suppose au moins une thématique et une couverture territoriale.'
    showEtatModal.value = false
    return
  }
  etatLoading.value = true
  try {
    await modifierStation(id, { etat: nouvelEtat.value } as any)
    showEtatModal.value = false
    successMsg.value = `Etat changé en "${etatLabel(nouvelEtat.value)}"`
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { etatLoading.value = false }
}

const charger = async () => {
  await chargerStation(id)
  if (stationDetail.value) {
    const s = stationDetail.value
    stationForm.nom = s.nom
    stationForm.description = s.description || ''
    stationForm.stream_url = s.stream_url || ''
    stationForm.audio_url = s.audio_url || ''
    stationForm.image_couverture_url = s.image_couverture_url || ''
    stationForm.genre = s.genre || ''
    stationForm.genres_liste = s.genres_liste || []
    stationForm.pays_id = s.pays_id || ''
    stationForm.ville = s.ville || ''
    stationForm.type_station = s.type_station || 'nationale'
    stationForm.a_la_une = s.a_la_une || false
    stationForm.origine_publication = s.origine_publication || 'territoire'
    stationForm.role_partie_prenante = s.role_partie_prenante || ''
    stationForm.role_partie_prenante_autre = s.role_partie_prenante_autre || ''
    stationForm.contact_email = s.contact_email || ''
    stationForm.contact_telephone = s.contact_telephone || ''
    stationForm.contact_whatsapp = s.contact_whatsapp || ''
    stationForm.contact_site_web = s.contact_site_web || ''
    stationForm.contact_adresse = s.contact_adresse || ''
  }
}

const chargerFiche = async () => {
  const [themes, couverture] = await Promise.all([
    obtenirThematiques('station_radio', id, true),
    obtenirCouverture('station_radio', id, true),
  ])
  thematiquesChoisies.value = themes.map(t => t.id)
  couvertureContinentale.value = couverture?.couverture_continentale ?? false
  territoiresChoisis.value = couverture?.territoires.map(t => t.id) ?? []
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    if (!stationForm.audio_url.trim() && !stationForm.stream_url.trim()) {
      erreurLocale.value = 'Fournissez un fichier/lien audio ou une URL de flux live'
      return
    }
    if (stationForm.role_partie_prenante === 'autre' && !stationForm.role_partie_prenante_autre.trim()) {
      erreurLocale.value = 'Précisez le rôle lorsque « Autre » est sélectionné'
      return
    }
    if (exigeFiche.value && !ficheComplete.value) {
      erreurLocale.value = 'Une station publiée doit porter au moins une thématique et une couverture territoriale.'
      return
    }
    const body: any = {
      nom: stationForm.nom.trim(),
      stream_url: stationForm.stream_url.trim(),
      audio_url: stationForm.audio_url.trim(),
      type_station: stationForm.type_station,
      a_la_une: stationForm.a_la_une,
      origine_publication: stationForm.origine_publication,
    }
    if (stationForm.role_partie_prenante) {
      body.role_partie_prenante = stationForm.role_partie_prenante
      if (stationForm.role_partie_prenante === 'autre') body.role_partie_prenante_autre = stationForm.role_partie_prenante_autre.trim()
    }
    if (stationForm.description.trim()) body.description = stationForm.description.trim()
    if (stationForm.image_couverture_url.trim()) body.image_couverture_url = stationForm.image_couverture_url.trim()
    if (stationForm.genre.trim()) body.genre = stationForm.genre.trim()
    if (stationForm.genres_liste.length) body.genres_liste = stationForm.genres_liste
    if (stationForm.pays_id) body.pays_id = stationForm.pays_id
    if (stationForm.ville.trim()) body.ville = stationForm.ville.trim()
    // Envoyés même vides : c'est ce qui permet d'effacer un contact.
    body.contact_email = stationForm.contact_email.trim()
    body.contact_telephone = stationForm.contact_telephone.trim()
    body.contact_whatsapp = stationForm.contact_whatsapp.trim()
    body.contact_site_web = stationForm.contact_site_web.trim()
    body.contact_adresse = stationForm.contact_adresse.trim()
    await modifierStation(id, body)

    // Endpoints séparés : le serveur valide ces deux invariants à part.
    await definirThematiques('station_radio', id, thematiquesChoisies.value, true)
    await definirCouverture('station_radio', id, couvertureContinentale.value, territoiresChoisis.value, true)

    successMsg.value = 'Mis à jour avec succès'
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { saving.value = false }
}

const metaCreeParNom = computed(() => stationDetail.value?.cree_par_nom)
const metaId = computed(() => stationDetail.value?.id)
const emissionsStation = computed<AdminEmission[]>(() => emissions.value)

onMounted(async () => {
  await charger()
  const [pays, referentiels] = await Promise.all([listerPays(), listerReferentielsEdition()])
  paysDisponibles.value = pays
  thematiquesRef.value = referentiels.thematiques
  territoiresRef.value = referentiels.territoires
  await chargerFiche()
  filtresEmissions.type = 'radio'
  filtresEmissions.support_id = id
  await chargerEmissions()
})
</script>

<template>
  <div>
    <AdminPageHeader :titre="titreDetail" sous-titre="Modifier la station radio">
      <template #actions>
        <NuxtLink to="/admin/radio" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !detailCharge" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="detailCharge">
      <div class="flex items-center gap-3 mb-4">
        <span :class="['badge', etatBadge(etatCourant)]">{{ etatLabel(etatCourant) }}</span>
        <button class="btn btn-outline btn-xs" @click="ouvrirEtatModal">
          <font-awesome-icon icon="arrows-rotate" class="mr-1" /> Changer état
        </button>
      </div>

      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">x</button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-6">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Nom de la station *</span></label>
                <input v-model="stationForm.nom" type="text" class="input input-bordered" required>
              </div>
              <AdminMediaUploadField v-model="stationForm.audio_url" kind="audio" label="Audio de la station (fichier ou lien)" />
              <div class="form-control">
                <label class="label"><span class="label-text">URL de flux live (optionnel)</span></label>
                <input v-model="stationForm.stream_url" type="text" class="input input-bordered" placeholder="https://stream.example.com/live">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="stationForm.description" class="textarea textarea-bordered h-32" />
              </div>
              <OpportuniteAfriqueImageUploadField v-model="stationForm.image_couverture_url" label="Image de couverture (optionnel)" />
              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input v-model="stationForm.a_la_une" type="checkbox" class="checkbox checkbox-primary" />
                  <span class="label-text">À la une (station mise en avant sur sa page)</span>
                </label>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Publication &amp; rôle</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Origine de publication *</span></label>
                <select v-model="stationForm.origine_publication" class="select select-bordered">
                  <option v-for="o in ORIGINES_PUBLICATION_RADIO" :key="o.valeur" :value="o.valeur">
                    {{ o.libelle }} : page {{ o.page }}
                  </option>
                </select>
                <label class="label"><span class="label-text-alt">{{ aideOrigine }}</span></label>
              </div>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Rôle de la partie prenante</span></label>
                  <select v-model="stationForm.role_partie_prenante" class="select select-bordered">
                    <option value="">Non spécifié</option>
                    <option v-for="r in ROLES_PARTIE_PRENANTE_RADIO" :key="r.valeur" :value="r.valeur">{{ r.libelle }}</option>
                  </select>
                </div>
                <div v-if="stationForm.role_partie_prenante === 'autre'" class="form-control">
                  <label class="label"><span class="label-text">Préciser le rôle *</span></label>
                  <input v-model="stationForm.role_partie_prenante_autre" type="text" class="input input-bordered" required placeholder="Ex: Collectif de podcasteurs">
                </div>
              </div>
            </div>

            <AdminContactsSupportFields
              v-model:email="stationForm.contact_email"
              v-model:telephone="stationForm.contact_telephone"
              v-model:whatsapp="stationForm.contact_whatsapp"
              v-model:site-web="stationForm.contact_site_web"
              v-model:adresse="stationForm.contact_adresse"
              libelle-support="la station"
            />

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
                  <label class="label"><span class="label-text">Territoire</span></label>
                  <select v-model="stationForm.pays_id" class="select select-bordered">
                    <option value="">Aucun</option>
                    <option v-for="p in paysDisponibles" :key="p.id" :value="p.id">{{ p.nom }}</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Ville</span></label>
                  <input v-model="stationForm.ville" type="text" class="input input-bordered">
                </div>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Thématiques &amp; couverture</h3>
              <MediaSelecteurThematiques
                v-model="thematiquesChoisies"
                :options="thematiquesRef"
                :requis="exigeFiche"
              />
              <MediaSelecteurCouverture
                :continentale="couvertureContinentale"
                :territoires="territoiresChoisis"
                :options="territoiresRef"
                :requis="exigeFiche"
                @update:continentale="couvertureContinentale = $event"
                @update:territoires="territoiresChoisis = $event"
              />
              <p v-if="!exigeFiche" class="text-sm text-base-content/60">
                Facultatives sur un brouillon ; exigées pour publier la station.
              </p>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="metaCreeParNom">Créé par {{ metaCreeParNom }}</span>
                <br>ID: {{ metaId?.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Équipe éditoriale (010) : voir la note de `/admin/television/[id]` :
           un seul composant, monté membre et admin, l'autorité seule diffère. -->
      <div class="card bg-base-100 shadow-sm mt-6">
        <div class="card-body">
          <MediaGestionEquipe
            type-porteur="station_radio"
            :porteur-id="id"
            base="admin"
            titre="Équipe éditoriale de la station"
          />
        </div>
      </div>

      <!-- Émissions de cette station : listées ici, éditées sur leur écran -->
      <div class="card bg-base-100 shadow-sm mt-6">
        <div class="card-body">
          <div class="flex items-center justify-between gap-3">
            <h3 class="text-lg font-semibold">Émissions de cette station</h3>
            <NuxtLink :to="`/admin/medias/emissions?type=radio&support_id=${id}`" class="btn btn-primary btn-sm">
              <font-awesome-icon icon="film" class="mr-1" /> Gérer les émissions
            </NuxtLink>
          </div>
          <p class="text-sm text-base-content/60">
            Une émission regroupe ses <strong>épisodes</strong>. La grille vise l'émission ;
            l'épisode diffusé se déduit de la rotation.
          </p>

          <div v-if="emissionsStation.length" class="overflow-x-auto mt-2">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>Émission</th>
                  <th class="w-28 text-center">Cadence</th>
                  <th class="w-24 text-center">Épisodes</th>
                  <th class="w-28 text-center">État</th>
                  <th class="w-24" />
                </tr>
              </thead>
              <tbody>
                <tr v-for="e in emissionsStation" :key="e.id">
                  <td class="font-medium">{{ e.titre }}</td>
                  <td class="text-center text-sm">{{ libelleCadence(e.cadence) }}</td>
                  <td class="text-center">
                    {{ e.nombre_episodes }}
                    <span v-if="e.episodes_en_attente" class="badge badge-warning badge-sm ml-1">
                      {{ e.episodes_en_attente }} en attente
                    </span>
                  </td>
                  <td class="text-center">
                    <span :class="['badge badge-sm', ETATS_EMISSION[e.etat]?.badge || 'badge-info']">
                      {{ ETATS_EMISSION[e.etat]?.libelle || e.etat }}
                    </span>
                  </td>
                  <td>
                    <NuxtLink :to="`/admin/medias/emissions/${e.id}`" class="btn btn-ghost btn-xs">
                      <font-awesome-icon icon="pen" class="mr-1" /> Ouvrir
                    </NuxtLink>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <div v-else class="text-center py-8 text-base-content/50">
            <font-awesome-icon icon="microphone" class="text-3xl mb-2" />
            <p>Aucune émission rattachée à cette station pour l'instant.</p>
          </div>
        </div>
      </div>

      <!-- Modal changement d'état -->
      <div v-if="showEtatModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">Changer l'état</h3>
          <div v-if="!ficheComplete" class="alert alert-warning mb-3">
            <font-awesome-icon icon="triangle-exclamation" />
            <span>Thématique ou couverture manquante : la publication sera refusée.</span>
          </div>
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
