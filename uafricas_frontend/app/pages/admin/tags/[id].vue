<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const route = useRoute()
const id = route.params.id as string
const { tagDetail, chargerDetail, modifier, loading, error } = useAdminTags()
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const form = reactive({ nom: '' })
const charger = async () => { await chargerDetail(id); if (tagDetail.value) form.nom = tagDetail.value.nom }
const sauvegarder = async () => {
  saving.value = true; erreurLocale.value = null; successMsg.value = null
  try { await modifier(id, { nom: form.nom.trim() }); successMsg.value = 'Tag mis a jour'; setTimeout(() => { successMsg.value = null }, 3000) }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { saving.value = false }
}
onMounted(() => charger())
</script>
<template>
  <div>
    <AdminPageHeader :titre="tagDetail?.nom || 'Chargement...'" sous-titre="Modifier le tag">
      <template #actions><NuxtLink to="/admin/tags" class="btn btn-ghost btn-sm"><font-awesome-icon icon="arrow-left" class="mr-1" /> Retour</NuxtLink></template>
    </AdminPageHeader>
    <div v-if="loading && !tagDetail" class="flex justify-center py-12"><span class="loading loading-spinner loading-lg" /></div>
    <div v-else-if="tagDetail" class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4"><font-awesome-icon icon="circle-exclamation" /><span>{{ erreurLocale || error }}</span></div>
        <div v-if="successMsg" class="alert alert-success mb-4"><font-awesome-icon icon="circle-check" /><span>{{ successMsg }}</span></div>
        <form @submit.prevent="sauvegarder" class="space-y-4">
          <div class="form-control"><label class="label"><span class="label-text">Nom du tag *</span></label><input v-model="form.nom" type="text" class="input input-bordered" required></div>
          <div class="flex items-center justify-between pt-4">
            <div class="text-sm text-base-content/50"><span>Slug: {{ tagDetail.slug }}</span><span class="ml-4">Utilisations: {{ tagDetail.nombre_utilisations }}</span></div>
            <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving"><font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
