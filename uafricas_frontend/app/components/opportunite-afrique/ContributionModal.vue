<script setup lang="ts">
import {
  SECTIONS_FICHE_PAYS,
  SOUS_TYPES_PAR_CATEGORIE,
  LIBELLES_SOUS_TYPE,
  type TypeObjetContribution,
  type SectionAfripulse,
  type DomainePersonnalite,
  type CategorieSavoir,
  type CategorieSiteTouristique,
  type SousTypeSite,
} from '~/composables/useOpportuniteAfrique'

interface AfripulseContext {
  type_objet_contribution: TypeObjetContribution
  section_afripulse: SectionAfripulse
  type_contribution: 'ajout' | 'edition' | 'suppression'
  target_id?: string
  /** Valeurs actuelles de l'élément (pré-remplissage en édition / aperçu en suppression) */
  donnees_actuelles?: Record<string, unknown>
  /** Libellé lisible de l'élément concerné (confirmation de suppression) */
  libelle?: string
}

/** Contexte « champ ciblé » : contribution scalaire sur un champ précis de la fiche
 *  (ex. bloc « À savoir avant de voyager »). Réutilise le canal legacy. */
interface LegacyFieldContext {
  /** Nom de la section/colonne fiche_pays (ex. voyage_infos_visa) */
  section: string
  /** Libellé lisible du champ (titre du modal + en-tête) */
  label: string
  /** Valeur actuelle (pré-remplissage du textarea) */
  valeurActuelle?: string | null
}

interface Props {
  isOpen: boolean
  ficheId: string
  paysNom: string
  afripulseContext?: AfripulseContext | null
  legacyContext?: LegacyFieldContext | null
}

const props = withDefaults(defineProps<Props>(), {
  afripulseContext: null,
  legacyContext: null,
})

type SubmitLegacy = {
  mode: 'legacy'
  section: string
  type_contribution: string
  nouvelle_valeur: string
  justification: string
}

type SubmitAfripulse = {
  mode: 'afripulse'
  type_objet_contribution: TypeObjetContribution
  section_afripulse: SectionAfripulse
  type_contribution: 'ajout' | 'edition' | 'suppression'
  target_id?: string
  nouvelle_valeur_jsonb: Record<string, unknown>
  justification: string
}

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', data: SubmitLegacy | SubmitAfripulse): void
}>()

const contexteAfripulse = ref<AfripulseContext | null>(props.afripulseContext)
const contexteLegacy = ref<LegacyFieldContext | null>(props.legacyContext)
const estModeAfripulse = computed(() => contexteAfripulse.value !== null)
/** Mode legacy ciblé sur un champ précis (section + valeur pré-remplie, sélecteurs masqués) */
const estChampCible = computed(() => contexteLegacy.value !== null)
const typeAction = computed(() => contexteAfripulse.value?.type_contribution ?? 'ajout')
const estSuppression = computed(() => typeAction.value === 'suppression')

/** Famille du site dérivée de la section (sites_prives → privé, sinon emblématique) */
const familleSite = computed<CategorieSiteTouristique>(() =>
  contexteAfripulse.value?.section_afripulse === 'sites_prives' ? 'prive' : 'emblematique',
)
const estSitePrive = computed(() => familleSite.value === 'prive')
/** Sous-types proposés selon la famille (cohérence famille↔sous-type FR-003) */
const sousTypesDisponibles = computed(() =>
  SOUS_TYPES_PAR_CATEGORIE[familleSite.value].map(value => ({
    value,
    label: LIBELLES_SOUS_TYPE[value],
  })),
)

const form = reactive({
  section: '',
  type_contribution: 'modification',
  nouvelle_valeur: '',
  justification: '',
  loading: false,
  submitted: false,
  error: false,
  errorMessage: '',
})

const formAfripulse = reactive({
  nom: '',
  description: '',
  image_url: '',
  // Site touristique enrichi (US1/US2/US4)
  sous_type: '' as SousTypeSite | '',
  images: [] as string[],
  gestionnaire: '',
  ville: '',
  village: '',
  info_pertinente: '',
  latitude: '' as string | number,
  longitude: '' as string | number,
  contact_telephone: '',
  contact_courriel: '',
  contact_adresse: '',
  constitution_statut_juridique: '',
  constitution_numero: '',
  constitution_document_url: '',
  site_web_url: '',
  nom_complet: '',
  domaine: 'autre' as DomainePersonnalite,
  biographie_courte: '',
  annee_naissance: '' as string | number,
  annee_deces: '' as string | number,
  portrait_url: '',
  lien_reference: '',
  titre: '',
  categorie: 'autre' as CategorieSavoir,
  explication: '',
  exemple: '',
})

const domaines: { value: DomainePersonnalite, label: string }[] = [
  { value: 'politique', label: 'Politique' },
  { value: 'artiste_musicien', label: 'Artiste musicien' },
  { value: 'artiste_autre', label: 'Artiste (autre)' },
  { value: 'sportif', label: 'Sportif' },
  { value: 'entrepreneur', label: 'Entrepreneur' },
  { value: 'scientifique', label: 'Scientifique' },
  { value: 'militaire_historique', label: 'Militaire / Historique' },
  { value: 'autre', label: 'Autre' },
]

const categoriesSavoir: { value: CategorieSavoir, label: string }[] = [
  { value: 'langue_argot', label: 'Langue et argot' },
  { value: 'coutumes', label: 'Coutumes' },
  { value: 'etiquette', label: 'Étiquette' },
  { value: 'securite', label: 'Sécurité' },
  { value: 'sante', label: 'Santé' },
  { value: 'transports', label: 'Transports' },
  { value: 'autre', label: 'Autre' },
]

const sections = SECTIONS_FICHE_PAYS

const titreModal = computed(() => {
  if (estChampCible.value) return `Contribuer : ${contexteLegacy.value!.label}`
  if (!estModeAfripulse.value) return 'Proposer une contribution'
  const ctx = contexteAfripulse.value!
  const cibles: Record<TypeObjetContribution, string> = {
    fiche_pays: 'la fiche territoire',
    site_touristique: 'un site touristique',
    secteur_developpement: 'un secteur d\'opportunité',
    personnalite_connue: 'une personnalité',
    savoir_pratique: 'un savoir pratique',
    recommandation_visiteur: 'une recommandation',
    photo_visiteur: 'une photo',
  }
  const cible = cibles[ctx.type_objet_contribution] ?? 'un élément'
  if (ctx.type_contribution === 'edition') return `Modifier ${cible}`
  if (ctx.type_contribution === 'suppression') return `Supprimer ${cible}`
  return `Proposer ${cible}`
})

const labelBouton = computed(() => {
  if (form.loading) return 'Envoi...'
  if (estSuppression.value) return 'Proposer la suppression'
  if (estChampCible.value || typeAction.value === 'edition') return 'Proposer la modification'
  return 'Soumettre'
})

const resetForm = () => {
  form.section = ''
  form.type_contribution = 'modification'
  form.nouvelle_valeur = ''
  form.justification = ''
  form.loading = false
  form.submitted = false
  form.error = false
  form.errorMessage = ''
  formAfripulse.nom = ''
  formAfripulse.description = ''
  formAfripulse.image_url = ''
  formAfripulse.sous_type = ''
  formAfripulse.images = []
  formAfripulse.gestionnaire = ''
  formAfripulse.ville = ''
  formAfripulse.village = ''
  formAfripulse.info_pertinente = ''
  formAfripulse.latitude = ''
  formAfripulse.longitude = ''
  formAfripulse.contact_telephone = ''
  formAfripulse.contact_courriel = ''
  formAfripulse.contact_adresse = ''
  formAfripulse.constitution_statut_juridique = ''
  formAfripulse.constitution_numero = ''
  formAfripulse.constitution_document_url = ''
  formAfripulse.site_web_url = ''
  formAfripulse.nom_complet = ''
  formAfripulse.domaine = 'autre'
  formAfripulse.biographie_courte = ''
  formAfripulse.annee_naissance = ''
  formAfripulse.annee_deces = ''
  formAfripulse.portrait_url = ''
  formAfripulse.lien_reference = ''
  formAfripulse.titre = ''
  formAfripulse.categorie = 'autre'
  formAfripulse.explication = ''
  formAfripulse.exemple = ''
}

/** Pré-remplit le formulaire legacy pour une contribution sur un champ ciblé */
const prefillLegacy = (ctx: LegacyFieldContext | null) => {
  if (!ctx) return
  form.section = ctx.section
  form.type_contribution = 'modification'
  form.nouvelle_valeur = ctx.valeurActuelle ?? ''
}

/** Pré-remplit le formulaire Afripulse avec les valeurs actuelles (édition / suppression) */
const prefillAfripulse = (ctx: AfripulseContext | null) => {
  const d = ctx?.donnees_actuelles
  if (!d) return
  if (typeof d.nom === 'string') formAfripulse.nom = d.nom
  if (typeof d.description === 'string') formAfripulse.description = d.description
  if (typeof d.image_url === 'string') formAfripulse.image_url = d.image_url
  if (typeof d.sous_type === 'string') formAfripulse.sous_type = d.sous_type as SousTypeSite
  if (Array.isArray(d.images)) formAfripulse.images = (d.images as unknown[]).filter((x): x is string => typeof x === 'string')
  else if (typeof d.image_url === 'string' && d.image_url) formAfripulse.images = [d.image_url]
  if (typeof d.gestionnaire === 'string') formAfripulse.gestionnaire = d.gestionnaire
  if (typeof d.ville === 'string') formAfripulse.ville = d.ville
  if (typeof d.village === 'string') formAfripulse.village = d.village
  if (typeof d.info_pertinente === 'string') formAfripulse.info_pertinente = d.info_pertinente
  if (typeof d.latitude === 'number') formAfripulse.latitude = d.latitude
  if (typeof d.longitude === 'number') formAfripulse.longitude = d.longitude
  if (typeof d.contact_telephone === 'string') formAfripulse.contact_telephone = d.contact_telephone
  if (typeof d.contact_courriel === 'string') formAfripulse.contact_courriel = d.contact_courriel
  if (typeof d.contact_adresse === 'string') formAfripulse.contact_adresse = d.contact_adresse
  if (typeof d.constitution_statut_juridique === 'string') formAfripulse.constitution_statut_juridique = d.constitution_statut_juridique
  if (typeof d.constitution_numero === 'string') formAfripulse.constitution_numero = d.constitution_numero
  if (typeof d.constitution_document_url === 'string') formAfripulse.constitution_document_url = d.constitution_document_url
  if (typeof d.site_web_url === 'string') formAfripulse.site_web_url = d.site_web_url
  if (typeof d.nom_complet === 'string') formAfripulse.nom_complet = d.nom_complet
  if (typeof d.domaine === 'string') formAfripulse.domaine = d.domaine as DomainePersonnalite
  if (typeof d.biographie_courte === 'string') formAfripulse.biographie_courte = d.biographie_courte
  if (typeof d.annee_naissance === 'number') formAfripulse.annee_naissance = d.annee_naissance
  if (typeof d.annee_deces === 'number') formAfripulse.annee_deces = d.annee_deces
  if (typeof d.portrait_url === 'string') formAfripulse.portrait_url = d.portrait_url
  if (typeof d.lien_reference === 'string') formAfripulse.lien_reference = d.lien_reference
  if (typeof d.titre === 'string') formAfripulse.titre = d.titre
  if (typeof d.categorie === 'string') formAfripulse.categorie = d.categorie as CategorieSavoir
  if (typeof d.explication === 'string') formAfripulse.explication = d.explication
  if (typeof d.exemple === 'string') formAfripulse.exemple = d.exemple
}

const construirePayloadAfripulse = (): Record<string, unknown> | null => {
  if (!contexteAfripulse.value) return null
  const type = contexteAfripulse.value.type_objet_contribution
  if (type === 'site_touristique') {
    // Champs requis (FR-005). Contact requis pour un site privé (FR-006).
    const lat = formAfripulse.latitude === '' ? null : Number(formAfripulse.latitude)
    const lon = formAfripulse.longitude === '' ? null : Number(formAfripulse.longitude)
    const requisOk =
      formAfripulse.nom.trim()
      && formAfripulse.gestionnaire.trim()
      && formAfripulse.ville.trim()
      && formAfripulse.info_pertinente.trim()
      && formAfripulse.sous_type
      && lat !== null && !Number.isNaN(lat)
      && lon !== null && !Number.isNaN(lon)
    if (!requisOk) return null
    if (estSitePrive.value) {
      const aContact =
        formAfripulse.contact_telephone.trim()
        || formAfripulse.contact_courriel.trim()
        || formAfripulse.contact_adresse.trim()
      if (!aContact) return null
    }
    return {
      nom: formAfripulse.nom.trim(),
      categorie: familleSite.value,
      sous_type: formAfripulse.sous_type,
      description: formAfripulse.description.trim() || null,
      info_pertinente: formAfripulse.info_pertinente.trim(),
      images: formAfripulse.images,
      image_url: formAfripulse.images[0] || null,
      gestionnaire: formAfripulse.gestionnaire.trim(),
      ville: formAfripulse.ville.trim(),
      village: formAfripulse.village.trim() || null,
      latitude: lat,
      longitude: lon,
      contact_telephone: formAfripulse.contact_telephone.trim() || null,
      contact_courriel: formAfripulse.contact_courriel.trim() || null,
      contact_adresse: formAfripulse.contact_adresse.trim() || null,
      constitution_statut_juridique: formAfripulse.constitution_statut_juridique.trim() || null,
      constitution_numero: formAfripulse.constitution_numero.trim() || null,
      constitution_document_url: formAfripulse.constitution_document_url.trim() || null,
      site_web_url: formAfripulse.site_web_url.trim() || null,
    }
  }
  if (type === 'secteur_developpement') {
    if (!formAfripulse.nom.trim() || !formAfripulse.description.trim()) return null
    return {
      nom: formAfripulse.nom.trim(),
      description: formAfripulse.description.trim(),
    }
  }
  if (type === 'personnalite_connue') {
    if (!formAfripulse.nom_complet.trim() || !formAfripulse.biographie_courte.trim()) return null
    return {
      nom_complet: formAfripulse.nom_complet.trim(),
      domaine: formAfripulse.domaine,
      biographie_courte: formAfripulse.biographie_courte.trim(),
      annee_naissance: formAfripulse.annee_naissance ? Number(formAfripulse.annee_naissance) : null,
      annee_deces: formAfripulse.annee_deces ? Number(formAfripulse.annee_deces) : null,
      portrait_url: formAfripulse.portrait_url.trim() || null,
      lien_reference: formAfripulse.lien_reference.trim() || null,
    }
  }
  if (type === 'savoir_pratique') {
    if (!formAfripulse.titre.trim() || !formAfripulse.explication.trim()) return null
    return {
      titre: formAfripulse.titre.trim(),
      categorie: formAfripulse.categorie,
      explication: formAfripulse.explication.trim(),
      exemple: formAfripulse.exemple.trim() || null,
    }
  }
  return null
}

const handleSubmit = () => {
  form.error = false
  form.errorMessage = ''

  if (estModeAfripulse.value) {
    const ctx = contexteAfripulse.value!
    // En suppression, aucun champ n'est requis : le backend retire l'élément ciblé via target_id.
    let payload: Record<string, unknown> | null
    if (estSuppression.value) {
      payload = ctx.donnees_actuelles ?? {}
    } else {
      payload = construirePayloadAfripulse()
      if (!payload) {
        form.error = true
        form.errorMessage = 'Veuillez remplir les champs obligatoires.'
        return
      }
    }
    form.loading = true
    emit('submit', {
      mode: 'afripulse',
      type_objet_contribution: ctx.type_objet_contribution,
      section_afripulse: ctx.section_afripulse,
      type_contribution: ctx.type_contribution,
      target_id: ctx.target_id,
      nouvelle_valeur_jsonb: payload,
      justification: form.justification.trim(),
    })
    return
  }

  if (!form.section) {
    form.error = true
    form.errorMessage = 'Veuillez sélectionner une section.'
    return
  }
  if (form.nouvelle_valeur.trim().length < 3) {
    form.error = true
    form.errorMessage = 'La valeur proposée doit contenir au moins 3 caractères.'
    return
  }
  form.loading = true
  emit('submit', {
    mode: 'legacy',
    section: form.section,
    type_contribution: form.type_contribution,
    nouvelle_valeur: form.nouvelle_valeur.trim(),
    justification: form.justification.trim(),
  })
}

defineExpose({
  setLoading: (val: boolean) => { form.loading = val },
  setError: (msg: string) => {
    form.error = true
    form.errorMessage = msg
    form.loading = false
  },
  setSuccess: () => {
    form.submitted = true
    form.loading = false
    setTimeout(() => {
      resetForm()
      contexteAfripulse.value = null
      emit('close')
    }, 2500)
  },
  openWithContext: (ctx: AfripulseContext) => {
    resetForm()
    contexteAfripulse.value = ctx
    prefillAfripulse(ctx)
  },
})

watch(() => props.isOpen, (isOpen) => {
  if (!isOpen) {
    resetForm()
    contexteAfripulse.value = null
    contexteLegacy.value = null
  }
})

watch(() => props.afripulseContext, (ctx) => {
  if (ctx) {
    resetForm()
    contexteLegacy.value = null
    contexteAfripulse.value = ctx
    prefillAfripulse(ctx)
  } else {
    contexteAfripulse.value = null
  }
})

watch(() => props.legacyContext, (ctx) => {
  if (ctx) {
    resetForm()
    contexteAfripulse.value = null
    contexteLegacy.value = ctx
    prefillLegacy(ctx)
  } else {
    contexteLegacy.value = null
  }
})
</script>

<template>
  <Transition name="modal-fade">
    <div
      v-if="isOpen"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50"
      @click.self="emit('close')"
    >
      <div
        class="relative w-full max-w-2xl bg-white shadow-xl rounded-lg max-h-[90vh] overflow-hidden flex flex-col"
        @click.stop
      >
        <div class="flex items-start justify-between px-6 py-4 border-b border-gray-200">
          <div>
            <h2 class="font-oswald text-xl font-bold text-gray-900">{{ titreModal }}</h2>
            <p class="text-sm text-gray-500 mt-1">{{ paysNom }}</p>
          </div>
          <button
            type="button"
            class="text-gray-400 hover:text-gray-600 transition-colors"
            aria-label="Fermer"
            @click="emit('close')"
          >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <form class="px-6 py-5 space-y-5 overflow-y-auto flex-1" @submit.prevent="handleSubmit">
          <div
            v-if="form.submitted"
            class="bg-green-50 border border-green-200 text-green-800 rounded-md p-4 text-sm"
          >
            Contribution soumise avec succès ! Elle sera examinée par un administrateur.
          </div>

          <div
            v-if="form.error"
            class="bg-red-50 border border-red-200 text-red-800 rounded-md p-4 text-sm"
          >
            {{ form.errorMessage }}
          </div>

          <template v-if="estModeAfripulse && contexteAfripulse">
            <div
              v-if="estSuppression"
              class="bg-amber-50 border border-amber-200 text-amber-800 rounded-md p-4 text-sm"
            >
              <p class="font-medium mb-1">Proposer la suppression de cet élément</p>
              <p v-if="contexteAfripulse.libelle">
                Élément concerné : <strong>« {{ contexteAfripulse.libelle }} »</strong>.
              </p>
              <p class="mt-2">
                Aucune suppression ne sera effectuée immédiatement. Votre proposition sera
                examinée par un administrateur avant d'être appliquée.
              </p>
            </div>

            <template v-else-if="contexteAfripulse.type_objet_contribution === 'site_touristique'">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Nom du site *</label>
                <input
                  v-model="formAfripulse.nom"
                  type="text"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Type de site *</label>
                <select
                  v-model="formAfripulse.sous_type"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                >
                  <option value="">Sélectionnez un type</option>
                  <option v-for="st in sousTypesDisponibles" :key="st.value" :value="st.value">
                    {{ st.label }}
                  </option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Gestionnaire *</label>
                <input
                  v-model="formAfripulse.gestionnaire"
                  type="text"
                  required
                  placeholder="Nom du gestionnaire / propriétaire"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                />
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Ville *</label>
                  <input
                    v-model="formAfripulse.ville"
                    type="text"
                    required
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Village (optionnel)</label>
                  <input
                    v-model="formAfripulse.village"
                    type="text"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                </div>
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Latitude (GPS) *</label>
                  <input
                    v-model="formAfripulse.latitude"
                    type="number"
                    step="any"
                    required
                    placeholder="5.1962"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Longitude (GPS) *</label>
                  <input
                    v-model="formAfripulse.longitude"
                    type="number"
                    step="any"
                    required
                    placeholder="-3.7388"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                </div>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Information pertinente *</label>
                <textarea
                  v-model="formAfripulse.info_pertinente"
                  rows="3"
                  required
                  placeholder="Accès, horaires, particularités utiles au visiteur…"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Description (optionnel)</label>
                <textarea
                  v-model="formAfripulse.description"
                  rows="3"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
                />
              </div>

              <!-- Contacts : requis (au moins un) pour un site privé (FR-006) -->
              <fieldset class="border border-gray-200 rounded-md p-4">
                <legend class="text-sm font-medium text-gray-700 px-2">
                  Contacts {{ estSitePrive ? '(au moins un requis)' : '(optionnel)' }}
                </legend>
                <div class="space-y-3">
                  <input
                    v-model="formAfripulse.contact_telephone"
                    type="tel"
                    placeholder="Téléphone"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                  <input
                    v-model="formAfripulse.contact_courriel"
                    type="email"
                    placeholder="Courriel"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                  <input
                    v-model="formAfripulse.contact_adresse"
                    type="text"
                    placeholder="Adresse"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                </div>
              </fieldset>

              <!-- Lien officiel du site web (facultatif) -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Site web (optionnel)</label>
                <input
                  v-model="formAfripulse.site_web_url"
                  type="url"
                  inputmode="url"
                  placeholder="https://exemple.com"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                />
                <p class="text-xs text-gray-400 mt-1">Doit commencer par http:// ou https://</p>
              </div>

              <OpportuniteAfriqueMultiImageUploadField
                v-model="formAfripulse.images"
                :max="5"
                label="Images du site (5 max) — la 1re sert de couverture"
              />

              <!-- Constitution légale (facultatif — US4) -->
              <fieldset class="border border-gray-200 rounded-md p-4">
                <legend class="text-sm font-medium text-gray-700 px-2">Constitution légale (optionnel)</legend>
                <div class="space-y-3">
                  <input
                    v-model="formAfripulse.constitution_statut_juridique"
                    type="text"
                    placeholder="Statut juridique (ex. SARL)"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                  <input
                    v-model="formAfripulse.constitution_numero"
                    type="text"
                    placeholder="Numéro d'enregistrement"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                  <OpportuniteAfriqueImageUploadField
                    v-model="formAfripulse.constitution_document_url"
                    label="Document de constitution (optionnel)"
                  />
                </div>
              </fieldset>
            </template>

            <template v-else-if="contexteAfripulse.type_objet_contribution === 'secteur_developpement'">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Nom du secteur *</label>
                <input
                  v-model="formAfripulse.nom"
                  type="text"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Description *</label>
                <textarea
                  v-model="formAfripulse.description"
                  rows="4"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
                />
              </div>
            </template>

            <template v-else-if="contexteAfripulse.type_objet_contribution === 'personnalite_connue'">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Nom complet *</label>
                <input
                  v-model="formAfripulse.nom_complet"
                  type="text"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Domaine *</label>
                <select
                  v-model="formAfripulse.domaine"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                >
                  <option v-for="d in domaines" :key="d.value" :value="d.value">{{ d.label }}</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Biographie courte *</label>
                <textarea
                  v-model="formAfripulse.biographie_courte"
                  rows="3"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
                />
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Année de naissance</label>
                  <input
                    v-model="formAfripulse.annee_naissance"
                    type="number"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Année de décès</label>
                  <input
                    v-model="formAfripulse.annee_deces"
                    type="number"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                  />
                </div>
              </div>
              <OpportuniteAfriqueImageUploadField
                v-model="formAfripulse.portrait_url"
                label="Portrait (optionnel)"
              />
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Lien de référence (optionnel)</label>
                <input
                  v-model="formAfripulse.lien_reference"
                  type="url"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                />
              </div>
            </template>

            <template v-else-if="contexteAfripulse.type_objet_contribution === 'savoir_pratique'">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Titre *</label>
                <input
                  v-model="formAfripulse.titre"
                  type="text"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Catégorie *</label>
                <select
                  v-model="formAfripulse.categorie"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                >
                  <option v-for="c in categoriesSavoir" :key="c.value" :value="c.value">{{ c.label }}</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Explication *</label>
                <textarea
                  v-model="formAfripulse.explication"
                  rows="4"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Exemple (optionnel)</label>
                <textarea
                  v-model="formAfripulse.exemple"
                  rows="2"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
                />
              </div>
            </template>
          </template>

          <template v-else>
            <!-- Champ ciblé : section pré-déterminée, on ne montre que la valeur à proposer -->
            <template v-if="estChampCible && contexteLegacy">
              <div class="bg-blue-50 border border-blue-200 text-blue-800 rounded-md p-3 text-sm">
                Vous proposez une mise à jour de <strong>« {{ contexteLegacy.label }} »</strong>.
                Votre contribution sera examinée par un administrateur avant publication.
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">{{ contexteLegacy.label }} *</label>
                <textarea
                  v-model="form.nouvelle_valeur"
                  rows="5"
                  required
                  placeholder="Saisissez l'information…"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
                />
              </div>
            </template>

            <!-- Mode générique : sélection libre de la section -->
            <template v-else>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Section concernée *</label>
                <select
                  v-model="form.section"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
                >
                  <option value="">Sélectionnez une section</option>
                  <option v-for="s in sections" :key="s.value" :value="s.value">{{ s.label }}</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Type *</label>
                <div class="flex flex-wrap gap-4">
                  <label class="flex items-center gap-2">
                    <input v-model="form.type_contribution" type="radio" value="modification" />
                    <span class="text-sm">Modification</span>
                  </label>
                  <label class="flex items-center gap-2">
                    <input v-model="form.type_contribution" type="radio" value="ajout" />
                    <span class="text-sm">Ajout</span>
                  </label>
                  <label class="flex items-center gap-2">
                    <input v-model="form.type_contribution" type="radio" value="suppression" />
                    <span class="text-sm">Suppression</span>
                  </label>
                </div>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Nouvelle valeur *</label>
                <textarea
                  v-model="form.nouvelle_valeur"
                  rows="4"
                  required
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
                />
              </div>
            </template>
          </template>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Justification (optionnel)</label>
            <textarea
              v-model="form.justification"
              rows="2"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
              placeholder="Source, contexte, raison..."
            />
          </div>
        </form>

        <div class="flex justify-end gap-3 px-6 py-4 border-t border-gray-200 bg-gray-50">
          <button
            type="button"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
            @click="emit('close')"
          >
            Annuler
          </button>
          <button
            type="button"
            :disabled="form.loading"
            class="px-4 py-2 text-sm font-medium text-white rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            :class="estSuppression
              ? 'bg-red-600 hover:bg-red-700'
              : 'bg-custom-chocolat hover:bg-custom-chocolat/90'"
            @click="handleSubmit"
          >
            {{ labelBouton }}
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.2s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
