<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const { creer, loading, error } = useAdminCategories()
const router = useRouter()
const form = reactive({ nom: '', contexte: '', parent_id: '', description: '', icone: '', ordre: 0 })
const erreurLocale = ref<string | null>(null)
const soumettre = async () => {
  erreurLocale.value = null
  if (!form.nom.trim()) { erreurLocale.value = 'Le nom de la categorie est requis'; return }
  try {
    const body: any = { nom: form.nom.trim(), ordre: form.ordre }
    if (form.contexte) body.contexte = form.contexte
    if (form.parent_id.trim()) body.parent_id = form.parent_id.trim()
    if (form.description.trim()) body.description = form.description.trim()
    if (form.icone.trim()) body.icone = form.icone.trim()
    await creer(body)
    router.push('/admin/categories')
  } catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation' }
}
</script>
<template>
  <div>
    <AdminPageHeader titre="Nouvelle catégorie" sous-titre="Ajouter une catégorie">
      <template #actions><NuxtLink to="/admin/categories" class="btn btn-ghost btn-sm"><font-awesome-icon icon="arrow-left" class="mr-1" /> Retour</NuxtLink></template>
    </AdminPageHeader>
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4"><font-awesome-icon icon="circle-exclamation" /><span>{{ erreurLocale || error }}</span></div>
        <form @submit.prevent="soumettre" class="space-y-4">
          <div class="form-control"><label class="label"><span class="label-text">Nom de la catégorie *</span></label><input v-model="form.nom" type="text" class="input input-bordered" required></div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control"><label class="label"><span class="label-text">Contexte</span></label>
              <select v-model="form.contexte" class="select select-bordered"><option value="">Non specifie</option><option value="annonce">Annonce</option><option value="livre">Livre</option><option value="radio">Radio</option><option value="television">Television</option><option value="evenement">Événement</option><option value="formation">Formation</option><option value="projet">Projet</option></select>
            </div>
            <div class="form-control"><label class="label"><span class="label-text">Ordre d'affichage</span></label><input v-model.number="form.ordre" type="number" class="input input-bordered" min="0"></div>
          </div>
          <div class="form-control"><label class="label"><span class="label-text">ID catégorie parente (UUID)</span></label><input v-model="form.parent_id" type="text" class="input input-bordered" placeholder="Laisser vide pour une catégorie racine"></div>
          <div class="form-control"><label class="label"><span class="label-text">Icone</span></label><input v-model="form.icone" type="text" class="input input-bordered" placeholder="Ex: fa-tag"></div>
          <div class="form-control"><label class="label"><span class="label-text">Description</span></label><textarea v-model="form.description" class="textarea textarea-bordered" rows="3" /></div>
          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/categories" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading"><font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
