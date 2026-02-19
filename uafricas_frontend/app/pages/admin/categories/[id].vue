<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const route = useRoute()
const id = route.params.id as string
const { categorieDetail, chargerDetail, modifier, loading, error } = useAdminCategories()
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const form = reactive({ nom: '', contexte: '', parent_id: '', description: '', icone: '', ordre: 0, actif: true })
const charger = async () => {
  await chargerDetail(id)
  if (categorieDetail.value) { const c = categorieDetail.value; form.nom = c.nom; form.contexte = c.contexte || ''; form.parent_id = c.parent_id || ''; form.description = c.description || ''; form.icone = c.icone || ''; form.ordre = c.ordre || 0; form.actif = c.actif }
}
const sauvegarder = async () => {
  saving.value = true; erreurLocale.value = null; successMsg.value = null
  try { const body: any = { ...form }; if (!body.parent_id) delete body.parent_id; await modifier(id, body); successMsg.value = 'Categorie mise a jour'; setTimeout(() => { successMsg.value = null }, 3000) }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { saving.value = false }
}
onMounted(() => charger())
</script>
<template>
  <div>
    <AdminPageHeader :titre="categorieDetail?.nom || 'Chargement...'" sous-titre="Modifier la categorie">
      <template #actions><NuxtLink to="/admin/categories" class="btn btn-ghost btn-sm"><font-awesome-icon icon="arrow-left" class="mr-1" /> Retour</NuxtLink></template>
    </AdminPageHeader>
    <div v-if="loading && !categorieDetail" class="flex justify-center py-12"><span class="loading loading-spinner loading-lg" /></div>
    <div v-else-if="categorieDetail" class="space-y-4">
      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div v-if="erreurLocale || error" class="alert alert-error mb-4"><font-awesome-icon icon="circle-exclamation" /><span>{{ erreurLocale || error }}</span></div>
          <div v-if="successMsg" class="alert alert-success mb-4"><font-awesome-icon icon="circle-check" /><span>{{ successMsg }}</span></div>
          <form @submit.prevent="sauvegarder" class="space-y-4">
            <div class="form-control"><label class="label"><span class="label-text">Nom *</span></label><input v-model="form.nom" type="text" class="input input-bordered" required></div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control"><label class="label"><span class="label-text">Contexte</span></label>
                <select v-model="form.contexte" class="select select-bordered"><option value="">Non specifie</option><option value="annonce">Annonce</option><option value="livre">Livre</option><option value="radio">Radio</option><option value="television">Television</option><option value="evenement">Evenement</option><option value="formation">Formation</option><option value="projet">Projet</option></select>
              </div>
              <div class="form-control"><label class="label"><span class="label-text">Ordre</span></label><input v-model.number="form.ordre" type="number" class="input input-bordered" min="0"></div>
            </div>
            <div class="form-control"><label class="label"><span class="label-text">ID categorie parente (UUID)</span></label><input v-model="form.parent_id" type="text" class="input input-bordered" placeholder="Vide = racine"></div>
            <div class="form-control"><label class="label"><span class="label-text">Icone</span></label><input v-model="form.icone" type="text" class="input input-bordered"></div>
            <div class="form-control"><label class="label"><span class="label-text">Description</span></label><textarea v-model="form.description" class="textarea textarea-bordered" rows="3" /></div>
            <div class="form-control"><label class="label cursor-pointer justify-start gap-3"><input v-model="form.actif" type="checkbox" class="toggle toggle-success" /><span class="label-text">Categorie active</span></label></div>
            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">Slug: {{ categorieDetail.slug }}</div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving"><font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer</button>
            </div>
          </form>
        </div>
      </div>
      <div v-if="categorieDetail.enfants && categorieDetail.enfants.length > 0" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="card-title text-base"><font-awesome-icon icon="sitemap" class="mr-2" />Sous-categories ({{ categorieDetail.enfants.length }})</h3>
          <div class="overflow-x-auto">
            <table class="table table-sm">
              <thead><tr><th>Nom</th><th>Slug</th><th class="text-center">Ordre</th><th class="text-center">Actif</th><th></th></tr></thead>
              <tbody>
                <tr v-for="enfant in categorieDetail.enfants" :key="enfant.id">
                  <td class="font-medium">{{ enfant.nom }}</td><td class="text-base-content/60">{{ enfant.slug }}</td><td class="text-center">{{ enfant.ordre }}</td>
                  <td class="text-center"><span :class="enfant.actif ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">{{ enfant.actif ? 'Oui' : 'Non' }}</span></td>
                  <td><NuxtLink :to="`/admin/categories/${enfant.id}`" class="btn btn-ghost btn-xs"><font-awesome-icon icon="pen-to-square" /></NuxtLink></td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
