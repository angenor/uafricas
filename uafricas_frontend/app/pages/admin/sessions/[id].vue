<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const { sessionDetail, tableauBlanc, loading, error, chargerDetail, chargerTableauBlanc } = useAdminSessions()

const ongletActif = ref('infos')

const formatDuree = (secondes: number | null) => {
  if (!secondes) return '-'
  const h = Math.floor(secondes / 3600)
  const m = Math.floor((secondes % 3600) / 60)
  const s = secondes % 60
  if (h > 0) return `${h}h ${m}min ${s}s`
  return `${m}min ${s}s`
}

const formatDate = (date: string | null) => {
  if (!date) return '-'
  return new Date(date).toLocaleDateString('fr-FR', {
    day: '2-digit', month: 'long', year: 'numeric',
    hour: '2-digit', minute: '2-digit',
  })
}

const etatClass = (etat: string | null) => {
  switch (etat) {
    case 'planifiee': return 'badge-info'
    case 'en_cours': return 'badge-success'
    case 'terminee': return 'badge-neutral'
    case 'annulee': return 'badge-error'
    default: return 'badge-ghost'
  }
}

const roleLabel = (role: string | null) => {
  switch (role) {
    case 'moderateur': return 'Moderateur'
    case 'participant': return 'Participant'
    case 'observateur': return 'Observateur'
    default: return role || '-'
  }
}

const roleClass = (role: string | null) => {
  switch (role) {
    case 'moderateur': return 'badge-primary'
    case 'participant': return 'badge-success'
    case 'observateur': return 'badge-warning'
    default: return 'badge-ghost'
  }
}

watch(ongletActif, (val) => {
  if (val === 'tableau-blanc' && !tableauBlanc.value) {
    chargerTableauBlanc(id)
  }
})

onMounted(() => chargerDetail(id))
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="sessionDetail?.titre || 'Session sans titre'"
      sous-titre="Detail de la session AfroLang"
    >
      <template #actions>
        <NuxtLink to="/admin/sessions" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !sessionDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="sessionDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-accent text-accent-content rounded-full w-16 h-16">
            <span class="text-xl"><font-awesome-icon icon="video" /></span>
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ sessionDetail.titre || 'Session sans titre' }}</h2>
          <p class="text-sm text-base-content/60">
            {{ sessionDetail.salle_titre || '-' }}
            {{ sessionDetail.salle_langue ? ` (${sessionDetail.salle_langue})` : '' }}
            {{ sessionDetail.salle_privee_titre || 'Salle privee' }}
          </p>
          <div class="flex gap-2 mt-1">
            <span class="badge badge-sm" :class="etatClass(sessionDetail.etat)">
              {{ sessionDetail.etat || '-' }}
            </span>
            <span class="badge badge-outline badge-sm">
              {{ sessionDetail.participants.length }} participant(s)
            </span>
            <span v-if="sessionDetail.nombre_participants_pic" class="badge badge-outline badge-sm">
              Pic: {{ sessionDetail.nombre_participants_pic }}
            </span>
          </div>
        </div>
      </div>

      <!-- Onglets -->
      <div role="tablist" class="tabs tabs-bordered mb-6">
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'infos' }" @click="ongletActif = 'infos'">
          <font-awesome-icon icon="circle-info" class="mr-1" /> Infos
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'participants' }" @click="ongletActif = 'participants'">
          <font-awesome-icon icon="users" class="mr-1" /> Participants
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'tableau-blanc' }" @click="ongletActif = 'tableau-blanc'">
          <font-awesome-icon icon="chalkboard" class="mr-1" /> Tableau blanc
        </button>
      </div>

      <!-- Onglet Infos -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Etat</h4>
              <span class="badge" :class="etatClass(sessionDetail.etat)">
                {{ sessionDetail.etat || '-' }}
              </span>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Moderateur</h4>
              <p>{{ sessionDetail.moderateur_nom || '-' }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Date debut prevue</h4>
              <p>{{ formatDate(sessionDetail.date_debut_prevue) }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Demarree le</h4>
              <p>{{ formatDate(sessionDetail.demarre_at) }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Terminee le</h4>
              <p>{{ formatDate(sessionDetail.termine_at) }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Duree</h4>
              <p>{{ formatDuree(sessionDetail.duree_secondes) }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Max participants</h4>
              <p>{{ sessionDetail.max_participants ?? 'Non limite' }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Pic de participants</h4>
              <p>{{ sessionDetail.nombre_participants_pic ?? '-' }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Tableau blanc actif</h4>
              <p>{{ sessionDetail.tableau_blanc_actif ? 'Oui' : 'Non' }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Createur</h4>
              <p>{{ sessionDetail.cree_par_nom || '-' }}</p>
            </div>
          </div>
          <div class="mt-4 text-sm text-base-content/50">
            ID: {{ sessionDetail.id }}
          </div>
        </div>
      </div>

      <!-- Onglet Participants -->
      <div v-if="ongletActif === 'participants'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Participants ({{ sessionDetail.participants.length }})</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Nom</th>
                <th class="w-28">Role</th>
                <th class="w-36">Rejoint le</th>
                <th class="w-36">Quitte le</th>
                <th class="w-24">Duree</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="p in sessionDetail.participants" :key="p.id">
                <td>{{ p.utilisateur_prenom }} {{ p.utilisateur_nom }}</td>
                <td>
                  <span class="badge badge-sm" :class="roleClass(p.role_session)">
                    {{ roleLabel(p.role_session) }}
                  </span>
                </td>
                <td>{{ formatDate(p.rejoint_at) }}</td>
                <td>{{ formatDate(p.quitte_at) }}</td>
                <td>{{ formatDuree(p.duree_secondes) }}</td>
              </tr>
              <tr v-if="!sessionDetail.participants.length">
                <td colspan="5" class="text-center text-base-content/50 py-4">Aucun participant</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Onglet Tableau blanc -->
      <div v-if="ongletActif === 'tableau-blanc'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Snapshot du tableau blanc</h3>
          <template v-if="tableauBlanc">
            <div class="flex gap-4 mb-4 text-sm text-base-content/60">
              <span>Version: {{ tableauBlanc.version ?? '-' }}</span>
              <span>Derniere maj: {{ formatDate(tableauBlanc.updated_at) }}</span>
            </div>
            <div class="bg-base-200 rounded-lg p-4 overflow-auto max-h-[500px]">
              <pre class="text-xs whitespace-pre-wrap">{{ JSON.stringify(tableauBlanc.donnees, null, 2) }}</pre>
            </div>
          </template>
          <template v-else>
            <div class="text-center text-base-content/50 py-8">
              <font-awesome-icon icon="chalkboard" class="text-4xl mb-2" />
              <p>Aucun tableau blanc pour cette session</p>
            </div>
          </template>
        </div>
      </div>
    </template>
  </div>
</template>
