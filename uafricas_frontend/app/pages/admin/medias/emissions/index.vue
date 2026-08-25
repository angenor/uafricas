<script setup lang="ts">
/**
 * Programmes (émissions) télé et radio, back-office (feature 009, US1, FR-046).
 *
 * Une seule liste pour les deux familles : les tables diffèrent, les routes
 * non. Le filtre « Famille » n'est qu'un paramètre de la même requête.
 */
import type { AdminEmission, TypeSupportAdmin } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })
useHead({ title: 'Programmes médias, Administration' })

const route = useRoute()

const {
  emissions, filtres, pagination, loading, error,
  chargerEmissions, creerEmission, supprimerEmission, changerEtatEmission,
  listerSupports, CADENCES, libelleCadence, ETATS_EMISSION,
  allerPage, reinitialiserPagination,
} = useAdminMediaEmissions()

const supportsTele = ref<{ id: string; nom: string }[]>([])
const supportsRadio = ref<{ id: string; nom: string }[]>([])

/** Les supports proposés suivent la famille filtrée : mélanger chaînes et
 * stations dans un même sélecteur rendrait le choix ambigu. */
const supportsProposes = computed(() => {
  if (filtres.type === 'tele') return supportsTele.value
  if (filtres.type === 'radio') return supportsRadio.value
  return [...supportsTele.value, ...supportsRadio.value]
})

const suppressionCible = ref<AdminEmission | null>(null)
const messageSuppression = ref<string | null>(null)

// ── Création ──────────────────────────────────────────────
// Le strict nécessaire ici : support, titre, cadence , le reste sur l'écran
// de détail. Un programme naît sans épisode et sans fichier : c'est la
// différence de fond avec l'ancien « programme », qui était le média lui-même.
const showCreation = ref(false)
const creationErreur = ref<string | null>(null)
const creation = reactive({
  type_support: 'chaine_tv' as TypeSupportAdmin,
  support_id: '',
  titre: '',
  cadence: 'ponctuelle',
  description: '',
})

const supportsCreation = computed(() =>
  creation.type_support === 'chaine_tv' ? supportsTele.value : supportsRadio.value,
)

watch(() => creation.type_support, () => { creation.support_id = '' })

const ouvrirCreation = () => {
  creationErreur.value = null
  creation.type_support = filtres.type === 'radio' ? 'station_radio' : 'chaine_tv'
  creation.support_id = filtres.support_id || ''
  creation.titre = ''
  creation.cadence = 'ponctuelle'
  creation.description = ''
  showCreation.value = true
}

const executerCreation = async () => {
  creationErreur.value = null
  if (!creation.support_id) { creationErreur.value = 'Choisissez le support de rattachement.'; return }
  if (!creation.titre.trim()) { creationErreur.value = 'Le titre du programme est obligatoire.'; return }
  try {
    const cree = await creerEmission({
      type_support: creation.type_support,
      support_id: creation.support_id,
      titre: creation.titre.trim(),
      cadence: creation.cadence as any,
      description: creation.description.trim(),
    })
    showCreation.value = false
    if (cree?.id) await navigateTo(`/admin/medias/emissions/${cree.id}`)
  }
  catch (e: any) {
    creationErreur.value = e?.data?.error || e?.message || 'Erreur lors de la création'
  }
}

const executerSuppression = async () => {
  if (!suppressionCible.value) return
  messageSuppression.value = null
  try {
    await supprimerEmission(suppressionCible.value.id)
    suppressionCible.value = null
    await chargerEmissions()
  }
  catch (e: any) {
    // 409 attendu : un programme portant des épisodes publiés n'est pas
    // supprimable : le message serveur dit combien.
    messageSuppression.value = e?.data?.error || e?.message || 'Suppression impossible'
  }
}

const basculerEtat = async (emission: AdminEmission) => {
  const cible = emission.etat === 'publie' ? 'suspendu' : 'publie'
  await changerEtatEmission(emission.id, cible)
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.type = ''
  filtres.support_id = ''
  filtres.etat = ''
  filtres.cadence = ''
  reinitialiserPagination()
  chargerEmissions()
}

const appliquer = () => {
  reinitialiserPagination()
  chargerEmissions()
}

// Un changement de famille invalide le support choisi dans l'autre.
watch(() => filtres.type, () => {
  if (filtres.support_id && !supportsProposes.value.some(s => s.id === filtres.support_id)) {
    filtres.support_id = ''
  }
})

watch(() => pagination.page, () => chargerEmissions())

onMounted(async () => {
  // Pré-filtrage depuis « Gérer les programmes » d'une chaîne ou d'une station.
  if (route.query.type === 'tele' || route.query.type === 'radio') filtres.type = route.query.type as string
  if (route.query.support_id) filtres.support_id = route.query.support_id as string

  const [tele, radio] = await Promise.all([
    listerSupports('chaine_tv' as TypeSupportAdmin),
    listerSupports('station_radio' as TypeSupportAdmin),
  ])
  supportsTele.value = tele
  supportsRadio.value = radio
  await chargerEmissions()
})

const dateFormatee = (iso: string | null) =>
  iso ? new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' }) : '-'
</script>

<template>
  <div>
    <AdminPageHeader titre="Programmes" sous-titre="Programmes télé et émissions radio, et leurs épisodes">
      <template #actions>
        <NuxtLink to="/admin/medias/moderation-episodes" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="inbox" class="mr-1" /> Épisodes à valider
        </NuxtLink>
        <button class="btn btn-primary btn-sm" @click="ouvrirCreation">
          <font-awesome-icon icon="plus" class="mr-1" /> Créer
        </button>
      </template>
    </AdminPageHeader>

    <div class="flex flex-wrap gap-3 mb-6">
      <input
        v-model="filtres.recherche"
        type="search"
        placeholder="Titre du programme ou du support…"
        class="input input-bordered input-sm w-64"
        @keyup.enter="appliquer"
      >
      <select v-model="filtres.type" class="select select-bordered select-sm" @change="appliquer">
        <option value="">Les deux familles</option>
        <option value="tele">Télévision</option>
        <option value="radio">Radio</option>
      </select>
      <select v-model="filtres.support_id" class="select select-bordered select-sm" @change="appliquer">
        <option value="">Tous les supports</option>
        <option v-for="s in supportsProposes" :key="s.id" :value="s.id">{{ s.nom }}</option>
      </select>
      <select v-model="filtres.etat" class="select select-bordered select-sm" @change="appliquer">
        <option value="">Tous les états</option>
        <option v-for="(def, etat) in ETATS_EMISSION" :key="etat" :value="etat">{{ def.libelle }}</option>
      </select>
      <select v-model="filtres.cadence" class="select select-bordered select-sm" @change="appliquer">
        <option value="">Toutes les cadences</option>
        <option v-for="c in CADENCES" :key="c.valeur" :value="c.valeur">{{ c.libelle }}</option>
      </select>
      <button class="btn btn-ghost btn-sm" @click="reinitialiser">Réinitialiser</button>
      <span class="text-sm opacity-70 self-center ml-auto">{{ pagination.total }} programme(s)</span>
    </div>

    <div v-if="error" class="alert alert-error mb-4">
      <span>{{ error }}</span>
    </div>
    <div v-if="messageSuppression" class="alert alert-warning mb-4">
      <font-awesome-icon icon="triangle-exclamation" />
      <span>{{ messageSuppression }}</span>
      <button class="btn btn-ghost btn-xs" @click="messageSuppression = null">x</button>
    </div>

    <div v-if="loading" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="!emissions.length" class="text-center py-16 opacity-60">
      Aucun programme ne correspond à ces filtres.
    </div>

    <div v-else class="overflow-x-auto">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>Programme</th>
            <th>Support</th>
            <th class="w-28 text-center">Cadence</th>
            <th class="w-32 text-center">Épisodes</th>
            <th class="w-28 text-center">Dernier</th>
            <th class="w-28 text-center">État</th>
            <th class="w-32" />
          </tr>
        </thead>
        <tbody>
          <tr v-for="emission in emissions" :key="emission.id">
            <td class="font-medium">
              {{ emission.titre }}
              <span v-if="emission.theme_phare" class="badge badge-ghost badge-sm ml-1">
                {{ emission.theme_phare.nom }}
              </span>
            </td>
            <td>
              <span>{{ emission.support?.nom || '-' }}</span>
              <span class="badge badge-sm ml-1" :class="emission.type_support === 'chaine_tv' ? 'badge-info' : 'badge-accent'">
                {{ emission.type_support === 'chaine_tv' ? 'Télé' : 'Radio' }}
              </span>
            </td>
            <td class="text-center text-sm">{{ libelleCadence(emission.cadence) }}</td>
            <td class="text-center">
              {{ emission.nombre_episodes }}
              <span v-if="emission.episodes_en_attente" class="badge badge-warning badge-sm ml-1">
                {{ emission.episodes_en_attente }} en attente
              </span>
              <span v-if="emission.episodes_rejetes" class="badge badge-error badge-sm ml-1">
                {{ emission.episodes_rejetes }} rejeté(s)
              </span>
            </td>
            <td class="text-center text-sm whitespace-nowrap">{{ dateFormatee(emission.dernier_episode_at) }}</td>
            <td class="text-center">
              <span class="badge badge-sm" :class="ETATS_EMISSION[emission.etat]?.badge || 'badge-info'">
                {{ ETATS_EMISSION[emission.etat]?.libelle || emission.etat }}
              </span>
            </td>
            <td>
              <div class="flex gap-1 justify-end">
                <NuxtLink :to="`/admin/medias/emissions/${emission.id}`" class="btn btn-ghost btn-xs" title="Ouvrir">
                  <font-awesome-icon icon="pen-to-square" />
                </NuxtLink>
                <button
                  class="btn btn-ghost btn-xs"
                  :title="emission.etat === 'publie' ? 'Suspendre' : 'Publier'"
                  @click="basculerEtat(emission)"
                >
                  <font-awesome-icon :icon="emission.etat === 'publie' ? 'eye-slash' : 'eye'" />
                </button>
                <button class="btn btn-ghost btn-xs text-error" title="Supprimer" @click="suppressionCible = emission">
                  <font-awesome-icon icon="trash" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="pagination.totalPages > 1" class="join mt-6 flex justify-center">
      <button class="join-item btn btn-sm" :disabled="pagination.page <= 1" @click="allerPage(pagination.page - 1)">«</button>
      <span class="join-item btn btn-sm btn-disabled">{{ pagination.page }} / {{ pagination.totalPages }}</span>
      <button class="join-item btn btn-sm" :disabled="pagination.page >= pagination.totalPages" @click="allerPage(pagination.page + 1)">»</button>
    </div>

    <!-- Création : l'essentiel, puis renvoi vers l'écran de détail -->
    <div v-if="showCreation" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">Nouveau programme</h3>

        <div v-if="creationErreur" class="alert alert-error mb-3">
          <font-awesome-icon icon="circle-exclamation" />
          <span>{{ creationErreur }}</span>
        </div>

        <div class="form-control mb-3">
          <label class="label"><span class="label-text">Famille *</span></label>
          <select v-model="creation.type_support" class="select select-bordered">
            <option value="chaine_tv">Télévision</option>
            <option value="station_radio">Radio</option>
          </select>
        </div>

        <div class="form-control mb-3">
          <label class="label"><span class="label-text">Support de rattachement *</span></label>
          <select v-model="creation.support_id" class="select select-bordered">
            <option value="">Choisir</option>
            <option v-for="s in supportsCreation" :key="s.id" :value="s.id">{{ s.nom }}</option>
          </select>
        </div>

        <div class="form-control mb-3">
          <label class="label"><span class="label-text">Titre du programme *</span></label>
          <input v-model="creation.titre" type="text" class="input input-bordered" placeholder="Ex: Le Grand Débat">
        </div>

        <div class="form-control mb-3">
          <label class="label"><span class="label-text">Cadence</span></label>
          <select v-model="creation.cadence" class="select select-bordered">
            <option v-for="c in CADENCES" :key="c.valeur" :value="c.valeur">{{ c.libelle }}</option>
          </select>
          <label class="label">
            <span class="label-text-alt">
              {{ CADENCES.find(c => c.valeur === creation.cadence)?.aide }}
            </span>
          </label>
        </div>

        <div class="form-control">
          <label class="label"><span class="label-text">Description</span></label>
          <textarea v-model="creation.description" class="textarea textarea-bordered h-24" />
        </div>

        <p class="text-sm text-base-content/60 mt-3">
          Le programme naît en brouillon, sans épisode ni fichier. Ses épisodes s'ajoutent depuis son écran.
        </p>

        <div class="modal-action">
          <button class="btn btn-ghost" @click="showCreation = false">Annuler</button>
          <button class="btn btn-primary" @click="executerCreation">Créer</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showCreation = false" />
    </div>

    <div v-if="suppressionCible" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Supprimer « {{ suppressionCible.titre }} » ?</h3>
        <p class="py-4 text-sm">
          Un programme portant des épisodes publiés ne peut pas être supprimé : suspendez-le plutôt,
          ses épisodes quitteront l'espace public sans disparaître.
        </p>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="suppressionCible = null">Annuler</button>
          <button class="btn btn-error" @click="executerSuppression">Supprimer</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="suppressionCible = null" />
    </div>
  </div>
</template>
