<script setup lang="ts">
/**
 * File de modération des propositions de médias (US4).
 *
 * Triée par ancienneté côté serveur : une file se traite dans l'ordre
 * d'arrivée, sans quoi les plus anciennes s'enfoncent indéfiniment.
 */
import {
  LIBELLES_TYPE_OBJET,
  LIBELLES_STATUT,
  type PropositionMediaAPI,
  type StatutProposition,
  type TypeObjetPropose,
} from '~/composables/useMediaProposition'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

useHead({ title: 'Propositions de médias, Administration' })

const { lister, chargement, erreur } = useAdminMediaPropositions()

const propositions = ref<PropositionMediaAPI[]>([])
const total = ref(0)
const page = ref(1)
const totalPages = ref(1)

// La file s'ouvre par défaut sur ce qui attend une décision.
const filtreStatut = ref<StatutProposition | ''>('en_attente')
const filtreType = ref<TypeObjetPropose | ''>('')

const BADGES_STATUT: Record<StatutProposition, string> = {
  en_attente: 'badge-warning',
  validee: 'badge-success',
  rejetee: 'badge-error',
  retiree: 'badge-ghost',
}

const charger = async () => {
  const res = await lister({
    statut: filtreStatut.value || undefined,
    type_objet: filtreType.value || undefined,
    page: page.value,
    par_page: 20,
  })
  propositions.value = res?.propositions ?? []
  total.value = res?.total ?? 0
  totalPages.value = res?.total_pages ?? 1
}

watch([filtreStatut, filtreType], () => {
  page.value = 1
  charger()
})
watch(page, charger)
onMounted(charger)

const dateFormatee = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' })

const auteur = (p: PropositionMediaAPI) =>
  `${p.auteur_prenom ?? ''} ${p.auteur_nom ?? ''}`.trim() || 'Membre'
</script>

<template>
  <div class="p-6">
    <header class="mb-6">
      <h1 class="text-2xl font-bold">Propositions de médias</h1>
      <p class="text-sm opacity-70 mt-1">
        Chaînes, stations et émissions soumises par les membres. Rien n’est publié
        sans validation.
      </p>
    </header>

    <!-- Filtres -->
    <div class="flex flex-wrap gap-3 mb-6">
      <select v-model="filtreStatut" class="select select-bordered select-sm">
        <option value="">Tous les statuts</option>
        <option v-for="(libelle, statut) in LIBELLES_STATUT" :key="statut" :value="statut">
          {{ libelle }}
        </option>
      </select>
      <select v-model="filtreType" class="select select-bordered select-sm">
        <option value="">Tous les types</option>
        <option v-for="(libelle, type) in LIBELLES_TYPE_OBJET" :key="type" :value="type">
          {{ libelle }}
        </option>
      </select>
      <span class="text-sm opacity-70 self-center ml-auto">{{ total }} proposition(s)</span>
    </div>

    <div v-if="erreur" class="alert alert-error mb-4">
      <span>{{ erreur }}</span>
    </div>

    <div v-if="chargement" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg"></span>
    </div>

    <div v-else-if="propositions.length === 0" class="text-center py-16 opacity-60">
      Aucune proposition ne correspond à ces filtres.
    </div>

    <div v-else class="overflow-x-auto">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>Contenu proposé</th>
            <th>Type</th>
            <th>Auteur</th>
            <th>Reçue le</th>
            <th>Statut</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="proposition in propositions" :key="proposition.id">
            <td class="font-medium">
              {{ proposition.donnees.nom || '-' }}
            </td>
            <td>{{ LIBELLES_TYPE_OBJET[proposition.type_objet] }}</td>
            <td>
              <div>{{ auteur(proposition) }}</div>
              <div class="text-xs opacity-60">{{ proposition.auteur_email }}</div>
            </td>
            <td class="whitespace-nowrap">{{ dateFormatee(proposition.created_at) }}</td>
            <td>
              <span class="badge badge-sm" :class="BADGES_STATUT[proposition.statut]">
                {{ LIBELLES_STATUT[proposition.statut] }}
              </span>
            </td>
            <td class="text-right">
              <NuxtLink
                :to="`/admin/medias/propositions/${proposition.id}`"
                class="btn btn-sm btn-primary"
              >
                Examiner
              </NuxtLink>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="totalPages > 1" class="join mt-6 flex justify-center">
      <button class="join-item btn btn-sm" :disabled="page <= 1" @click="page--">«</button>
      <span class="join-item btn btn-sm btn-disabled">{{ page }} / {{ totalPages }}</span>
      <button class="join-item btn btn-sm" :disabled="page >= totalPages" @click="page++">»</button>
    </div>
  </div>
</template>
