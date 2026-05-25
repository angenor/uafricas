<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const { paysDetail, chargerDetail, modifier, loading, error } = useAdminPays()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive({
  nom: '',
  code_iso2: '',
  code_iso3: '',
  indicatif_tel: '',
  capitale: '',
  continent: '',
  longitude: null as number | null,
  latitude: null as number | null,
  actif: true,
})

const charger = async () => {
  await chargerDetail(id)
  if (paysDetail.value) {
    const p = paysDetail.value
    form.nom = p.nom
    form.code_iso2 = p.code_iso2 || ''
    form.code_iso3 = p.code_iso3 || ''
    form.indicatif_tel = p.indicatif_tel || ''
    form.capitale = p.capitale || ''
    form.continent = p.continent || ''
    form.longitude = p.longitude
    form.latitude = p.latitude
    form.actif = p.actif
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    await modifier(id, { ...form })
    successMsg.value = 'Territoire mis a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally { saving.value = false }
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="paysDetail?.nom || 'Chargement...'" sous-titre="Modifier le territoire">
      <template #actions>
        <NuxtLink to="/admin/pays" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !paysDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="paysDetail" class="card bg-base-100 shadow-sm">
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
            <label class="label"><span class="label-text">Nom du territoire *</span></label>
            <input v-model="form.nom" type="text" class="input input-bordered" required>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Code ISO2</span></label>
              <input v-model="form.code_iso2" type="text" class="input input-bordered" maxlength="2">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Code ISO3</span></label>
              <input v-model="form.code_iso3" type="text" class="input input-bordered" maxlength="3">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Indicatif tel.</span></label>
              <input v-model="form.indicatif_tel" type="text" class="input input-bordered">
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

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input v-model="form.actif" type="checkbox" class="toggle toggle-success" />
              <span class="label-text">Territoire actif</span>
            </label>
          </div>

          <div class="flex items-center justify-between pt-4">
            <div class="text-sm text-base-content/50">
              ID: {{ paysDetail.id.substring(0, 8) }}...
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
