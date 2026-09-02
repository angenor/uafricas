<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const { roleDetail, chargerDetail, modifier, chargerPermissions, permissions, assignerPermissions, loading, error } = useAdminRoles()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive({
  nom: '',
  description: '',
})

const selectedPermIds = ref<string[]>([])

const charger = async () => {
  await Promise.all([chargerDetail(id), chargerPermissions()])
  if (roleDetail.value) {
    form.nom = roleDetail.value.nom
    form.description = roleDetail.value.description || ''
    selectedPermIds.value = roleDetail.value.permissions.map(p => p.id)
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    // Modifier les infos du role
    if (!roleDetail.value?.est_systeme) {
      await modifier(id, {
        nom: form.nom.trim() || undefined,
        description: form.description.trim() || undefined,
      })
    }
    // Synchroniser les permissions
    await assignerPermissions(id, selectedPermIds.value)

    successMsg.value = 'Role mis a jour avec succes'
    setTimeout(() => { successMsg.value = null }, 3000)
    await chargerDetail(id)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
  finally { saving.value = false }
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="roleDetail ? roleDetail.nom : 'Chargement...'" sous-titre="Modifier le role et ses permissions">
      <template #actions>
        <NuxtLink to="/admin/roles" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !roleDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="roleDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-3 mb-6">
        <div>
          <h2 class="text-lg font-bold">{{ roleDetail.nom }}</h2>
          <p class="text-sm text-base-content/60">
            <code>{{ roleDetail.slug }}</code>
            <span class="mx-2">-</span>
            {{ roleDetail.nombre_utilisateurs }} utilisateur(s)
          </p>
          <div class="flex gap-2 mt-1">
            <span v-if="roleDetail.est_systeme" class="badge badge-warning badge-sm">
              <font-awesome-icon icon="lock" class="mr-1" /> Système
            </span>
          </div>
        </div>
      </div>

      <!-- Alertes -->
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" /><span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null"><font-awesome-icon icon="xmark" /></button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" /><span>{{ successMsg }}</span>
      </div>

      <div class="card bg-base-100 shadow-sm mb-6">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-6">
            <!-- Infos role (desactive si systeme) -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Nom</span></label>
                <input v-model="form.nom" type="text" class="input input-bordered" :disabled="roleDetail.est_systeme">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <input v-model="form.description" type="text" class="input input-bordered" :disabled="roleDetail.est_systeme">
              </div>
            </div>

            <!-- Matrice permissions -->
            <div>
              <h3 class="font-semibold mb-3">
                <font-awesome-icon icon="key" class="mr-1" />
                Permissions ({{ selectedPermIds.length }} selectionnee{{ selectedPermIds.length > 1 ? 's' : '' }})
              </h3>
              <AdminPermissionMatrix
                :permissions="permissions"
                v-model:selected-ids="selectedPermIds"
              />
            </div>

            <div class="flex justify-end pt-2">
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
