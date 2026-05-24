<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero -->
    <SabbatiqueHero />

    <!-- Breadcrumb -->
    <CommonBreadcrumbNav
      class="mx-4 md:mx-16 lg:mx-64 pt-6"
      :custom-breadcrumbs="[
        { label: 'Échanges Sabbatiques', to: '/echanges-sabbatiques' },
        { label: 'Proposer un échange' },
      ]"
    />

    <!-- Formulaire -->
    <section class="px-4 md:px-16 lg:px-64 py-12">
      <div class="bg-white rounded-md shadow-lg border-t-8 border-custom-green max-w-3xl mx-auto">
        <div class="px-6 md:px-10 py-8">
          <h1 class="text-2xl font-bold text-custom-chocolat mb-2">
            Proposer un projet d'échange sabbatique
          </h1>
          <p class="text-gray-500 text-sm mb-8">
            Les champs marqués d'un <span class="text-red-500">*</span> sont obligatoires.
          </p>

          <!-- Message de succès -->
          <div
            v-if="succes"
            class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg text-sm mb-6 flex items-center gap-2"
          >
            <font-awesome-icon :icon="['fas', 'circle-check']" />
            Votre projet d'échange a été soumis avec succès !
          </div>

          <!-- Message d'erreur -->
          <div
            v-if="erreur"
            class="bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-lg text-sm mb-6 flex items-center gap-2"
          >
            <font-awesome-icon :icon="['fas', 'circle-exclamation']" />
            {{ erreur }}
          </div>

          <form @submit.prevent="handleSubmit" class="space-y-5">
            <!-- Type de programme -->
            <div>
              <label for="type-programme" class="block text-sm font-medium text-gray-700 mb-1">
                Type de programme <span class="text-red-500">*</span>
              </label>
              <div class="flex gap-4">
                <label
                  v-for="type in TYPES_SELECTION"
                  :key="type.value"
                  class="flex-1 cursor-pointer"
                >
                  <input
                    type="radio"
                    name="type-programme"
                    :value="type.value"
                    v-model="form.type"
                    class="hidden"
                  />
                  <div
                    class="text-center py-3 px-4 rounded-md border-2 transition-all text-sm font-medium"
                    :class="form.type === type.value
                      ? 'border-custom-green bg-custom-green/10 text-custom-green'
                      : 'border-gray-200 text-gray-500 hover:border-gray-300'"
                  >
                    <font-awesome-icon
                      :icon="['fas', type.icon]"
                      class="mr-2"
                    />
                    {{ type.label }}
                  </div>
                </label>
              </div>
            </div>

            <!-- Titre -->
            <div>
              <label for="titre" class="block text-sm font-medium text-gray-700 mb-1">
                Titre du projet <span class="text-red-500">*</span>
              </label>
              <input
                id="titre"
                v-model="form.titre"
                type="text"
                class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                placeholder="Ex: Programme d'échange en ingénierie agricole"
              />
            </div>

            <!-- Description (EditorJs) -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">
                Description du projet <span class="text-red-500">*</span>
              </label>
              <CommonEditorJs
                ref="editorRef"
                id="sabbatique-description-editor"
                v-model="form.descriptionData"
                placeholder="Décrivez votre projet d'échange : objectifs, activités prévues, profil recherché..."
                :tools="['header', 'list', 'paragraph', 'quote', 'delimiter', 'marker', 'underline']"
                min-height="200px"
              />
            </div>

            <!-- Domaine -->
            <div>
              <label for="domaine" class="block text-sm font-medium text-gray-700 mb-1">
                Domaine d'intervention <span class="text-red-500">*</span>
              </label>
              <select
                id="domaine"
                v-model="form.domaine"
                class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden"
              >
                <option value="" disabled>Choisir un domaine</option>
                <option
                  v-for="domaine in DOMAINES_FORM"
                  :key="domaine.value"
                  :value="domaine.value"
                >
                  {{ domaine.label }}
                </option>
              </select>
            </div>

            <!-- Pays & Ville -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label for="pays" class="block text-sm font-medium text-gray-700 mb-1">
                  Pays <span class="text-red-500">*</span>
                </label>
                <select
                  id="pays"
                  v-model="form.pays"
                  class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden"
                >
                  <option value="" disabled>Choisir un pays</option>
                  <option
                    v-for="pays in PAYS_FORM"
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

            <!-- Durée du programme -->
            <div>
              <label for="duree" class="block text-sm font-medium text-gray-700 mb-1">
                Durée du programme <span class="text-red-500">*</span>
              </label>
              <select
                id="duree"
                v-model="form.duree"
                class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden"
              >
                <option value="" disabled>Choisir une durée</option>
                <option
                  v-for="duree in DUREES"
                  :key="duree.value"
                  :value="duree.value"
                >
                  {{ duree.label }}
                </option>
              </select>
            </div>

            <!-- Dates début & fin -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label for="date-debut" class="block text-sm font-medium text-gray-700 mb-1">
                  Date & heure de début <span class="text-red-500">*</span>
                </label>
                <input
                  id="date-debut"
                  v-model="form.dateDebut"
                  type="datetime-local"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden"
                />
              </div>
              <div>
                <label for="date-fin" class="block text-sm font-medium text-gray-700 mb-1">
                  Date & heure de fin <span class="text-red-500">*</span>
                </label>
                <input
                  id="date-fin"
                  v-model="form.dateFin"
                  type="datetime-local"
                  class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden"
                />
              </div>
            </div>

            <!-- Prise en charge -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Prise en charge proposée
              </label>
              <div class="flex flex-wrap gap-3">
                <label
                  v-for="prise in PRISES_EN_CHARGE"
                  :key="prise.value"
                  class="flex items-center gap-2 cursor-pointer"
                >
                  <input
                    type="checkbox"
                    :value="prise.value"
                    v-model="form.prisesEnCharge"
                    class="w-4 h-4 text-custom-green border-gray-300 rounded focus:ring-3 focus:ring-custom-green"
                  />
                  <span class="text-sm text-gray-600">{{ prise.label }}</span>
                </label>
              </div>
            </div>

            <!-- Image de couverture -->
            <div class="p-3 bg-custom-green/20 border border-custom-green rounded-md">
              <label for="couverture" class="block text-sm font-medium text-custom-green mb-1">
                Image de couverture
              </label>
              <input
                id="couverture"
                type="file"
                accept="image/*"
                @change="handleCouvertureChange"
                class="w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:bg-custom-green file:text-white hover:file:bg-custom-green/90"
              />
            </div>

            <!-- Document PDF -->
            <div class="p-3 bg-custom-chocolat/10 border border-custom-chocolat/50 rounded-md">
              <label for="document" class="block text-sm font-medium text-custom-chocolat mb-1">
                Document du projet (PDF)
              </label>
              <input
                id="document"
                type="file"
                accept=".pdf"
                @change="handleDocumentChange"
                class="w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:bg-custom-chocolat file:text-white hover:file:bg-custom-chocolat/90"
              />
            </div>

            <!-- Informations organisateur -->
            <div class="border-t pt-5 mt-5">
              <h3 class="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-4">
                Informations de l'organisateur
              </h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label for="org-nom" class="block text-sm font-medium text-gray-700 mb-1">
                    Nom / Organisation
                  </label>
                  <input
                    id="org-nom"
                    v-model="form.organisateurNom"
                    type="text"
                    class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                    placeholder="Ex: ONG Santé Pour Tous"
                  />
                </div>
                <div>
                  <label for="org-email" class="block text-sm font-medium text-gray-700 mb-1">
                    Email de contact
                  </label>
                  <input
                    id="org-email"
                    v-model="form.organisateurEmail"
                    type="email"
                    class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                    placeholder="contact@organisation.org"
                  />
                </div>
              </div>
            </div>

            <!-- Boutons -->
            <div class="flex flex-col sm:flex-row gap-3 justify-center pt-4">
              <NuxtLink
                to="/echanges-sabbatiques"
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
import { reactive, ref, computed, onMounted } from 'vue'
import AOS from 'aos'
import {
  useSabbatiques,
  DOMAINES,
  DUREES,
  PAYS_AFRICAINS,
  PRISES_EN_CHARGE,
  type TypeProgramme,
} from '~/composables/useSabbatiques'
import { editorJsToHtml, type EditorJsData } from '~/composables/useEditorJs'

const route = useRoute()
const { creerProgramme } = useSabbatiques()

const typesValides: TypeProgramme[] = ['interafricain', 'hors_afrique']
const typeParam = route.query.type as string | undefined
const typeInitial = typesValides.includes(typeParam as TypeProgramme) ? (typeParam as TypeProgramme) : ''

useHead({
  title: 'Proposer un projet d\'échange - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Soumettez votre projet d\'échange sabbatique pour contribuer au développement en Afrique'
    }
  ]
})

const TYPES_SELECTION = [
  { value: 'interafricain', label: 'Interafricain', icon: 'earth-africa' },
  { value: 'hors_afrique', label: 'Hors Afrique vers Afrique', icon: 'plane-arrival' }
]

// Constantes sans l'option "Tous" pour le formulaire
const DOMAINES_FORM = DOMAINES.filter(d => d.value !== '')
const PAYS_FORM = PAYS_AFRICAINS.filter(p => p.value !== '')

const editorRef = ref<{ save: () => Promise<EditorJsData | null>; clear: () => Promise<void> } | null>(null)

const form = reactive({
  type: typeInitial as string,
  titre: '',
  descriptionData: undefined as EditorJsData | undefined,
  domaine: '' as string,
  pays: '' as string,
  ville: '',
  duree: '' as string,
  dateDebut: '',
  dateFin: '',
  prisesEnCharge: [] as string[],
  couvertureFile: null as File | null,
  documentFile: null as File | null,
  organisateurNom: '',
  organisateurEmail: ''
})

const loading = ref(false)
const succes = ref(false)
const erreur = ref<string | null>(null)

const hasDescription = computed(() => {
  return form.descriptionData && form.descriptionData.blocks && form.descriptionData.blocks.length > 0
})

const isFormValid = computed(() => {
  return form.type &&
    form.titre.trim() &&
    hasDescription.value &&
    form.domaine &&
    form.pays &&
    form.duree &&
    form.dateDebut &&
    form.dateFin
})

const handleCouvertureChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    form.couvertureFile = target.files[0]
  }
}

const handleDocumentChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    form.documentFile = target.files[0]
  }
}

const resetForm = async () => {
  form.type = ''
  form.titre = ''
  form.descriptionData = undefined
  form.domaine = ''
  form.pays = ''
  form.ville = ''
  form.duree = ''
  form.dateDebut = ''
  form.dateFin = ''
  form.prisesEnCharge = []
  form.couvertureFile = null
  form.documentFile = null
  form.organisateurNom = ''
  form.organisateurEmail = ''
  if (editorRef.value) {
    await editorRef.value.clear()
  }
}

const handleSubmit = async () => {
  if (!isFormValid.value) return

  loading.value = true
  erreur.value = null
  succes.value = false

  try {
    let descriptionHtml = ''
    if (editorRef.value) {
      const savedData = await editorRef.value.save()
      if (savedData) {
        descriptionHtml = editorJsToHtml(savedData)
      }
    } else if (form.descriptionData) {
      descriptionHtml = editorJsToHtml(form.descriptionData)
    }

    const result = await creerProgramme(
      {
        type: form.type,
        titre: form.titre,
        description: descriptionHtml,
        domaine: form.domaine,
        pays: form.pays,
        ville: form.ville || undefined,
        duree: form.duree,
        dateDebut: form.dateDebut,
        dateFin: form.dateFin,
        prisesEnCharge: form.prisesEnCharge,
        organisateurNom: form.organisateurNom || undefined,
        organisateurEmail: form.organisateurEmail || undefined,
      },
      form.couvertureFile,
      form.documentFile,
    )

    if (result) {
      succes.value = true
      await resetForm()
      window.scrollTo({ top: 0, behavior: 'smooth' })
    } else {
      erreur.value = 'Une erreur est survenue lors de la soumission'
    }
  } catch (e: any) {
    erreur.value = e?.message || 'Une erreur est survenue lors de la soumission'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  AOS.init({
    duration: 800,
    easing: 'ease-out-cubic',
    once: true
  })
})
</script>
