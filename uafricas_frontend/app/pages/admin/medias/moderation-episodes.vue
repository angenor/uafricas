<script setup lang="ts">
/**
 * File de modération des **épisodes** (feature 009, US1, FR-040 à FR-043).
 *
 * Triée par **échéance** et non par ancienneté : un épisode attendu à l'antenne
 * samedi ne doit pas être traité au même rang qu'un contenu sans date. Les
 * épisodes non programmés viennent ensuite, par ordre d'arrivée.
 *
 * À ne pas confondre avec `/admin/medias/propositions` : là-bas ce sont des
 * brouillons soumis par des contributeurs extérieurs, ici de vraies lignes
 * `episode_*` déjà rattachées à leur programme.
 */
import { MOTIF_REJET_MIN, type EpisodeAModerer } from '~/composables/useAdminMediaModeration'

definePageMeta({ layout: 'admin', middleware: ['admin'] })
useHead({ title: 'Épisodes à valider, Administration' })

const {
  file, total, pagination, filtres, loading, error,
  charger, valider, rejeter, delaiLisible, urgence,
} = useAdminMediaModeration()

const messageAction = ref<string | null>(null)
const erreurAction = ref<string | null>(null)

const annoncer = (message: string) => {
  messageAction.value = message
  setTimeout(() => { messageAction.value = null }, 3500)
}

const executerValidation = async (episode: EpisodeAModerer) => {
  erreurAction.value = null
  try {
    await valider(episode.id)
    annoncer(`« ${episode.titre} » est publié.`)
  }
  catch (e: any) {
    erreurAction.value = e?.data?.error || e?.message || 'Validation impossible'
  }
}

// ── Rejet motivé ──────────────────────────────────────────────
const cibleRejet = ref<EpisodeAModerer | null>(null)
const motif = ref('')

const motifValide = computed(() => motif.value.trim().length >= MOTIF_REJET_MIN)

const ouvrirRejet = (episode: EpisodeAModerer) => {
  cibleRejet.value = episode
  motif.value = ''
  erreurAction.value = null
}

const executerRejet = async () => {
  if (!cibleRejet.value || !motifValide.value) return
  const titre = cibleRejet.value.titre
  try {
    await rejeter(cibleRejet.value.id, motif.value.trim())
    cibleRejet.value = null
    annoncer(`« ${titre} » est refusé. Son auteur reçoit le motif et peut corriger.`)
  }
  catch (e: any) {
    erreurAction.value = e?.data?.error || e?.message || 'Rejet impossible'
  }
}

const classeUrgence = (heures: number | null) => ({
  depassee: 'text-error font-semibold',
  proche: 'text-warning font-semibold',
  normale: 'opacity-70',
}[urgence(heures)])

const ancienneteLisible = (heures: number) =>
  heures < 24 ? `${heures} h` : `${Math.floor(heures / 24)} j`

const auteur = (episode: EpisodeAModerer) => episode.auteur_nom_complet || 'Membre'

watch([() => filtres.type, () => filtres.tri, () => filtres.etat], () => {
  pagination.page = 1
  charger()
})
watch(() => pagination.page, charger)
onMounted(charger)

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / pagination.taille)))
</script>

<template>
  <div>
    <AdminPageHeader titre="Épisodes à valider" sous-titre="Épisodes versés par les co-détenteurs des supports">
      <template #actions>
        <NuxtLink to="/admin/medias/emissions" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="film" class="mr-1" /> Programmes
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div class="flex flex-wrap gap-3 mb-6">
      <select v-model="filtres.type" class="select select-bordered select-sm">
        <option value="">Les deux familles</option>
        <option value="tele">Télévision</option>
        <option value="radio">Radio</option>
      </select>
      <select v-model="filtres.etat" class="select select-bordered select-sm">
        <option value="en_attente">En attente</option>
        <option value="rejete">Refusés</option>
        <option value="publie">Publiés</option>
      </select>
      <select v-model="filtres.tri" class="select select-bordered select-sm">
        <option value="echeance">Par échéance de diffusion</option>
        <option value="anciennete">Par ancienneté</option>
      </select>
      <span class="text-sm opacity-70 self-center ml-auto">{{ total }} épisode(s)</span>
    </div>

    <div v-if="error || erreurAction" class="alert alert-error mb-4">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ erreurAction || error }}</span>
      <button class="btn btn-ghost btn-xs" @click="erreurAction = null">x</button>
    </div>
    <div v-if="messageAction" class="alert alert-success mb-4">
      <font-awesome-icon icon="circle-check" />
      <span>{{ messageAction }}</span>
    </div>

    <div v-if="loading" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="!file.length" class="text-center py-16 opacity-60">
      Rien à traiter dans cette file.
    </div>

    <div v-else class="overflow-x-auto">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>Épisode</th>
            <th>Programme / support</th>
            <th>Auteur</th>
            <th class="w-32">Attente</th>
            <th class="w-44">Prochaine diffusion</th>
            <th class="w-56" />
          </tr>
        </thead>
        <tbody>
          <tr v-for="episode in file" :key="episode.id">
            <td>
              <div class="font-medium">
                <span v-if="episode.numero_episode" class="opacity-50">{{ episode.numero_episode }}. </span>
                {{ episode.titre }}
              </div>
              <div class="text-xs opacity-60 line-clamp-1">{{ episode.description }}</div>
              <a
                v-if="episode.media_url"
                :href="episode.media_url"
                target="_blank"
                rel="noopener"
                class="text-xs link"
              >
                Ouvrir le média
              </a>
              <span v-else class="text-xs text-error">Média manquant</span>
            </td>
            <td>
              <div>{{ episode.emission?.nom || '-' }}</div>
              <div class="text-xs opacity-60">
                {{ episode.support?.nom || '-' }}
                <span class="badge badge-xs ml-1" :class="episode.type_support === 'chaine_tv' ? 'badge-info' : 'badge-accent'">
                  {{ episode.type_support === 'chaine_tv' ? 'Télé' : 'Radio' }}
                </span>
              </div>
            </td>
            <td class="text-sm">{{ auteur(episode) }}</td>
            <td class="text-sm whitespace-nowrap">{{ ancienneteLisible(episode.anciennete_heures) }}</td>
            <td class="text-sm whitespace-nowrap" :class="classeUrgence(episode.heures_avant_echeance)">
              {{ delaiLisible(episode.heures_avant_echeance) }}
            </td>
            <td>
              <div class="flex gap-2 justify-end">
                <NuxtLink :to="`/admin/medias/emissions/${episode.emission_id}`" class="btn btn-ghost btn-xs">
                  Programme
                </NuxtLink>
                <template v-if="episode.etat === 'en_attente'">
                  <button class="btn btn-success btn-xs" @click="executerValidation(episode)">
                    <font-awesome-icon icon="check" class="mr-1" /> Valider
                  </button>
                  <button class="btn btn-error btn-xs" @click="ouvrirRejet(episode)">
                    <font-awesome-icon icon="xmark" class="mr-1" /> Refuser
                  </button>
                </template>
                <span v-else-if="episode.motif_rejet" class="text-xs opacity-70 max-w-48 truncate" :title="episode.motif_rejet">
                  {{ episode.motif_rejet }}
                </span>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="totalPages > 1" class="join mt-6 flex justify-center">
      <button class="join-item btn btn-sm" :disabled="pagination.page <= 1" @click="pagination.page--">«</button>
      <span class="join-item btn btn-sm btn-disabled">{{ pagination.page }} / {{ totalPages }}</span>
      <button class="join-item btn btn-sm" :disabled="pagination.page >= totalPages" @click="pagination.page++">»</button>
    </div>

    <!-- Rejet : le motif est obligatoire, il part dans la notification -->
    <div v-if="cibleRejet" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-2">Refuser « {{ cibleRejet.titre }} »</h3>
        <p class="text-sm text-base-content/70 mb-3">
          Le motif est transmis à son auteur, qui pourra corriger et resoumettre l'épisode.
          Un refus sans explication serait une impasse.
        </p>
        <textarea
          v-model="motif"
          class="textarea textarea-bordered w-full h-28"
          :placeholder="`Motif du refus (${MOTIF_REJET_MIN} caractères minimum)`"
        />
        <p class="text-xs mt-1" :class="motifValide ? 'opacity-60' : 'text-warning'">
          {{ motif.trim().length }} / {{ MOTIF_REJET_MIN }} caractères minimum
        </p>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="cibleRejet = null">Annuler</button>
          <button class="btn btn-error" :disabled="!motifValide" @click="executerRejet">
            Refuser l'épisode
          </button>
        </div>
      </div>
      <div class="modal-backdrop" @click="cibleRejet = null" />
    </div>
  </div>
</template>
