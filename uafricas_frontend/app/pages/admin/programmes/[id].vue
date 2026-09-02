<script setup lang="ts">
import type { TableColumn } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const { programmeDetail, candidaturesProgramme, chargerDetail, modifier, changerEtat, chargerCandidaturesProgramme, loading, error } = useAdminProgrammes()
const { listerTousDomaines } = useAdminDomaines()
const { listerPays } = useCentresCulturels()

// Sélecteurs de référentiel (audit #20 : fini les UUID en saisie libre)
const domainesListe = ref<{ id: string, nom: string }[]>([])
const paysListe = ref<{ id: string, nom: string }[]>([])

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const ongletActif = ref<'infos' | 'candidatures'>('infos')

const form = reactive({
  titre: '',
  description: '',
  pays_id: '',
  ville: '',
  adresse: '',
  prise_en_charge_billet: false,
  prise_en_charge_hebergement: false,
  prise_en_charge_subsistance: false,
  prise_en_charge_details: '',
  duree: '',
  domaine_id: '',
  date_debut: '',
  date_fin: '',
  nombre_places: null as number | null,
  prerequis: '',
  langues_requises: [] as string[],
})

const langueInput = ref('')

const durees = [
  { label: '1 semaine', value: '1_semaine' },
  { label: '2 semaines', value: '2_semaines' },
  { label: '3 semaines', value: '3_semaines' },
  { label: '6 semaines', value: '6_semaines' },
  { label: '1 mois', value: '1_mois' },
  { label: '2 mois', value: '2_mois' },
  { label: '3 mois', value: '3_mois' },
  { label: '6 mois', value: '6_mois' },
  { label: '1 an', value: '1_an' },
]

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'badge-info',
    en_attente_validation: 'badge-warning',
    publie: 'badge-success',
    en_cours: 'badge-success',
    termine: 'badge-neutral',
    suspendu: 'badge-warning',
    annule: 'badge-error',
  }
  return map[etat] || 'badge-neutral'
}

const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'Brouillon',
    en_attente_validation: 'En attente',
    publie: 'Publie',
    en_cours: 'En cours',
    termine: 'Termine',
    suspendu: 'Suspendu',
    annule: 'Annule',
  }
  return map[etat] || etat
}

const statutBadge = (statut: string) => {
  const map: Record<string, string> = {
    soumise: 'badge-info',
    en_revue: 'badge-warning',
    acceptee: 'badge-success',
    refusee: 'badge-error',
    retiree: 'badge-neutral',
  }
  return map[statut] || 'badge-neutral'
}

const statutLabel = (statut: string) => {
  const map: Record<string, string> = {
    soumise: 'Soumise',
    en_revue: 'En revue',
    acceptee: 'Acceptee',
    refusee: 'Refusee',
    retiree: 'Retiree',
  }
  return map[statut] || statut
}

const candidatureColonnes: TableColumn[] = [
  { key: 'candidat_nom', label: 'Candidat' },
  { key: 'candidat_email', label: 'Email' },
  { key: 'statut', label: 'Statut', width: 'w-24', align: 'center' },
  { key: 'created_at', label: 'Date', width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const ajouterLangue = () => {
  const l = langueInput.value.trim()
  if (l && !form.langues_requises.includes(l)) {
    form.langues_requises.push(l)
    langueInput.value = ''
  }
}

const retirerLangue = (index: number) => {
  form.langues_requises.splice(index, 1)
}

const charger = async () => {
  await chargerDetail(id)
  if (programmeDetail.value) {
    const p = programmeDetail.value
    form.titre = p.titre
    form.description = p.description || ''
    form.pays_id = p.pays_id || ''
    form.ville = p.ville || ''
    form.adresse = p.adresse || ''
    form.prise_en_charge_billet = p.prise_en_charge_billet
    form.prise_en_charge_hebergement = p.prise_en_charge_hebergement
    form.prise_en_charge_subsistance = p.prise_en_charge_subsistance
    form.prise_en_charge_details = p.prise_en_charge_details || ''
    form.duree = p.duree || ''
    form.domaine_id = p.domaine_id || ''
    form.date_debut = p.date_debut || ''
    form.date_fin = p.date_fin || ''
    form.nombre_places = p.nombre_places
    form.prerequis = p.prerequis || ''
    form.langues_requises = p.langues_requises || []
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    await modifier(id, { ...form })
    successMsg.value = 'Programme mis a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally { saving.value = false }
}

// Changement etat rapide
const showEtatModal = ref(false)
const nouvelEtat = ref('')
const etatLoading = ref(false)

const ouvrirEtatModal = () => {
  if (programmeDetail.value) {
    nouvelEtat.value = programmeDetail.value.etat
    showEtatModal.value = true
  }
}

const executerChangerEtat = async () => {
  if (!programmeDetail.value || nouvelEtat.value === programmeDetail.value.etat) return
  etatLoading.value = true
  try {
    await changerEtat(id, nouvelEtat.value)
    showEtatModal.value = false
    await chargerDetail(id)
  } catch {} finally { etatLoading.value = false }
}

onMounted(async () => {
  const [domaines, pays] = await Promise.all([listerTousDomaines(), listerPays()])
  domainesListe.value = domaines.map(d => ({ id: d.id, nom: d.nom }))
  paysListe.value = pays
  await charger()
  await chargerCandidaturesProgramme(id)
})
</script>

<template>
  <div>
    <AdminPageHeader :titre="programmeDetail?.titre || 'Chargement...'" sous-titre="Modifier le programme d'echange">
      <template #actions>
        <NuxtLink to="/admin/programmes" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !programmeDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="programmeDetail">
      <!-- Etat actuel -->
      <div class="flex items-center gap-3 mb-4">
        <span :class="['badge', etatBadge(programmeDetail.etat)]">
          {{ etatLabel(programmeDetail.etat) }}
        </span>
        <button class="btn btn-outline btn-xs" @click="ouvrirEtatModal">
          <font-awesome-icon icon="arrows-rotate" class="mr-1" /> Changer état
        </button>
        <span v-if="programmeDetail.nombre_candidatures > 0" class="text-sm text-base-content/70">
          {{ programmeDetail.nombre_candidatures }} candidature(s)
        </span>
      </div>

      <!-- Onglets -->
      <div class="tabs tabs-bordered mb-4">
        <button :class="['tab', { 'tab-active': ongletActif === 'infos' }]" @click="ongletActif = 'infos'">
          <font-awesome-icon icon="info-circle" class="mr-1" /> Informations
        </button>
        <button :class="['tab', { 'tab-active': ongletActif === 'candidatures' }]" @click="ongletActif = 'candidatures'">
          <font-awesome-icon icon="file-lines" class="mr-1" /> Candidatures
          <span v-if="programmeDetail.nombre_candidatures" class="badge badge-sm ml-1">{{ programmeDetail.nombre_candidatures }}</span>
        </button>
      </div>

      <!-- Onglet Infos -->
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

            <!-- Localisation -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Localisation & domaine</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Territoire de destination</span></label>
                  <select v-model="form.pays_id" class="select select-bordered">
                    <option value="">Sélectionner un territoire</option>
                    <option v-for="p in paysListe" :key="p.id" :value="p.id">{{ p.nom }}</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Domaine</span></label>
                  <select v-model="form.domaine_id" class="select select-bordered">
                    <option value="">Sélectionner un domaine</option>
                    <option v-for="d in domainesListe" :key="d.id" :value="d.id">{{ d.nom }}</option>
                  </select>
                </div>
              </div>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Ville</span></label>
                  <input v-model="form.ville" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Adresse</span></label>
                  <input v-model="form.adresse" type="text" class="input input-bordered">
                </div>
              </div>
            </div>

            <!-- Dates & duree -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Dates & capacite</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Durée</span></label>
                  <select v-model="form.duree" class="select select-bordered">
                    <option value="">Non specifie</option>
                    <option v-for="d in durees" :key="d.value" :value="d.value">{{ d.label }}</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Date de debut</span></label>
                  <input v-model="form.date_debut" type="date" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Date de fin</span></label>
                  <input v-model="form.date_fin" type="date" class="input input-bordered">
                </div>
              </div>
              <div class="form-control w-48">
                <label class="label"><span class="label-text">Nombre de places</span></label>
                <input v-model.number="form.nombre_places" type="number" min="1" class="input input-bordered">
              </div>
            </div>

            <!-- Couverture -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Prise en charge</h3>
              <div class="flex flex-wrap gap-6">
                <label class="label cursor-pointer gap-2">
                  <input v-model="form.prise_en_charge_billet" type="checkbox" class="checkbox checkbox-primary" />
                  <span class="label-text">Billet d'avion</span>
                </label>
                <label class="label cursor-pointer gap-2">
                  <input v-model="form.prise_en_charge_hebergement" type="checkbox" class="checkbox checkbox-primary" />
                  <span class="label-text">Hebergement</span>
                </label>
                <label class="label cursor-pointer gap-2">
                  <input v-model="form.prise_en_charge_subsistance" type="checkbox" class="checkbox checkbox-primary" />
                  <span class="label-text">Subsistance</span>
                </label>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Détails</span></label>
                <textarea v-model="form.prise_en_charge_details" class="textarea textarea-bordered" />
              </div>
            </div>

            <!-- Prerequis -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Prerequis</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Prerequis</span></label>
                <textarea v-model="form.prerequis" class="textarea textarea-bordered" />
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Langues requises</span></label>
                <div class="flex gap-2">
                  <input v-model="langueInput" type="text" class="input input-bordered flex-1" placeholder="Ex: Francais" @keydown.enter.prevent="ajouterLangue">
                  <button type="button" class="btn btn-outline" @click="ajouterLangue">Ajouter</button>
                </div>
                <div v-if="form.langues_requises.length" class="flex flex-wrap gap-2 mt-2">
                  <span v-for="(l, i) in form.langues_requises" :key="i" class="badge badge-primary gap-1">
                    {{ l }}
                    <button type="button" class="btn btn-ghost btn-xs p-0" @click="retirerLangue(i)">&times;</button>
                  </span>
                </div>
              </div>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="programmeDetail.cree_par_nom">Cree par {{ programmeDetail.cree_par_nom }}</span>
                <span v-if="programmeDetail.valide_par_nom"> &middot; Valide par {{ programmeDetail.valide_par_nom }}</span>
                <br>ID: {{ programmeDetail.id.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Candidatures -->
      <div v-else-if="ongletActif === 'candidatures'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div v-if="!candidaturesProgramme.length" class="text-center py-8 text-base-content/50">
            <font-awesome-icon icon="inbox" class="text-4xl mb-2" />
            <p>Aucune candidature pour ce programme</p>
          </div>
          <div v-else class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th v-for="col in candidatureColonnes" :key="col.key">{{ col.label }}</th>
                  <th class="w-20">Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="c in candidaturesProgramme" :key="c.id">
                  <td>{{ c.candidat_nom }}</td>
                  <td>{{ c.candidat_email }}</td>
                  <td class="text-center">
                    <span :class="['badge badge-sm', statutBadge(c.statut)]">
                      {{ statutLabel(c.statut) }}
                    </span>
                  </td>
                  <td>{{ new Date(c.created_at).toLocaleDateString('fr-FR') }}</td>
                  <td>
                    <NuxtLink :to="`/admin/candidatures/${c.id}`" class="btn btn-ghost btn-xs">
                      <font-awesome-icon icon="eye" />
                    </NuxtLink>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Modal etat -->
      <div v-if="showEtatModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">Changer l'état du programme</h3>
          <div class="form-control">
            <select v-model="nouvelEtat" class="select select-bordered">
              <option value="brouillon">Brouillon</option>
              <option value="en_attente_validation">En attente de validation</option>
              <option value="publie">Publie</option>
              <option value="en_cours">En cours</option>
              <option value="termine">Termine</option>
              <option value="suspendu">Suspendu</option>
              <option value="annule">Annule</option>
            </select>
          </div>
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showEtatModal = false">Annuler</button>
            <button class="btn btn-primary" :class="{ loading: etatLoading }" :disabled="etatLoading || nouvelEtat === programmeDetail.etat" @click="executerChangerEtat">
              Confirmer
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showEtatModal = false" />
      </div>
    </template>
  </div>
</template>
