<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const { sallePriveeDetail, loading, error, chargerDetail } = useAdminSallesPrivees()

// Sessions liées
const { sessions: sessionsListe, chargerListe: chargerSessionsBase, filtres: sessionsFiltres } = useAdminSessions()
const ongletActif = ref('infos')

const chargerSessions = async () => {
  sessionsFiltres.salle_privee_id = id
  await chargerSessionsBase()
}

watch(ongletActif, (val) => {
  if (val === 'sessions' && sessionsListe.value.length === 0) {
    chargerSessions()
  }
})

onMounted(() => chargerDetail(id))
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="sallePriveeDetail?.titre || 'Chargement...'"
      sous-titre="Détail de la salle privee (lecture seule)"
    >
      <template #actions>
        <NuxtLink to="/admin/salles-privees" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !sallePriveeDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="sallePriveeDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-secondary text-secondary-content rounded-full w-16 h-16">
            <span class="text-xl"><font-awesome-icon icon="lock" /></span>
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ sallePriveeDetail.titre }}</h2>
          <p class="text-sm text-base-content/60">
            Salle parente : {{ sallePriveeDetail.salle_titre || '-' }}
            {{ sallePriveeDetail.salle_langue ? ` (${sallePriveeDetail.salle_langue})` : '' }}
          </p>
          <div class="flex gap-2 mt-1">
            <span :class="sallePriveeDetail.actif ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">
              {{ sallePriveeDetail.actif ? 'Active' : 'Inactive' }}
            </span>
            <span class="badge badge-outline badge-sm">{{ sallePriveeDetail.nombre_sessions }} sessions</span>
            <span v-if="sallePriveeDetail.max_participants" class="badge badge-outline badge-sm">
              Max {{ sallePriveeDetail.max_participants }} participants
            </span>
          </div>
        </div>
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
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Description</h4>
              <p>{{ sallePriveeDetail.description || 'Aucune description' }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Createur</h4>
              <p>{{ sallePriveeDetail.cree_par_nom || '-' }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Code d'acces</h4>
              <p class="font-mono">{{ sallePriveeDetail.code_acces || 'Aucun' }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Max participants</h4>
              <p>{{ sallePriveeDetail.max_participants ?? 'Non limite' }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">Date de creation</h4>
              <p>{{ new Date(sallePriveeDetail.created_at).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' }) }}</p>
            </div>
            <div>
              <h4 class="font-semibold text-sm text-base-content/60 mb-1">ID</h4>
              <p class="font-mono text-sm">{{ sallePriveeDetail.id }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Onglet Sessions -->
      <div v-if="ongletActif === 'sessions'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Historique des sessions</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Titre</th>
                <th class="w-24">État</th>
                <th class="w-36">Date</th>
                <th class="w-24 text-center">Pic participants</th>
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
                    {{ session.etat || '-' }}
                  </span>
                </td>
                <td>{{ session.demarre_at ? new Date(session.demarre_at).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' }) : '-' }}</td>
                <td class="text-center">{{ session.nombre_participants_pic ?? '-' }}</td>
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
