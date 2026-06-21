<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminProgrammations()
const router = useRouter()
const route = useRoute()

const form = reactive({
  centre_culturel_id: (route.query.centre as string) || '',
  titre: '',
  description: '',
  image_couverture_url: '',
  lieu: '',
  mode: 'presentiel',
  lien_en_ligne: '',
  date_heure_debut: '',
  date_heure_fin: '',
  nombre_places: null as number | null,
})

const erreurLocale = ref<string | null>(null)

const toRFC3339 = (datetimeLocal: string): string => {
  if (!datetimeLocal) return ''
  return new Date(datetimeLocal).toISOString()
}

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.centre_culturel_id.trim()) {
    erreurLocale.value = "L'ID du centre culturel est requis"
    return
  }
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre est requis'
    return
  }
  if (!form.date_heure_debut) {
    erreurLocale.value = 'La date de debut est requise'
    return
  }
  try {
    const body: any = {
      centre_culturel_id: form.centre_culturel_id.trim(),
      titre: form.titre.trim(),
      mode: form.mode,
      date_heure_debut: toRFC3339(form.date_heure_debut),
    }
    if (form.description.trim()) body.description = form.description.trim()
    if (form.image_couverture_url.trim()) body.image_couverture_url = form.image_couverture_url.trim()
    if (form.lieu.trim()) body.lieu = form.lieu.trim()
    if ((form.mode === 'en_ligne' || form.mode === 'hybride') && form.lien_en_ligne.trim()) {
      body.lien_en_ligne = form.lien_en_ligne.trim()
    }
    if (form.date_heure_fin) body.date_heure_fin = toRFC3339(form.date_heure_fin)
    if (form.nombre_places !== null) body.nombre_places = form.nombre_places
    await creer(body)
    router.push('/admin/programmations')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle programmation" sous-titre="Ajouter une programmation a un centre culturel">
      <template #actions>
        <NuxtLink to="/admin/programmations" class="btn btn-ghost btn-sm">
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

          <div class="form-control">
            <label class="label"><span class="label-text">Image illustrative (couverture)</span></label>
            <OpportuniteAfriqueImageUploadField
              v-model="form.image_couverture_url"
              label=""
            />
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

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/programmations" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
