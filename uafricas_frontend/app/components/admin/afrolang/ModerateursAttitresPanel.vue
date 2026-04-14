<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useAdminAfrolangSalles } from '~/composables/useAdminAfrolangSalles'
import type { ModerateurAttitre } from '~/composables/useAfrolang'
import { formatDate } from '~/composables/useAfrolang'

interface Props {
  salleId: string | null
}

const props = defineProps<Props>()

const {
  listerModerateursAttitres,
  designerModerateur,
  retirerModerateur,
} = useAdminAfrolangSalles()

const moderateurs = ref<ModerateurAttitre[]>([])
const chargement = ref(false)
const messageRetour = ref<string | null>(null)

const form = ref({
  utilisateur_id: '',
  disponibilite: '',
})

const charger = async () => {
  if (!props.salleId) {
    moderateurs.value = []
    return
  }
  chargement.value = true
  moderateurs.value = await listerModerateursAttitres(props.salleId)
  chargement.value = false
}

const designer = async () => {
  if (!props.salleId || !form.value.utilisateur_id.trim()) return
  messageRetour.value = null
  const ok = await designerModerateur(props.salleId, {
    utilisateur_id: form.value.utilisateur_id.trim(),
    disponibilite: form.value.disponibilite.trim() || undefined,
  })
  if (ok) {
    messageRetour.value = 'Modérateur désigné.'
    form.value.utilisateur_id = ''
    form.value.disponibilite = ''
    await charger()
  } else {
    messageRetour.value = 'Échec de la désignation.'
  }
}

const retirer = async (utilisateurId: string) => {
  if (!props.salleId) return
  if (!confirm('Retirer ce modérateur attitré ?')) return
  const ok = await retirerModerateur(props.salleId, utilisateurId)
  if (ok) {
    messageRetour.value = 'Modérateur retiré.'
    await charger()
  } else {
    messageRetour.value = 'Échec du retrait.'
  }
}

watch(() => props.salleId, () => charger())
onMounted(() => charger())
</script>

<template>
  <div class="space-y-4">
    <div v-if="!salleId" class="alert alert-info text-sm">
      Sélectionnez une salle publique pour gérer ses modérateurs attitrés.
    </div>

    <div v-else>
      <div class="card bg-base-100 shadow-sm mb-4">
        <div class="card-body">
          <h3 class="card-title text-base mb-2">Désigner un modérateur</h3>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-2">
            <div class="form-control md:col-span-1">
              <label class="label py-1"><span class="label-text text-xs">UUID utilisateur</span></label>
              <input v-model="form.utilisateur_id" type="text" class="input input-bordered input-sm" />
            </div>
            <div class="form-control md:col-span-1">
              <label class="label py-1"><span class="label-text text-xs">Disponibilité</span></label>
              <input v-model="form.disponibilite" type="text" class="input input-bordered input-sm" placeholder="Ex : lun-ven 18h-20h" />
            </div>
            <div class="flex items-end">
              <button
                class="btn btn-primary btn-sm w-full"
                :disabled="!form.utilisateur_id.trim()"
                @click="designer"
              >
                Désigner
              </button>
            </div>
          </div>
          <p v-if="messageRetour" class="text-xs mt-2 opacity-70">{{ messageRetour }}</p>
        </div>
      </div>

      <div v-if="chargement" class="text-center py-6">
        <span class="loading loading-spinner loading-md" />
      </div>

      <div v-else-if="moderateurs.length === 0" class="text-center text-base-content/60 py-6">
        Aucun modérateur attitré pour cette salle.
      </div>

      <div v-else class="overflow-x-auto">
        <table class="table table-sm">
          <thead>
            <tr>
              <th>Utilisateur</th>
              <th>Disponibilité</th>
              <th>Désigné le</th>
              <th>État</th>
              <th class="text-right">Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="m in moderateurs" :key="m.id">
              <td>
                <div class="flex flex-col">
                  <span class="font-medium">{{ m.prenom }} {{ m.nom }}</span>
                  <span class="text-xs opacity-70">{{ m.email || '—' }}</span>
                </div>
              </td>
              <td>{{ m.disponibilite || '—' }}</td>
              <td>{{ formatDate(m.designe_at) }}</td>
              <td>
                <span class="badge" :class="m.actif ? 'badge-success' : 'badge-ghost'">
                  {{ m.actif ? 'Actif' : 'Retiré' }}
                </span>
              </td>
              <td class="text-right">
                <button
                  v-if="m.actif"
                  class="btn btn-xs btn-error"
                  @click="retirer(m.utilisateur_id)"
                >
                  Retirer
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
