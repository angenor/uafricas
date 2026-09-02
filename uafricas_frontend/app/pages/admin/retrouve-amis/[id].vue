<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  avisDetail, loading, error,
  chargerDetailAvis, changerEtatAvis,
} = useAdminRetrouvAmis()

const showEtat = ref(false)
const etatLoading = ref(false)
const nouvelEtat = ref('')
const successMsg = ref<string | null>(null)

const etatBadgeClass = (etat: string): string => {
  const classes: Record<string, string> = {
    actif: 'badge-success',
    suspendu: 'badge-error',
    cloture: 'badge-ghost',
    en_attente: 'badge-warning',
    mutuelle: 'badge-info',
    acceptee: 'badge-success',
    declinee: 'badge-error',
    archivee: 'badge-neutral',
    approuve: 'badge-success',
    rejete: 'badge-error',
  }
  return classes[etat] || 'badge-neutral'
}

const etatLabel = (etat: string): string => {
  const labels: Record<string, string> = {
    actif: 'Actif',
    suspendu: 'Suspendu',
    cloture: 'Cloture',
    en_attente: 'En attente',
    mutuelle: 'Mutuelle',
    acceptee: 'Acceptee',
    declinee: 'Declinee',
    archivee: 'Archivee',
    approuve: 'Approuve',
    rejete: 'Rejete',
  }
  return labels[etat] || etat
}

const typeCibleLabel = (type: string): string => {
  const labels: Record<string, string> = {
    profil: 'Profil utilisateur',
    avis: 'Avis de recherche',
  }
  return labels[type] || type
}

const motifLabel = (motif: string): string => {
  const labels: Record<string, string> = {
    faux_avis: 'Faux avis',
    contenu_inapproprie: 'Contenu inapproprie',
    usurpation_identite: 'Usurpation d\'identite',
    spam: 'Spam',
    autre: 'Autre',
  }
  return labels[motif] || motif
}

const motifBadgeClass = (motif: string): string => {
  const classes: Record<string, string> = {
    faux_avis: 'badge-error',
    contenu_inapproprie: 'badge-warning',
    usurpation_identite: 'badge-error',
    spam: 'badge-neutral',
    autre: 'badge-ghost',
  }
  return classes[motif] || 'badge-neutral'
}

const ouvrirChangerEtat = () => {
  if (!avisDetail.value) return
  nouvelEtat.value = avisDetail.value.etat === 'actif' ? 'suspendu' : 'actif'
  showEtat.value = true
}

const executerChangerEtat = async () => {
  if (!avisDetail.value) return
  etatLoading.value = true
  try {
    await changerEtatAvis(avisDetail.value.id, nouvelEtat.value)
    showEtat.value = false
    successMsg.value = `Etat mis a jour : ${etatLabel(nouvelEtat.value)}`
    setTimeout(() => { successMsg.value = null }, 3000)
    await chargerDetailAvis(id)
  } catch {} finally { etatLoading.value = false }
}

const formatDate = (date: string): string => {
  return new Date(date).toLocaleDateString('fr-FR', {
    day: '2-digit', month: '2-digit', year: 'numeric',
    hour: '2-digit', minute: '2-digit',
  })
}

onMounted(() => chargerDetailAvis(id))
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="avisDetail ? `Avis : ${avisDetail.nom_recherche}` : 'Chargement...'"
      sous-titre="Détail de l'avis de recherche"
    >
      <template #actions>
        <NuxtLink to="/admin/retrouve-amis" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <!-- Breadcrumb -->
    <div class="breadcrumbs text-sm mb-6">
      <ul>
        <li><NuxtLink to="/admin">Admin</NuxtLink></li>
        <li><NuxtLink to="/admin/retrouve-amis">Retrouve Amis</NuxtLink></li>
        <li>{{ avisDetail?.nom_recherche || 'Chargement...' }}</li>
      </ul>
    </div>

    <div v-if="loading && !avisDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="avisDetail">
      <!-- Alertes -->
      <div v-if="error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ error }}</span>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Informations generales -->
      <div class="card bg-base-100 shadow-sm mb-6">
        <div class="card-body">
          <div class="flex items-start justify-between">
            <div class="flex items-center gap-4">
              <div class="avatar placeholder">
                <div class="bg-neutral text-neutral-content rounded-full w-16 h-16 flex items-center justify-center">
                  <font-awesome-icon icon="magnifying-glass" class="text-xl" />
                </div>
              </div>
              <div>
                <h2 class="text-lg font-bold">{{ avisDetail.nom_recherche }} {{ avisDetail.prenom_recherche || '' }}</h2>
                <p v-if="avisDetail.surnom" class="text-sm text-base-content/60">Surnom : {{ avisDetail.surnom }}</p>
                <div class="flex items-center gap-2 mt-1">
                  <span class="badge badge-sm" :class="etatBadgeClass(avisDetail.etat)">{{ etatLabel(avisDetail.etat) }}</span>
                  <span v-if="avisDetail.pays" class="badge badge-outline badge-xs">{{ avisDetail.pays.nom }}</span>
                  <span v-if="avisDetail.ville" class="badge badge-outline badge-xs">{{ avisDetail.ville }}</span>
                </div>
              </div>
            </div>
            <div class="flex gap-2">
              <button
                v-if="avisDetail.etat !== 'cloture'"
                class="btn btn-sm"
                :class="avisDetail.etat === 'actif' ? 'btn-error' : 'btn-success'"
                @click="ouvrirChangerEtat"
              >
                <font-awesome-icon :icon="avisDetail.etat === 'actif' ? 'ban' : 'circle-check'" class="mr-1" />
                {{ avisDetail.etat === 'actif' ? 'Suspendre' : 'Reactiver' }}
              </button>
            </div>
          </div>

          <div class="divider my-3" />

          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div>
              <p class="text-xs text-base-content/50 uppercase font-semibold">Auteur</p>
              <p class="text-sm font-medium">
                <NuxtLink :to="`/admin/utilisateurs/${avisDetail.auteur.id}`" class="link link-primary">
                  {{ avisDetail.auteur.prenom }} {{ avisDetail.auteur.nom }}
                </NuxtLink>
              </p>
              <p v-if="avisDetail.auteur.email" class="text-xs text-base-content/50">{{ avisDetail.auteur.email }}</p>
            </div>
            <div>
              <p class="text-xs text-base-content/50 uppercase font-semibold">Ecole</p>
              <p class="text-sm">{{ avisDetail.ecole || '--' }}</p>
            </div>
            <div>
              <p class="text-xs text-base-content/50 uppercase font-semibold">Ville / Territoire</p>
              <p class="text-sm">{{ avisDetail.ville || '--' }} {{ avisDetail.pays ? `, ${avisDetail.pays.nom}` : '' }}</p>
            </div>
            <div>
              <p class="text-xs text-base-content/50 uppercase font-semibold">Période</p>
              <p class="text-sm">
                <template v-if="avisDetail.periode_debut || avisDetail.periode_fin">
                  {{ avisDetail.periode_debut || '?' }} - {{ avisDetail.periode_fin || '?' }}
                </template>
                <template v-else>--</template>
              </p>
            </div>
            <div>
              <p class="text-xs text-base-content/50 uppercase font-semibold">Date de creation</p>
              <p class="text-sm">{{ formatDate(avisDetail.created_at) }}</p>
            </div>
            <div>
              <p class="text-xs text-base-content/50 uppercase font-semibold">Derniere mise a jour</p>
              <p class="text-sm">{{ formatDate(avisDetail.updated_at) }}</p>
            </div>
          </div>

          <div v-if="avisDetail.description" class="mt-4">
            <p class="text-xs text-base-content/50 uppercase font-semibold mb-1">Description</p>
            <p class="text-sm whitespace-pre-line bg-base-200/50 rounded-lg p-3">{{ avisDetail.description }}</p>
          </div>

          <!-- Stats rapides -->
          <div class="flex gap-4 mt-4">
            <div class="stat bg-base-200/50 rounded-lg p-3">
              <div class="stat-title text-xs">Correspondances</div>
              <div class="stat-value text-lg">{{ avisDetail.nb_correspondances }}</div>
            </div>
            <div class="stat bg-base-200/50 rounded-lg p-3">
              <div class="stat-title text-xs">Signalements</div>
              <div class="stat-value text-lg" :class="{ 'text-error': avisDetail.nb_signalements > 0 }">{{ avisDetail.nb_signalements }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Correspondances -->
      <div class="card bg-base-100 shadow-sm mb-6">
        <div class="card-body">
          <h3 class="font-semibold text-lg mb-4">
            <font-awesome-icon icon="link" class="mr-2 text-base-content/50" />
            Correspondances ({{ avisDetail.correspondances.length }})
          </h3>

          <div v-if="!avisDetail.correspondances.length" class="text-center text-base-content/50 py-8">
            Aucune correspondance trouvee
          </div>

          <div v-else class="overflow-x-auto">
            <table class="table table-sm">
              <thead>
                <tr>
                  <th>Score</th>
                  <th>Type de cible</th>
                  <th>Utilisateur cible</th>
                  <th>État</th>
                  <th>Date</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="corr in avisDetail.correspondances" :key="corr.id">
                  <td>
                    <div class="flex items-center gap-2">
                      <progress class="progress progress-primary w-16" :value="corr.score" max="100" />
                      <span class="text-sm font-mono">{{ corr.score }}%</span>
                    </div>
                  </td>
                  <td>
                    <span class="badge badge-outline badge-sm">{{ typeCibleLabel(corr.type_cible) }}</span>
                  </td>
                  <td>
                    <template v-if="corr.cible_utilisateur">
                      <NuxtLink :to="`/admin/utilisateurs/${corr.cible_utilisateur.id}`" class="link link-primary text-sm">
                        {{ corr.cible_utilisateur.prenom }} {{ corr.cible_utilisateur.nom }}
                      </NuxtLink>
                    </template>
                    <span v-else class="text-base-content/30">--</span>
                  </td>
                  <td>
                    <span class="badge badge-sm" :class="etatBadgeClass(corr.etat)">{{ etatLabel(corr.etat) }}</span>
                  </td>
                  <td class="text-sm text-base-content/70">{{ formatDate(corr.created_at) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Signalements -->
      <div class="card bg-base-100 shadow-sm mb-6">
        <div class="card-body">
          <h3 class="font-semibold text-lg mb-4">
            <font-awesome-icon icon="flag" class="mr-2 text-error/50" />
            Signalements ({{ avisDetail.signalements.length }})
          </h3>

          <div v-if="!avisDetail.signalements.length" class="text-center text-base-content/50 py-8">
            Aucun signalement
          </div>

          <div v-else class="space-y-3">
            <div
              v-for="sig in avisDetail.signalements"
              :key="sig.id"
              class="flex items-start gap-3 p-3 rounded-lg border border-base-200"
            >
              <div class="avatar placeholder">
                <div class="bg-base-300 text-base-content rounded-full w-8 h-8">
                  <font-awesome-icon icon="user" class="text-xs" />
                </div>
              </div>
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 flex-wrap">
                  <span class="font-semibold text-sm">{{ sig.signale_par.prenom }} {{ sig.signale_par.nom }}</span>
                  <span class="badge badge-sm" :class="motifBadgeClass(sig.motif)">{{ motifLabel(sig.motif) }}</span>
                  <span class="badge badge-sm" :class="etatBadgeClass(sig.etat)">{{ etatLabel(sig.etat) }}</span>
                  <span class="text-xs text-base-content/50">{{ formatDate(sig.created_at) }}</span>
                </div>
                <p v-if="sig.description" class="text-sm mt-1 text-base-content/70">{{ sig.description }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>

    <!-- Modal confirmation changement d'etat -->
    <div v-if="showEtat" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">
          {{ nouvelEtat === 'suspendu' ? 'Suspendre' : 'Reactiver' }} l'avis
        </h3>
        <p class="py-2 text-sm text-base-content/70">
          Avis de recherche : <strong>{{ avisDetail?.nom_recherche }}</strong>
        </p>
        <p class="text-sm text-base-content/60">
          {{ nouvelEtat === 'suspendu'
            ? 'Cet avis ne sera plus visible par les autres utilisateurs.'
            : 'Cet avis sera de nouveau visible et actif.' }}
        </p>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showEtat = false">Annuler</button>
          <button
            class="btn"
            :class="nouvelEtat === 'suspendu' ? 'btn-error' : 'btn-success'"
            :disabled="etatLoading"
            @click="executerChangerEtat"
          >
            <span v-if="etatLoading" class="loading loading-spinner loading-xs" />
            {{ nouvelEtat === 'suspendu' ? 'Suspendre' : 'Reactiver' }}
          </button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showEtat = false" />
    </div>
  </div>
</template>
