<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminSalles()
const router = useRouter()

const form = reactive({
  titre: '',
  description: '',
  langue_cible: '',
  moderateur_id: '',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre de la salle est requis'
    return
  }
  try {
    const body: any = { titre: form.titre.trim() }
    if (form.description.trim()) body.description = form.description.trim()
    if (form.langue_cible.trim()) body.langue_cible = form.langue_cible.trim()
    if (form.moderateur_id.trim()) body.moderateur_id = form.moderateur_id.trim()
    await creer(body)
    router.push('/admin/salles')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle salle" sous-titre="Creer une salle AfroLang">
      <template #actions>
        <NuxtLink to="/admin/salles" class="btn btn-ghost btn-sm">
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
            <label class="label"><span class="label-text">Titre de la salle *</span></label>
            <input v-model="form.titre" type="text" class="input input-bordered" required>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Description</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Langue cible</span></label>
              <input v-model="form.langue_cible" type="text" class="input input-bordered" placeholder="Ex: Swahili, Wolof, Yoruba...">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Moderateur (UUID)</span></label>
              <input v-model="form.moderateur_id" type="text" class="input input-bordered" placeholder="UUID de l'utilisateur moderateur">
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/salles" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
