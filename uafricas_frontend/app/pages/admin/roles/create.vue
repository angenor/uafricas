<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, chargerPermissions, permissions, assignerPermissions, loading, error } = useAdminRoles()
const router = useRouter()

const form = reactive({
  nom: '',
  description: '',
})

const selectedPermIds = ref<string[]>([])
const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.nom.trim()) {
    erreurLocale.value = 'Nom du role requis'
    return
  }
  try {
    const result = await creer({ nom: form.nom.trim(), description: form.description.trim() || undefined })
    if (result?.id && selectedPermIds.value.length > 0) {
      await assignerPermissions(result.id, selectedPermIds.value)
    }
    router.push('/admin/roles')
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}

onMounted(() => chargerPermissions())
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau role" sous-titre="Creer un role et assigner des permissions">
      <template #actions>
        <NuxtLink to="/admin/roles" class="btn btn-ghost btn-sm">
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

        <form @submit.prevent="soumettre" class="space-y-6">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Nom du role *</span></label>
              <input v-model="form.nom" type="text" class="input input-bordered" placeholder="Ex: editeur" required>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <input v-model="form.description" type="text" class="input input-bordered" placeholder="Description du role">
            </div>
          </div>

          <div>
            <h3 class="font-semibold mb-3">Permissions</h3>
            <AdminPermissionMatrix
              :permissions="permissions"
              v-model:selected-ids="selectedPermIds"
            />
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/roles" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer le role
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
