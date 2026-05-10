<script setup lang="ts">
// Détail + actions admin sur une proposition de salle
// Feature 001-admin-salles-publiques, US2
import type { PropositionSalle } from '~/composables/useAfrolang'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const router = useRouter()
const id = route.params.id as string

const {
  obtenirProposition,
  validerProposition,
  rejeterProposition,
  loading,
  error,
} = useAdminPropositionsSalle()

const proposition = ref<PropositionSalle | null>(null)
const saving = ref(false)
const successMsg = ref<string | null>(null)
const erreurAction = ref<string | null>(null)

const charger = async () => {
  proposition.value = await obtenirProposition(id)
}

const onValider = async (commentaire: string | null) => {
  saving.value = true
  erreurAction.value = null
  try {
    const res = await validerProposition(id, commentaire ?? undefined)
    if (!res) throw new Error('Validation échouée')
    successMsg.value = `Salle créée avec succès (id : ${res.salle_id}).`
    proposition.value = res.proposition
    setTimeout(() => router.push(`/admin/salles/${res.salle_id}`), 1500)
  }
  catch (e: any) {
    erreurAction.value = e?.data?.error || e?.message || 'Erreur de validation'
  }
  finally {
    saving.value = false
  }
}

const onRejeter = async (commentaire: string) => {
  saving.value = true
  erreurAction.value = null
  try {
    const res = await rejeterProposition(id, commentaire)
    if (!res) throw new Error('Rejet échoué')
    proposition.value = res
    successMsg.value = 'Proposition rejetée — l\'auteur a été notifié.'
  }
  catch (e: any) {
    erreurAction.value = e?.data?.error || e?.message || 'Erreur de rejet'
  }
  finally {
    saving.value = false
  }
}

onMounted(charger)
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Proposition de salle"
      sous-titre="Modération communautaire (Afrolang)"
    >
      <template #actions>
        <NuxtLink to="/admin/afrolang/propositions" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="error || erreurAction" class="alert alert-error mb-4">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ erreurAction || error }}</span>
    </div>
    <div v-if="successMsg" class="alert alert-success mb-4">
      <font-awesome-icon icon="circle-check" />
      <span>{{ successMsg }}</span>
    </div>

    <div v-if="loading && !proposition" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <AdminAfrolangPropositionDetail
      v-else-if="proposition"
      :proposition="proposition"
      :saving="saving"
      @valider="onValider"
      @rejeter="onRejeter"
    />
  </div>
</template>
