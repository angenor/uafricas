<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminInnovations()
const router = useRouter()

const form = reactive({
  titre: '',
  description: '',
  image_couverture_url: '',
  domaine_id: '',
  organisation_id: '',
  pays_id: '',
  ville: '',
  etat: 'brouillon',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre est requis'
    return
  }
  try {
    const body: any = {
      titre: form.titre.trim(),
      etat: form.etat,
    }
    if (form.description.trim()) body.description = form.description.trim()
    if (form.image_couverture_url.trim()) body.image_couverture_url = form.image_couverture_url.trim()
    if (form.domaine_id) body.domaine_id = form.domaine_id
    if (form.organisation_id) body.organisation_id = form.organisation_id
    if (form.pays_id) body.pays_id = form.pays_id
    if (form.ville.trim()) body.ville = form.ville.trim()
    await creer(body)
    router.push('/admin/innovations')
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle innovation" sous-titre="Ajouter une innovation">
      <template #actions>
        <NuxtLink to="/admin/innovations" class="btn btn-ghost btn-sm">
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
            <label class="label"><span class="label-text">Titre *</span></label>
            <input v-model="form.titre" type="text" class="input input-bordered" required>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Description</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="4" />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">URL image de couverture</span></label>
            <input v-model="form.image_couverture_url" type="text" class="input input-bordered" placeholder="https://...">
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Ville</span></label>
              <input v-model="form.ville" type="text" class="input input-bordered">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Etat</span></label>
              <select v-model="form.etat" class="select select-bordered">
                <option value="brouillon">Brouillon</option>
                <option value="publie">Publie</option>
                <option value="suspendu">Suspendu</option>
              </select>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/innovations" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
