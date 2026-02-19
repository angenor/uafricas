<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const { partenariatDetail, chargerDetail, modifier, loading, error } = useAdminPartenariats()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive({
  type_partenariat: '',
  description: '',
  date_debut: '',
  date_fin: '',
  actif: true,
})

const charger = async () => {
  await chargerDetail(id)
  if (partenariatDetail.value) {
    const p = partenariatDetail.value
    form.type_partenariat = p.type_partenariat || ''
    form.description = p.description || ''
    form.date_debut = p.date_debut || ''
    form.date_fin = p.date_fin || ''
    form.actif = p.actif
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    await modifier(id, { ...form })
    successMsg.value = 'Partenariat mis a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
  finally { saving.value = false }
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="partenariatDetail ? `Partenariat — ${partenariatDetail.organisation_denomination}` : 'Chargement...'" sous-titre="Modifier le partenariat">
      <template #actions>
        <NuxtLink to="/admin/partenariats" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !partenariatDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="partenariatDetail" class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4">
          <font-awesome-icon icon="circle-exclamation" /><span>{{ erreurLocale || error }}</span>
        </div>
        <div v-if="successMsg" class="alert alert-success mb-4">
          <font-awesome-icon icon="circle-check" /><span>{{ successMsg }}</span>
        </div>

        <form @submit.prevent="sauvegarder" class="space-y-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Organisation</span></label>
            <input type="text" class="input input-bordered" :value="partenariatDetail.organisation_denomination" disabled>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Type</span></label>
              <select v-model="form.type_partenariat" class="select select-bordered">
                <option value="">Non specifie</option>
                <option value="Sponsor">Sponsor</option>
                <option value="Contributeur">Contributeur</option>
                <option value="Associe">Associe</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-2">
                <input v-model="form.actif" type="checkbox" class="toggle toggle-success">
                <span class="label-text">{{ form.actif ? 'Actif' : 'Inactif' }}</span>
              </label>
            </div>
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Description</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Date de debut</span></label>
              <input v-model="form.date_debut" type="date" class="input input-bordered">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Date de fin</span></label>
              <input v-model="form.date_fin" type="date" class="input input-bordered">
            </div>
          </div>
          <div class="flex items-center justify-between pt-4">
            <div v-if="partenariatDetail.approuve_par_nom" class="text-sm text-base-content/50">
              Approuve par {{ partenariatDetail.approuve_par_nom }}
            </div>
            <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
              <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
