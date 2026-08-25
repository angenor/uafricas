<script setup lang="ts">
// Panneau admin : gestion des administrateurs d'une salle publique
// Feature 001-admin-salles-publiques, US3
import type { SalleAdministrateurAPI } from '~/composables/useAdminAfrolangSalles'

const props = defineProps<{
  salleId: string
}>()

const {
  listerAdministrateurs,
  nommerAdministrateur,
  revoquerAdministrateur,
} = useAdminAfrolangSalles()

// Recherche utilisateur (réutilise useAdminUtilisateurs)
const { utilisateurs, chargerListe: chargerUtilisateurs, filtres: filtresUsers } = useAdminUtilisateurs()

const liste = ref<SalleAdministrateurAPI[]>([])
const chargement = ref(false)
const erreur = ref<string | null>(null)
const success = ref<string | null>(null)
const saving = ref(false)

// Recherche utilisateur
const recherche = ref('')
const utilisateurChoisi = ref('')

const utilisateursDisponibles = computed(() => {
  const idsActifs = new Set(
    liste.value.filter(a => a.actif).map(a => a.utilisateur.id),
  )
  return utilisateurs.value.filter(u => !idsActifs.has(u.id))
})

const charger = async () => {
  chargement.value = true
  erreur.value = null
  try {
    liste.value = await listerAdministrateurs(props.salleId)
  }
  catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur de chargement'
  }
  finally {
    chargement.value = false
  }
}

const rechercherUtilisateurs = async () => {
  filtresUsers.recherche = recherche.value
  await chargerUtilisateurs()
}

const nommer = async () => {
  if (!utilisateurChoisi.value) return
  saving.value = true
  erreur.value = null
  success.value = null
  try {
    const res = await nommerAdministrateur(props.salleId, utilisateurChoisi.value)
    if (!res) throw new Error('Échec de la nomination')
    success.value = 'Utilisateur nommé administrateur de salle.'
    utilisateurChoisi.value = ''
    recherche.value = ''
    await charger()
  }
  catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur lors de la nomination'
  }
  finally {
    saving.value = false
  }
}

// Modal révocation
const showRevokeModal = ref(false)
const cibleRevocation = ref<SalleAdministrateurAPI | null>(null)
const motifRevocation = ref('')

const ouvrirRevocation = (admin: SalleAdministrateurAPI) => {
  cibleRevocation.value = admin
  motifRevocation.value = ''
  showRevokeModal.value = true
}

const confirmerRevocation = async () => {
  if (!cibleRevocation.value) return
  saving.value = true
  erreur.value = null
  try {
    const motif = motifRevocation.value.trim() || undefined
    const res = await revoquerAdministrateur(
      props.salleId,
      cibleRevocation.value.utilisateur.id,
      motif,
    )
    if (!res) throw new Error('Échec de la révocation')
    success.value = 'Administrateur révoqué, l\'utilisateur a été notifié.'
    showRevokeModal.value = false
    cibleRevocation.value = null
    motifRevocation.value = ''
    await charger()
  }
  catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur lors de la révocation'
  }
  finally {
    saving.value = false
  }
}

const formatDate = (iso: string | null) =>
  iso
    ? new Date(iso).toLocaleDateString('fr-FR', {
        day: '2-digit', month: '2-digit', year: 'numeric',
      })
    : '-'

onMounted(charger)
</script>

<template>
  <div class="space-y-6">
    <div v-if="erreur" class="alert alert-error">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ erreur }}</span>
    </div>
    <div v-if="success" class="alert alert-success">
      <font-awesome-icon icon="circle-check" />
      <span>{{ success }}</span>
    </div>

    <!-- Nomination -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <h3 class="card-title text-base">Nommer un administrateur de salle</h3>
        <p class="text-sm text-base-content/60">
          Distinct du rôle « administrateur de la plateforme ». Capacités effectives à venir.
        </p>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-2 mt-2">
          <div class="form-control md:col-span-2">
            <label class="label">
              <span class="label-text">Recherche utilisateur (nom / e-mail)</span>
            </label>
            <div class="join">
              <input
                v-model="recherche"
                type="text"
                class="input input-bordered join-item w-full"
                placeholder="Rechercher..."
                @keyup.enter="rechercherUtilisateurs"
              />
              <button class="btn btn-primary join-item" @click="rechercherUtilisateurs">
                <font-awesome-icon icon="magnifying-glass" />
              </button>
            </div>
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Utilisateur</span></label>
            <select v-model="utilisateurChoisi" class="select select-bordered">
              <option value="" disabled>Sélectionner...</option>
              <option
                v-for="u in utilisateursDisponibles"
                :key="u.id"
                :value="u.id"
              >
                {{ u.prenom }} {{ u.nom }} - {{ u.email }}
              </option>
            </select>
          </div>
        </div>
        <div class="flex justify-end mt-4">
          <button
            class="btn btn-primary"
            :disabled="!utilisateurChoisi || saving"
            :class="{ loading: saving }"
            @click="nommer"
          >
            <font-awesome-icon icon="user-plus" class="mr-1" /> Nommer
          </button>
        </div>
      </div>
    </div>

    <!-- Liste actuelle -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <h3 class="card-title text-base">
          Historique des administrateurs
          <span class="badge badge-outline badge-sm ml-2">{{ liste.length }}</span>
        </h3>
        <div v-if="chargement" class="flex justify-center py-6">
          <span class="loading loading-spinner" />
        </div>
        <table v-else class="table table-sm">
          <thead>
            <tr>
              <th>Utilisateur</th>
              <th>Nommé par</th>
              <th>Nommé le</th>
              <th>Statut</th>
              <th>Détails</th>
              <th />
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="a in liste"
              :key="a.id"
              :class="{ 'opacity-60': !a.actif }"
            >
              <td>
                <div class="flex items-center gap-2">
                  <div class="avatar placeholder">
                    <div class="bg-secondary text-secondary-content rounded-full w-8 h-8">
                      <span class="text-xs">{{ (a.utilisateur.prenom?.[0] || '') + (a.utilisateur.nom?.[0] || '') }}</span>
                    </div>
                  </div>
                  <div>
                    <div class="font-medium">{{ a.utilisateur.prenom }} {{ a.utilisateur.nom }}</div>
                    <div class="text-xs text-base-content/50">{{ a.utilisateur.email }}</div>
                  </div>
                </div>
              </td>
              <td class="text-sm">{{ a.nomme_par.prenom }} {{ a.nomme_par.nom }}</td>
              <td class="text-sm">{{ formatDate(a.nomme_at) }}</td>
              <td>
                <span v-if="a.actif" class="badge badge-success badge-sm">Actif</span>
                <span v-else-if="a.suspendu_at" class="badge badge-warning badge-sm">Suspendu</span>
                <span v-else class="badge badge-neutral badge-sm">Révoqué</span>
              </td>
              <td class="text-xs text-base-content/60">
                <div v-if="a.suspendu_at">
                  Suspendu le {{ formatDate(a.suspendu_at) }}
                  <span v-if="a.motif_suspension">, {{ a.motif_suspension }}</span>
                </div>
                <div v-else-if="a.revoque_at">
                  Révoqué le {{ formatDate(a.revoque_at) }}
                  <span v-if="a.revoque_par">par {{ a.revoque_par.prenom }} {{ a.revoque_par.nom }}</span>
                  <div v-if="a.motif_revocation">Motif&nbsp;: {{ a.motif_revocation }}</div>
                </div>
              </td>
              <td>
                <button
                  v-if="a.actif"
                  class="btn btn-ghost btn-xs text-error"
                  @click="ouvrirRevocation(a)"
                >
                  <font-awesome-icon icon="user-minus" /> Révoquer
                </button>
              </td>
            </tr>
            <tr v-if="!chargement && liste.length === 0">
              <td colspan="6" class="text-center text-base-content/50 py-6">
                Aucun administrateur nommé.
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Modal révocation -->
    <dialog v-if="showRevokeModal" open class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Révoquer l'administrateur</h3>
        <p class="py-2 text-sm">
          Vous allez révoquer
          <strong v-if="cibleRevocation">
            {{ cibleRevocation.utilisateur.prenom }} {{ cibleRevocation.utilisateur.nom }}
          </strong>.
          L'utilisateur sera notifié.
        </p>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Motif (facultatif)</span>
          </label>
          <textarea
            v-model="motifRevocation"
            class="textarea textarea-bordered"
            rows="3"
            placeholder="Ex : changement d'organisation..."
          />
        </div>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showRevokeModal = false">Annuler</button>
          <button class="btn btn-error" :disabled="saving" @click="confirmerRevocation">
            Confirmer
          </button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button @click="showRevokeModal = false">close</button>
      </form>
    </dialog>
  </div>
</template>
