<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const route = useRoute()
const id = route.params.id as string
const { programmationDetail, chargerDetail, modifier, loading, error } = useAdminProgrammations()
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const form = reactive({
  centre_culturel_id: '',
  titre: '',
  description: '',
  lieu: '',
  mode: 'presentiel',
  lien_en_ligne: '',
  date_heure_debut: '',
  date_heure_fin: '',
  nombre_places: null as number | null,
})

const toDatetimeLocal = (isoDate: string | null): string => {
  if (!isoDate) return ''
  const d = new Date(isoDate)
  const pad = (n: number) => String(n).padStart(2, '0')
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`
}

const toRFC3339 = (datetimeLocal: string): string => {
  if (!datetimeLocal) return ''
  return new Date(datetimeLocal).toISOString()
}

const charger = async () => {
  await chargerDetail(id)
  if (programmationDetail.value) {
    const p = programmationDetail.value
    form.centre_culturel_id = p.centre_culturel_id
    form.titre = p.titre
    form.description = p.description || ''
    form.lieu = p.lieu || ''
    form.mode = p.mode || 'presentiel'
    form.lien_en_ligne = p.lien_en_ligne || ''
    form.date_heure_debut = toDatetimeLocal(p.date_heure_debut)
    form.date_heure_fin = toDatetimeLocal(p.date_heure_fin)
    form.nombre_places = p.nombre_places
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = {
      centre_culturel_id: form.centre_culturel_id,
      titre: form.titre.trim(),
      mode: form.mode,
      date_heure_debut: toRFC3339(form.date_heure_debut),
    }
    if (form.description.trim()) body.description = form.description.trim()
    if (form.lieu.trim()) body.lieu = form.lieu.trim()
    if ((form.mode === 'en_ligne' || form.mode === 'hybride') && form.lien_en_ligne.trim()) {
      body.lien_en_ligne = form.lien_en_ligne.trim()
    }
    if (form.date_heure_fin) body.date_heure_fin = toRFC3339(form.date_heure_fin)
    if (form.nombre_places !== null) body.nombre_places = form.nombre_places
    await modifier(id, body)
    successMsg.value = 'Programmation mise a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    saving.value = false
  }
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="programmationDetail?.titre || 'Chargement...'" sous-titre="Modifier la programmation">
      <template #actions>
        <NuxtLink to="/admin/programmations" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !programmationDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="programmationDetail" class="space-y-4">
      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div v-if="erreurLocale || error" class="alert alert-error mb-4">
            <font-awesome-icon icon="circle-exclamation" />
            <span>{{ erreurLocale || error }}</span>
          </div>
          <div v-if="successMsg" class="alert alert-success mb-4">
            <font-awesome-icon icon="circle-check" />
            <span>{{ successMsg }}</span>
          </div>

          <div v-if="programmationDetail.centre_nom" class="mb-4">
            <span class="badge badge-info badge-sm">
              <font-awesome-icon icon="building" class="mr-1" />
              Centre : {{ programmationDetail.centre_nom }}
            </span>
          </div>

          <form @submit.prevent="sauvegarder" class="space-y-4">
            <div class="form-control">
              <label class="label"><span class="label-text">ID centre culturel (UUID) *</span></label>
              <input v-model="form.centre_culturel_id" type="text" class="input input-bordered" required placeholder="UUID du centre culturel">
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Titre *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Lieu</span></label>
                <input v-model="form.lieu" type="text" class="input input-bordered" placeholder="Adresse ou nom du lieu">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Mode *</span></label>
                <select v-model="form.mode" class="select select-bordered">
                  <option value="presentiel">Presentiel</option>
                  <option value="en_ligne">En ligne</option>
                  <option value="hybride">Hybride</option>
                </select>
              </div>
            </div>

            <div v-if="form.mode === 'en_ligne' || form.mode === 'hybride'" class="form-control">
              <label class="label"><span class="label-text">Lien en ligne</span></label>
              <input v-model="form.lien_en_ligne" type="url" class="input input-bordered" placeholder="https://...">
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Date et heure de debut *</span></label>
                <input v-model="form.date_heure_debut" type="datetime-local" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Date et heure de fin</span></label>
                <input v-model="form.date_heure_fin" type="datetime-local" class="input input-bordered">
              </div>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Nombre de places</span></label>
              <input v-model.number="form.nombre_places" type="number" class="input input-bordered" min="0" placeholder="Illimite si vide">
            </div>

            <div class="flex items-center justify-between pt-4">
              <NuxtLink to="/admin/programmations" class="btn btn-ghost">Retour a la liste</NuxtLink>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>
