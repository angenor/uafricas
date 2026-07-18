<template>
  <div class="min-h-screen bg-linear-to-br from-slate-50 to-slate-100">
    <!-- Loading -->
    <div
      v-if="loading"
      class="min-h-screen flex items-center justify-center"
    >
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-emerald-500 border-t-transparent mx-auto mb-4"></div>
        <p class="text-gray-500">Chargement...</p>
      </div>
    </div>

    <!-- Not found -->
    <div
      v-else-if="!annonce"
      class="min-h-screen flex items-center justify-center px-4"
    >
      <div class="text-center">
        <font-awesome-icon
          :icon="['fas', 'circle-exclamation']"
          class="w-20 h-20 text-gray-300 mx-auto mb-4"
        />
        <h1 class="text-2xl font-bold text-gray-800 mb-2">Annonce introuvable</h1>
        <p class="text-gray-500 mb-6">Cette annonce n'existe pas ou a été supprimée.</p>
        <NuxtLink
          to="/marche-africain"
          class="inline-flex items-center gap-2 px-6 py-3 bg-emerald-500 text-white font-medium rounded-xl hover:bg-emerald-600 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
          Retour aux annonces
        </NuxtLink>
      </div>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero image -->
      <div class="relative h-64 md:h-80 lg:h-96 bg-gray-900">
        <img
          :src="annonce.photo_url || '/images/placeholder.jpg'"
          :alt="annonce.titre"
          class="w-full h-full object-cover opacity-90"
        />
        <div class="absolute inset-0 bg-linear-to-t from-black/60 via-transparent to-black/20"></div>

        <!-- Badge type -->
        <span
          class="absolute top-4 left-4 px-4 py-2 rounded-full text-sm font-semibold shadow-lg"
          :class="getTypeColor(annonce.type_echange)"
        >
          {{ annonce.type_echange }}
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
                <span>{{ paysAffiche }}</span>
                <span v-if="annonce.ville"> - {{ annonce.ville }}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'calendar-days']" class="w-4 h-4 text-gray-400" />
                <span>Publié le {{ dateFormatee }}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'tag']" class="w-4 h-4 text-gray-400" />
                <span>{{ annonce.categorie }}</span>
              </div>
              <div v-if="annonce.secteur" class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'briefcase']" class="w-4 h-4 text-gray-400" />
                <span>{{ annonce.secteur }}</span>
              </div>
            </div>

            <!-- Prix -->
            <div
              class="text-3xl md:text-4xl font-bold mb-3"
              :class="annonce.type_echange === 'Don' ? 'text-blue-600' : 'text-custom-chocolat'"
            >
              {{ prixFormate }}
            </div>

            <!-- Titre -->
            <h1 class="text-2xl md:text-3xl font-bold text-gray-900">
              {{ annonce.titre }}
            </h1>

            <!-- Quantité minimum -->
            <div
              v-if="annonce.quantite && annonce.quantite > 1"
              class="mt-3 inline-flex items-center gap-2 px-3 py-1.5 bg-amber-50 text-amber-700 rounded-lg text-sm"
            >
              <font-awesome-icon :icon="['fas', 'boxes-stacked']" class="w-4 h-4" />
              Quantité minimum : {{ annonce.quantite }} unités
            </div>
          </div>

          <!-- Image produit -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <div class="rounded-2xl overflow-hidden border border-gray-200">
              <img
                :src="annonce.photo_url || '/images/placeholder.jpg'"
                :alt="annonce.titre"
                class="w-full max-h-125 object-cover bg-gray-50"
              />
            </div>
          </div>

          <!-- Description -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'align-left']" class="w-4 h-4 text-custom-green" />
              Description
            </h2>
            <p class="text-gray-600 leading-relaxed whitespace-pre-line">
              {{ annonce.description }}
            </p>
          </div>

          <!-- Contact -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'user']" class="w-4 h-4 text-custom-green" />
              Annonceur
            </h2>

            <div class="flex items-center gap-4">
              <div class="w-12 h-12 bg-linear-to-br from-emerald-400 to-teal-500 rounded-full flex items-center justify-center text-white font-bold text-lg">
                {{ annonce.user.prenom.charAt(0) }}{{ annonce.user.nom.charAt(0) }}
              </div>
              <div>
                <p class="font-medium text-gray-800">
                  {{ annonce.user.prenom }} {{ annonce.user.nom }}
                </p>
              </div>
            </div>

            <!-- Badges de credibilite de l'annonceur -->
            <div class="mt-4 flex flex-wrap gap-2">
              <span
                v-for="badge in badgesCredibilite"
                :key="badge.cle"
                class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-semibold border"
                :class="badge.valide
                  ? 'bg-emerald-50 text-emerald-700 border-emerald-200'
                  : 'bg-red-50 text-red-600 border-red-200'"
                :title="badge.valide ? badge.libelleValide : badge.libelleInvalide"
              >
                <font-awesome-icon :icon="['fas', badge.icone]" class="w-3.5 h-3.5" />
                {{ badge.libelle }}
                <font-awesome-icon
                  :icon="['fas', badge.valide ? 'circle-check' : 'circle-xmark']"
                  class="w-3.5 h-3.5"
                />
              </span>
            </div>

            <!-- Coordonnées publiques (annonceur entreprise) -->
            <div
              v-if="annonce.type_annonceur === 'entreprise'"
              class="mt-4 rounded-xl border border-gray-200 bg-gray-50 p-4 space-y-2"
            >
              <p v-if="annonce.nom_entreprise" class="flex items-center gap-2 text-sm font-semibold text-gray-800">
                <font-awesome-icon :icon="['fas', 'building']" class="w-4 h-4 text-custom-green" />
                {{ annonce.nom_entreprise }}
              </p>
              <p v-if="annonce.contact_telephone" class="flex items-center gap-2 text-sm text-gray-600">
                <font-awesome-icon :icon="['fas', 'phone']" class="w-4 h-4 text-gray-400" />
                <a :href="`tel:${annonce.contact_telephone}`" class="hover:text-custom-green">{{ annonce.contact_telephone }}</a>
              </p>
              <p v-if="annonce.contact_email" class="flex items-center gap-2 text-sm text-gray-600">
                <font-awesome-icon :icon="['fas', 'envelope']" class="w-4 h-4 text-gray-400" />
                <a :href="`mailto:${annonce.contact_email}`" class="hover:text-custom-green">{{ annonce.contact_email }}</a>
              </p>
              <p v-if="annonce.contact_adresse" class="flex items-center gap-2 text-sm text-gray-600">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-gray-400" />
                {{ annonce.contact_adresse }}
              </p>
            </div>

            <!-- Site web ou page réseau social -->
            <a
              v-if="annonce.site_web_url"
              :href="annonce.site_web_url"
              target="_blank"
              rel="noopener noreferrer"
              class="mt-3 inline-flex items-center gap-2 text-sm font-medium text-custom-green hover:underline"
            >
              <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-4 h-4" />
              Site web / réseau social
            </a>
          </div>

          <!-- Actions -->
          <div class="p-6 md:p-8">
            <!-- Propriétaire de l'annonce (FR-013) -->
            <div
              v-if="estProprietaire"
              class="bg-emerald-50 border border-emerald-200 rounded-xl p-6 text-center"
            >
              <font-awesome-icon :icon="['fas', 'circle-info']" class="w-8 h-8 text-emerald-500 mx-auto mb-3" />
              <p class="text-emerald-800 font-medium mb-4">Ceci est votre annonce.</p>
              <NuxtLink
                to="/marche-africain/mes-annonces"
                class="inline-flex items-center gap-2 px-6 py-3 bg-custom-green text-white font-medium rounded-xl hover:bg-custom-green/90 transition-colors"
              >
                <font-awesome-icon :icon="['fas', 'sliders']" class="w-4 h-4" />
                Gérer mes annonces
              </NuxtLink>
            </div>

            <!-- Si authentifié (et pas propriétaire) -->
            <div v-else-if="isAuthenticated" class="space-y-4">
              <button
                @click="ouvrirContact"
                class="w-full py-4 font-semibold rounded-xl transition-all flex items-center justify-center gap-3 bg-linear-to-r from-emerald-500 to-teal-500 text-white hover:from-emerald-600 hover:to-teal-600 shadow-lg hover:shadow-xl"
              >
                <font-awesome-icon :icon="['fas', 'hand-point-up']" class="w-5 h-5" />
                Contacter l'auteur
              </button>

              <MarcheFavoriBouton
                :annonce-id="annonce.id"
                avec-libelle
                variante="detail"
                class="w-full"
              />
            </div>

            <!-- Si non authentifié -->
            <div
              v-else
              class="bg-amber-50 border border-amber-200 rounded-xl p-6 text-center"
            >
              <font-awesome-icon
                :icon="['fas', 'lock']"
                class="w-8 h-8 text-amber-500 mx-auto mb-3"
              />
              <p class="text-amber-800 font-medium mb-4">
                Connectez-vous pour contacter l'auteur
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
            to="/marche-africain"
            class="inline-flex items-center gap-2 text-gray-600 hover:text-emerald-600 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
            Retour à la liste des annonces
          </NuxtLink>
        </div>
      </div>

      <!-- Modal de contact -->
      <Teleport to="body">
        <Transition
          enter-active-class="transition ease-out duration-300"
          enter-from-class="opacity-0"
          enter-to-class="opacity-100"
          leave-active-class="transition ease-in duration-200"
          leave-from-class="opacity-100"
          leave-to-class="opacity-0"
        >
          <div v-if="showContactModal" class="fixed inset-0 z-50 flex items-center justify-center p-4">
            <div class="absolute inset-0 bg-black/50" @click="showContactModal = false"></div>
            <div class="relative bg-white rounded-2xl shadow-xl max-w-md w-full p-6">
              <h2 class="text-xl font-bold text-gray-800 mb-1">Contacter l'auteur</h2>
              <p class="text-sm text-gray-500 mb-4">
                À propos de : <span class="font-medium text-gray-700">{{ annonce.titre }}</span>
              </p>
              <textarea
                v-model="messageContact"
                rows="4"
                maxlength="2000"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
                placeholder="Bonjour, je suis intéressé(e) par votre annonce…"
              ></textarea>
              <p v-if="erreurContact" class="text-sm text-red-600 mt-2">{{ erreurContact }}</p>
              <div class="flex items-center justify-end gap-3 mt-4">
                <button
                  type="button"
                  class="px-5 py-2.5 rounded-xl border border-gray-300 text-gray-600 hover:bg-gray-50"
                  @click="showContactModal = false"
                >
                  Annuler
                </button>
                <button
                  type="button"
                  :disabled="contactEnCours"
                  class="px-6 py-2.5 rounded-xl bg-linear-to-r from-emerald-500 to-teal-500 text-white font-semibold hover:from-emerald-600 hover:to-teal-600 disabled:opacity-60 flex items-center gap-2"
                  @click="envoyerContact"
                >
                  <span v-if="contactEnCours" class="animate-spin rounded-full h-4 w-4 border-2 border-white border-t-transparent"></span>
                  Envoyer
                </button>
              </div>
            </div>
          </div>
        </Transition>
      </Teleport>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import {
  useMarcheAfricain,
  formatPrix,
  formatDate,
  type AnnonceDetailAPI,
  type TypeEchange,
} from '~/composables/useMarcheAfricain'
import { useMessagerie } from '~/composables/useMessagerie'
import { useUserStore } from '~/stores/user'
import { navigateTo } from '#app'

const route = useRoute()
const userStore = useUserStore()
const { obtenirAnnonce, contacterAuteur, erreur } = useMarcheAfricain()
const { listerConversations, demanderOuverture } = useMessagerie()

// State
const loading = ref(true)
const annonce = ref<AnnonceDetailAPI | null>(null)
const showContactModal = ref(false)
const messageContact = ref('')
const contactEnCours = ref(false)
const erreurContact = ref<string | null>(null)

// Computed
const isAuthenticated = computed(() => userStore.isAuthenticated)
const estProprietaire = computed(
  () => isAuthenticated.value && annonce.value?.user.uid === userStore.user?.id,
)

const prixFormate = computed(() => {
  if (!annonce.value) return ''
  return formatPrix(annonce.value.prix, annonce.value.devise)
})

const dateFormatee = computed(() => {
  if (!annonce.value) return ''
  return formatDate(annonce.value.created_at)
})

const breadcrumbs = computed(() => [
  { label: 'Marché Africain', to: '/marche-africain' },
  { label: annonce.value?.titre || 'Détail', to: undefined },
])

// Badges de credibilite de l'annonceur (vert = verifie, rouge = non verifie)
const badgesCredibilite = computed(() => {
  const u = annonce.value?.user
  return [
    {
      cle: 'telephone',
      icone: 'phone',
      libelle: 'Téléphone',
      valide: !!u?.telephone_verifie,
      libelleValide: 'Numéro de téléphone validé par OTP',
      libelleInvalide: 'Numéro de téléphone non validé',
    },
    {
      cle: 'identite',
      icone: 'id-card',
      libelle: 'Identité',
      valide: !!u?.documents_verifie,
      libelleValide: "Pièce d'identité vérifiée",
      libelleInvalide: "Pièce d'identité non vérifiée",
    },
    {
      cle: 'compte',
      icone: 'user-shield',
      libelle: 'Compte validé',
      valide: !!u?.compte_valide,
      libelleValide: "Compte validé par l'administration",
      libelleInvalide: "Compte non validé par l'administration",
    },
  ]
})

const paysAffiche = computed(() => {
  if (!annonce.value) return ''
  if (annonce.value.pays.length > 0) {
    return annonce.value.pays.join(', ')
  }
  return 'Non spécifié'
})

// Methods
const getTypeColor = (type: string): string => {
  switch (type as TypeEchange) {
    case 'Vente':
      return 'bg-white text-gray-700'
    case 'Troc':
      return 'bg-purple-100 text-purple-700'
    case 'Don':
      return 'bg-blue-100 text-blue-700'
    case "Opportunité d'investissement":
      return 'bg-amber-100 text-amber-700'
    default:
      return 'bg-gray-100 text-gray-700'
  }
}

const ouvrirContact = () => {
  if (!isAuthenticated.value) {
    navigateTo('/login')
    return
  }
  erreurContact.value = null
  showContactModal.value = true
}

const envoyerContact = async () => {
  if (!annonce.value) return
  if (messageContact.value.trim().length === 0) {
    erreurContact.value = 'Veuillez écrire un message.'
    return
  }
  contactEnCours.value = true
  erreurContact.value = null
  try {
    const resultat = await contacterAuteur(annonce.value.id, messageContact.value.trim())
    if (resultat) {
      showContactModal.value = false
      messageContact.value = ''
      // Ouvre la fenêtre de messagerie sur la conversation avec l'auteur
      await listerConversations()
      demanderOuverture({
        id: resultat.ami_id,
        nom: annonce.value.user.nom,
        prenom: annonce.value.user.prenom,
        slug: null,
        photoUrl: null,
        fonction: null,
        pays: null,
      })
    } else {
      erreurContact.value = erreur.value || "Impossible de contacter l'auteur."
    }
  } finally {
    contactEnCours.value = false
  }
}

// Lifecycle
onMounted(async () => {
  const id = route.params.id as string
  const resultat = await obtenirAnnonce(id)
  annonce.value = resultat

  if (annonce.value) {
    useHead({
      title: `${annonce.value.titre} - Marché Africain - AfricanS`,
      meta: [
        {
          name: 'description',
          content: annonce.value.description.substring(0, 160),
        },
      ],
    })
  }

  loading.value = false
})
</script>
