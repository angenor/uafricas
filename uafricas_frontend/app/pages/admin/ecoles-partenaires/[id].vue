<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const { ecoleDetail, loading, error, chargerDetail, modifier } = useAdminEcolesPartenaires()
const { listerPays } = useCentresCulturels()

const paysListe = ref<{ id: string; nom: string }[]>([])
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive({
  nom: '',
  ville: '',
  pays_id: '',
  type: 'publique' as 'publique' | 'privee',
  site_web: '',
  email_contact: '',
  telephone_contact: '',
  whatsapp_contact: '',
  actif: true,
})

const charger = async () => {
  await chargerDetail(id)
  if (ecoleDetail.value) {
    const e = ecoleDetail.value
    form.nom = e.nom
    form.ville = e.ville
    form.pays_id = e.pays_id
    form.type = e.type
    form.site_web = e.site_web || ''
    form.email_contact = e.email_contact
    form.telephone_contact = e.telephone_contact || ''
    form.whatsapp_contact = e.whatsapp_contact || ''
    form.actif = e.actif
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    await modifier(id, {
      nom: form.nom.trim(),
      ville: form.ville.trim(),
      pays_id: form.pays_id,
      type: form.type,
      site_web: form.site_web.trim(),
      email_contact: form.email_contact.trim(),
      telephone_contact: form.telephone_contact.trim(),
      whatsapp_contact: form.whatsapp_contact.trim(),
      actif: form.actif,
    })
    successMsg.value = 'École partenaire mise à jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

onMounted(async () => {
  paysListe.value = await listerPays()
  await charger()
})
</script>

<template>
  <div>
    <AdminPageHeader :titre="ecoleDetail?.nom || 'Chargement...'" sous-titre="Édition de l'école partenaire">
      <template #actions>
        <NuxtLink to="/admin/ecoles-partenaires" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !ecoleDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="ecoleDetail">
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">
          <font-awesome-icon icon="xmark" />
        </button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Nom de l'établissement *</span></label>
              <input v-model="form.nom" type="text" class="input input-bordered" required>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Territoire *</span></label>
                <select v-model="form.pays_id" class="select select-bordered" required>
                  <option value="" disabled>Sélectionner un territoire</option>
                  <option v-for="p in paysListe" :key="p.id" :value="p.id">{{ p.nom }}</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Ville *</span></label>
                <input v-model="form.ville" type="text" class="input input-bordered" required>
              </div>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Type d'établissement *</span></label>
              <select v-model="form.type" class="select select-bordered">
                <option value="publique">Public</option>
                <option value="privee">Privé</option>
              </select>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Email de contact *</span></label>
                <input v-model="form.email_contact" type="email" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Site web</span></label>
                <input v-model="form.site_web" type="url" class="input input-bordered" placeholder="https://...">
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Téléphone</span></label>
                <input v-model="form.telephone_contact" type="text" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">WhatsApp</span></label>
                <input v-model="form.whatsapp_contact" type="text" class="input input-bordered">
              </div>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="form.actif" type="checkbox" class="toggle toggle-success" />
                <span class="label-text">École active</span>
              </label>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span :class="ecoleDetail.actif ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">
                  {{ ecoleDetail.actif ? 'Active' : 'Inactive' }}
                </span>
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>
    </template>
  </div>
</template>
