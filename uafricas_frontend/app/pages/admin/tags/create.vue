<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const { creer, loading, error } = useAdminTags()
const router = useRouter()
const form = reactive({ nom: '' })
const erreurLocale = ref<string | null>(null)
const soumettre = async () => {
  erreurLocale.value = null
  if (!form.nom.trim()) { erreurLocale.value = 'Le nom du tag est requis'; return }
  try { await creer({ nom: form.nom.trim() }); router.push('/admin/tags') }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation' }
}
</script>
<template>
  <div>
    <AdminPageHeader titre="Nouveau tag" sous-titre="Ajouter un tag">
      <template #actions><NuxtLink to="/admin/tags" class="btn btn-ghost btn-sm"><font-awesome-icon icon="arrow-left" class="mr-1" /> Retour</NuxtLink></template>
    </AdminPageHeader>
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4"><font-awesome-icon icon="circle-exclamation" /><span>{{ erreurLocale || error }}</span></div>
        <form @submit.prevent="soumettre" class="space-y-4">
          <div class="form-control"><label class="label"><span class="label-text">Nom du tag *</span></label><input v-model="form.nom" type="text" class="input input-bordered" required placeholder="Ex: developpement-durable"></div>
          <p class="text-sm text-base-content/50">Le slug sera genere automatiquement a partir du nom.</p>
          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/tags" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading"><font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
