<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative bg-gradient-to-br from-custom-green via-emerald-600 to-teal-600 overflow-hidden">
      <!-- Background pattern -->
      <div class="absolute inset-0 opacity-10">
        <div class="absolute inset-0" style="background-image: url('data:image/svg+xml,%3Csvg width=&quot;60&quot; height=&quot;60&quot; viewBox=&quot;0 0 60 60&quot; xmlns=&quot;http://www.w3.org/2000/svg&quot;%3E%3Cg fill=&quot;none&quot; fill-rule=&quot;evenodd&quot;%3E%3Cg fill=&quot;%23ffffff&quot; fill-opacity=&quot;0.4&quot;%3E%3Cpath d=&quot;M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z&quot;/%3E%3C/g%3E%3C/g%3E%3C/svg%3E');" />
      </div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center text-white select-none">
        <!-- Badge -->
        <div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-xs px-4 py-2 rounded-full text-sm font-medium mb-4">
          <font-awesome-icon :icon="['fas', 'handshake']" class="w-4 h-4" />
          Rejoignez notre réseau
        </div>

        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Devenir Partenaire
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Rejoignez le réseau UAfricas et contribuez au développement durable du continent africain en devenant partenaire de notre plateforme.
          </p>
        </div>
      </div>

      <!-- Wave bottom -->
      <div class="absolute bottom-0 left-0 right-0">
        <svg class="w-full h-12 md:h-16" viewBox="0 0 1440 54" fill="none" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none">
          <path d="M0 22L60 16.7C120 11 240 1 360 0.7C480 1 600 11 720 16.7C840 22 960 22 1080 16.7C1200 11 1320 1 1380 0.7L1440 0V54H1380C1320 54 1200 54 1080 54C960 54 840 54 720 54C600 54 480 54 360 54C240 54 120 54 60 54H0V22Z" fill="#f9fafb"/>
        </svg>
      </div>
    </div>

    <!-- Breadcrumb -->
    <div class="bg-gray-50">
      <div class="max-w-7xl mx-auto px-4 py-4">
        <CommonBreadcrumbNav />
      </div>
    </div>

    <!-- Formulaire -->
    <section class="px-4 md:px-16 lg:px-64 py-8">
      <div class="bg-white rounded-md shadow-lg border-t-8 border-custom-green max-w-3xl mx-auto">
        <div class="px-6 md:px-10 py-8">
          <h1 class="text-2xl font-bold text-custom-chocolat mb-2">
            Demande de partenariat
          </h1>
          <p class="text-gray-500 text-sm mb-8">
            Remplissez ce formulaire pour soumettre votre demande de partenariat avec UAfricas.
            Les champs marqués d'un <span class="text-red-500">*</span> sont obligatoires.
          </p>

          <!-- Message de succès -->
          <div
            v-if="succes"
            class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg text-sm mb-6 flex items-center gap-2"
          >
            <font-awesome-icon :icon="['fas', 'circle-check']" />
            Votre demande de partenariat a été soumise avec succès ! Elle sera examinée par notre équipe.
          </div>

          <!-- Message d'erreur -->
          <div
            v-if="erreurMessage"
            class="bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-lg text-sm mb-6 flex items-center gap-2"
          >
            <font-awesome-icon :icon="['fas', 'circle-exclamation']" />
            {{ erreurMessage }}
          </div>

          <form @submit.prevent="handleSubmit" class="space-y-6">
            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 1 : Type de partenariat             -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset>
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'handshake']" class="text-custom-green" />
                Type de partenariat
              </legend>

              <!-- Radio cards -->
              <div class="space-y-3 mb-4">
                <label
                  v-for="type in TYPES_PARTENARIAT"
                  :key="type.value"
                  class="flex items-start gap-3 p-4 border-2 rounded-lg cursor-pointer transition-all"
                  :class="form.typePartenariat === type.value
                    ? 'border-custom-green bg-custom-green/10'
                    : 'border-gray-200 hover:border-gray-300'"
                >
                  <input
                    v-model="form.typePartenariat"
                    type="radio"
                    name="typePartenariat"
                    :value="type.value"
                    class="mt-1 accent-custom-green"
                  />
                  <div>
                    <div class="font-medium text-gray-800">{{ type.label }}</div>
                    <div class="text-sm text-gray-500">{{ type.description }}</div>
                  </div>
                </label>
              </div>

              <!-- Motivation -->
              <div>
                <label for="description-partenariat" class="block text-sm font-medium text-gray-700 mb-1">
                  Motivation / Description du partenariat <span class="text-red-500">*</span>
                </label>
                <textarea
                  id="description-partenariat"
                  v-model="form.descriptionPartenariat"
                  rows="4"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Décrivez votre motivation et ce que vous souhaitez apporter en tant que partenaire..."
                />
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 2 : Informations de l'organisation  -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'building']" class="text-custom-green" />
                Informations de l'organisation
              </legend>

              <!-- Dénomination -->
              <div class="mb-4">
                <label for="denomination" class="block text-sm font-medium text-gray-700 mb-1">
                  Nom de l'organisation <span class="text-red-500">*</span>
                </label>
                <input
                  id="denomination"
                  v-model="form.denomination"
                  type="text"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                  placeholder="Ex: ONG Développement Durable Afrique"
                />
              </div>

              <!-- Type d'organisation -->
              <div class="mb-4">
                <label for="type-organisation" class="block text-sm font-medium text-gray-700 mb-1">
                  Type d'organisation <span class="text-red-500">*</span>
                </label>
                <select
                  id="type-organisation"
                  v-model="form.typeOrganisation"
                  class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden"
                >
                  <option value="" disabled>Choisir un type</option>
                  <option
                    v-for="type in TYPES_ORGANISATION"
                    :key="type.value"
                    :value="type.value"
                  >
                    {{ type.label }}
                  </option>
                </select>
              </div>

              <!-- Description de l'organisation -->
              <div class="mb-4">
                <label for="description-organisation" class="block text-sm font-medium text-gray-700 mb-1">
                  Description de l'organisation
                </label>
                <textarea
                  id="description-organisation"
                  v-model="form.descriptionOrganisation"
                  rows="3"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Présentez brièvement votre organisation, ses missions et ses réalisations..."
                />
              </div>

              <!-- Numéro de registre -->
              <div>
                <label for="numero-registre" class="block text-sm font-medium text-gray-700 mb-1">
                  Numéro de registre de commerce
                </label>
                <input
                  id="numero-registre"
                  v-model="form.numeroRegistre"
                  type="text"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                  placeholder="Ex: RCCM-CI-ABJ-2024-B-12345"
                />
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 3 : Localisation                    -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'earth-africa']" class="text-custom-green" />
                Localisation
              </legend>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                <div>
                  <label for="pays" class="block text-sm font-medium text-gray-700 mb-1">
                    Territoire
                  </label>
                  <select
                    id="pays"
                    v-model="form.pays"
                    class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden"
                  >
                    <option value="" disabled>Choisir un territoire</option>
                    <option
                      v-for="pays in PAYS_PARTENARIAT"
                      :key="pays.value"
                      :value="pays.value"
                    >
                      {{ pays.label }}
                    </option>
                  </select>
                </div>
                <div>
                  <label for="ville" class="block text-sm font-medium text-gray-700 mb-1">
                    Ville
                  </label>
                  <input
                    id="ville"
                    v-model="form.ville"
                    type="text"
                    class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                    placeholder="Ex: Abidjan"
                  />
                </div>
              </div>

              <div>
                <label for="adresse" class="block text-sm font-medium text-gray-700 mb-1">
                  Adresse complète
                </label>
                <textarea
                  id="adresse"
                  v-model="form.adresse"
                  rows="2"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Ex: Plateau, Rue du Commerce, Immeuble Alpha, 3e étage"
                />
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 4 : Contact                         -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'address-book']" class="text-custom-green" />
                Contact
              </legend>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                <div>
                  <label for="email" class="block text-sm font-medium text-gray-700 mb-1">
                    Email de l'organisation
                  </label>
                  <input
                    id="email"
                    v-model="form.email"
                    type="email"
                    class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                    placeholder="contact@organisation.org"
                  />
                </div>
                <div>
                  <label for="telephone" class="block text-sm font-medium text-gray-700 mb-1">
                    Téléphone
                  </label>
                  <input
                    id="telephone"
                    v-model="form.telephone"
                    type="tel"
                    class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                    placeholder="+225 07 12 34 56 78"
                  />
                </div>
              </div>

              <div>
                <label for="site-web" class="block text-sm font-medium text-gray-700 mb-1">
                  Site web
                </label>
                <input
                  id="site-web"
                  v-model="form.siteWeb"
                  type="url"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                  placeholder="https://www.exemple.org"
                />
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 5 : Documents                       -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'image']" class="text-custom-green" />
                Documents
              </legend>

              <!-- Logo -->
              <div class="mb-4">
                <div class="p-3 bg-custom-green/20 border border-custom-green rounded-md">
                  <label for="logo" class="block text-sm font-medium text-custom-green mb-1">
                    Logo de l'organisation (5 Mo max)
                  </label>
                  <input
                    id="logo"
                    type="file"
                    accept="image/*"
                    @change="handleLogoChange"
                    class="w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:bg-custom-green file:text-white hover:file:bg-custom-green/90"
                  />
                </div>

                <!-- Preview logo -->
                <div v-if="logoPreview" class="mt-3 relative inline-block">
                  <img
                    :src="logoPreview"
                    alt="Aperçu du logo"
                    class="w-32 h-32 object-contain rounded-lg border border-gray-200 bg-white p-2"
                  />
                  <button
                    type="button"
                    class="absolute -top-2 -right-2 w-6 h-6 bg-red-500 text-white rounded-full flex items-center justify-center text-xs hover:bg-red-600 transition-colors"
                    @click="supprimerLogo"
                  >
                    <font-awesome-icon :icon="['fas', 'times']" />
                  </button>
                </div>
              </div>

              <!-- Document légal -->
              <div>
                <div class="p-3 bg-custom-chocolat/10 border border-custom-chocolat/30 rounded-md">
                  <label for="document-legal" class="block text-sm font-medium text-custom-chocolat mb-1">
                    Document légal — PDF (10 Mo max)
                  </label>
                  <input
                    id="document-legal"
                    type="file"
                    accept="application/pdf"
                    @change="handleDocumentLegalChange"
                    class="w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:bg-custom-chocolat file:text-white hover:file:bg-custom-chocolat/90"
                  />
                </div>

                <!-- Nom du fichier sélectionné -->
                <div v-if="documentLegalNom" class="mt-3 flex items-center gap-2 text-sm text-gray-600">
                  <font-awesome-icon :icon="['fas', 'file-lines']" class="text-custom-chocolat" />
                  <span>{{ documentLegalNom }}</span>
                  <button
                    type="button"
                    class="ml-2 text-red-400 hover:text-red-600 transition-colors"
                    @click="supprimerDocumentLegal"
                  >
                    <font-awesome-icon :icon="['fas', 'times']" class="w-3 h-3" />
                  </button>
                </div>
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Boutons d'action                            -->
            <!-- ═══════════════════════════════════════════ -->
            <div class="flex flex-col sm:flex-row gap-3 justify-center pt-4 border-t">
              <NuxtLink
                to="/"
                class="px-6 py-2.5 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 transition-colors text-center"
              >
                Annuler
              </NuxtLink>
              <button
                type="submit"
                :disabled="!isFormValid || loading"
                class="px-6 py-2.5 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
              >
                <font-awesome-icon
                  v-if="loading"
                  :icon="['fas', 'spinner']"
                  class="animate-spin"
                />
                {{ loading ? 'Soumission en cours...' : 'Soumettre la demande' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import {
  usePartenariat,
  TYPES_PARTENARIAT,
  TYPES_ORGANISATION,
  PAYS_PARTENARIAT,
} from '~/composables/usePartenariat'
import { useUserStore } from '~/stores/user'

useHead({
  title: 'Devenir Partenaire - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Devenez partenaire de UAfricas et contribuez au développement durable du continent africain.',
    },
  ],
})

const router = useRouter()
const userStore = useUserStore()
const { soumettreDemande, chargement } = usePartenariat()
const { redirigerVersConnexion } = useAuth()

// Auth guard
onMounted(() => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
  }
})

// Form state
const form = reactive({
  // Partenariat
  typePartenariat: '',
  descriptionPartenariat: '',
  // Organisation
  denomination: '',
  typeOrganisation: '',
  descriptionOrganisation: '',
  numeroRegistre: '',
  // Localisation
  pays: '',
  ville: '',
  adresse: '',
  // Contact
  email: '',
  telephone: '',
  siteWeb: '',
})

// Fichiers
const logoFile = ref<File | null>(null)
const logoPreview = ref<string | null>(null)
const documentLegalFile = ref<File | null>(null)
const documentLegalNom = ref<string | null>(null)

const loading = computed(() => chargement.value)
const succes = ref(false)
const erreurMessage = ref<string | null>(null)

// Validation
const isFormValid = computed(() => {
  return (
    form.typePartenariat.trim().length > 0 &&
    form.denomination.trim().length > 0 &&
    form.typeOrganisation.trim().length > 0 &&
    form.descriptionPartenariat.trim().length > 0
  )
})

// ── Gestion du logo ──────────────────────────────────────────
const handleLogoChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (!target.files || !target.files[0]) return

  const file = target.files[0]

  if (file.size > 5 * 1024 * 1024) {
    erreurMessage.value = 'Le logo ne doit pas dépasser 5 Mo.'
    target.value = ''
    return
  }

  logoFile.value = file

  const reader = new FileReader()
  reader.onload = (e) => {
    logoPreview.value = e.target?.result as string
  }
  reader.readAsDataURL(file)
}

const supprimerLogo = () => {
  logoFile.value = null
  logoPreview.value = null
  const input = document.getElementById('logo') as HTMLInputElement
  if (input) input.value = ''
}

// ── Gestion du document légal ────────────────────────────────
const handleDocumentLegalChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (!target.files || !target.files[0]) return

  const file = target.files[0]

  if (file.size > 10 * 1024 * 1024) {
    erreurMessage.value = 'Le document ne doit pas dépasser 10 Mo.'
    target.value = ''
    return
  }

  documentLegalFile.value = file
  documentLegalNom.value = file.name
}

const supprimerDocumentLegal = () => {
  documentLegalFile.value = null
  documentLegalNom.value = null
  const input = document.getElementById('document-legal') as HTMLInputElement
  if (input) input.value = ''
}

// ── Reset ────────────────────────────────────────────────────
const resetForm = () => {
  form.typePartenariat = ''
  form.descriptionPartenariat = ''
  form.denomination = ''
  form.typeOrganisation = ''
  form.descriptionOrganisation = ''
  form.numeroRegistre = ''
  form.pays = ''
  form.ville = ''
  form.adresse = ''
  form.email = ''
  form.telephone = ''
  form.siteWeb = ''
  supprimerLogo()
  supprimerDocumentLegal()
}

// ── Submit ───────────────────────────────────────────────────
const handleSubmit = async () => {
  if (!isFormValid.value) return

  erreurMessage.value = null
  succes.value = false

  try {
    const result = await soumettreDemande(
      {
        type_partenariat: form.typePartenariat,
        description_partenariat: form.descriptionPartenariat.trim(),
        denomination: form.denomination.trim(),
        type_organisation: form.typeOrganisation,
        description_organisation: form.descriptionOrganisation || undefined,
        numero_registre: form.numeroRegistre || undefined,
        pays: form.pays || undefined,
        ville: form.ville || undefined,
        adresse: form.adresse || undefined,
        email: form.email || undefined,
        telephone: form.telephone || undefined,
        site_web: form.siteWeb || undefined,
      },
      logoFile.value,
      documentLegalFile.value,
    )

    if (result) {
      succes.value = true
      resetForm()
      window.scrollTo({ top: 0, behavior: 'smooth' })
    } else {
      erreurMessage.value = 'Une erreur est survenue lors de la soumission de votre demande.'
    }
  } catch (e: any) {
    erreurMessage.value = e?.message || 'Une erreur est survenue lors de la soumission.'
  }
}
</script>
