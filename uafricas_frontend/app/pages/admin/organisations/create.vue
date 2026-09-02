<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminOrganisations()
const router = useRouter()

const form = reactive({
  denomination: '',
  type_organisation: '',
  email: '',
  telephone: '',
  ville: '',
  description: '',
  numero_registre: '',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.denomination.trim()) {
    erreurLocale.value = 'Denomination requise'
    return
  }
  try {
    const body: any = { denomination: form.denomination.trim() }
    if (form.type_organisation) body.type_organisation = form.type_organisation
    if (form.email.trim()) body.email = form.email.trim()
    if (form.telephone.trim()) body.telephone = form.telephone.trim()
    if (form.ville.trim()) body.ville = form.ville.trim()
    if (form.description.trim()) body.description = form.description.trim()
    if (form.numero_registre.trim()) body.numero_registre = form.numero_registre.trim()
    await creer(body)
    router.push('/admin/organisations')
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle organisation" sous-titre="Enregistrer une organisation partenaire">
      <template #actions>
        <NuxtLink to="/admin/organisations" class="btn btn-ghost btn-sm">
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
            <label class="label"><span class="label-text">Denomination *</span></label>
            <input v-model="form.denomination" type="text" class="input input-bordered" required>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Type d'organisation</span></label>
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
              <label class="label"><span class="label-text">Ville</span></label>
              <input v-model="form.ville" type="text" class="input input-bordered">
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
            <label class="label"><span class="label-text">Description</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Numéro de registre</span></label>
            <input v-model="form.numero_registre" type="text" class="input input-bordered">
          </div>
          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/organisations" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
