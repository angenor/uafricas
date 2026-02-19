<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const { candidatureDetail, chargerDetail, changerStatut, loading, error } = useAdminCandidatures()

const successMsg = ref<string | null>(null)
const erreurLocale = ref<string | null>(null)
const showStatutModal = ref(false)
const nouveauStatut = ref('')
const notesInternes = ref('')
const statutLoading = ref(false)

const statutBadge = (statut: string) => {
  const map: Record<string, string> = {
    soumise: 'badge-info',
    en_revue: 'badge-warning',
    acceptee: 'badge-success',
    refusee: 'badge-error',
    retiree: 'badge-neutral',
  }
  return map[statut] || 'badge-neutral'
}

const statutLabel = (statut: string) => {
  const map: Record<string, string> = {
    soumise: 'Soumise',
    en_revue: 'En revue',
    acceptee: 'Acceptee',
    refusee: 'Refusee',
    retiree: 'Retiree',
  }
  return map[statut] || statut
}

const ouvrirStatutModal = (statut: string) => {
  nouveauStatut.value = statut
  notesInternes.value = ''
  showStatutModal.value = true
}

const executerChangerStatut = async () => {
  statutLoading.value = true
  erreurLocale.value = null
  try {
    await changerStatut(id, nouveauStatut.value, notesInternes.value || undefined)
    showStatutModal.value = false
    successMsg.value = `Candidature ${nouveauStatut.value === 'acceptee' ? 'acceptee' : nouveauStatut.value === 'refusee' ? 'refusee' : 'mise a jour'}`
    setTimeout(() => { successMsg.value = null }, 3000)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally { statutLoading.value = false }
}

const runtimeConfig = useRuntimeConfig()
const backendUrl = runtimeConfig.public?.apiBase || 'http://localhost:8080'

onMounted(() => chargerDetail(id))
</script>

<template>
  <div>
    <AdminPageHeader titre="Revue de candidature" sous-titre="Examiner et traiter la candidature">
      <template #actions>
        <NuxtLink to="/admin/candidatures" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !candidatureDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="candidatureDetail">
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <!-- Colonne principale -->
        <div class="lg:col-span-2 space-y-4">
          <!-- Statut actuel -->
          <div class="card bg-base-100 shadow-sm">
            <div class="card-body">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="text-lg font-bold">Statut</h3>
                  <span :class="['badge mt-1', statutBadge(candidatureDetail.statut)]">
                    {{ statutLabel(candidatureDetail.statut) }}
                  </span>
                </div>
                <div class="flex gap-2">
                  <button
                    v-if="candidatureDetail.statut !== 'acceptee'"
                    class="btn btn-success btn-sm"
                    @click="ouvrirStatutModal('acceptee')"
                  >
                    <font-awesome-icon icon="check" class="mr-1" /> Accepter
                  </button>
                  <button
                    v-if="candidatureDetail.statut !== 'refusee'"
                    class="btn btn-error btn-sm"
                    @click="ouvrirStatutModal('refusee')"
                  >
                    <font-awesome-icon icon="xmark" class="mr-1" /> Refuser
                  </button>
                  <button
                    v-if="candidatureDetail.statut === 'soumise'"
                    class="btn btn-warning btn-sm"
                    @click="ouvrirStatutModal('en_revue')"
                  >
                    <font-awesome-icon icon="hourglass-half" class="mr-1" /> En revue
                  </button>
                </div>
              </div>
              <div v-if="candidatureDetail.traite_par_nom" class="text-sm text-base-content/70 mt-2">
                Traitee par {{ candidatureDetail.traite_par_nom }}
              </div>
            </div>
          </div>

          <!-- Programme -->
          <div class="card bg-base-100 shadow-sm">
            <div class="card-body">
              <h3 class="text-lg font-bold mb-2">Programme</h3>
              <NuxtLink :to="`/admin/programmes/${candidatureDetail.programme_id}`" class="link link-primary text-lg">
                {{ candidatureDetail.programme_titre }}
              </NuxtLink>
            </div>
          </div>

          <!-- Lettre de motivation -->
          <div class="card bg-base-100 shadow-sm">
            <div class="card-body">
              <h3 class="text-lg font-bold mb-2">Lettre de motivation</h3>
              <div v-if="candidatureDetail.lettre_motivation" class="prose max-w-none whitespace-pre-wrap">
                {{ candidatureDetail.lettre_motivation }}
              </div>
              <p v-else class="text-base-content/50 italic">Aucune lettre de motivation fournie</p>
            </div>
          </div>

          <!-- CV -->
          <div class="card bg-base-100 shadow-sm">
            <div class="card-body">
              <h3 class="text-lg font-bold mb-2">CV</h3>
              <div v-if="candidatureDetail.cv_url">
                <a
                  :href="`${backendUrl}${candidatureDetail.cv_url}`"
                  target="_blank"
                  class="btn btn-outline btn-sm"
                >
                  <font-awesome-icon icon="download" class="mr-1" /> Telecharger le CV
                </a>
              </div>
              <p v-else class="text-base-content/50 italic">Aucun CV fourni</p>
            </div>
          </div>

          <!-- Notes internes -->
          <div v-if="candidatureDetail.notes_internes" class="card bg-base-100 shadow-sm">
            <div class="card-body">
              <h3 class="text-lg font-bold mb-2">Notes internes</h3>
              <div class="bg-base-200 p-3 rounded-lg whitespace-pre-wrap">
                {{ candidatureDetail.notes_internes }}
              </div>
            </div>
          </div>
        </div>

        <!-- Sidebar - Profil candidat -->
        <div class="space-y-4">
          <div class="card bg-base-100 shadow-sm">
            <div class="card-body">
              <h3 class="text-lg font-bold mb-3">Profil du candidat</h3>
              <div class="flex items-center gap-3 mb-3">
                <div v-if="candidatureDetail.candidat_photo_url" class="avatar">
                  <div class="w-12 rounded-full">
                    <img :src="`${backendUrl}${candidatureDetail.candidat_photo_url}`" :alt="candidatureDetail.candidat_nom">
                  </div>
                </div>
                <div v-else class="avatar placeholder">
                  <div class="bg-neutral text-neutral-content w-12 rounded-full">
                    <span class="text-lg">{{ candidatureDetail.candidat_prenom[0] }}{{ candidatureDetail.candidat_nom[0] }}</span>
                  </div>
                </div>
                <div>
                  <div class="font-bold">{{ candidatureDetail.candidat_prenom }} {{ candidatureDetail.candidat_nom }}</div>
                  <div class="text-sm text-base-content/70">{{ candidatureDetail.candidat_email }}</div>
                </div>
              </div>
              <NuxtLink :to="`/admin/utilisateurs/${candidatureDetail.candidat_id}`" class="btn btn-outline btn-sm w-full">
                <font-awesome-icon icon="user" class="mr-1" /> Voir le profil complet
              </NuxtLink>
            </div>
          </div>

          <div class="card bg-base-100 shadow-sm">
            <div class="card-body">
              <h3 class="text-sm font-bold mb-2">Informations</h3>
              <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <span class="text-base-content/70">Soumise le</span>
                  <span>{{ new Date(candidatureDetail.created_at).toLocaleDateString('fr-FR') }}</span>
                </div>
                <div v-if="candidatureDetail.updated_at" class="flex justify-between">
                  <span class="text-base-content/70">Mise a jour</span>
                  <span>{{ new Date(candidatureDetail.updated_at).toLocaleDateString('fr-FR') }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-base-content/70">ID</span>
                  <span class="font-mono text-xs">{{ candidatureDetail.id.substring(0, 8) }}...</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Modal changement statut -->
      <div v-if="showStatutModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">
            {{ nouveauStatut === 'acceptee' ? 'Accepter' : nouveauStatut === 'refusee' ? 'Refuser' : 'Mettre en revue' }} la candidature
          </h3>
          <div class="form-control">
            <label class="label"><span class="label-text">Commentaire / notes internes</span></label>
            <textarea
              v-model="notesInternes"
              class="textarea textarea-bordered h-24"
              :placeholder="nouveauStatut === 'acceptee' ? 'Felicitations, votre candidature...' : nouveauStatut === 'refusee' ? 'Raison du refus...' : 'Notes de revue...'"
            />
          </div>
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showStatutModal = false">Annuler</button>
            <button
              :class="['btn', nouveauStatut === 'acceptee' ? 'btn-success' : nouveauStatut === 'refusee' ? 'btn-error' : 'btn-warning', { loading: statutLoading }]"
              :disabled="statutLoading"
              @click="executerChangerStatut"
            >
              Confirmer
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showStatutModal = false" />
      </div>
    </template>
  </div>
</template>
