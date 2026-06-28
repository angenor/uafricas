<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  contributionsSuspendues, loading, error,
  chargerContributionsSuspendues, reactiverContribution,
} = useAdminProfilsPays()

const reactivationEnCours = ref<string | null>(null)

const TYPE_LABELS: Record<string, string> = {
  site_touristique: 'Site touristique',
  secteur_developpement: 'Secteur d\'opportunité',
  recette_culinaire: 'Recette culinaire',
  personnalite_connue: 'Personnalité',
  savoir_pratique: 'Conseil pratique',
}
const typeLabel = (t: string) => TYPE_LABELS[t] || t

const formatDate = (d: string | null) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric' })
}

const cleLigne = (type: string, id: string) => `${type}:${id}`

const reactiver = async (typeObjet: string, objetId: string) => {
  if (!window.confirm('Réactiver cette contribution ? Ses signalements seront effacés et elle redeviendra visible et modifiable.')) {
    return
  }
  reactivationEnCours.value = cleLigne(typeObjet, objetId)
  try {
    await reactiverContribution(typeObjet, objetId)
  } finally {
    reactivationEnCours.value = null
  }
}

onMounted(() => chargerContributionsSuspendues())
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Contributions suspendues"
      sous-titre="Contributions afripulse suspendues par signalement communautaire (plus de 10 signalements)"
    >
      <template #actions>
        <button class="btn btn-ghost btn-sm" @click="chargerContributionsSuspendues">
          <font-awesome-icon icon="rotate-right" class="mr-1" /> Rafraîchir
        </button>
        <NuxtLink to="/admin/profils-pays" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="error" class="alert alert-error mb-4">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ error }}</span>
    </div>

    <div v-if="loading && contributionsSuspendues.length === 0" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else class="card bg-base-100 shadow-sm">
      <div class="card-body p-0">
        <div class="overflow-x-auto">
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Type</th>
                <th>Contribution</th>
                <th>Territoire</th>
                <th class="text-center">Signalements</th>
                <th>Créée le</th>
                <th class="text-center">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-if="contributionsSuspendues.length === 0">
                <td colspan="6" class="text-center py-8 text-base-content/50">
                  <font-awesome-icon icon="circle-check" class="text-2xl mb-2 block text-success" />
                  Aucune contribution suspendue
                </td>
              </tr>
              <tr v-for="c in contributionsSuspendues" :key="cleLigne(c.type_objet, c.objet_id)" class="hover">
                <td>
                  <span class="badge badge-sm badge-outline badge-warning">{{ typeLabel(c.type_objet) }}</span>
                </td>
                <td class="font-medium max-w-xs truncate" :title="c.libelle">{{ c.libelle }}</td>
                <td class="text-sm">{{ c.pays_nom || '-' }}</td>
                <td class="text-center">
                  <span class="badge badge-sm badge-error">{{ c.nombre_signalements }}</span>
                </td>
                <td class="text-sm">{{ formatDate(c.created_at) }}</td>
                <td class="text-center">
                  <button
                    class="btn btn-success btn-xs"
                    :class="{ loading: reactivationEnCours === cleLigne(c.type_objet, c.objet_id) }"
                    :disabled="reactivationEnCours === cleLigne(c.type_objet, c.objet_id)"
                    @click="reactiver(c.type_objet, c.objet_id)"
                  >
                    <font-awesome-icon icon="rotate-left" class="mr-1" /> Réactiver
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
