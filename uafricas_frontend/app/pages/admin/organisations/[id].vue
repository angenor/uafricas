<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const { organisationDetail, chargerDetail, modifier, loading, error } = useAdminOrganisations()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive({
  denomination: '',
  type_organisation: '',
  email: '',
  telephone: '',
  ville: '',
  description: '',
  numero_registre: '',
  etat: '',
})

const charger = async () => {
  await chargerDetail(id)
  if (organisationDetail.value) {
    const o = organisationDetail.value
    form.denomination = o.denomination
    form.type_organisation = o.type_organisation || ''
    form.email = o.email || ''
    form.telephone = o.telephone || ''
    form.ville = o.ville || ''
    form.description = o.description || ''
    form.numero_registre = o.numero_registre || ''
    form.etat = o.etat
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    await modifier(id, { ...form })
    successMsg.value = 'Organisation mise a jour'
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
    <AdminPageHeader :titre="organisationDetail?.denomination || 'Chargement...'" sous-titre="Modifier l'organisation">
      <template #actions>
        <NuxtLink to="/admin/organisations" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !organisationDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="organisationDetail" class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4">
          <font-awesome-icon icon="circle-exclamation" />
          <span>{{ erreurLocale || error }}</span>
        </div>
        <div v-if="successMsg" class="alert alert-success mb-4">
          <font-awesome-icon icon="circle-check" />
          <span>{{ successMsg }}</span>
        </div>

        <form @submit.prevent="sauvegarder" class="space-y-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Denomination</span></label>
            <input v-model="form.denomination" type="text" class="input input-bordered">
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Type</span></label>
              <select v-model="form.type_organisation" class="select select-bordered">
                <option value="">Non specifie</option>
                <option value="ONG">ONG</option>
                <option value="Entreprise">Entreprise</option>
                <option value="Association">Association</option>
                <option value="Cooperative">Cooperative</option>
                <option value="Institution">Institution</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">État</span></label>
              <select v-model="form.etat" class="select select-bordered">
                <option value="actif">Actif</option>
                <option value="en_attente">En attente</option>
                <option value="suspendu">Suspendu</option>
              </select>
            </div>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Email</span></label>
              <input v-model="form.email" type="email" class="input input-bordered">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Téléphone</span></label>
              <input v-model="form.telephone" type="tel" class="input input-bordered">
            </div>
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Ville</span></label>
            <input v-model="form.ville" type="text" class="input input-bordered">
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Description</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
          </div>

          <div class="flex items-center justify-between pt-4">
            <div class="text-sm text-base-content/50">
              <span v-if="organisationDetail.cree_par_nom">Cree par {{ organisationDetail.cree_par_nom }}</span>
              <span class="ml-2">{{ organisationDetail.nombre_membres }} membre(s)</span>
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
