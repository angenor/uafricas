<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminCentresCulturels()
const router = useRouter()

const form = reactive({
  nom: '',
  description: '',
  pays_id: '',
  ville: '',
  adresse: '',
  longitude: null as number | null,
  latitude: null as number | null,
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.nom.trim()) {
    erreurLocale.value = 'Le nom du centre culturel est requis'
    return
  }
  try {
    const body: any = { nom: form.nom.trim() }
    if (form.description.trim()) body.description = form.description.trim()
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.ville.trim()) body.ville = form.ville.trim()
    if (form.adresse.trim()) body.adresse = form.adresse.trim()
    if (form.longitude !== null) body.longitude = form.longitude
    if (form.latitude !== null) body.latitude = form.latitude
    await creer(body)
    router.push('/admin/centres-culturels')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau centre culturel" sous-titre="Ajouter un centre culturel">
      <template #actions>
        <NuxtLink to="/admin/centres-culturels" class="btn btn-ghost btn-sm">
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
            <label class="label"><span class="label-text">Nom du centre *</span></label>
            <input v-model="form.nom" type="text" class="input input-bordered" required>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Description</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Territoire (UUID)</span></label>
              <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="UUID du territoire">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Ville</span></label>
              <input v-model="form.ville" type="text" class="input input-bordered">
            </div>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Adresse</span></label>
            <textarea v-model="form.adresse" class="textarea textarea-bordered" rows="2" />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Longitude</span></label>
              <input v-model.number="form.longitude" type="number" step="0.0000001" class="input input-bordered">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Latitude</span></label>
              <input v-model.number="form.latitude" type="number" step="0.0000001" class="input input-bordered">
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/centres-culturels" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
