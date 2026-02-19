<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const { salleDetail, loading, error, chargerDetail, modifier } = useAdminSalles()

const ongletActif = ref('infos')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive({
  titre: '',
  description: '',
  langue_cible: '',
  moderateur_id: '',
  actif: true,
})

const charger = async () => {
  await chargerDetail(id)
  if (salleDetail.value) {
    const s = salleDetail.value
    form.titre = s.titre
    form.description = s.description || ''
    form.langue_cible = s.langue_cible || ''
    form.moderateur_id = s.moderateur_id || ''
    form.actif = s.actif
  }
}

const sauvegarderInfos = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = {}
    if (form.titre.trim()) body.titre = form.titre.trim()
    if (form.description.trim()) body.description = form.description.trim()
    if (form.langue_cible.trim()) body.langue_cible = form.langue_cible.trim()
    if (form.moderateur_id.trim()) body.moderateur_id = form.moderateur_id.trim()
    body.actif = form.actif
    await modifier(id, body)
    successMsg.value = 'Salle mise a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

// Sessions de cette salle (via composable sessions)
const { sessions: sessionsListe, chargerListe: chargerSessions } = useAdminSessions()

const chargerSessionsSalle = async () => {
  const { filtres } = useAdminSessions()
  filtres.salle_id = id
  await chargerSessions()
}

watch(ongletActif, (val) => {
  if (val === 'sessions' && sessionsListe.value.length === 0) {
    chargerSessionsSalle()
  }
})

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="salleDetail?.titre || 'Chargement...'"
      sous-titre="Edition de la salle AfroLang"
    >
      <template #actions>
        <NuxtLink to="/admin/salles" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !salleDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="salleDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-primary text-primary-content rounded-full w-16 h-16">
            <span class="text-xl"><font-awesome-icon icon="video" /></span>
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ salleDetail.titre }}</h2>
          <p class="text-sm text-base-content/60">
            {{ salleDetail.langue_cible || 'Langue non definie' }}
            {{ salleDetail.moderateur_nom ? ` — Mod: ${salleDetail.moderateur_nom}` : '' }}
          </p>
          <div class="flex gap-2 mt-1">
            <span :class="salleDetail.actif ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">
              {{ salleDetail.actif ? 'Active' : 'Inactive' }}
            </span>
            <span class="badge badge-outline badge-sm">{{ salleDetail.nombre_salles_privees }} salles privees</span>
            <span class="badge badge-outline badge-sm">{{ salleDetail.nombre_sessions }} sessions</span>
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
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'sessions' }" @click="ongletActif = 'sessions'">
          <font-awesome-icon icon="video" class="mr-1" /> Sessions
        </button>
      </div>

      <!-- Onglet Infos -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarderInfos" class="space-y-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Titre de la salle *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Langue cible</span></label>
                <input v-model="form.langue_cible" type="text" class="input input-bordered" placeholder="Ex: Swahili, Wolof...">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Moderateur (UUID)</span></label>
                <input v-model="form.moderateur_id" type="text" class="input input-bordered" placeholder="UUID de l'utilisateur">
              </div>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="form.actif" type="checkbox" class="toggle toggle-success" />
                <span class="label-text">Salle active</span>
              </label>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                ID: {{ salleDetail.id.substring(0, 8) }}... | Slug: {{ salleDetail.slug }}
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Sessions -->
      <div v-if="ongletActif === 'sessions'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Sessions de cette salle</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Titre</th>
                <th class="w-24">Etat</th>
                <th class="w-36">Date</th>
                <th class="w-24 text-center">Participants</th>
                <th class="w-16">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="session in sessionsListe" :key="session.id">
                <td>{{ session.titre || 'Sans titre' }}</td>
                <td>
                  <span class="badge badge-sm" :class="{
                    'badge-info': session.etat === 'planifiee',
                    'badge-success': session.etat === 'en_cours',
                    'badge-neutral': session.etat === 'terminee',
                    'badge-error': session.etat === 'annulee',
                  }">
                    {{ session.etat || '—' }}
                  </span>
                </td>
                <td>{{ session.demarre_at ? new Date(session.demarre_at).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' }) : '—' }}</td>
                <td class="text-center">{{ session.nombre_participants_pic ?? '—' }}</td>
                <td>
                  <NuxtLink :to="`/admin/sessions/${session.id}`" class="btn btn-ghost btn-xs">
                    <font-awesome-icon icon="eye" />
                  </NuxtLink>
                </td>
              </tr>
              <tr v-if="!sessionsListe.length">
                <td colspan="5" class="text-center text-base-content/50 py-4">Aucune session</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>
  </div>
</template>
