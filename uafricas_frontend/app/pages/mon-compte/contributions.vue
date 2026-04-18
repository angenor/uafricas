<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 via-white to-gray-50 pt-28 pb-16">
    <div class="max-w-6xl mx-auto px-4">
      <!-- En-tête -->
      <div class="mb-8" data-aos="fade-up">
        <h1 class="text-3xl font-bold text-gray-800 font-display">Mes contributions</h1>
        <p class="mt-2 text-gray-600 text-sm">
          Suivez l'état de vos propositions sur les fiches pays Afripulse.
        </p>
      </div>

      <!-- Filtres -->
      <div
        class="bg-white rounded-2xl shadow-sm border border-gray-100 p-4 mb-6 flex flex-wrap gap-3 items-end"
        data-aos="fade-up"
      >
        <div class="flex-1 min-w-[160px]">
          <label class="block text-xs font-medium text-gray-600 mb-1">État</label>
          <select
            v-model="filtres.etat"
            class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-custom-green"
            @change="rafraichir"
          >
            <option value="">Tous</option>
            <option value="en_attente">En attente</option>
            <option value="approuvee">Approuvée</option>
            <option value="refusee">Refusée</option>
            <option value="obsolete">Obsolète / retirée</option>
          </select>
        </div>
        <div class="flex-1 min-w-[200px]">
          <label class="block text-xs font-medium text-gray-600 mb-1">Type</label>
          <select
            v-model="filtres.type_objet"
            class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-custom-green"
            @change="rafraichir"
          >
            <option value="">Tous</option>
            <option value="fiche_pays">Nouvelle fiche pays</option>
            <option value="site_touristique">Site touristique</option>
            <option value="secteur_developpement">Secteur d'opportunité</option>
            <option value="personnalite_connue">Personnalité connue</option>
            <option value="savoir_pratique">Savoir pratique</option>
            <option value="recommandation_visiteur">Recommandation</option>
            <option value="photo_visiteur">Photo</option>
          </select>
        </div>
        <button
          class="px-4 py-2 text-sm bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors"
          @click="reinitialiser"
        >
          Réinitialiser
        </button>
      </div>

      <!-- Chargement -->
      <div v-if="chargement" class="flex items-center justify-center py-24">
        <font-awesome-icon
          icon="fa-solid fa-spinner"
          class="text-4xl text-custom-chocolat animate-spin"
        />
      </div>

      <!-- Vide -->
      <div
        v-else-if="!contributions.length"
        class="bg-white rounded-2xl shadow-sm border border-gray-100 p-12 text-center"
      >
        <font-awesome-icon
          icon="fa-solid fa-clipboard-list"
          class="text-5xl text-gray-300 mb-4"
        />
        <p class="text-gray-600">Aucune contribution pour le moment.</p>
        <NuxtLink
          to="/opportunite-afrique"
          class="inline-block mt-4 px-5 py-2 bg-custom-chocolat text-white rounded-full text-sm font-medium hover:opacity-90 transition-opacity"
        >
          Découvrir les fiches pays
        </NuxtLink>
      </div>

      <!-- Liste -->
      <div v-else class="space-y-3" data-aos="fade-up">
        <div
          v-for="c in contributions"
          :key="c.id"
          class="bg-white rounded-xl shadow-sm border border-gray-100 p-4 hover:shadow-md transition-shadow"
        >
          <div class="flex flex-wrap items-start gap-4 justify-between">
            <div class="flex-1 min-w-0">
              <div class="flex flex-wrap items-center gap-2 mb-2">
                <span
                  class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-xs font-medium"
                  :class="classesEtat(c.etat)"
                >
                  <font-awesome-icon :icon="iconeEtat(c.etat)" class="text-[10px]" />
                  {{ libelleEtat(c.etat) }}
                </span>
                <span class="text-xs text-gray-500">
                  {{ libelleTypeObjet(c.type_objet_contribution) }}
                </span>
                <span class="text-xs text-gray-400">•</span>
                <span class="text-xs text-gray-500 capitalize">
                  {{ libelleTypeContribution(c.type_contribution) }}
                </span>
              </div>

              <NuxtLink
                v-if="c.pays_nom"
                :to="`/opportunite-afrique/${c.fiche_pays_id}`"
                class="font-semibold text-gray-800 hover:text-custom-green transition-colors"
              >
                {{ c.pays_nom }}
              </NuxtLink>
              <span v-else class="font-semibold text-gray-800">Nouvelle fiche pays</span>

              <div
                v-if="(c.etat === 'refusee' || c.etat === 'obsolete') && c.note_moderation"
                class="mt-2 p-3 rounded-lg bg-red-50 border border-red-100 text-sm text-red-700"
              >
                <span class="font-medium">Motif :</span> {{ c.note_moderation }}
              </div>
            </div>

            <div class="text-right text-xs text-gray-500 shrink-0">
              <div>Soumise le {{ formatDate(c.created_at) }}</div>
              <div v-if="c.traite_at">Traitée le {{ formatDate(c.traite_at) }}</div>
            </div>
          </div>
        </div>

        <!-- Pagination -->
        <div v-if="totalPages > 1" class="flex justify-center gap-2 pt-6">
          <button
            class="px-3 py-1.5 text-sm rounded-lg bg-white border border-gray-200 hover:bg-gray-50 disabled:opacity-40"
            :disabled="page <= 1"
            @click="allerPage(page - 1)"
          >
            Précédent
          </button>
          <span class="px-3 py-1.5 text-sm text-gray-600">
            Page {{ page }} / {{ totalPages }}
          </span>
          <button
            class="px-3 py-1.5 text-sm rounded-lg bg-white border border-gray-200 hover:bg-gray-50 disabled:opacity-40"
            :disabled="page >= totalPages"
            @click="allerPage(page + 1)"
          >
            Suivant
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useUserStore } from '~/stores/user'
import { formatDate } from '~/composables/useOpportuniteAfrique'

definePageMeta({ middleware: 'auth' })

useAOS()

interface MaContribution {
  id: string
  fiche_pays_id: string
  pays_nom: string | null
  type_objet_contribution: string
  section_afripulse: string | null
  type_contribution: string
  etat: string
  note_moderation: string | null
  created_at: string
  traite_at: string | null
}

interface ReponseAPI {
  success: boolean
  data: {
    contributions: MaContribution[]
    total: number
    page: number
    par_page: number
  } | null
  error: string | null
}

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const userStore = useUserStore()

const contributions = ref<MaContribution[]>([])
const chargement = ref(false)
const total = ref(0)
const page = ref(1)
const parPage = ref(20)

const filtres = reactive({
  etat: '',
  type_objet: '',
})

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / parPage.value)))

const charger = async () => {
  chargement.value = true
  try {
    const params = new URLSearchParams({
      page: String(page.value),
      par_page: String(parPage.value),
    })
    if (filtres.etat) params.set('etat', filtres.etat)
    if (filtres.type_objet) params.set('type_objet', filtres.type_objet)

    const reponse = await $fetch<ReponseAPI>(
      `${apiBase}/api/fiches-pays/moi/contributions?${params}`,
      { headers: { Authorization: `Bearer ${userStore.accessToken}` } },
    )
    if (reponse.success && reponse.data) {
      contributions.value = reponse.data.contributions
      total.value = reponse.data.total
    }
  }
  catch (e) {
    console.error('Erreur chargement contributions:', e)
  }
  finally {
    chargement.value = false
  }
}

const rafraichir = () => {
  page.value = 1
  charger()
}

const reinitialiser = () => {
  filtres.etat = ''
  filtres.type_objet = ''
  rafraichir()
}

const allerPage = (n: number) => {
  if (n < 1 || n > totalPages.value) return
  page.value = n
  charger()
}

const libelleEtat = (e: string): string => {
  const m: Record<string, string> = {
    en_attente: 'En attente',
    approuvee: 'Approuvée',
    refusee: 'Refusée',
    obsolete: 'Obsolète',
  }
  return m[e] ?? e
}

const classesEtat = (e: string): string => {
  const m: Record<string, string> = {
    en_attente: 'bg-amber-50 text-amber-700 border border-amber-200',
    approuvee: 'bg-emerald-50 text-emerald-700 border border-emerald-200',
    refusee: 'bg-red-50 text-red-700 border border-red-200',
    obsolete: 'bg-gray-100 text-gray-600 border border-gray-200',
  }
  return m[e] ?? 'bg-gray-100 text-gray-600'
}

const iconeEtat = (e: string): string => {
  const m: Record<string, string> = {
    en_attente: 'fa-solid fa-clock',
    approuvee: 'fa-solid fa-check',
    refusee: 'fa-solid fa-xmark',
    obsolete: 'fa-solid fa-ban',
  }
  return m[e] ?? 'fa-solid fa-circle'
}

const libelleTypeObjet = (t: string): string => {
  const m: Record<string, string> = {
    fiche_pays: 'Nouvelle fiche pays',
    site_touristique: 'Site touristique',
    secteur_developpement: 'Secteur d\'opportunité',
    personnalite_connue: 'Personnalité connue',
    savoir_pratique: 'Savoir pratique',
    recommandation_visiteur: 'Recommandation',
    photo_visiteur: 'Photo',
  }
  return m[t] ?? t
}

const libelleTypeContribution = (t: string): string => {
  const m: Record<string, string> = {
    ajout: 'Ajout',
    edition: 'Édition',
    suppression: 'Suppression',
  }
  return m[t] ?? t
}

onMounted(charger)
</script>
