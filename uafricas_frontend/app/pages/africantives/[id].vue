<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100">
    <!-- Loading -->
    <div
      v-if="loading"
      class="min-h-screen flex items-center justify-center"
    >
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-orange-500 border-t-transparent mx-auto mb-4"></div>
        <p class="text-gray-500">Chargement...</p>
      </div>
    </div>

    <!-- Not found -->
    <div
      v-else-if="!initiative"
      class="min-h-screen flex items-center justify-center px-4"
    >
      <div class="text-center">
        <font-awesome-icon
          :icon="['fas', 'circle-exclamation']"
          class="w-20 h-20 text-gray-300 mx-auto mb-4"
        />
        <h1 class="text-2xl font-bold text-gray-800 mb-2">Initiative introuvable</h1>
        <p class="text-gray-500 mb-6">Cette initiative n'existe pas ou a été supprimée.</p>
        <NuxtLink
          to="/africantives"
          class="inline-flex items-center gap-2 px-6 py-3 bg-orange-500 text-white font-medium rounded-xl hover:bg-orange-600 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
          Retour aux initiatives
        </NuxtLink>
      </div>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero image -->
      <div class="relative h-64 md:h-80 lg:h-96 bg-gray-900">
        <img
          :src="initiative.image_couverture_url || '/images/placeholder.jpg'"
          :alt="initiative.titre"
          class="w-full h-full object-cover opacity-90"
        />
        <div class="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-black/20"></div>

        <!-- Badge domaine -->
        <span
          v-if="initiative.domaine"
          class="absolute top-4 left-4 px-4 py-2 rounded-full text-sm font-semibold shadow-lg bg-orange-100 text-orange-700"
        >
          {{ initiative.domaine }}
        </span>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 -mt-16 relative z-10 pb-16">
        <div class="bg-white rounded-2xl shadow-xl overflow-hidden">
          <!-- Header -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <!-- Breadcrumb -->
            <CommonBreadcrumbNav class="mb-6" :custom-breadcrumbs="breadcrumbs" />

            <!-- Info bar -->
            <div class="flex flex-wrap items-center gap-4 text-sm text-gray-500 mb-4">
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-custom-green" />
                <span>{{ initiative.pays || 'Afrique' }}</span>
                <span v-if="initiative.ville"> - {{ initiative.ville }}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'calendar-days']" class="w-4 h-4 text-gray-400" />
                <span>Publié le {{ dateFormatee }}</span>
              </div>
              <div v-if="initiative.domaine" class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'layer-group']" class="w-4 h-4 text-gray-400" />
                <span>{{ initiative.domaine }}</span>
              </div>
            </div>

            <!-- Titre -->
            <h1 class="text-2xl md:text-3xl font-bold text-gray-900">
              {{ initiative.titre }}
            </h1>
          </div>

          <!-- Image -->
          <div v-if="initiative.image_couverture_url" class="p-6 md:p-8 border-b border-gray-100">
            <div class="rounded-2xl overflow-hidden border border-gray-200">
              <img
                :src="initiative.image_couverture_url"
                :alt="initiative.titre"
                class="w-full max-h-[500px] object-cover bg-gray-50"
              />
            </div>
          </div>

          <!-- Description -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'align-left']" class="w-4 h-4 text-custom-chocolat" />
              Description de l'initiative
            </h2>
            <p class="text-gray-600 leading-relaxed whitespace-pre-line">
              {{ initiative.description }}
            </p>
          </div>

          <!-- Auteur -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'user']" class="w-4 h-4 text-custom-chocolat" />
              Porteur de l'initiative
            </h2>

            <div class="flex items-center gap-4">
              <div
                v-if="initiative.user.photo_url"
                class="w-12 h-12 rounded-full overflow-hidden"
              >
                <img :src="initiative.user.photo_url" :alt="initiative.user.prenom" class="w-full h-full object-cover" />
              </div>
              <div
                v-else
                class="w-12 h-12 bg-gradient-to-br from-orange-400 to-red-500 rounded-full flex items-center justify-center text-white font-bold text-lg"
              >
                {{ initiative.user.prenom.charAt(0) }}{{ initiative.user.nom.charAt(0) }}
              </div>
              <div>
                <p class="font-medium text-gray-800">
                  {{ initiative.user.prenom }} {{ initiative.user.nom }}
                </p>
              </div>
            </div>
          </div>

          <!-- Liens -->
          <div
            v-if="initiative.site_web_url || initiative.lien_reseau_social"
            class="p-6 md:p-8 border-b border-gray-100"
          >
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'link']" class="w-4 h-4 text-custom-chocolat" />
              Liens
            </h2>
            <div class="flex flex-wrap gap-3">
              <a
                v-if="initiative.site_web_url"
                :href="initiative.site_web_url"
                target="_blank"
                rel="noopener noreferrer"
                class="inline-flex items-center gap-2 px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors text-sm"
              >
                <font-awesome-icon :icon="['fas', 'globe']" class="w-4 h-4 text-custom-green" />
                Site web
                <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3 h-3 text-gray-400" />
              </a>
              <a
                v-if="initiative.lien_reseau_social"
                :href="initiative.lien_reseau_social"
                target="_blank"
                rel="noopener noreferrer"
                class="inline-flex items-center gap-2 px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors text-sm"
              >
                <font-awesome-icon :icon="['fas', 'share-nodes']" class="w-4 h-4 text-custom-green" />
                Réseau social
                <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3 h-3 text-gray-400" />
              </a>
            </div>
          </div>

          <!-- Contacts de l'initiateur -->
          <div
            v-if="aContacts"
            class="p-6 md:p-8 border-b border-gray-100"
          >
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'address-book']" class="w-4 h-4 text-custom-chocolat" />
              Contacts de l'initiateur
            </h2>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div
                v-for="(contact, idx) in contacts"
                :key="idx"
                class="bg-gray-50 rounded-xl p-4 space-y-2"
              >
                <p class="text-sm font-medium text-gray-500">Contact {{ idx + 1 }}</p>
                <a
                  v-if="contact.courriel"
                  :href="`mailto:${contact.courriel}`"
                  class="flex items-center gap-2 text-sm text-gray-700 hover:text-orange-600 transition-colors break-all"
                >
                  <font-awesome-icon :icon="['fas', 'envelope']" class="w-4 h-4 text-gray-400 shrink-0" />
                  {{ contact.courriel }}
                </a>
                <a
                  v-if="contact.telephone"
                  :href="`tel:${contact.telephone}`"
                  class="flex items-center gap-2 text-sm text-gray-700 hover:text-orange-600 transition-colors"
                >
                  <font-awesome-icon :icon="['fas', 'phone']" class="w-4 h-4 text-gray-400 shrink-0" />
                  {{ contact.telephone }}
                </a>
                <p
                  v-if="contact.adresse"
                  class="flex items-center gap-2 text-sm text-gray-700"
                >
                  <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-gray-400 shrink-0" />
                  {{ contact.adresse }}
                </p>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="p-6 md:p-8">
            <div v-if="isAuthenticated" class="space-y-4">
              <button
                @click="contacterPorteur"
                :disabled="contactEnvoye"
                class="w-full py-4 font-semibold rounded-xl transition-all flex items-center justify-center gap-3"
                :class="contactEnvoye
                  ? 'bg-gray-100 text-gray-500 cursor-not-allowed'
                  : 'bg-gradient-to-r from-orange-500 to-red-500 text-white hover:from-orange-600 hover:to-red-600 shadow-lg hover:shadow-xl'"
              >
                <font-awesome-icon
                  :icon="contactEnvoye ? ['fas', 'check-circle'] : ['fas', 'envelope']"
                  class="w-5 h-5"
                />
                {{ contactEnvoye ? 'Demande de contact envoyée' : 'Contacter le porteur' }}
              </button>
            </div>

            <div
              v-else
              class="bg-amber-50 border border-amber-200 rounded-xl p-6 text-center"
            >
              <font-awesome-icon
                :icon="['fas', 'lock']"
                class="w-8 h-8 text-amber-500 mx-auto mb-3"
              />
              <p class="text-amber-800 font-medium mb-4">
                Connectez-vous pour contacter le porteur de l'initiative
              </p>
              <NuxtLink
                to="/login"
                class="inline-flex items-center gap-2 px-6 py-3 bg-custom-chocolat text-white font-medium rounded-xl hover:bg-custom-chocolat/90 transition-colors"
              >
                <font-awesome-icon :icon="['fas', 'right-to-bracket']" class="w-4 h-4" />
                Se connecter
              </NuxtLink>
            </div>
          </div>
        </div>

        <!-- Retour -->
        <div class="mt-8 text-center">
          <NuxtLink
            to="/africantives"
            class="inline-flex items-center gap-2 text-gray-600 hover:text-orange-600 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
            Retour à la liste des initiatives
          </NuxtLink>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import {
  useAfricantives,
  formatDate,
  type AfricantiveDetailAPI,
} from '~/composables/useAfricantives'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { obtenirAfricantive } = useAfricantives()

// State
const loading = ref(true)
const initiative = ref<AfricantiveDetailAPI | null>(null)
const contactEnvoye = ref(false)

// Computed
const isAuthenticated = computed(() => userStore.isAuthenticated)

const breadcrumbs = computed(() => [
  { label: 'Africantives', to: '/africantives' },
  { label: initiative.value?.titre || 'Détail', to: undefined },
])

const dateFormatee = computed(() => {
  if (!initiative.value) return ''
  return formatDate(initiative.value.created_at)
})

// Contacts de l'initiateur (filtrer ceux entierement vides)
const contacts = computed(() => {
  const i = initiative.value
  if (!i) return []
  return [
    { courriel: i.contact1_courriel, telephone: i.contact1_telephone, adresse: i.contact1_adresse },
    { courriel: i.contact2_courriel, telephone: i.contact2_telephone, adresse: i.contact2_adresse },
  ].filter(c => c.courriel || c.telephone || c.adresse)
})

const aContacts = computed(() => contacts.value.length > 0)

// Methods
const contacterPorteur = () => {
  contactEnvoye.value = true
  alert('Votre demande de contact a été envoyée au porteur de l\'initiative !')
}

// Lifecycle
onMounted(async () => {
  const id = route.params.id as string
  const resultat = await obtenirAfricantive(id)
  initiative.value = resultat

  if (initiative.value) {
    useHead({
      title: `${initiative.value.titre} - Africantives - AfricanS`,
      meta: [
        {
          name: 'description',
          content: initiative.value.description.substring(0, 160),
        },
      ],
    })
  }

  loading.value = false
})
</script>
