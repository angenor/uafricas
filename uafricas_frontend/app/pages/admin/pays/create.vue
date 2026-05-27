<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminPays()
const router = useRouter()

const form = reactive({
  nom: '',
  code_iso2: '',
  code_iso3: '',
  indicatif_tel: '',
  capitale: '',
  continent: '',
  longitude: null as number | null,
  latitude: null as number | null,
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.nom.trim()) {
    erreurLocale.value = 'Le nom du territoire est requis'
    return
  }
  try {
    const body: any = { nom: form.nom.trim() }
    if (form.code_iso2.trim()) body.code_iso2 = form.code_iso2.trim()
    if (form.code_iso3.trim()) body.code_iso3 = form.code_iso3.trim()
    if (form.indicatif_tel.trim()) body.indicatif_tel = form.indicatif_tel.trim()
    if (form.capitale.trim()) body.capitale = form.capitale.trim()
    if (form.continent.trim()) body.continent = form.continent.trim()
    if (form.longitude !== null) body.longitude = form.longitude
    if (form.latitude !== null) body.latitude = form.latitude
    await creer(body)
    router.push('/admin/pays')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau territoire" sous-titre="Ajouter un territoire de reference">
      <template #actions>
        <NuxtLink to="/admin/pays" class="btn btn-ghost btn-sm">
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
            <label class="label"><span class="label-text">Nom du territoire *</span></label>
            <input v-model="form.nom" type="text" class="input input-bordered" required>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Code ISO2</span></label>
              <input v-model="form.code_iso2" type="text" class="input input-bordered" maxlength="2" placeholder="Ex: CM">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Code ISO3</span></label>
              <input v-model="form.code_iso3" type="text" class="input input-bordered" maxlength="3" placeholder="Ex: CMR">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Indicatif tel.</span></label>
              <input v-model="form.indicatif_tel" type="text" class="input input-bordered" placeholder="Ex: +237">
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Capitale</span></label>
              <input v-model="form.capitale" type="text" class="input input-bordered">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Continent</span></label>
              <select v-model="form.continent" class="select select-bordered">
                <option value="">Non specifie</option>
                <option value="Afrique">Afrique</option>
                <option value="Europe">Europe</option>
                <option value="Amerique">Amerique</option>
                <option value="Asie">Asie</option>
                <option value="Oceanie">Oceanie</option>
              </select>
            </div>
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
            <NuxtLink to="/admin/pays" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
