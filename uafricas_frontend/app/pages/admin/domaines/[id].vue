<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const route = useRoute()
const id = route.params.id as string
const { domaineDetail, chargerDetail, modifier, loading, error } = useAdminDomaines()
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const form = reactive({ nom: '', description: '', icone: '', actif: true })
const charger = async () => {
  await chargerDetail(id)
  if (domaineDetail.value) { const d = domaineDetail.value; form.nom = d.nom; form.description = d.description || ''; form.icone = d.icone || ''; form.actif = d.actif }
}
const sauvegarder = async () => {
  saving.value = true; erreurLocale.value = null; successMsg.value = null
  try { await modifier(id, { ...form }); successMsg.value = 'Domaine mis a jour'; setTimeout(() => { successMsg.value = null }, 3000) }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { saving.value = false }
}
onMounted(() => charger())
</script>
<template>
  <div>
    <AdminPageHeader :titre="domaineDetail?.nom || 'Chargement...'" sous-titre="Modifier le domaine">
      <template #actions><NuxtLink to="/admin/domaines" class="btn btn-ghost btn-sm"><font-awesome-icon icon="arrow-left" class="mr-1" /> Retour</NuxtLink></template>
    </AdminPageHeader>
    <div v-if="loading && !domaineDetail" class="flex justify-center py-12"><span class="loading loading-spinner loading-lg" /></div>
    <div v-else-if="domaineDetail" class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4"><font-awesome-icon icon="circle-exclamation" /><span>{{ erreurLocale || error }}</span></div>
        <div v-if="successMsg" class="alert alert-success mb-4"><font-awesome-icon icon="circle-check" /><span>{{ successMsg }}</span></div>
        <form @submit.prevent="sauvegarder" class="space-y-4">
          <div class="form-control"><label class="label"><span class="label-text">Nom du domaine *</span></label><input v-model="form.nom" type="text" class="input input-bordered" required></div>
          <div class="form-control"><label class="label"><span class="label-text">Icone</span></label><input v-model="form.icone" type="text" class="input input-bordered"></div>
          <div class="form-control"><label class="label"><span class="label-text">Description</span></label><textarea v-model="form.description" class="textarea textarea-bordered" rows="3" /></div>
          <div class="form-control"><label class="label cursor-pointer justify-start gap-3"><input v-model="form.actif" type="checkbox" class="toggle toggle-success" /><span class="label-text">Domaine actif</span></label></div>
          <div class="flex items-center justify-between pt-4">
            <div class="text-sm text-base-content/50">Slug: {{ domaineDetail.slug }}</div>
            <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving"><font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
