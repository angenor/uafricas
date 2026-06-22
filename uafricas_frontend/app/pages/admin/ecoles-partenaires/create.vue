<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminEcolesPartenaires()
const { listerPays } = useCentresCulturels()
const router = useRouter()

const paysListe = ref<{ id: string; nom: string }[]>([])

const form = reactive({
  nom: '',
  ville: '',
  pays_id: '',
  type: 'publique' as 'publique' | 'privee',
  site_web: '',
  email_contact: '',
  telephone_contact: '',
  whatsapp_contact: '',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.nom.trim()) { erreurLocale.value = 'Le nom est requis'; return }
  if (!form.ville.trim()) { erreurLocale.value = 'La ville est requise'; return }
  if (!form.pays_id) { erreurLocale.value = 'Le territoire est requis'; return }
  if (!form.email_contact.trim()) { erreurLocale.value = "L'email de contact est requis"; return }
  try {
    await creer({
      nom: form.nom.trim(),
      ville: form.ville.trim(),
      pays_id: form.pays_id,
      type: form.type,
      site_web: form.site_web.trim(),
      email_contact: form.email_contact.trim(),
      telephone_contact: form.telephone_contact.trim(),
      whatsapp_contact: form.whatsapp_contact.trim(),
    })
    router.push('/admin/ecoles-partenaires')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la création'
  }
}

onMounted(async () => {
  paysListe.value = await listerPays()
})
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle école partenaire" sous-titre="Ajouter un établissement partenaire">
      <template #actions>
        <NuxtLink to="/admin/ecoles-partenaires" class="btn btn-ghost btn-sm">
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

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/ecoles-partenaires" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Créer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
