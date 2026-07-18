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
          <font-awesome-icon :icon="['fas', 'paper-plane']" class="w-4 h-4" />
          Proposez votre initiative
        </div>

        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Soumettre un Projet
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Présentez votre projet de développement et bénéficiez du soutien de notre communauté d'investisseurs et de partenaires africains.
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
            Soumettre un projet de développement
          </h1>
          <p class="text-gray-500 text-sm mb-8">
            Présentez votre projet pour bénéficier du soutien de notre communauté d'investisseurs et de partenaires africains.
            Les champs marqués d'un <span class="text-red-500">*</span> sont obligatoires.
          </p>

          <!-- Message de succès -->
          <div
            v-if="succes"
            class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg text-sm mb-6 flex items-center gap-2"
          >
            <font-awesome-icon :icon="['fas', 'circle-check']" />
            Votre projet a été soumis avec succès ! Il sera examiné par notre équipe avant publication.
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
            <!-- Section 1 : Informations principales       -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset>
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'circle-info']" class="text-custom-green" />
                Informations principales
              </legend>

              <!-- Titre -->
              <div class="mb-4">
                <label for="titre" class="block text-sm font-medium text-gray-700 mb-1">
                  Titre du projet <span class="text-red-500">*</span>
                </label>
                <input
                  id="titre"
                  v-model="form.titre"
                  type="text"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                  placeholder="Ex: Construction d'un centre de formation numérique"
                />
              </div>

              <!-- Description -->
              <div class="mb-4">
                <label for="description" class="block text-sm font-medium text-gray-700 mb-1">
                  Description du projet <span class="text-red-500">*</span>
                </label>
                <textarea
                  id="description"
                  v-model="form.description"
                  rows="5"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Décrivez votre projet en détail : contexte, besoins identifiés, approche proposée..."
                />
              </div>

              <!-- Objectifs dynamiques -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">
                  Objectifs du projet <span class="text-red-500">*</span>
                </label>
                <div class="space-y-2">
                  <div
                    v-for="(_, index) in objectifs"
                    :key="index"
                    class="flex items-center gap-2"
                  >
                    <span class="text-xs text-gray-400 w-6 text-right">{{ index + 1 }}.</span>
                    <input
                      v-model="objectifs[index]"
                      type="text"
                      class="flex-1 border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green text-sm"
                      :placeholder="`Objectif ${index + 1}`"
                    />
                    <button
                      v-if="objectifs.length > 1"
                      type="button"
                      class="p-2 text-red-400 hover:text-red-600 transition-colors"
                      @click="supprimerObjectif(index)"
                    >
                      <font-awesome-icon :icon="['fas', 'trash']" class="w-4 h-4" />
                    </button>
                  </div>
                </div>
                <button
                  type="button"
                  class="mt-2 text-sm text-custom-green hover:text-custom-green/80 flex items-center gap-1 transition-colors"
                  @click="ajouterObjectif"
                >
                  <font-awesome-icon :icon="['fas', 'plus']" class="w-3 h-3" />
                  Ajouter un objectif
                </button>
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 2 : Organisation                    -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'building']" class="text-custom-green" />
                Organisation porteuse
              </legend>

              <div class="mb-4">
                <label for="nom-organisation" class="block text-sm font-medium text-gray-700 mb-1">
                  Nom de l'organisation
                </label>
                <input
                  id="nom-organisation"
                  v-model="form.nomOrganisation"
                  type="text"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                  placeholder="Ex: ONG Développement Durable Afrique"
                />
              </div>

              <div class="mb-4">
                <label for="description-organisation" class="block text-sm font-medium text-gray-700 mb-1">
                  Description de l'organisation
                </label>
                <textarea
                  id="description-organisation"
                  v-model="form.descriptionOrganisation"
                  rows="3"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Présentez brièvement votre organisation..."
                />
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
            <!-- Section 3 : Localisation                    -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'globe-africa']" class="text-custom-green" />
                Localisation
              </legend>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
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
                      v-for="pays in paysOptions"
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
                    placeholder="Ex: Dakar"
                  />
                </div>
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 4 : Budget et durée                 -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'coins']" class="text-custom-green" />
                Budget et durée
              </legend>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                <div>
                  <label for="cout-total" class="block text-sm font-medium text-gray-700 mb-1">
                    Coût total estimé
                  </label>
                  <input
                    id="cout-total"
                    v-model.number="form.coutTotal"
                    type="number"
                    min="0"
                    step="1000"
                    class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                    placeholder="Ex: 5000000"
                  />
                </div>
                <div>
                  <label for="devise" class="block text-sm font-medium text-gray-700 mb-1">
                    Devise
                  </label>
                  <select
                    id="devise"
                    v-model="form.devise"
                    class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden"
                  >
                    <option
                      v-for="devise in DEVISES_FORM"
                      :key="devise.value"
                      :value="devise.value"
                    >
                      {{ devise.label }}
                    </option>
                  </select>
                </div>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label for="duree" class="block text-sm font-medium text-gray-700 mb-1">
                    Durée du projet
                  </label>
                  <select
                    id="duree"
                    v-model="form.dureeMois"
                    class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden"
                  >
                    <option :value="null" disabled>Choisir une durée</option>
                    <option
                      v-for="duree in DUREES_MOIS_FORM"
                      :key="duree.value"
                      :value="duree.value"
                    >
                      {{ duree.label }}
                    </option>
                  </select>
                </div>
                <div>
                  <label for="date-debut" class="block text-sm font-medium text-gray-700 mb-1">
                    Date de début souhaitée
                  </label>
                  <input
                    id="date-debut"
                    v-model="form.dateDebutSouhaitee"
                    type="date"
                    class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                  />
                </div>
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 5 : Détails du projet               -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'clipboard-list']" class="text-custom-green" />
                Détails du projet
              </legend>

              <div class="mb-4">
                <label for="resultats-attendus" class="block text-sm font-medium text-gray-700 mb-1">
                  Résultats attendus
                </label>
                <textarea
                  id="resultats-attendus"
                  v-model="form.resultatsAttendus"
                  rows="3"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Quels résultats concrets attendez-vous ?"
                />
              </div>

              <div class="mb-4">
                <label for="activites-programmees" class="block text-sm font-medium text-gray-700 mb-1">
                  Activités programmées
                </label>
                <textarea
                  id="activites-programmees"
                  v-model="form.activitesProgrammees"
                  rows="3"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Quelles activités sont prévues pour atteindre les objectifs ?"
                />
              </div>

              <div class="mb-4">
                <label for="echeanciers" class="block text-sm font-medium text-gray-700 mb-1">
                  Échéanciers
                </label>
                <textarea
                  id="echeanciers"
                  v-model="form.echeanciers"
                  rows="3"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Décrivez le calendrier prévu pour les différentes phases du projet..."
                />
              </div>

              <div class="mb-4">
                <label for="contribution-autonomisation" class="block text-sm font-medium text-gray-700 mb-1">
                  Contribution à l'autonomisation
                </label>
                <textarea
                  id="contribution-autonomisation"
                  v-model="form.contributionAutonomisation"
                  rows="3"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Comment ce projet contribue-t-il à l'autonomisation des communautés ?"
                />
              </div>

              <div>
                <label for="difficultes-risques" class="block text-sm font-medium text-gray-700 mb-1">
                  Difficultés et risques
                </label>
                <textarea
                  id="difficultes-risques"
                  v-model="form.difficultesRisques"
                  rows="3"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-y"
                  placeholder="Quels sont les risques identifiés et les mesures de mitigation prévues ?"
                />
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 6 : Contact                         -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'address-book']" class="text-custom-green" />
                Contact
              </legend>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label for="contact-email" class="block text-sm font-medium text-gray-700 mb-1">
                    Email de contact
                  </label>
                  <input
                    id="contact-email"
                    v-model="form.contactEmail"
                    type="email"
                    class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                    placeholder="contact@organisation.org"
                  />
                </div>
                <div>
                  <label for="contact-telephone" class="block text-sm font-medium text-gray-700 mb-1">
                    Téléphone
                  </label>
                  <input
                    id="contact-telephone"
                    v-model="form.contactTelephone"
                    type="tel"
                    class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                    placeholder="+221 77 123 45 67"
                  />
                </div>
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Section 7 : Image de couverture             -->
            <!-- ═══════════════════════════════════════════ -->
            <fieldset class="border-t pt-6">
              <legend class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'image']" class="text-custom-green" />
                Image de couverture
              </legend>

              <div class="p-3 bg-custom-green/20 border border-custom-green rounded-md">
                <label for="couverture" class="block text-sm font-medium text-custom-green mb-1">
                  Choisir une image (5 Mo max)
                </label>
                <input
                  id="couverture"
                  type="file"
                  accept="image/*"
                  @change="handleCouvertureChange"
                  class="w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:bg-custom-green file:text-white hover:file:bg-custom-green/90"
                />
              </div>

              <!-- Preview -->
              <div v-if="couverturePreview" class="mt-3 relative inline-block">
                <img
                  :src="couverturePreview"
                  alt="Aperçu de la couverture"
                  class="w-48 h-32 object-cover rounded-lg border border-gray-200"
                />
                <button
                  type="button"
                  class="absolute -top-2 -right-2 w-6 h-6 bg-red-500 text-white rounded-full flex items-center justify-center text-xs hover:bg-red-600 transition-colors"
                  @click="supprimerCouverture"
                >
                  <font-awesome-icon :icon="['fas', 'times']" />
                </button>
              </div>
            </fieldset>

            <!-- ═══════════════════════════════════════════ -->
            <!-- Boutons d'action                            -->
            <!-- ═══════════════════════════════════════════ -->
            <div class="flex flex-col sm:flex-row gap-3 justify-center pt-4 border-t">
              <NuxtLink
                to="/financer-projet"
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
                {{ loading ? 'Soumission en cours...' : 'Soumettre le projet' }}
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
  useProjets,
  PAYS_PROJETS,
  DEVISES_FORM,
  DUREES_MOIS_FORM,
} from '~/composables/useProjets'
import { useUserStore } from '~/stores/user'

useHead({
  title: 'Soumettre un Projet - AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Soumettez votre projet de développement africain et bénéficiez du soutien de notre communauté.',
    },
  ],
})

const router = useRouter()
const userStore = useUserStore()
const { creerProjet, chargement } = useProjets()

// Auth guard
onMounted(() => {
  if (!userStore.isAuthenticated) {
    router.push('/login')
  }
})

// Options pays (sans l'option "Tous les pays")
const paysOptions = PAYS_PROJETS.filter(p => p.value !== '')

// Form state
const form = reactive({
  titre: '',
  description: '',
  nomOrganisation: '',
  descriptionOrganisation: '',
  siteWeb: '',
  pays: '',
  ville: '',
  coutTotal: null as number | null,
  devise: 'XOF',
  dureeMois: null as number | null,
  dateDebutSouhaitee: '',
  resultatsAttendus: '',
  activitesProgrammees: '',
  echeanciers: '',
  contributionAutonomisation: '',
  difficultesRisques: '',
  contactEmail: '',
  contactTelephone: '',
})

const objectifs = reactive<string[]>([''])
const couvertureFile = ref<File | null>(null)
const couverturePreview = ref<string | null>(null)
const loading = computed(() => chargement.value)
const succes = ref(false)
const erreurMessage = ref<string | null>(null)

// Validation
const isFormValid = computed(() => {
  return (
    form.titre.trim().length > 0 &&
    form.description.trim().length > 0 &&
    objectifs.some(o => o.trim().length > 0)
  )
})

// Objectifs
const ajouterObjectif = () => {
  objectifs.push('')
}

const supprimerObjectif = (index: number) => {
  objectifs.splice(index, 1)
}

// Couverture
const handleCouvertureChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (!target.files || !target.files[0]) return

  const file = target.files[0]

  // Vérifier la taille (5 Mo max)
  if (file.size > 5 * 1024 * 1024) {
    erreurMessage.value = 'L\'image ne doit pas dépasser 5 Mo.'
    target.value = ''
    return
  }

  couvertureFile.value = file

  // Preview
  const reader = new FileReader()
  reader.onload = (e) => {
    couverturePreview.value = e.target?.result as string
  }
  reader.readAsDataURL(file)
}

const supprimerCouverture = () => {
  couvertureFile.value = null
  couverturePreview.value = null
  // Reset file input
  const input = document.getElementById('couverture') as HTMLInputElement
  if (input) input.value = ''
}

// Reset
const resetForm = () => {
  form.titre = ''
  form.description = ''
  form.nomOrganisation = ''
  form.descriptionOrganisation = ''
  form.siteWeb = ''
  form.pays = ''
  form.ville = ''
  form.coutTotal = null
  form.devise = 'XOF'
  form.dureeMois = null
  form.dateDebutSouhaitee = ''
  form.resultatsAttendus = ''
  form.activitesProgrammees = ''
  form.echeanciers = ''
  form.contributionAutonomisation = ''
  form.difficultesRisques = ''
  form.contactEmail = ''
  form.contactTelephone = ''
  objectifs.splice(0, objectifs.length, '')
  supprimerCouverture()
}

// Submit
const handleSubmit = async () => {
  if (!isFormValid.value) return

  erreurMessage.value = null
  succes.value = false

  try {
    // Construire les objectifs en JSON
    const objectifsFiltres = objectifs.filter(o => o.trim().length > 0)
    const objectifsJson = JSON.stringify(objectifsFiltres)

    const result = await creerProjet(
      {
        titre: form.titre.trim(),
        description: form.description.trim(),
        objectifs: objectifsJson,
        nom_organisation: form.nomOrganisation || undefined,
        description_organisation: form.descriptionOrganisation || undefined,
        site_web: form.siteWeb || undefined,
        pays: form.pays || undefined,
        ville: form.ville || undefined,
        contact_email: form.contactEmail || undefined,
        contact_telephone: form.contactTelephone || undefined,
        cout_total: form.coutTotal ?? undefined,
        devise: form.devise || undefined,
        duree_mois: form.dureeMois ?? undefined,
        date_commencement_souhaitee: form.dateDebutSouhaitee || undefined,
        resultats_attendus: form.resultatsAttendus || undefined,
        activites_programmees: form.activitesProgrammees || undefined,
        echeanciers: form.echeanciers || undefined,
        contribution_autonomisation: form.contributionAutonomisation || undefined,
        difficultes_risques: form.difficultesRisques || undefined,
      },
      couvertureFile.value,
    )

    if (result) {
      succes.value = true
      resetForm()
      window.scrollTo({ top: 0, behavior: 'smooth' })
    } else {
      erreurMessage.value = 'Une erreur est survenue lors de la soumission du projet.'
    }
  } catch (e: any) {
    erreurMessage.value = e?.message || 'Une erreur est survenue lors de la soumission.'
  }
}
</script>
