<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useAdminAfrolangSalles } from '~/composables/useAdminAfrolangSalles'
import { useAdminUtilisateurs } from '~/composables/useAdminUtilisateurs'
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

// Recherche utilisateur (réutilise useAdminUtilisateurs, comme SalleAdministrateursPanel).
const {
  utilisateurs,
  filtres: filtresUsers,
  chargerListe: chargerUtilisateurs,
} = useAdminUtilisateurs()

const moderateurs = ref<ModerateurAttitre[]>([])
const chargement = ref(false)
const messageRetour = ref<string | null>(null)
const saving = ref(false)

const recherche = ref('')
const utilisateurChoisi = ref('')
const disponibilite = ref('')

// Exclure les attitrés DÉJÀ actifs sur cette salle (évite le doublon UNIQUE).
const utilisateursDisponibles = computed(() => {
  const actifs = new Set(
    moderateurs.value.filter(m => m.actif).map(m => m.utilisateur_id),
  )
  return utilisateurs.value.filter(u => !actifs.has(u.id))
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

const rechercherUtilisateurs = async () => {
  filtresUsers.recherche = recherche.value
  await chargerUtilisateurs()
}

const designer = async () => {
  if (!props.salleId || !utilisateurChoisi.value) return
  saving.value = true
  messageRetour.value = null
  const ok = await designerModerateur(props.salleId, {
    utilisateur_id: utilisateurChoisi.value,
    disponibilite: disponibilite.value.trim() || undefined,
  })
  saving.value = false
  if (ok) {
    messageRetour.value = 'Modérateur désigné.'
    utilisateurChoisi.value = ''
    disponibilite.value = ''
    recherche.value = ''
    await charger()
  }
  else {
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
  }
  else {
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
          <p class="text-xs text-base-content/60 mb-2">
            Le modérateur désigné prendra la modération des sessions de cette salle
            (passation par consentement du démarreur, ou automatique après 60 s).
          </p>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
            <div class="form-control md:col-span-2">
              <label class="label py-1"><span class="label-text text-xs">Rechercher un utilisateur (nom / e-mail)</span></label>
              <div class="join">
                <input
                  v-model="recherche"
                  type="text"
                  class="input input-bordered input-sm join-item w-full"
                  placeholder="Rechercher…"
                  @keyup.enter="rechercherUtilisateurs"
                />
                <button class="btn btn-primary btn-sm join-item" @click="rechercherUtilisateurs">
                  <font-awesome-icon icon="magnifying-glass" />
                </button>
              </div>
            </div>
            <div class="form-control">
              <label class="label py-1"><span class="label-text text-xs">Utilisateur</span></label>
              <select v-model="utilisateurChoisi" class="select select-bordered select-sm">
                <option value="" disabled>Sélectionner…</option>
                <option v-for="u in utilisateursDisponibles" :key="u.id" :value="u.id">
                  {{ u.prenom }} {{ u.nom }} — {{ u.email }}
                </option>
              </select>
            </div>
            <div class="form-control">
              <label class="label py-1"><span class="label-text text-xs">Disponibilité (facultatif)</span></label>
              <input v-model="disponibilite" type="text" class="input input-bordered input-sm" placeholder="Ex : lun-ven 18h-20h" />
            </div>
          </div>
          <div class="flex items-center justify-between mt-3">
            <p v-if="messageRetour" class="text-xs opacity-70">{{ messageRetour }}</p>
            <p v-else class="text-xs opacity-50">
              L'utilisateur n'apparaît pas ? Affinez la recherche.
            </p>
            <button
              class="btn btn-primary btn-sm"
              :disabled="!utilisateurChoisi || saving"
              :class="{ loading: saving }"
              @click="designer"
            >
              <font-awesome-icon icon="user-plus" class="mr-1" /> Désigner
            </button>
          </div>
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
