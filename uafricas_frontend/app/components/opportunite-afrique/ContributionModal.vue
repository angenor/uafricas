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
  // Secteur d'opportunité enrichi
  localite: '',
  references_utiles: '',
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
  // Recette culinaire
  territoires_consommation: '',
  histoire: '',
  ingredients: [] as string[],
  etapes_preparation: [] as string[],
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
    recette_culinaire: 'une recette culinaire',
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
  formAfripulse.localite = ''
  formAfripulse.references_utiles = ''
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
  formAfripulse.territoires_consommation = ''
  formAfripulse.histoire = ''
  formAfripulse.ingredients = []
  formAfripulse.etapes_preparation = []
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
  if (typeof d.localite === 'string') formAfripulse.localite = d.localite
  if (typeof d.references_utiles === 'string') formAfripulse.references_utiles = d.references_utiles
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
  if (typeof d.territoires_consommation === 'string') formAfripulse.territoires_consommation = d.territoires_consommation
  if (typeof d.histoire === 'string') formAfripulse.histoire = d.histoire
  if (Array.isArray(d.ingredients)) formAfripulse.ingredients = (d.ingredients as unknown[]).filter((x): x is string => typeof x === 'string')
  if (Array.isArray(d.etapes_preparation)) formAfripulse.etapes_preparation = (d.etapes_preparation as unknown[]).filter((x): x is string => typeof x === 'string')
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
      localite: formAfripulse.localite.trim() || null,
      contact_telephone: formAfripulse.contact_telephone.trim() || null,
      contact_courriel: formAfripulse.contact_courriel.trim() || null,
      contact_adresse: formAfripulse.contact_adresse.trim() || null,
      references_utiles: formAfripulse.references_utiles.trim() || null,
      site_web_url: formAfripulse.site_web_url.trim() || null,
      image_url: formAfripulse.image_url.trim() || null,
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
  if (type === 'recette_culinaire') {
    if (!formAfripulse.titre.trim()) return null
    return {
      titre: formAfripulse.titre.trim(),
      territoires_consommation: formAfripulse.territoires_consommation.trim() || null,
      histoire: formAfripulse.histoire.trim() || null,
      ingredients: formAfripulse.ingredients.map(i => i.trim()).filter(i => i),
      etapes_preparation: formAfripulse.etapes_preparation.map(e => e.trim()).filter(e => e),
      images: formAfripulse.images,
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
  <AfricansModale
    :model-value="isOpen"
    :titre="titreModal"
    :sous-titre="paysNom"
    icone="fa-solid fa-hand-holding-heart"
    :ton="estSuppression ? 'chocolat' : 'vert'"
    @update:model-value="emit('close')"
  >
    <form id="form-contribution-afripulse" class="flex flex-col gap-5" @submit.prevent="handleSubmit">
          <div
            v-if="form.submitted"
            class="rounded-[10px] border border-af-vert/30 bg-af-vert/5 p-4 text-[14px]/[1.4] text-af-vert"
          >
            Contribution soumise avec succès ! Elle sera examinée par un administrateur.
          </div>

          <div
            v-if="form.error"
            class="rounded-[10px] border border-af-live/30 bg-af-live/5 p-4 text-[14px]/[1.4] text-af-live"
          >
            {{ form.errorMessage }}
          </div>

          <template v-if="estModeAfripulse && contexteAfripulse">
            <div
              v-if="estSuppression"
              class="rounded-[10px] border border-af-chocolat/30 bg-af-chocolat/5 p-4 text-[14px]/[1.4] text-af-chocolat"
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
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Nom du site *</label>
                <input
                  v-model="formAfripulse.nom"
                  type="text"
                  required
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Type de site *</label>
                <select
                  v-model="formAfripulse.sous_type"
                  required
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                >
                  <option value="">Sélectionnez un type</option>
                  <option v-for="st in sousTypesDisponibles" :key="st.value" :value="st.value">
                    {{ st.label }}
                  </option>
                </select>
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Gestionnaire *</label>
                <input
                  v-model="formAfripulse.gestionnaire"
                  type="text"
                  required
                  placeholder="Nom du gestionnaire / propriétaire"
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Ville *</label>
                  <input
                    v-model="formAfripulse.ville"
                    type="text"
                    required
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                </div>
                <div>
                  <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Village (optionnel)</label>
                  <input
                    v-model="formAfripulse.village"
                    type="text"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                </div>
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Latitude (GPS) *</label>
                  <input
                    v-model="formAfripulse.latitude"
                    type="number"
                    step="any"
                    required
                    placeholder="5.1962"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                </div>
                <div>
                  <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Longitude (GPS) *</label>
                  <input
                    v-model="formAfripulse.longitude"
                    type="number"
                    step="any"
                    required
                    placeholder="-3.7388"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                </div>
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Information pertinente *</label>
                <textarea
                  v-model="formAfripulse.info_pertinente"
                  rows="3"
                  required
                  placeholder="Accès, horaires, particularités utiles au visiteur…"
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Description (optionnel)</label>
                <textarea
                  v-model="formAfripulse.description"
                  rows="3"
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>

              <!-- Contacts : requis (au moins un) pour un site privé (FR-006) -->
              <fieldset class="rounded-[10px] border border-af-bordure p-4">
                <legend class="px-2 text-[14px]/[1.4] font-bold text-af-encre">
                  Contacts {{ estSitePrive ? '(au moins un requis)' : '(optionnel)' }}
                </legend>
                <div class="space-y-3">
                  <input
                    v-model="formAfripulse.contact_telephone"
                    type="tel"
                    placeholder="Téléphone"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                  <input
                    v-model="formAfripulse.contact_courriel"
                    type="email"
                    placeholder="Courriel"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                  <input
                    v-model="formAfripulse.contact_adresse"
                    type="text"
                    placeholder="Adresse"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                </div>
              </fieldset>

              <!-- Lien officiel du site web (facultatif) -->
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Site web (optionnel)</label>
                <input
                  v-model="formAfripulse.site_web_url"
                  type="url"
                  inputmode="url"
                  placeholder="https://exemple.com"
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
                <p class="mt-1 text-[12px]/[1.4] text-af-atone">Doit commencer par http:// ou https://</p>
              </div>

              <OpportuniteAfriqueMultiImageUploadField
                v-model="formAfripulse.images"
                :max="5"
                label="Images du site (5 max) : la 1re sert de couverture"
              />

              <!-- Constitution légale (facultatif : US4) -->
              <fieldset class="rounded-[10px] border border-af-bordure p-4">
                <legend class="px-2 text-[14px]/[1.4] font-bold text-af-encre">Constitution légale (optionnel)</legend>
                <div class="space-y-3">
                  <input
                    v-model="formAfripulse.constitution_statut_juridique"
                    type="text"
                    placeholder="Statut juridique (ex. SARL)"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                  <input
                    v-model="formAfripulse.constitution_numero"
                    type="text"
                    placeholder="Numéro d'enregistrement"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
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
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Nom du secteur *</label>
                <input
                  v-model="formAfripulse.nom"
                  type="text"
                  required
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Description *</label>
                <textarea
                  v-model="formAfripulse.description"
                  rows="4"
                  required
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Localité (optionnel)</label>
                <input
                  v-model="formAfripulse.localite"
                  type="text"
                  placeholder="Ville, région ou zone concernée"
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>

              <!-- Contacts (optionnels) -->
              <fieldset class="rounded-[10px] border border-af-bordure p-4">
                <legend class="px-2 text-[14px]/[1.4] font-bold text-af-encre">Contacts (optionnel)</legend>
                <div class="space-y-3">
                  <input
                    v-model="formAfripulse.contact_telephone"
                    type="tel"
                    placeholder="Téléphone"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                  <input
                    v-model="formAfripulse.contact_courriel"
                    type="email"
                    placeholder="Courriel"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                  <input
                    v-model="formAfripulse.contact_adresse"
                    type="text"
                    placeholder="Adresse"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                </div>
              </fieldset>

              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Références (optionnel)</label>
                <textarea
                  v-model="formAfripulse.references_utiles"
                  rows="2"
                  placeholder="Sources, rapports, organismes de référence…"
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>

              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Site web (optionnel)</label>
                <input
                  v-model="formAfripulse.site_web_url"
                  type="url"
                  inputmode="url"
                  placeholder="https://exemple.com"
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
                <p class="mt-1 text-[12px]/[1.4] text-af-atone">Doit commencer par http:// ou https://</p>
              </div>

              <OpportuniteAfriqueImageUploadField
                v-model="formAfripulse.image_url"
                label="Image illustrative (optionnel)"
              />
            </template>

            <template v-else-if="contexteAfripulse.type_objet_contribution === 'personnalite_connue'">
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Nom complet *</label>
                <input
                  v-model="formAfripulse.nom_complet"
                  type="text"
                  required
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Domaine *</label>
                <select
                  v-model="formAfripulse.domaine"
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                >
                  <option v-for="d in domaines" :key="d.value" :value="d.value">{{ d.label }}</option>
                </select>
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Biographie courte *</label>
                <textarea
                  v-model="formAfripulse.biographie_courte"
                  rows="3"
                  required
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Année de naissance</label>
                  <input
                    v-model="formAfripulse.annee_naissance"
                    type="number"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                </div>
                <div>
                  <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Année de décès</label>
                  <input
                    v-model="formAfripulse.annee_deces"
                    type="number"
                    class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                  />
                </div>
              </div>
              <OpportuniteAfriqueImageUploadField
                v-model="formAfripulse.portrait_url"
                label="Portrait (optionnel)"
              />
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Lien de référence (optionnel)</label>
                <input
                  v-model="formAfripulse.lien_reference"
                  type="url"
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
            </template>

            <template v-else-if="contexteAfripulse.type_objet_contribution === 'savoir_pratique'">
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Titre *</label>
                <input
                  v-model="formAfripulse.titre"
                  type="text"
                  required
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Catégorie *</label>
                <select
                  v-model="formAfripulse.categorie"
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                >
                  <option v-for="c in categoriesSavoir" :key="c.value" :value="c.value">{{ c.label }}</option>
                </select>
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Explication *</label>
                <textarea
                  v-model="formAfripulse.explication"
                  rows="4"
                  required
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Exemple (optionnel)</label>
                <textarea
                  v-model="formAfripulse.exemple"
                  rows="2"
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
            </template>

            <template v-else-if="contexteAfripulse.type_objet_contribution === 'recette_culinaire'">
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Titre de la recette *</label>
                <input
                  v-model="formAfripulse.titre"
                  type="text"
                  required
                  placeholder="Ex. : Thiéboudienne, Mafé, Ndolé…"
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>

              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Zone (ville) de consommation</label>
                <input
                  v-model="formAfripulse.territoires_consommation"
                  type="text"
                  placeholder="Ex. : Dakar, Bamako, Abidjan…"
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>

              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Petite histoire ou présentation</label>
                <textarea
                  v-model="formAfripulse.histoire"
                  rows="3"
                  placeholder="Origine, occasion, anecdotes…"
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>

              <!-- Ingrédients nécessaires -->
              <fieldset class="rounded-[10px] border border-af-bordure p-4">
                <legend class="px-2 text-[14px]/[1.4] font-bold text-af-encre">Ingrédients nécessaires</legend>
                <div class="space-y-2">
                  <div v-for="(ing, i) in formAfripulse.ingredients" :key="i" class="flex gap-2 items-center">
                    <span class="text-xs text-af-atone-2 w-5 shrink-0">{{ i + 1 }}.</span>
                    <input
                      v-model="formAfripulse.ingredients[i]"
                      type="text"
                      placeholder="Ex. : 500 g de riz brisé"
                      class="flex-1 rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                    />
                    <button
                      type="button"
                      class="cursor-pointer px-2 py-1 text-[14px]/[1.4] text-af-corps transition hover:text-af-live"
                      @click="formAfripulse.ingredients.splice(i, 1)"
                    >
                      <font-awesome-icon :icon="['fas', 'xmark']" class="w-4 h-4" />
                    </button>
                  </div>
                  <button
                    type="button"
                    class="mt-1 px-3 py-1.5 text-xs font-medium text-custom-chocolat border border-custom-chocolat/40 rounded-md hover:bg-custom-chocolat/5 cursor-pointer"
                    @click="formAfripulse.ingredients.push('')"
                  >
                    + Ajouter un ingrédient
                  </button>
                </div>
              </fieldset>

              <!-- Mode de préparation (étapes 1 à 10) -->
              <fieldset class="rounded-[10px] border border-af-bordure p-4">
                <legend class="px-2 text-[14px]/[1.4] font-bold text-af-encre">Mode de préparation (étapes 1 à 10)</legend>
                <div class="space-y-2">
                  <div v-for="(etape, i) in formAfripulse.etapes_preparation" :key="i" class="flex gap-2 items-start">
                    <span class="text-xs font-semibold text-custom-chocolat w-6 shrink-0 mt-2.5">{{ i + 1 }}</span>
                    <textarea
                      v-model="formAfripulse.etapes_preparation[i]"
                      rows="2"
                      placeholder="Décrivez cette étape…"
                      class="flex-1 resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                    />
                    <button
                      type="button"
                      class="mt-1.5 cursor-pointer px-2 py-1 text-[14px]/[1.4] text-af-corps transition hover:text-af-live"
                      @click="formAfripulse.etapes_preparation.splice(i, 1)"
                    >
                      <font-awesome-icon :icon="['fas', 'xmark']" class="w-4 h-4" />
                    </button>
                  </div>
                  <button
                    v-if="formAfripulse.etapes_preparation.length < 10"
                    type="button"
                    class="mt-1 px-3 py-1.5 text-xs font-medium text-custom-chocolat border border-custom-chocolat/40 rounded-md hover:bg-custom-chocolat/5 cursor-pointer"
                    @click="formAfripulse.etapes_preparation.push('')"
                  >
                    + Ajouter une étape
                  </button>
                  <p v-else class="text-[12px]/[1.4] text-af-atone">Maximum 10 étapes atteint.</p>
                </div>
              </fieldset>

              <OpportuniteAfriqueMultiImageUploadField
                v-model="formAfripulse.images"
                :max="5"
                label="Images illustratives (5 max)"
              />
            </template>
          </template>

          <template v-else>
            <!-- Champ ciblé : section pré-déterminée, on ne montre que la valeur à proposer -->
            <template v-if="estChampCible && contexteLegacy">
              <div class="bg-af-chocolat/5 border border-af-chocolat/20 text-af-corps rounded-md p-3 text-sm">
                Vous proposez une mise à jour de <strong>« {{ contexteLegacy.label }} »</strong>.
                Votre contribution sera examinée par un administrateur avant publication.
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">{{ contexteLegacy.label }} *</label>
                <textarea
                  v-model="form.nouvelle_valeur"
                  rows="5"
                  required
                  placeholder="Saisissez l'information…"
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
            </template>

            <!-- Mode générique : sélection libre de la section -->
            <template v-else>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Section concernée *</label>
                <select
                  v-model="form.section"
                  required
                  class="w-full rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                >
                  <option value="">Sélectionnez une section</option>
                  <option v-for="s in sections" :key="s.value" :value="s.value">{{ s.label }}</option>
                </select>
              </div>
              <div>
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Type *</label>
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
                <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">Nouvelle valeur *</label>
                <textarea
                  v-model="form.nouvelle_valeur"
                  rows="4"
                  required
                  class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
                />
              </div>
            </template>
          </template>

          <div>
            <label class="mb-1.5 block text-[14px]/[1.4] font-bold text-af-encre">
              {{ contexteAfripulse?.type_objet_contribution === 'recette_culinaire' ? 'Lien (optionnel)' : 'Justification (optionnel)' }}
            </label>
            <textarea
              v-model="form.justification"
              rows="2"
              class="w-full resize-y rounded-[10px] border border-af-bordure bg-white px-3 py-2.5 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
              :placeholder="contexteAfripulse?.type_objet_contribution === 'recette_culinaire' ? 'Lien vers la recette complète, une vidéo, la source…' : 'Source, contexte, raison...'"
            />
          </div>
    </form>

    <template #actions>
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="emit('close')"
      >
        Annuler
      </button>
      <!-- Bouton brut et non AfricansBouton : la variante destructrice
           (proposition de SUPPRESSION) n'existe pas dans les trois variantes
           de la maquette, et la couleur est ici porteuse de sens. -->
      <button
        type="submit"
        form="form-contribution-afripulse"
        :disabled="form.loading"
        class="inline-flex h-10 items-center justify-center rounded-lg px-6 text-base font-bold text-white transition hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
        :class="estSuppression ? 'bg-af-live' : 'bg-af-degrade'"
      >
        {{ labelBouton }}
      </button>
    </template>
  </AfricansModale>
</template>
