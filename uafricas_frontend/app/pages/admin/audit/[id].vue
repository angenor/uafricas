<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const { auditDetail, chargerDetail, loading, error } = useAdminAudit()

const actionBadge = (action: string) => {
  const map: Record<string, string> = {
    CREATE: 'badge-success',
    UPDATE: 'badge-info',
    DELETE: 'badge-error',
    LOGIN: 'badge-neutral',
  }
  return map[action] || 'badge-neutral'
}

const formatDate = (v: string) => new Date(v).toLocaleString('fr-FR', {
  day: '2-digit', month: '2-digit', year: 'numeric',
  hour: '2-digit', minute: '2-digit', second: '2-digit',
})

onMounted(() => chargerDetail(id))
</script>

<template>
  <div>
    <AdminPageHeader :titre="`Audit #${auditDetail?.id?.slice(0, 8) || '...'}`" sous-titre="Detail de l'evenement">
      <template #actions>
        <NuxtLink to="/admin/audit" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !auditDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="error" class="alert alert-error">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ error }}</span>
    </div>

    <div v-else-if="auditDetail" class="space-y-4">
      <!-- En-tete : action + utilisateur + date -->
      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div class="flex flex-wrap items-center gap-4">
            <span class="badge badge-lg" :class="actionBadge(auditDetail.action)">
              {{ auditDetail.action }}
            </span>
            <div>
              <span class="font-semibold">{{ auditDetail.schema_name }}.{{ auditDetail.table_name }}</span>
              <span v-if="auditDetail.record_id" class="ml-2 font-mono text-xs text-base-content/60">
                {{ auditDetail.record_id }}
              </span>
            </div>
            <div class="ml-auto text-sm text-base-content/60">
              {{ formatDate(auditDetail.created_at) }}
            </div>
          </div>
        </div>
      </div>

      <!-- Infos contextuelles -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Utilisateur -->
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <h3 class="card-title text-sm">
              <font-awesome-icon icon="user" class="text-base-content/50" /> Utilisateur
            </h3>
            <div v-if="auditDetail.utilisateur_nom" class="space-y-1">
              <p class="font-medium">{{ auditDetail.utilisateur_nom }}</p>
              <p v-if="auditDetail.utilisateur_email" class="text-sm text-base-content/60">
                {{ auditDetail.utilisateur_email }}
              </p>
              <p v-if="auditDetail.utilisateur_id" class="font-mono text-xs text-base-content/40">
                {{ auditDetail.utilisateur_id }}
              </p>
            </div>
            <p v-else class="text-base-content/40 italic">Action systeme</p>
          </div>
        </div>

        <!-- Contexte technique -->
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <h3 class="card-title text-sm">
              <font-awesome-icon icon="gear" class="text-base-content/50" /> Contexte technique
            </h3>
            <div class="space-y-1">
              <div class="flex items-center gap-2">
                <span class="text-sm text-base-content/60">IP :</span>
                <span v-if="auditDetail.ip_address" class="font-mono text-sm">{{ auditDetail.ip_address }}</span>
                <span v-else class="text-base-content/40">-</span>
              </div>
              <div>
                <span class="text-sm text-base-content/60">User Agent :</span>
                <p v-if="auditDetail.user_agent" class="font-mono text-xs text-base-content/70 mt-1 break-all">
                  {{ auditDetail.user_agent }}
                </p>
                <span v-else class="text-base-content/40">-</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Diff JSON before/after -->
      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="card-title text-sm mb-4">
            <font-awesome-icon icon="arrows-rotate" class="text-base-content/50" /> Changements (before / after)
          </h3>
          <AdminJsonDiff
            :ancien-etat="auditDetail.ancien_etat"
            :nouvel-etat="auditDetail.nouvel_etat"
          />
        </div>
      </div>

      <!-- Donnees brutes (collapsed) -->
      <details class="card bg-base-100 shadow-sm">
        <summary class="card-body cursor-pointer py-3">
          <h3 class="card-title text-sm">
            <font-awesome-icon icon="database" class="text-base-content/50" /> Donnees brutes (JSON)
          </h3>
        </summary>
        <div class="card-body pt-0">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <h4 class="text-sm font-semibold mb-2 text-error">Avant (ancien_etat)</h4>
              <pre v-if="auditDetail.ancien_etat" class="bg-base-200 p-3 rounded-lg text-xs overflow-x-auto max-h-96">{{ JSON.stringify(auditDetail.ancien_etat, null, 2) }}</pre>
              <p v-else class="text-base-content/40 italic">Aucune donnee</p>
            </div>
            <div>
              <h4 class="text-sm font-semibold mb-2 text-success">Apres (nouvel_etat)</h4>
              <pre v-if="auditDetail.nouvel_etat" class="bg-base-200 p-3 rounded-lg text-xs overflow-x-auto max-h-96">{{ JSON.stringify(auditDetail.nouvel_etat, null, 2) }}</pre>
              <p v-else class="text-base-content/40 italic">Aucune donnee</p>
            </div>
          </div>
        </div>
      </details>

      <!-- Metadonnees -->
      <div class="text-sm text-base-content/50 text-right">
        ID : <span class="font-mono">{{ auditDetail.id }}</span>
      </div>
    </div>
  </div>
</template>
