<script setup lang="ts">
import type { AdminProgrammation } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  centreDetail, loading, error,
  chargerDetail, modifier,
  ajouterMembre, modifierMembre, retirerMembre,
} = useAdminCentresCulturels()

const { adminFetch } = useAdmin()

const ongletActif = ref('infos')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

// Formulaire infos
const form = reactive({
  nom: '',
  description: '',
  pays_id: '',
  ville: '',
  adresse: '',
  longitude: null as number | null,
  latitude: null as number | null,
  actif: true,
})

// Ajout membre
const nouveauMembreUtilisateurId = ref('')
const nouveauMembreRole = ref('membre')

// Modification role membre
const editingMembreId = ref<string | null>(null)
const editingMembreRole = ref('')

// Programmations du centre
const programmations = ref<AdminProgrammation[]>([])
const programmationsLoading = ref(false)

const roleLabels: Record<string, string> = {
  president: 'President',
  vice_president: 'Vice-president',
  resp_communication: 'Resp. Communication',
  membre: 'Membre',
}

const charger = async () => {
  await chargerDetail(id)
  if (centreDetail.value) {
    const c = centreDetail.value
    form.nom = c.nom
    form.description = c.description || ''
    form.pays_id = c.pays_id || ''
    form.ville = c.ville || ''
    form.adresse = c.adresse || ''
    form.longitude = c.longitude
    form.latitude = c.latitude
    form.actif = c.actif
  }
}

const chargerProgrammations = async () => {
  programmationsLoading.value = true
  try {
    const response = await adminFetch<{ success: boolean; data: { data: AdminProgrammation[] } }>(
      '/api/admin/programmations',
      { params: { centre_culturel_id: id, par_page: 50 } },
    )
    if (response.success && response.data) {
      programmations.value = response.data.data
    }
  } catch {} finally {
    programmationsLoading.value = false
  }
}

const sauvegarderInfos = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    await modifier(id, { ...form })
    successMsg.value = 'Centre culturel mis a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

const executerAjoutMembre = async () => {
  if (!nouveauMembreUtilisateurId.value) return
  erreurLocale.value = null
  try {
    await ajouterMembre(id, nouveauMembreUtilisateurId.value, nouveauMembreRole.value)
    nouveauMembreUtilisateurId.value = ''
    nouveauMembreRole.value = 'membre'
    await charger()
    successMsg.value = 'Membre ajoute'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de l\'ajout du membre'
  }
}

const commencerEditionRole = (membreId: string, roleActuel: string) => {
  editingMembreId.value = membreId
  editingMembreRole.value = roleActuel
}

const executerModificationRole = async (membreId: string) => {
  erreurLocale.value = null
  try {
    await modifierMembre(id, membreId, editingMembreRole.value)
    editingMembreId.value = null
    await charger()
    successMsg.value = 'Role modifie'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la modification du role'
  }
}

const executerSuppressionMembre = async (membreId: string) => {
  erreurLocale.value = null
  try {
    await retirerMembre(id, membreId)
    await charger()
    successMsg.value = 'Membre retire'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors du retrait du membre'
  }
}

watch(ongletActif, (val) => {
  if (val === 'programmations' && programmations.value.length === 0) {
    chargerProgrammations()
  }
})

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="centreDetail?.nom || 'Chargement...'"
      sous-titre="Edition du centre culturel"
    >
      <template #actions>
        <NuxtLink to="/admin/centres-culturels" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" />
          Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !centreDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="centreDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-neutral text-neutral-content rounded-full w-16 h-16">
            <span class="text-xl"><font-awesome-icon icon="landmark" /></span>
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ centreDetail.nom }}</h2>
          <p class="text-sm text-base-content/60">
            {{ centreDetail.ville || '—' }}{{ centreDetail.pays_nom ? `, ${centreDetail.pays_nom}` : '' }}
          </p>
          <div class="flex gap-2 mt-1">
            <span :class="centreDetail.actif ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">
              {{ centreDetail.actif ? 'Actif' : 'Inactif' }}
            </span>
            <span class="badge badge-outline badge-sm">{{ centreDetail.membres?.length || 0 }} membres</span>
          </div>
        </div>
      </div>

      <!-- Alertes -->
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">
          <font-awesome-icon icon="xmark" />
        </button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Onglets -->
      <div role="tablist" class="tabs tabs-bordered mb-6">
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'infos' }" @click="ongletActif = 'infos'">
          <font-awesome-icon icon="circle-info" class="mr-1" /> Infos
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'membres' }" @click="ongletActif = 'membres'">
          <font-awesome-icon icon="users" class="mr-1" /> Membres
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'programmations' }" @click="ongletActif = 'programmations'">
          <font-awesome-icon icon="calendar-days" class="mr-1" /> Programmations
        </button>
      </div>

      <!-- Onglet Infos -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarderInfos" class="space-y-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Nom du centre *</span></label>
              <input v-model="form.nom" type="text" class="input input-bordered" required>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Territoire (UUID)</span></label>
                <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="UUID du territoire">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Ville</span></label>
                <input v-model="form.ville" type="text" class="input input-bordered">
              </div>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Adresse</span></label>
              <textarea v-model="form.adresse" class="textarea textarea-bordered" rows="2" />
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Longitude</span></label>
                <input v-model.number="form.longitude" type="number" step="0.0000001" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Latitude</span></label>
                <input v-model.number="form.latitude" type="number" step="0.0000001" class="input input-bordered">
              </div>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="form.actif" type="checkbox" class="toggle toggle-success" />
                <span class="label-text">Centre actif</span>
              </label>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                ID: {{ centreDetail.id.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Membres -->
      <div v-if="ongletActif === 'membres'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Membres du centre</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Nom / Prenom</th>
                <th>Role</th>
                <th>Date</th>
                <th class="w-28">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="membre in centreDetail.membres" :key="membre.id">
                <td>{{ membre.utilisateur_prenom }} {{ membre.utilisateur_nom }}</td>
                <td>
                  <template v-if="editingMembreId === membre.id">
                    <select v-model="editingMembreRole" class="select select-bordered select-xs">
                      <option value="president">President</option>
                      <option value="vice_president">Vice-president</option>
                      <option value="resp_communication">Resp. Communication</option>
                      <option value="membre">Membre</option>
                    </select>
                  </template>
                  <template v-else>
                    <span class="badge badge-sm badge-outline">{{ roleLabels[membre.role] || membre.role }}</span>
                  </template>
                </td>
                <td>{{ new Date(membre.created_at).toLocaleDateString('fr-FR') }}</td>
                <td>
                  <div class="flex gap-1">
                    <template v-if="editingMembreId === membre.id">
                      <button class="btn btn-ghost btn-xs text-success" @click="executerModificationRole(membre.id)">
                        <font-awesome-icon icon="check" />
                      </button>
                      <button class="btn btn-ghost btn-xs" @click="editingMembreId = null">
                        <font-awesome-icon icon="xmark" />
                      </button>
                    </template>
                    <template v-else>
                      <button class="btn btn-ghost btn-xs" @click="commencerEditionRole(membre.id, membre.role)">
                        <font-awesome-icon icon="pen-to-square" />
                      </button>
                      <button class="btn btn-ghost btn-xs text-error" @click="executerSuppressionMembre(membre.id)">
                        <font-awesome-icon icon="trash" />
                      </button>
                    </template>
                  </div>
                </td>
              </tr>
              <tr v-if="!centreDetail.membres?.length">
                <td colspan="4" class="text-center text-base-content/50 py-4">Aucun membre</td>
              </tr>
            </tbody>
          </table>
          <div class="flex gap-2 mt-4">
            <input v-model="nouveauMembreUtilisateurId" type="text" class="input input-bordered input-sm flex-1" placeholder="UUID de l'utilisateur">
            <select v-model="nouveauMembreRole" class="select select-bordered select-sm">
              <option value="president">President</option>
              <option value="vice_president">Vice-president</option>
              <option value="resp_communication">Resp. Communication</option>
              <option value="membre">Membre</option>
            </select>
            <button class="btn btn-primary btn-sm" :disabled="!nouveauMembreUtilisateurId" @click="executerAjoutMembre">
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>
        </div>
      </div>

      <!-- Onglet Programmations -->
      <div v-if="ongletActif === 'programmations'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div class="flex items-center justify-between mb-4">
            <h3 class="font-semibold">Programmations du centre</h3>
            <NuxtLink :to="`/admin/programmations/create?centre_id=${id}`" class="btn btn-primary btn-sm">
              <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle programmation
            </NuxtLink>
          </div>

          <div v-if="programmationsLoading" class="flex justify-center py-8">
            <span class="loading loading-spinner loading-md" />
          </div>

          <table v-else class="table table-sm">
            <thead>
              <tr>
                <th>Titre</th>
                <th class="w-24">Mode</th>
                <th class="w-36">Date debut</th>
                <th class="w-24 text-center">Places</th>
                <th class="w-16">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="prog in programmations" :key="prog.id">
                <td>{{ prog.titre }}</td>
                <td>
                  <span v-if="prog.mode" class="badge badge-sm badge-outline">{{ prog.mode }}</span>
                  <span v-else class="text-base-content/40">—</span>
                </td>
                <td>{{ new Date(prog.date_heure_debut).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' }) }}</td>
                <td class="text-center">{{ prog.nombre_places ?? '—' }}</td>
                <td>
                  <NuxtLink :to="`/admin/programmations/${prog.id}`" class="btn btn-ghost btn-xs">
                    <font-awesome-icon icon="pen-to-square" />
                  </NuxtLink>
                </td>
              </tr>
              <tr v-if="!programmations.length">
                <td colspan="5" class="text-center text-base-content/50 py-4">Aucune programmation</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>
  </div>
</template>
