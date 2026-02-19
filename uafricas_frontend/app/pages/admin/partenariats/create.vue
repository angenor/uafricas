<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminPartenariats()
const router = useRouter()

const form = reactive({
  organisation_id: '',
  type_partenariat: '',
  description: '',
  date_debut: '',
  date_fin: '',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.organisation_id.trim()) {
    erreurLocale.value = 'Organisation requise'
    return
  }
  try {
    const body: any = { organisation_id: form.organisation_id.trim() }
    if (form.type_partenariat) body.type_partenariat = form.type_partenariat
    if (form.description.trim()) body.description = form.description.trim()
    if (form.date_debut) body.date_debut = form.date_debut
    if (form.date_fin) body.date_fin = form.date_fin
    await creer(body)
    router.push('/admin/partenariats')
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau partenariat" sous-titre="Creer un partenariat avec une organisation">
      <template #actions>
        <NuxtLink to="/admin/partenariats" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4">
          <font-awesome-icon icon="circle-exclamation" />
          <span>{{ erreurLocale || error }}</span>
        </div>

        <form @submit.prevent="soumettre" class="space-y-4">
          <div class="form-control">
            <label class="label"><span class="label-text">ID Organisation *</span></label>
            <input v-model="form.organisation_id" type="text" class="input input-bordered" placeholder="UUID de l'organisation" required>
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Type de partenariat</span></label>
            <select v-model="form.type_partenariat" class="select select-bordered">
              <option value="">Non specifie</option>
              <option value="Sponsor">Sponsor</option>
              <option value="Contributeur">Contributeur</option>
              <option value="Associe">Associe</option>
            </select>
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
          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/partenariats" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
