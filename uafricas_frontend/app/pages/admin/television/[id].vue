<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const type = (route.query.type as string) || 'chaines'

const {
  chaineDetail, programmeDetail, programmes, filtresProgrammes,
  chargerChaine, chargerProgramme, chargerProgrammes,
  modifierChaine, modifierProgramme, listerToutesChaines,
  definirVedetteGlobale, listerThemesPhares,
  loading, error,
} = useAdminTelevision()
const { listerPays } = useCentresCulturels()

const chainesDisponibles = ref<{ id: string; nom: string }[]>([])
const paysDisponibles = ref<{ id: string; nom: string }[]>([])
const themesPhares = ref<{ id: string; nom: string }[]>([])

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const chaineForm = reactive({
  nom: '', description: '', stream_url: '', image_couverture_url: '',
  categorie: 'generaliste', pays_id: '', langue: '', est_en_direct: false,
})

const programmeForm = reactive({
  nom_emission: '', description: '', image_couverture_url: '', video_url: '',
  info_animateur: '', info_producteur: '', pays_id: '', est_international: false,
  langue: '', chaine_id: '', a_la_une: false,
  theme_phare_id: '', theme_phare_autre: '',
})

// Valeur sentinelle du sélecteur : « Autre » n'est pas une catégorie en base,
// elle bascule la saisie vers le champ libre `theme_phare_autre`.
const THEME_AUTRE = '__autre__'
const themeEstAutre = computed(() => programmeForm.theme_phare_id === THEME_AUTRE)

const titreDetail = computed(() => {
  if (type === 'chaines') return chaineDetail.value?.nom || 'Chargement...'
  return programmeDetail.value?.nom_emission || 'Chargement...'
})
const sousTitreMap: Record<string, string> = { chaines: 'Modifier la chaîne TV', programmes: 'Modifier le programme télé' }

const etatCourant = computed(() => type === 'chaines' ? (chaineDetail.value?.etat || '') : (programmeDetail.value?.etat || ''))
const detailCharge = computed(() => type === 'chaines' ? !!chaineDetail.value : !!programmeDetail.value)

const etatBadge = (etat: string) => ({ brouillon: 'badge-warning', publie: 'badge-success', suspendu: 'badge-error', supprime: 'badge-ghost' } as Record<string, string>)[etat] || 'badge-info'
const etatLabel = (etat: string) => ({ brouillon: 'Brouillon', publie: 'Publie', suspendu: 'Suspendu', supprime: 'Supprime' } as Record<string, string>)[etat] || etat

const showEtatModal = ref(false)
const nouvelEtat = ref('')
const etatLoading = ref(false)
const ouvrirEtatModal = () => { nouvelEtat.value = etatCourant.value; showEtatModal.value = true }
const executerChangerEtat = async () => {
  if (nouvelEtat.value === etatCourant.value) return
  etatLoading.value = true
  try {
    if (type === 'chaines') await modifierChaine(id, { etat: nouvelEtat.value } as any)
    else await modifierProgramme(id, { etat: nouvelEtat.value } as any)
    showEtatModal.value = false
    successMsg.value = `Etat changé en "${etatLabel(nouvelEtat.value)}"`
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { etatLoading.value = false }
}

const charger = async () => {
  if (type === 'chaines') {
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
  else {
    await chargerProgramme(id)
    if (programmeDetail.value) {
      const p = programmeDetail.value
      programmeForm.nom_emission = p.nom_emission
      programmeForm.description = p.description || ''
      programmeForm.image_couverture_url = p.image_couverture_url || ''
      programmeForm.video_url = p.video_url || ''
      programmeForm.info_animateur = p.info_animateur || ''
      programmeForm.info_producteur = p.info_producteur || ''
      programmeForm.pays_id = p.pays_id || ''
      programmeForm.est_international = p.est_international || false
      programmeForm.langue = p.langue || ''
      programmeForm.chaine_id = p.chaine_id || ''
      programmeForm.a_la_une = p.a_la_une || false
      programmeForm.theme_phare_autre = p.theme_phare_autre || ''
      programmeForm.theme_phare_id = p.theme_phare_id || (p.theme_phare_autre ? THEME_AUTRE : '')
    }
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    if (type === 'chaines') {
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
    else {
      if (themeEstAutre.value && !programmeForm.theme_phare_autre.trim()) {
        erreurLocale.value = 'Précisez le thème phare choisi dans « Autre »'
        return
      }
      const body: any = {
        nom_emission: programmeForm.nom_emission.trim(),
        est_international: programmeForm.est_international,
        video_url: programmeForm.video_url.trim(),
        a_la_une: programmeForm.chaine_id ? programmeForm.a_la_une : false,
      }
      if (programmeForm.description.trim()) body.description = programmeForm.description.trim()
      if (programmeForm.image_couverture_url.trim()) body.image_couverture_url = programmeForm.image_couverture_url.trim()
      if (programmeForm.info_animateur.trim()) body.info_animateur = programmeForm.info_animateur.trim()
      if (programmeForm.info_producteur.trim()) body.info_producteur = programmeForm.info_producteur.trim()
      if (programmeForm.pays_id) body.pays_id = programmeForm.pays_id
      if (programmeForm.langue.trim()) body.langue = programmeForm.langue.trim()
      if (programmeForm.chaine_id) body.chaine_id = programmeForm.chaine_id
      if (themeEstAutre.value) body.theme_phare_autre = programmeForm.theme_phare_autre.trim()
      else if (programmeForm.theme_phare_id) body.theme_phare_id = programmeForm.theme_phare_id
      await modifierProgramme(id, body)
    }
    successMsg.value = 'Mis à jour avec succès'
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { saving.value = false }
}

// ── Vedette générale de la page Télé ────────────────────────
const estVedetteGlobale = computed(() => !!programmeDetail.value?.a_la_une_globale)
const peutDevenirVedette = computed(() => etatCourant.value === 'publie' && !estVedetteGlobale.value)

const showVedetteModal = ref(false)
const vedetteLoading = ref(false)
const executerVedetteGlobale = async () => {
  vedetteLoading.value = true
  erreurLocale.value = null
  try {
    await definirVedetteGlobale(id)
    showVedetteModal.value = false
    successMsg.value = 'Ce programme est désormais la vedette générale de la page Télé'
    setTimeout(() => { successMsg.value = null }, 4000)
    await charger()
  }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur'; showVedetteModal.value = false }
  finally { vedetteLoading.value = false }
}

const metaCreeParNom = computed(() => type === 'chaines' ? chaineDetail.value?.cree_par_nom : programmeDetail.value?.cree_par_nom)
const metaId = computed(() => type === 'chaines' ? chaineDetail.value?.id : programmeDetail.value?.id)

onMounted(async () => {
  await charger()
  paysDisponibles.value = await listerPays()
  if (type === 'programmes') {
    const [chaines, themes] = await Promise.all([listerToutesChaines(), listerThemesPhares()])
    chainesDisponibles.value = chaines
    themesPhares.value = themes
  }
  else {
    // Charger les vidéos (programmes) rattachées à cette chaîne
    filtresProgrammes.chaine_id = id
    await chargerProgrammes()
  }
})
</script>

<template>
  <div>
    <AdminPageHeader :titre="titreDetail" :sous-titre="sousTitreMap[type] || ''">
      <template #actions>
        <NuxtLink to="/admin/television" class="btn btn-ghost btn-sm">
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

      <!-- Chaîne TV -->
      <div v-if="type === 'chaines'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-6">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Nom de la chaîne *</span></label>
                <input v-model="chaineForm.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL de flux live (optionnel)</span></label>
                <input v-model="chaineForm.stream_url" type="text" class="input input-bordered" placeholder="https://stream.example.com/live">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="chaineForm.description" class="textarea textarea-bordered h-32" />
              </div>
              <OpportuniteAfriqueImageUploadField v-model="chaineForm.image_couverture_url" label="Image de couverture (optionnel)" />
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Classification</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Catégorie</span></label>
                  <select v-model="chaineForm.categorie" class="select select-bordered">
                    <option value="generaliste">Généraliste</option>
                    <option value="info">Info</option>
                    <option value="sport">Sport</option>
                    <option value="culture">Culture</option>
                    <option value="divertissement">Divertissement</option>
                    <option value="education">Éducation</option>
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
                <label class="label"><span class="label-text">Territoire</span></label>
                <select v-model="chaineForm.pays_id" class="select select-bordered">
                  <option value="">— Aucun —</option>
                  <option v-for="p in paysDisponibles" :key="p.id" :value="p.id">{{ p.nom }}</option>
                </select>
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

      <!-- Programme télé -->
      <div v-else class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-6">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Nom du programme *</span></label>
                <input v-model="programmeForm.nom_emission" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="programmeForm.description" class="textarea textarea-bordered h-32" />
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Médias</h3>
              <OpportuniteAfriqueImageUploadField v-model="programmeForm.image_couverture_url" label="Image de couverture (optionnel)" />
              <AdminMediaUploadField v-model="programmeForm.video_url" kind="video" label="Vidéo du programme (fichier ou lien)" />
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Télé & écran principal</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Télé (chaîne) de rattachement</span></label>
                <select v-model="programmeForm.chaine_id" class="select select-bordered">
                  <option value="">Aucune (programme libre)</option>
                  <option v-for="ch in chainesDisponibles" :key="ch.id" :value="ch.id">{{ ch.nom }}</option>
                </select>
                <label class="label"><span class="label-text-alt">Regroupe la vidéo sous cette télé sur la page publique /tele.</span></label>
              </div>
              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input v-model="programmeForm.a_la_une" type="checkbox" class="checkbox checkbox-primary" :disabled="!programmeForm.chaine_id" />
                  <span class="label-text">Programme à la une (joue en boucle sur l'écran principal de la télé)</span>
                </label>
                <label v-if="!programmeForm.chaine_id" class="label"><span class="label-text-alt text-warning">Sélectionnez d'abord une télé pour la mettre à la une.</span></label>
                <label class="label"><span class="label-text-alt">Portée : cette télé uniquement. À ne pas confondre avec la vedette générale, plus bas.</span></label>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Thème phare</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Thème phare du programme</span></label>
                <select v-model="programmeForm.theme_phare_id" class="select select-bordered">
                  <option value="">— Aucun —</option>
                  <option v-for="t in themesPhares" :key="t.id" :value="t.id">{{ t.nom }}</option>
                  <option :value="THEME_AUTRE">Autre (à préciser)</option>
                </select>
              </div>
              <div v-if="themeEstAutre" class="form-control">
                <label class="label"><span class="label-text">Préciser le thème *</span></label>
                <input v-model="programmeForm.theme_phare_autre" type="text" class="input input-bordered" maxlength="200" required placeholder="Ex: Diplomatie culturelle">
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Équipe & production</h3>
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
                  <label class="label"><span class="label-text">Territoire</span></label>
                  <select v-model="programmeForm.pays_id" class="select select-bordered">
                    <option value="">— Aucun —</option>
                    <option v-for="p in paysDisponibles" :key="p.id" :value="p.id">{{ p.nom }}</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Langue</span></label>
                  <input v-model="programmeForm.langue" type="text" class="input input-bordered">
                </div>
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

      <!-- Vedette générale : action immédiate hors formulaire (endpoint transactionnel dédié) -->
      <div v-if="type === 'programmes'" class="card bg-base-100 shadow-sm mt-6 border-2 border-warning">
        <div class="card-body">
          <div class="flex flex-wrap items-center justify-between gap-3">
            <h3 class="text-lg font-semibold flex items-center gap-2">
              <font-awesome-icon icon="star" class="text-warning" />
              Vedette générale de la page Télé
            </h3>
            <span v-if="estVedetteGlobale" class="badge badge-warning gap-1">
              <font-awesome-icon icon="star" /> Vedette actuelle
            </span>
          </div>

          <p class="text-sm text-base-content/70">
            Portée <strong>globale</strong> : un seul programme, toutes télés confondues, occupe la mise en avant
            en tête de <code>/tele</code>. À distinguer du « programme à la une » ci-dessus, qui ne vaut que pour
            sa propre télé.
          </p>

          <div v-if="estVedetteGlobale" class="alert alert-success mt-2">
            <font-awesome-icon icon="circle-check" />
            <span>Ce programme est actuellement la vedette générale. Pour la libérer, désignez un autre programme.</span>
          </div>
          <div v-else-if="etatCourant !== 'publie'" class="alert alert-warning mt-2">
            <font-awesome-icon icon="circle-exclamation" />
            <span>Seul un programme <strong>publié</strong> peut devenir vedette générale. Changez son état d'abord.</span>
          </div>
          <div v-else class="alert alert-warning mt-2">
            <font-awesome-icon icon="triangle-exclamation" />
            <span>Désigner ce programme <strong>remplacera immédiatement la vedette actuelle</strong> de toute la page Télé.</span>
          </div>

          <div class="flex justify-end pt-2">
            <button type="button" class="btn btn-warning" :disabled="!peutDevenirVedette" @click="showVedetteModal = true">
              <font-awesome-icon icon="star" class="mr-1" /> Désigner comme vedette générale
            </button>
          </div>
        </div>
      </div>

      <!-- Vidéos (programmes) rattachées à cette chaîne -->
      <div v-if="type === 'chaines'" class="card bg-base-100 shadow-sm mt-6">
        <div class="card-body">
          <div class="flex items-center justify-between gap-3">
            <h3 class="text-lg font-semibold">Vidéos de cette chaîne</h3>
            <NuxtLink :to="`/admin/television/create?type=programmes&chaine=${id}`" class="btn btn-primary btn-sm">
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter une vidéo
            </NuxtLink>
          </div>
          <p class="text-sm text-base-content/60">
            Une vidéo de la chaîne = un <strong>programme télé</strong> rattaché. Le programme marqué
            « à la une » <font-awesome-icon icon="star" class="text-warning" /> joue en boucle sur l'écran principal de la page <code>/tele</code>.
          </p>

          <div v-if="programmes.length" class="overflow-x-auto mt-2">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>Programme</th>
                  <th class="w-28 text-center">État</th>
                  <th class="w-24 text-center">À la une</th>
                  <th class="w-24"></th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="p in programmes" :key="p.id">
                  <td class="font-medium">{{ p.nom_emission }}</td>
                  <td class="text-center"><span :class="['badge badge-sm', etatBadge(p.etat)]">{{ etatLabel(p.etat) }}</span></td>
                  <td class="text-center"><font-awesome-icon v-if="p.a_la_une" icon="star" class="text-warning" /></td>
                  <td>
                    <NuxtLink :to="`/admin/television/${p.id}?type=programmes`" class="btn btn-ghost btn-xs">
                      <font-awesome-icon icon="pen" class="mr-1" /> Modifier
                    </NuxtLink>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <div v-else class="text-center py-8 text-base-content/50">
            <font-awesome-icon icon="video" class="text-3xl mb-2" />
            <p>Aucune vidéo rattachée à cette chaîne pour l'instant.</p>
            <p class="text-xs mt-1">Cliquez sur « Ajouter une vidéo » pour créer un programme rattaché à cette chaîne.</p>
          </div>
        </div>
      </div>

      <!-- Modal confirmation vedette générale -->
      <div v-if="showVedetteModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-2 flex items-center gap-2">
            <font-awesome-icon icon="star" class="text-warning" />
            Désigner la vedette générale
          </h3>
          <p class="text-sm text-base-content/80">
            « {{ programmeDetail?.nom_emission }} » deviendra l'unique programme mis en avant en tête de la page Télé.
          </p>
          <div class="alert alert-warning mt-4">
            <font-awesome-icon icon="triangle-exclamation" />
            <span>La vedette actuelle sera automatiquement rétrogradée. Cette bascule est immédiate et visible du public.</span>
          </div>
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showVedetteModal = false">Annuler</button>
            <button class="btn btn-warning" :class="{ loading: vedetteLoading }" :disabled="vedetteLoading" @click="executerVedetteGlobale">
              Confirmer
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showVedetteModal = false" />
      </div>

      <!-- Modal changement d'état -->
      <div v-if="showEtatModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">Changer l'état</h3>
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
