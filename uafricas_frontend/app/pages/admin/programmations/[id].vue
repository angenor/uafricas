<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const route = useRoute()
const id = route.params.id as string
const { programmationDetail, chargerDetail, modifier, listerInscriptions, loading, error } = useAdminProgrammations()
const saving = ref(false)
const inscriptions = ref<import('~/types/admin').AdminProgrammationInscription[]>([])
const inscriptionsLoading = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const form = reactive({
  centre_culturel_id: '',
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
    form.image_couverture_url = p.image_couverture_url || ''
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
    body.image_couverture_url = form.image_couverture_url.trim()
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

const chargerInscriptions = async () => {
  inscriptionsLoading.value = true
  try {
    inscriptions.value = await listerInscriptions(id)
  } finally {
    inscriptionsLoading.value = false
  }
}

onMounted(async () => {
  await charger()
  await chargerInscriptions()
})
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

            <div class="flex items-center justify-between pt-4">
              <NuxtLink to="/admin/programmations" class="btn btn-ghost">Retour a la liste</NuxtLink>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Inscrits -->
      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div class="flex items-center justify-between mb-2">
            <h3 class="font-semibold">
              <font-awesome-icon icon="users" class="mr-1" />
              Inscrits
              <span class="badge badge-neutral badge-sm ml-1">{{ inscriptions.length }}</span>
            </h3>
            <button class="btn btn-ghost btn-xs" :disabled="inscriptionsLoading" @click="chargerInscriptions">
              <font-awesome-icon icon="arrows-rotate" :class="{ 'animate-spin': inscriptionsLoading }" />
            </button>
          </div>

          <div v-if="inscriptionsLoading" class="flex justify-center py-8">
            <span class="loading loading-spinner loading-md" />
          </div>

          <div v-else class="overflow-x-auto">
            <table class="table table-sm">
              <thead>
                <tr>
                  <th>Nom / Prenom</th>
                  <th>Titre</th>
                  <th>Pays</th>
                  <th>Lieu de residence</th>
                  <th>Email</th>
                  <th class="w-28">Statut</th>
                  <th class="w-36">Date inscription</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="ins in inscriptions" :key="ins.id">
                  <td>{{ ins.prenom }} {{ ins.nom }}</td>
                  <td>{{ ins.titre || '-' }}</td>
                  <td>{{ ins.pays || '-' }}</td>
                  <td>{{ ins.lieu_residence || '-' }}</td>
                  <td>{{ ins.email }}</td>
                  <td><span class="badge badge-sm badge-outline">{{ ins.statut }}</span></td>
                  <td>{{ new Date(ins.created_at).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' }) }}</td>
                </tr>
                <tr v-if="!inscriptions.length">
                  <td colspan="7" class="text-center text-base-content/50 py-4">Aucun inscrit pour le moment</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
