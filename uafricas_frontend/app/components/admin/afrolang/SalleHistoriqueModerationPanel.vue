<script setup lang="ts">
// Historique chronologique des fermetures/réactivations administratives.
// Feature 001-ressources-fermeture-session — US3, T065. daisyUI v5.

interface Props {
  salleId: string
}
const props = defineProps<Props>()

interface EvenementModeration {
  id: string
  salle_id: string
  session_concernee_id: string | null
  type_action: 'fermeture_admin' | 'reactivation_admin'
  admin_id: string
  admin_nom: string | null
  admin_prenom: string | null
  motif: string | null
  created_at: string
}

const evenements = ref<EvenementModeration[]>([])
const total = ref(0)
const page = ref(1)
const limit = 50
const enCours = ref(false)

const { listerHistoriqueModeration } = useAdminAfrolangSalles()

const charger = async () => {
  enCours.value = true
  const result = await listerHistoriqueModeration(props.salleId, page.value, limit)
  if (result) {
    evenements.value = result.data
    total.value = result.total
  }
  enCours.value = false
}

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / limit)))
const formaterDate = (iso: string) =>
  new Date(iso).toLocaleString('fr-FR', {
    dateStyle: 'medium',
    timeStyle: 'short',
  })

const aller = (p: number) => {
  if (p < 1 || p > totalPages.value) return
  page.value = p
  charger()
}

onMounted(charger)
watch(() => props.salleId, charger)
</script>

<template>
  <div class="card bg-base-100 shadow">
    <div class="card-body">
      <h2 class="card-title">
        Historique de modération
      </h2>

      <div v-if="enCours" class="py-8 text-center">
        <span class="loading loading-spinner" />
      </div>
      <div
        v-else-if="evenements.length === 0"
        class="py-8 text-center text-sm text-base-content/60"
      >
        Aucune action de modération enregistrée sur cette salle.
      </div>

      <div v-else class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>Date</th>
              <th>Administrateur</th>
              <th>Action</th>
              <th>Motif</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="evt in evenements" :key="evt.id">
              <td class="whitespace-nowrap">
                {{ formaterDate(evt.created_at) }}
              </td>
              <td>
                {{ evt.admin_prenom ?? '' }} {{ evt.admin_nom ?? '—' }}
              </td>
              <td>
                <span
                  v-if="evt.type_action === 'fermeture_admin'"
                  class="badge badge-error gap-1"
                >
                  Fermeture
                </span>
                <span v-else class="badge badge-success gap-1">
                  Réactivation
                </span>
              </td>
              <td class="max-w-md">
                <span
                  v-if="evt.motif"
                  class="line-clamp-2 cursor-help"
                  :title="evt.motif"
                >{{ evt.motif }}</span>
                <span v-else class="text-base-content/40">—</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div v-if="totalPages > 1" class="mt-4 flex justify-center gap-2">
        <button
          class="btn btn-sm"
          :disabled="page <= 1"
          @click="aller(page - 1)"
        >
          ←
        </button>
        <span class="self-center text-sm">{{ page }} / {{ totalPages }}</span>
        <button
          class="btn btn-sm"
          :disabled="page >= totalPages"
          @click="aller(page + 1)"
        >
          →
        </button>
      </div>
    </div>
  </div>
</template>
