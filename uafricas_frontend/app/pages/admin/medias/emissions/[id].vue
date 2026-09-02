<script setup lang="ts">
/**
 * Fiche d'un **programme** en back-office, et gestion de ses épisodes
 * (feature 009, US1, FR-044 à FR-047).
 *
 * La vedette générale de `/medias/tele` se désigne ici, sur un **épisode** :
 * c'est un épisode qui occupe la tête de page, pas une série. Elle a suivi les
 * contenus lors du recadrage 09q : elle vivait auparavant sur `programme_tele`.
 */
import { porteurProgramme } from '~/composables/useMediaEquipe'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  emissionDetail, episodes,
  chargerEmission, modifierEmission, changerEtatEmission,
  chargerEpisodes, definirVedetteGlobale,
  listerThemesPhares, CADENCES, libelleCadence, ETATS_EMISSION, ETATS_EPISODE,
  loading, error,
} = useAdminMediaEmissions()

const themesPhares = ref<{ id: string; nom: string }[]>([])
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const formulaire = reactive({
  titre: '',
  description: '',
  cadence: 'ponctuelle',
  image_couverture_url: '',
  info_animateur: '',
  info_producteur: '',
  langue: '',
  theme_phare_id: '',
  theme_phare_autre: '',
  categorie_radio: '',
})

/** « Autre » n'est pas une catégorie en base : la sentinelle bascule la saisie
 * vers le champ libre. */
const THEME_AUTRE = '__autre__'
const themeEstAutre = computed(() => formulaire.theme_phare_id === THEME_AUTRE)

const estRadio = computed(() => emissionDetail.value?.type_support === 'station_radio')
const etatCourant = computed(() => emissionDetail.value?.etat || '')

const lienSupport = computed(() => {
  if (!emissionDetail.value) return null
  const base = estRadio.value ? '/admin/radio' : '/admin/television'
  return `${base}/${emissionDetail.value.support_id}`
})

const charger = async () => {
  await chargerEmission(id)
  const e = emissionDetail.value
  if (!e) return
  formulaire.titre = e.titre
  formulaire.description = e.description || ''
  formulaire.cadence = e.cadence || 'ponctuelle'
  formulaire.image_couverture_url = e.image_couverture_url || ''
  formulaire.info_animateur = e.info_animateur || ''
  formulaire.info_producteur = e.info_producteur || ''
  formulaire.langue = e.langue || ''
  formulaire.categorie_radio = e.categorie_radio || ''
  formulaire.theme_phare_autre = e.theme_phare_autre || ''
  formulaire.theme_phare_id = e.theme_phare_id || (e.theme_phare_autre ? THEME_AUTRE : '')
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    if (!formulaire.titre.trim()) { erreurLocale.value = 'Le titre est obligatoire.'; return }
    if (themeEstAutre.value && !formulaire.theme_phare_autre.trim()) {
      erreurLocale.value = 'Précisez le thème phare choisi dans « Autre ».'
      return
    }
    const body: any = {
      titre: formulaire.titre.trim(),
      description: formulaire.description.trim(),
      cadence: formulaire.cadence,
      image_couverture_url: formulaire.image_couverture_url.trim() || null,
      info_animateur: formulaire.info_animateur.trim() || null,
      info_producteur: formulaire.info_producteur.trim() || null,
      langue: formulaire.langue.trim() || null,
    }
    if (estRadio.value && formulaire.categorie_radio) body.categorie_radio = formulaire.categorie_radio
    if (themeEstAutre.value) body.theme_phare_autre = formulaire.theme_phare_autre.trim()
    else if (formulaire.theme_phare_id) body.theme_phare_id = formulaire.theme_phare_id

    await modifierEmission(id, body)
    successMsg.value = 'Programme mis à jour'
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { saving.value = false }
}

// ── État du programme ─────────────────────────────────────────
const showEtatModal = ref(false)
const nouvelEtat = ref('')
const etatLoading = ref(false)

const ouvrirEtatModal = () => { nouvelEtat.value = etatCourant.value; showEtatModal.value = true }

const executerChangerEtat = async () => {
  if (nouvelEtat.value === etatCourant.value) return
  etatLoading.value = true
  try {
    await changerEtatEmission(id, nouvelEtat.value)
    showEtatModal.value = false
    successMsg.value = `État changé en « ${ETATS_EMISSION[nouvelEtat.value]?.libelle || nouvelEtat.value} »`
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { etatLoading.value = false }
}

// ── Vedette générale de /medias/tele ──────────────────────────
const episodeVedette = computed(() => episodes.value.find(e => e.a_la_une_globale) || null)
const cibleVedette = ref<{ id: string; titre: string } | null>(null)
const vedetteLoading = ref(false)

const executerVedetteGlobale = async () => {
  if (!cibleVedette.value) return
  vedetteLoading.value = true
  erreurLocale.value = null
  try {
    await definirVedetteGlobale(cibleVedette.value.id)
    cibleVedette.value = null
    successMsg.value = 'Cet épisode est désormais la vedette générale de la page Télé'
    setTimeout(() => { successMsg.value = null }, 4000)
    await chargerEpisodes(id)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
    cibleVedette.value = null
  }
  finally { vedetteLoading.value = false }
}

const rafraichirEpisodes = async () => {
  await chargerEpisodes(id)
  await charger()
}

onMounted(async () => {
  await charger()
  themesPhares.value = await listerThemesPhares()
  await chargerEpisodes(id)
})
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="emissionDetail?.titre || 'Chargement...'"
      :sous-titre="estRadio ? 'Émission radio' : 'Programme télé'"
    >
      <template #actions>
        <NuxtLink :to="`/admin/medias/emissions?type=${estRadio ? 'radio' : 'tele'}`" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !emissionDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="emissionDetail">
      <div class="flex flex-wrap items-center gap-3 mb-4">
        <span class="badge" :class="ETATS_EMISSION[etatCourant]?.badge || 'badge-info'">
          {{ ETATS_EMISSION[etatCourant]?.libelle || etatCourant }}
        </span>
        <button class="btn btn-outline btn-xs" @click="ouvrirEtatModal">
          <font-awesome-icon icon="arrows-rotate" class="mr-1" /> Changer état
        </button>
        <NuxtLink v-if="lienSupport" :to="lienSupport" class="btn btn-ghost btn-xs">
          <font-awesome-icon icon="link" class="mr-1" /> {{ emissionDetail.support?.nom }}
        </NuxtLink>
        <span class="text-sm opacity-70">
          {{ emissionDetail.nombre_episodes }} épisode(s) publié(s) · {{ libelleCadence(emissionDetail.cadence) }}
        </span>
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
          <form class="space-y-6" @submit.prevent="sauvegarder">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Identité du programme</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Titre *</span></label>
                <input v-model="formulaire.titre" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="formulaire.description" class="textarea textarea-bordered h-28" />
              </div>
              <OpportuniteAfriqueImageUploadField v-model="formulaire.image_couverture_url" label="Image de couverture (optionnel)" />
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Cadence</h3>
              <div class="form-control">
                <select v-model="formulaire.cadence" class="select select-bordered">
                  <option v-for="c in CADENCES" :key="c.valeur" :value="c.valeur">{{ c.libelle }}</option>
                </select>
                <label class="label">
                  <span class="label-text-alt">
                    {{ CADENCES.find(c => c.valeur === formulaire.cadence)?.aide }}
                  </span>
                </label>
              </div>
              <p class="text-sm text-base-content/60">
                La cadence n'agit pas sur la rotation en grille, celle-ci suit la récurrence du créneau.
                Elle sert uniquement à alerter le détenteur d'une échéance sans épisode.
              </p>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Thème phare</h3>
              <div class="form-control">
                <select v-model="formulaire.theme_phare_id" class="select select-bordered">
                  <option value="">Aucun</option>
                  <option v-for="t in themesPhares" :key="t.id" :value="t.id">{{ t.nom }}</option>
                  <option :value="THEME_AUTRE">Autre (à préciser)</option>
                </select>
              </div>
              <div v-if="themeEstAutre" class="form-control">
                <label class="label"><span class="label-text">Préciser le thème *</span></label>
                <input v-model="formulaire.theme_phare_autre" type="text" class="input input-bordered" maxlength="200">
              </div>
            </div>

            <div v-if="estRadio" class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Catégorie radio</h3>
              <div class="form-control">
                <select v-model="formulaire.categorie_radio" class="select select-bordered">
                  <option value="">Aucune</option>
                  <option value="information">Information</option>
                  <option value="divertissement">Divertissement</option>
                  <option value="musique">Musique</option>
                  <option value="culture">Culture</option>
                  <option value="sport">Sport</option>
                  <option value="education">Éducation</option>
                  <option value="debat">Débat</option>
                  <option value="religieux">Religieux</option>
                </select>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Langue</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Langue</span></label>
                  <input v-model="formulaire.langue" type="text" class="input input-bordered" placeholder="Ex: Français">
                </div>
              </div>
            </div>

            <!-- Champs HÉRITÉS (010, FR-034). Ils ne s'affichent plus au public :
                 l'équipe éditoriale est désormais la seule source sur les
                 personnes. Ils restent lisibles ICI : les masquer priverait le
                 gestionnaire de la seule trace de ce qu'il doit reporter. -->
            <div v-if="formulaire.info_animateur || formulaire.info_producteur" class="space-y-2">
              <h3 class="text-lg font-semibold border-b pb-2">
                Champs hérités : reporter dans l'équipe
              </h3>
              <p class="text-sm text-base-content/60">
                Saisis avant la refonte des équipes éditoriales. Ils ne sont plus publiés :
                reportez-les dans le bloc « Équipe éditoriale » ci-dessous, puis videz-les.
              </p>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Animation (hérité)</span></label>
                  <input
                    v-model="formulaire.info_animateur"
                    type="text"
                    class="input input-bordered input-sm"
                    readonly
                  >
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Production (hérité)</span></label>
                  <input
                    v-model="formulaire.info_producteur"
                    type="text"
                    class="input input-bordered input-sm"
                    readonly
                  >
                </div>
              </div>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                ID: {{ emissionDetail.id.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Équipe éditoriale DU PROGRAMME (010). Le porteur se déduit de la
           famille du support : un programme n'a pas de discriminant propre. -->
      <div class="card bg-base-100 shadow-sm mt-6">
        <div class="card-body">
          <MediaGestionEquipe
            :type-porteur="porteurProgramme(emissionDetail.type_support)"
            :porteur-id="id"
            base="admin"
            titre="Équipe éditoriale du programme"
          />
        </div>
      </div>

      <!-- Épisodes : composant partagé avec l'espace détenteur -->
      <div class="card bg-base-100 shadow-sm mt-6">
        <div class="card-body">
          <MediaGestionEpisodes
            admin
            :sombre="false"
            :emission-id="id"
            :emission-titre="emissionDetail.titre"
            :type-support="emissionDetail.type_support"
            @change="rafraichirEpisodes"
          />
        </div>
      </div>

      <!-- Vedette générale : elle porte sur un ÉPISODE, pas sur la série -->
      <div v-if="!estRadio" class="card bg-base-100 shadow-sm mt-6 border-2 border-warning">
        <div class="card-body">
          <h3 class="text-lg font-semibold flex items-center gap-2">
            <font-awesome-icon icon="star" class="text-warning" />
            Vedette générale de la page Télé
          </h3>
          <p class="text-sm text-base-content/70">
            Un seul <strong>épisode</strong>, toutes chaînes confondues, occupe la mise en avant en tête de
            <code>/medias/tele</code>. À distinguer de « à la une », qui ne vaut que pour son propre programme.
          </p>

          <div v-if="episodeVedette" class="alert alert-success mt-2">
            <font-awesome-icon icon="circle-check" />
            <span>« {{ episodeVedette.titre }} » est actuellement la vedette générale.</span>
          </div>

          <div class="overflow-x-auto mt-2">
            <table class="table table-sm">
              <tbody>
                <tr v-for="ep in episodes.filter(e => e.etat === 'publie')" :key="ep.id">
                  <td class="font-medium">{{ ep.titre }}</td>
                  <td class="w-32 text-center">
                    <span class="badge badge-sm" :class="ETATS_EPISODE[ep.etat]?.badge || 'badge-info'">
                      {{ ETATS_EPISODE[ep.etat]?.libelle || ep.etat }}
                    </span>
                  </td>
                  <td class="w-56 text-right">
                    <span v-if="ep.a_la_une_globale" class="badge badge-warning gap-1">
                      <font-awesome-icon icon="star" /> Vedette
                    </span>
                    <button
                      v-else
                      class="btn btn-warning btn-xs"
                      @click="cibleVedette = { id: ep.id, titre: ep.titre }"
                    >
                      <font-awesome-icon icon="star" class="mr-1" /> Désigner
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
            <p v-if="!episodes.some(e => e.etat === 'publie')" class="text-sm text-base-content/50 py-4">
              Aucun épisode publié : la vedette générale ne peut désigner qu'un épisode publié.
            </p>
          </div>
        </div>
      </div>

      <!-- Confirmation vedette -->
      <div v-if="cibleVedette" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-2 flex items-center gap-2">
            <font-awesome-icon icon="star" class="text-warning" /> Désigner la vedette générale
          </h3>
          <p class="text-sm text-base-content/80">
            « {{ cibleVedette.titre }} » deviendra l'unique épisode mis en avant en tête de la page Télé.
          </p>
          <div class="alert alert-warning mt-4">
            <font-awesome-icon icon="triangle-exclamation" />
            <span>La vedette actuelle sera rétrogradée dans la même transaction. La bascule est immédiate et publique.</span>
          </div>
          <div class="modal-action">
            <button class="btn btn-ghost" @click="cibleVedette = null">Annuler</button>
            <button class="btn btn-warning" :class="{ loading: vedetteLoading }" :disabled="vedetteLoading" @click="executerVedetteGlobale">
              Confirmer
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="cibleVedette = null" />
      </div>

      <!-- Changement d'état -->
      <div v-if="showEtatModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">Changer l'état du programme</h3>
          <p class="text-sm text-base-content/70 mb-3">
            Suspendre retire ses épisodes de l'espace public sans les supprimer.
          </p>
          <div class="form-control">
            <select v-model="nouvelEtat" class="select select-bordered">
              <option value="brouillon">Brouillon</option>
              <option value="en_attente">En attente</option>
              <option value="publie">Publié</option>
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
