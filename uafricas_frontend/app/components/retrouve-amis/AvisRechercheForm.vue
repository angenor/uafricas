<script setup lang="ts">
import gsap from 'gsap'
import type { PaysInfo } from '~/composables/useRetrouvAmis'
import { TYPES_RELATION, GENRES_PERSONNE, RESEAUX_SOCIAUX } from '~/composables/useRetrouvAmis'
import { useAnimationsFormulaire } from '~/composables/useAnimationsFormulaire'

// ── Props & Emits ──────────────────────────────────────────────
interface Props {
  mode?: 'creation' | 'modification'
  chargementSoumission?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  mode: 'creation',
  chargementSoumission: false,
})

const emit = defineEmits<{
  submit: [data: FormData]
  annuler: []
}>()

// ── Etapes du formulaire ───────────────────────────────────────
const etapes = [
  { numero: 1, titre: 'Preferences' },
  { numero: 2, titre: 'Identite' },
  { numero: 3, titre: 'Relation' },
  { numero: 4, titre: 'Lieu de rencontre' },
  { numero: 5, titre: 'Photo & apparence' },
  { numero: 6, titre: 'Recapitulatif' },
]
const etapeCourante = ref(1)

// ── Refs template pour animations ──────────────────────────────
const conteneurFormRef = ref<HTMLElement | null>(null)
const etapeRef = ref<HTMLElement | null>(null)
const dotsRef = ref<HTMLElement | null>(null)
const reseauxSociauxRef = ref<HTMLElement | null>(null)
const photoPreviewImgRef = ref<HTMLElement | null>(null)

// ── Composable animations GSAP ─────────────────────────────────
const {
  enTransition,
  prefereReducedMotion,
  animerTransitionEtape,
  animerChampsEtape,
  animerProgression,
} = useAnimationsFormulaire(conteneurFormRef)

// ── Donnees du formulaire ──────────────────────────────────────
const formulaire = reactive({
  // Etape 1 : Preferences
  est_anonyme: false,
  partage_coordonnees: false,
  coordonnees_email: '',
  coordonnees_telephone: '',
  coordonnees_whatsapp: '',
  // Etape 2 : Identite
  nom_recherche: '',
  prenom_recherche: '',
  surnom: '',
  genre_recherche: '',
  comment_connu: '',
  // Etape 3 : Relation
  type_relation: '',
  type_relation_autre: '',
  // Etape 4 : Lieu de rencontre
  localite_rencontre: '',
  ecole_rencontre: '',
  ville_rencontre: '',
  pays_id: '',
  rencontre_reseaux_sociaux: false,
  reseaux_sociaux: [] as string[],
  // Etape 5 : Photo & apparence
  description_physique: '',
  description: '',
})

const photoFichier = ref<File | null>(null)
const photoPreview = ref<string | null>(null)

// ── Chargement des pays ────────────────────────────────────────
const listePays = ref<PaysInfo[]>([])

onMounted(async () => {
  try {
    const config = useRuntimeConfig()
    const apiBase = config.public.apiBaseUrl as string
    const reponse = await $fetch<{ success: boolean; data: PaysInfo[] }>(
      `${apiBase}/api/retrouve-amis/pays`,
    )
    if (reponse.success && reponse.data) {
      listePays.value = reponse.data
    }
  } catch {
    // Silencieux
  }
})

// ── Erreurs de validation ──────────────────────────────────────
const erreurs = reactive<Record<string, string>>({})

const nettoyerErreurs = () => {
  Object.keys(erreurs).forEach(k => delete erreurs[k])
}

// ── Gestion photo ──────────────────────────────────────────────
const onPhotoChange = (event: Event) => {
  const input = event.target as HTMLInputElement
  const fichier = input.files?.[0]
  if (!fichier) return

  // Valider type
  const typesAutorises = ['image/jpeg', 'image/png', 'image/webp']
  if (!typesAutorises.includes(fichier.type)) {
    erreurs.photo = 'Format invalide. Acceptes : JPEG, PNG, WebP.'
    return
  }

  // Valider taille (5 Mo)
  if (fichier.size > 5 * 1024 * 1024) {
    erreurs.photo = 'La photo ne doit pas depasser 5 Mo.'
    return
  }

  delete erreurs.photo
  photoFichier.value = fichier
  photoPreview.value = URL.createObjectURL(fichier)
}

const supprimerPhoto = () => {
  photoFichier.value = null
  if (photoPreview.value) {
    URL.revokeObjectURL(photoPreview.value)
    photoPreview.value = null
  }
}

// ── Validation par etape ───────────────────────────────────────
const validerEtape = (etape: number): boolean => {
  nettoyerErreurs()

  if (etape === 1) {
    if (!formulaire.partage_coordonnees) {
      erreurs.partage_coordonnees = 'Vous devez accepter le partage de coordonnees pour pouvoir continuer.'
    } else {
      const aCoordonnee = formulaire.coordonnees_email.trim()
        || formulaire.coordonnees_telephone.trim()
        || formulaire.coordonnees_whatsapp.trim()
      if (!aCoordonnee) {
        erreurs.coordonnees = 'Au moins une coordonnee est requise.'
      }
    }
  }

  if (etape === 2) {
    if (!formulaire.nom_recherche.trim()) {
      erreurs.nom_recherche = 'Le nom de la personne recherchee est requis.'
    }
  }

  return Object.keys(erreurs).length === 0
}

// ── Validation globale (critere requis) ────────────────────────
const validerGlobal = (): boolean => {
  nettoyerErreurs()

  if (!formulaire.nom_recherche.trim()) {
    erreurs.nom_recherche = 'Le nom est requis.'
    etapeCourante.value = 2
    return false
  }

  const aCritere = formulaire.type_relation
    || formulaire.localite_rencontre.trim()
    || formulaire.ecole_rencontre.trim()
    || formulaire.ville_rencontre.trim()
    || formulaire.rencontre_reseaux_sociaux

  if (!aCritere) {
    erreurs.global = 'Veuillez renseigner au moins un critere : type de relation, lieu de rencontre, ecole, ou reseaux sociaux.'
    return false
  }

  if (!formulaire.partage_coordonnees) {
    erreurs.partage_coordonnees = 'Vous devez accepter le partage de coordonnees pour pouvoir continuer.'
    etapeCourante.value = 1
    return false
  }

  const aCoordonnee = formulaire.coordonnees_email.trim()
    || formulaire.coordonnees_telephone.trim()
    || formulaire.coordonnees_whatsapp.trim()
  if (!aCoordonnee) {
    erreurs.coordonnees = 'Au moins une coordonnee est requise.'
    etapeCourante.value = 1
    return false
  }

  return true
}

// ── Navigation ─────────────────────────────────────────────────
const suivant = () => {
  if (enTransition.value) return
  if (validerEtape(etapeCourante.value)) {
    etapeCourante.value++
  }
}

const precedent = () => {
  if (enTransition.value) return
  nettoyerErreurs()
  etapeCourante.value--
}

// ── Watch transitions d'etapes (US1 + US2 + US3) ──────────────
watch(etapeCourante, async (nouvelle, ancienne) => {
  enTransition.value = true
  const direction = nouvelle > ancienne ? 'avant' : 'arriere'
  await nextTick()
  animerTransitionEtape(direction, etapeRef)
  animerProgression(nouvelle, ancienne, dotsRef)
  // Stagger des champs avec leger delai
  setTimeout(() => {
    animerChampsEtape(etapeRef)
    enTransition.value = false
  }, 200)
})

// ── Watch champs conditionnels reseaux sociaux (US3) ───────────
watch(() => formulaire.rencontre_reseaux_sociaux, async (valeur) => {
  if (prefereReducedMotion.value) return
  await nextTick()
  if (reseauxSociauxRef.value) {
    if (valeur) {
      gsap.fromTo(reseauxSociauxRef.value,
        { height: 0, opacity: 0, overflow: 'hidden' },
        { height: 'auto', opacity: 1, duration: 0.35, ease: 'power2.out' },
      )
    } else {
      gsap.to(reseauxSociauxRef.value,
        { height: 0, opacity: 0, duration: 0.25, ease: 'power2.in' },
      )
    }
  }
})

// ── Watch preview photo (US5) ──────────────────────────────────
watch(photoPreview, async (nouvelle) => {
  if (!nouvelle || prefereReducedMotion.value) return
  await nextTick()
  if (photoPreviewImgRef.value) {
    gsap.fromTo(photoPreviewImgRef.value,
      { scale: 0.8, opacity: 0 },
      { scale: 1, opacity: 1, duration: 0.3, ease: 'back.out(1.2)' },
    )
  }
})

// ── Micro-animation radio/selection (US5) ──────────────────────
const animerSelection = (event: Event) => {
  if (prefereReducedMotion.value) return
  const cible = (event.currentTarget as HTMLElement)
  gsap.fromTo(cible,
    { scale: 1 },
    { scale: 1.05, duration: 0.1, yoyo: true, repeat: 1, ease: 'power1.out' },
  )
}

// ── Soumission ─────────────────────────────────────────────────
const soumettre = () => {
  if (!validerGlobal()) return

  const fd = new FormData()
  fd.append('nom_recherche', formulaire.nom_recherche.trim())
  if (formulaire.prenom_recherche.trim()) fd.append('prenom_recherche', formulaire.prenom_recherche.trim())
  if (formulaire.surnom.trim()) fd.append('surnom', formulaire.surnom.trim())
  if (formulaire.genre_recherche) fd.append('genre_recherche', formulaire.genre_recherche)
  if (formulaire.comment_connu.trim()) fd.append('comment_connu', formulaire.comment_connu.trim())
  if (formulaire.type_relation) fd.append('type_relation', formulaire.type_relation)
  if (formulaire.type_relation === 'autre' && formulaire.type_relation_autre.trim()) {
    fd.append('type_relation_autre', formulaire.type_relation_autre.trim())
  }
  if (formulaire.localite_rencontre.trim()) fd.append('localite_rencontre', formulaire.localite_rencontre.trim())
  if (formulaire.ecole_rencontre.trim()) fd.append('ecole_rencontre', formulaire.ecole_rencontre.trim())
  if (formulaire.ville_rencontre.trim()) fd.append('ville_rencontre', formulaire.ville_rencontre.trim())
  if (formulaire.pays_id) fd.append('pays_id', formulaire.pays_id)
  if (formulaire.rencontre_reseaux_sociaux) {
    fd.append('rencontre_reseaux_sociaux', 'true')
    if (formulaire.reseaux_sociaux.length > 0) {
      fd.append('reseaux_sociaux', formulaire.reseaux_sociaux.join(','))
    }
  }
  if (formulaire.description_physique.trim()) fd.append('description_physique', formulaire.description_physique.trim())
  if (formulaire.description.trim()) fd.append('description', formulaire.description.trim())
  fd.append('est_anonyme', formulaire.est_anonyme ? 'true' : 'false')
  fd.append('partage_coordonnees', formulaire.partage_coordonnees ? 'true' : 'false')
  if (formulaire.partage_coordonnees) {
    if (formulaire.coordonnees_email.trim()) fd.append('coordonnees_email', formulaire.coordonnees_email.trim())
    if (formulaire.coordonnees_telephone.trim()) fd.append('coordonnees_telephone', formulaire.coordonnees_telephone.trim())
    if (formulaire.coordonnees_whatsapp.trim()) fd.append('coordonnees_whatsapp', formulaire.coordonnees_whatsapp.trim())
  }
  if (photoFichier.value) fd.append('photo', photoFichier.value)

  emit('submit', fd)
}

// ── Helpers pour le recapitulatif ──────────────────────────────
const labelRelation = computed(() => {
  if (!formulaire.type_relation) return null
  if (formulaire.type_relation === 'autre') {
    return formulaire.type_relation_autre.trim()
      ? `Autre : ${formulaire.type_relation_autre.trim()}`
      : 'Autre'
  }
  return TYPES_RELATION.find(t => t.value === formulaire.type_relation)?.label ?? formulaire.type_relation
})

const lignesRecap = computed(() => {
  const lignes: { label: string; valeur: string }[] = []
  lignes.push({ label: 'Nom recherche', valeur: formulaire.nom_recherche })
  if (formulaire.prenom_recherche.trim()) lignes.push({ label: 'Prenom', valeur: formulaire.prenom_recherche })
  if (formulaire.surnom.trim()) lignes.push({ label: 'Surnom', valeur: formulaire.surnom })
  if (formulaire.genre_recherche) {
    const label = GENRES_PERSONNE.find(g => g.value === formulaire.genre_recherche)?.label ?? formulaire.genre_recherche
    lignes.push({ label: 'Genre', valeur: label })
  }
  if (formulaire.comment_connu.trim()) lignes.push({ label: 'Comment connu', valeur: formulaire.comment_connu })
  if (labelRelation.value) lignes.push({ label: 'Type de relation', valeur: labelRelation.value })
  if (formulaire.localite_rencontre.trim()) lignes.push({ label: 'Localite de rencontre', valeur: formulaire.localite_rencontre })
  if (formulaire.ecole_rencontre.trim()) lignes.push({ label: 'Ecole de rencontre', valeur: formulaire.ecole_rencontre })
  if (formulaire.ville_rencontre.trim()) lignes.push({ label: 'Ville de rencontre', valeur: formulaire.ville_rencontre })
  if (formulaire.pays_id) {
    const nomPays = listePays.value.find(p => p.id === formulaire.pays_id)?.nom
    if (nomPays) lignes.push({ label: 'Territoire', valeur: nomPays })
  }
  if (formulaire.rencontre_reseaux_sociaux) {
    const labels = formulaire.reseaux_sociaux
      .map(r => RESEAUX_SOCIAUX.find(rs => rs.value === r)?.label ?? r)
    lignes.push({
      label: 'Rencontre sur reseaux sociaux',
      valeur: labels.length > 0 ? labels.join(', ') : 'Oui',
    })
  }
  if (photoFichier.value) lignes.push({ label: 'Photo', valeur: photoFichier.value.name })
  if (formulaire.description_physique.trim()) lignes.push({ label: 'Description physique', valeur: formulaire.description_physique })
  if (formulaire.description.trim()) lignes.push({ label: 'Description', valeur: formulaire.description })
  // Anonymat masqué pour le moment
  // lignes.push({ label: 'Anonymat', valeur: formulaire.est_anonyme ? 'Oui' : 'Non' })
  if (formulaire.partage_coordonnees) lignes.push({ label: 'Partage coordonnees', valeur: 'Oui' })
  return lignes
})

// Classes communes pour les inputs
const inputClass = 'w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat'
const inputErrorClass = 'w-full rounded-lg border border-red-400 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat'
const labelClass = 'mb-1 block text-sm font-medium text-gray-700'
</script>

<template>
  <div ref="conteneurFormRef" class="mx-auto w-full max-w-2xl rounded-xl border border-gray-200 bg-white p-6 shadow-sm md:p-8">
    <!-- Indicateur d'etapes -->
    <div ref="dotsRef" class="mb-8 flex items-center justify-center gap-1.5">
      <template v-for="etape in etapes" :key="etape.numero">
        <button
          type="button"
          class="dot-indicateur flex h-8 w-8 shrink-0 items-center justify-center rounded-full text-xs font-semibold transition-colors cursor-pointer"
          :class="{
            'bg-custom-chocolat text-white': etape.numero === etapeCourante,
            'bg-custom-green text-white': etape.numero < etapeCourante,
            'bg-gray-200 text-gray-500': etape.numero > etapeCourante,
          }"
          :disabled="enTransition"
          @click="!enTransition && etape.numero < etapeCourante && (etapeCourante = etape.numero)"
        >
          <font-awesome-icon v-if="etape.numero < etapeCourante" :icon="['fas', 'check']" class="text-xs" />
          <span v-else>{{ etape.numero }}</span>
        </button>
        <div
          v-if="etape.numero < etapes.length"
          class="segment-indicateur h-0.5 w-4 md:w-8"
          :class="etape.numero < etapeCourante ? 'bg-custom-green' : 'bg-gray-200'"
          :style="{ transform: etape.numero < etapeCourante ? 'scaleX(1)' : 'scaleX(0)', transformOrigin: 'left' }"
        />
      </template>
    </div>

    <h2 class="mb-1 text-center font-display text-xl font-bold text-gray-800">
      {{ etapes[etapeCourante - 1]?.titre }}
    </h2>
    <p class="mb-6 text-center text-sm text-gray-500">
      Etape {{ etapeCourante }} sur {{ etapes.length }}
    </p>

    <!-- Erreur globale -->
    <p v-if="erreurs.global" class="mb-4 rounded-lg bg-red-50 px-4 py-2 text-sm text-red-600">
      {{ erreurs.global }}
    </p>

    <!-- Conteneur anime des etapes -->
    <div ref="etapeRef">
    <!-- ── Etape 1 : Preferences ──────────────────────── -->
    <div v-if="etapeCourante === 1" class="space-y-5">
      <p class="text-sm text-gray-500">Definissez vos preferences de confidentialite avant de commencer.</p>

      <!-- Anonymat : masqué pour le moment, seuls les avis publics sont permis -->
      <!-- <div class="rounded-lg border border-gray-200 p-4">
        <label class="flex items-start gap-3 cursor-pointer">
          <input
            v-model="formulaire.est_anonyme"
            type="checkbox"
            class="mt-0.5 h-5 w-5 rounded border-gray-300 text-custom-chocolat accent-custom-chocolat"
          >
          <div>
            <span class="text-sm font-medium text-gray-700">Publier en anonyme</span>
            <p class="text-xs text-gray-500 mt-0.5">
              Votre nom et prenom ne seront pas affiches sur l'avis public. Seul « Anonyme » apparaitra.
            </p>
          </div>
        </label>
      </div> -->

      <!-- Partage coordonnees -->
      <div class="rounded-lg border border-gray-200 p-4 space-y-3">
        <label class="flex items-start gap-3 cursor-pointer">
          <input
            v-model="formulaire.partage_coordonnees"
            type="checkbox"
            class="mt-0.5 h-5 w-5 rounded border-gray-300 text-custom-chocolat accent-custom-chocolat"
          >
          <div>
            <span class="text-sm font-medium text-gray-700">Partager mes coordonnees</span>
            <p class="text-xs text-gray-500 mt-0.5">
              Vos coordonnees seront partagees uniquement avec les correspondances mutuelles (jamais affichees publiquement).
            </p>
          </div>
        </label>

        <p v-if="erreurs.partage_coordonnees" class="text-xs text-red-500 mt-1">{{ erreurs.partage_coordonnees }}</p>

        <template v-if="formulaire.partage_coordonnees">
          <div>
            <label :class="labelClass">Email</label>
            <input
              v-model="formulaire.coordonnees_email"
              type="email"
              placeholder="Ex : monmail@exemple.com"
              :class="inputClass"
            >
          </div>
          <div>
            <label :class="labelClass">Telephone</label>
            <input
              v-model="formulaire.coordonnees_telephone"
              type="tel"
              placeholder="Ex : +221 77 123 45 67"
              :class="inputClass"
            >
          </div>
          <div>
            <label :class="labelClass">WhatsApp</label>
            <input
              v-model="formulaire.coordonnees_whatsapp"
              type="tel"
              placeholder="Ex : +221 77 123 45 67"
              :class="inputClass"
            >
          </div>
          <p v-if="erreurs.coordonnees" class="text-xs text-red-500">{{ erreurs.coordonnees }}</p>
        </template>
      </div>
    </div>

    <!-- ── Etape 2 : Identite ─────────────────────────── -->
    <div v-if="etapeCourante === 2" class="space-y-4">
      <div>
        <label :class="labelClass">
          Nom de la personne recherchee <span class="text-red-500">*</span>
        </label>
        <input
          v-model="formulaire.nom_recherche"
          type="text"
          maxlength="100"
          placeholder="Ex : Diallo"
          :class="erreurs.nom_recherche ? inputErrorClass : inputClass"
        >
        <p v-if="erreurs.nom_recherche" class="mt-1 text-xs text-red-500">{{ erreurs.nom_recherche }}</p>
      </div>

      <div>
        <label :class="labelClass">Prenom</label>
        <input
          v-model="formulaire.prenom_recherche"
          type="text"
          maxlength="100"
          placeholder="Ex : Amadou"
          :class="inputClass"
        >
      </div>

      <div>
        <label :class="labelClass">Surnom</label>
        <input
          v-model="formulaire.surnom"
          type="text"
          maxlength="100"
          placeholder="Ex : Le Grand"
          :class="inputClass"
        >
      </div>

      <div>
        <label :class="labelClass">Genre de la personne recherchee</label>
        <div class="flex gap-4">
          <label v-for="genre in GENRES_PERSONNE" :key="genre.value" class="flex items-center gap-2 cursor-pointer" @click="animerSelection">
            <input
              v-model="formulaire.genre_recherche"
              type="radio"
              :value="genre.value"
              class="h-4 w-4 accent-custom-chocolat"
            >
            <span class="text-sm text-gray-700">{{ genre.label }}</span>
          </label>
          <label class="flex items-center gap-2 cursor-pointer">
            <input
              v-model="formulaire.genre_recherche"
              type="radio"
              value=""
              class="h-4 w-4 accent-custom-chocolat"
            >
            <span class="text-sm text-gray-500">Non precise</span>
          </label>
        </div>
      </div>

      <div>
        <label :class="labelClass">Comment le/la connaissiez-vous ?</label>
        <textarea
          v-model="formulaire.comment_connu"
          rows="2"
          maxlength="500"
          placeholder="Ex : On jouait ensemble au football dans le quartier..."
          :class="inputClass"
        />
      </div>
    </div>

    <!-- ── Etape 3 : Relation ─────────────────────────── -->
    <div v-if="etapeCourante === 3" class="space-y-4">
      <p class="text-sm text-gray-500">Quel est votre lien avec la personne ?</p>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <label
          v-for="relation in TYPES_RELATION"
          :key="relation.value"
          class="flex items-center gap-3 rounded-lg border p-4 cursor-pointer transition-colors"
          :class="formulaire.type_relation === relation.value
            ? 'border-custom-chocolat bg-amber-50'
            : 'border-gray-200 hover:border-gray-300'"
          @click="animerSelection"
        >
          <input
            v-model="formulaire.type_relation"
            type="radio"
            :value="relation.value"
            class="h-4 w-4 accent-custom-chocolat"
          >
          <span class="text-sm font-medium text-gray-700">{{ relation.label }}</span>
        </label>
      </div>

      <!-- Precision pour "Autre" -->
      <div v-if="formulaire.type_relation === 'autre'" class="mt-3">
        <label :class="labelClass">Precisez le type de relation</label>
        <input
          v-model="formulaire.type_relation_autre"
          type="text"
          maxlength="200"
          placeholder="Ex : Voisin, partenaire de sport, membre de la meme association..."
          :class="inputClass"
        >
      </div>
    </div>

    <!-- ── Etape 4 : Lieu de rencontre ────────────────── -->
    <div v-if="etapeCourante === 4" class="space-y-4">
      <p class="text-sm text-gray-500">Ou avez-vous rencontre cette personne pour la derniere fois ?</p>

      <!-- Rencontre sur les reseaux sociaux -->
      <div class="rounded-lg border border-gray-200 p-4 space-y-3">
        <label class="flex items-center gap-3 cursor-pointer">
          <input
            v-model="formulaire.rencontre_reseaux_sociaux"
            type="checkbox"
            class="h-5 w-5 rounded border-gray-300 accent-custom-chocolat"
          >
          <span class="text-sm font-medium text-gray-700">J'ai rencontré cette personne sur les reseaux sociaux</span>
        </label>

        <!-- Choix des reseaux sociaux -->
        <div v-if="formulaire.rencontre_reseaux_sociaux" ref="reseauxSociauxRef" class="ml-8 grid grid-cols-2 gap-2">
          <label
            v-for="reseau in RESEAUX_SOCIAUX"
            :key="reseau.value"
            class="flex items-center gap-2 cursor-pointer"
          >
            <input
              v-model="formulaire.reseaux_sociaux"
              type="checkbox"
              :value="reseau.value"
              class="h-4 w-4 rounded border-gray-300 accent-custom-chocolat"
            >
            <span class="text-sm text-gray-700">{{ reseau.label }}</span>
          </label>
        </div>
      </div>

      <template v-if="!formulaire.rencontre_reseaux_sociaux">
        <div>
          <label :class="labelClass">Localite / Quartier</label>
          <input
            v-model="formulaire.localite_rencontre"
            type="text"
            maxlength="200"
            placeholder="Ex : Quartier Plateau, Commune V"
            :class="inputClass"
          >
        </div>

        <div>
          <label :class="labelClass">Ecole / Lieu de travail</label>
          <input
            v-model="formulaire.ecole_rencontre"
            type="text"
            maxlength="250"
            placeholder="Ex : Lycee Lamine Gueye"
            :class="inputClass"
          >
        </div>

        <div>
          <label :class="labelClass">Ville</label>
          <input
            v-model="formulaire.ville_rencontre"
            type="text"
            maxlength="200"
            placeholder="Ex : Dakar"
            :class="inputClass"
          >
        </div>

        <div>
          <label :class="labelClass">Territoire</label>
          <select
            v-model="formulaire.pays_id"
            :class="inputClass"
          >
            <option value="">Selectionnez un territoire</option>
            <option v-for="pays in listePays" :key="pays.id" :value="pays.id">
              {{ pays.nom }}
            </option>
          </select>
        </div>
      </template>
    </div>

    <!-- ── Etape 5 : Photo & apparence ────────────────── -->
    <div v-if="etapeCourante === 5" class="space-y-4">
      <p class="text-sm text-gray-500">Ajoutez une photo et une description pour aider a identifier la personne.</p>

      <!-- Upload photo -->
      <div>
        <label :class="labelClass">Photo (optionnelle)</label>
        <div v-if="!photoPreview" class="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center">
          <font-awesome-icon :icon="['fas', 'camera']" class="text-3xl text-gray-400 mb-2" />
          <p class="text-sm text-gray-500 mb-2">JPEG, PNG ou WebP, max 5 Mo</p>
          <label class="inline-block px-4 py-2 bg-gray-100 text-gray-700 text-sm rounded-lg cursor-pointer hover:bg-gray-200 transition-colors">
            Choisir une photo
            <input type="file" accept="image/jpeg,image/png,image/webp" class="hidden" @change="onPhotoChange">
          </label>
        </div>
        <div v-else class="relative inline-block">
          <img ref="photoPreviewImgRef" :src="photoPreview" alt="Apercu" class="h-48 w-auto rounded-lg object-cover border border-gray-200">
          <button
            type="button"
            class="absolute -top-2 -right-2 h-6 w-6 rounded-full bg-red-500 text-white text-xs flex items-center justify-center hover:bg-red-600 cursor-pointer"
            @click="supprimerPhoto"
          >
            <font-awesome-icon :icon="['fas', 'times']" />
          </button>
        </div>
        <p v-if="erreurs.photo" class="mt-1 text-xs text-red-500">{{ erreurs.photo }}</p>
      </div>

      <!-- Description physique -->
      <div>
        <label :class="labelClass">Description physique</label>
        <textarea
          v-model="formulaire.description_physique"
          rows="3"
          placeholder="Taille, corpulence, signe distinctif, style vestimentaire..."
          :class="inputClass"
        />
      </div>

      <!-- Description generale -->
      <div>
        <label :class="labelClass">Description complementaire</label>
        <textarea
          v-model="formulaire.description"
          rows="3"
          placeholder="Tout detail supplementaire qui pourrait aider a retrouver cette personne..."
          :class="inputClass"
        />
      </div>
    </div>

    <!-- ── Etape 6 : Recapitulatif ────────────────────── -->
    <div v-if="etapeCourante === 6" class="space-y-5">
      <p class="text-sm text-gray-500">Verifiez les informations avant de publier votre avis de recherche.</p>

      <div class="rounded-lg border border-gray-200 bg-gray-50 p-4">
        <dl class="space-y-2">
          <div v-for="ligne in lignesRecap" :key="ligne.label" class="flex gap-2 text-sm">
            <dt class="shrink-0 font-medium text-gray-500 min-w-36">{{ ligne.label }} :</dt>
            <dd class="text-gray-800">{{ ligne.valeur }}</dd>
          </div>
        </dl>
      </div>

      <!-- Apercu photo -->
      <div v-if="photoPreview" class="text-center">
        <img :src="photoPreview" alt="Photo" class="h-32 w-auto rounded-lg object-cover border border-gray-200 mx-auto">
      </div>

      <div class="rounded-lg bg-amber-50 border border-amber-200 px-4 py-3 text-sm text-amber-800">
        <font-awesome-icon :icon="['fas', 'info-circle']" class="mr-2" />
        Votre avis sera publie immediatement et visible par tous les visiteurs.
        <span v-if="formulaire.partage_coordonnees">
          Vos coordonnees ne seront partagees qu'avec les correspondances mutuelles.
        </span>
      </div>
    </div>

    </div><!-- /etapeRef -->

    <!-- ── Boutons de navigation ──────────────────────── -->
    <div class="mt-8 flex items-center justify-between">
      <button
        v-if="etapeCourante > 1"
        type="button"
        class="rounded-lg border border-gray-300 px-5 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 hover:scale-105 active:scale-95 duration-200 cursor-pointer"
        :disabled="enTransition"
        @click="precedent"
      >
        Precedent
      </button>
      <button
        v-else
        type="button"
        class="rounded-lg border border-gray-300 px-5 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 hover:scale-105 active:scale-95 duration-200 cursor-pointer"
        @click="emit('annuler')"
      >
        Annuler
      </button>

      <button
        v-if="etapeCourante < etapes.length"
        type="button"
        class="rounded-lg bg-custom-chocolat px-5 py-2 text-sm font-medium text-white transition-colors hover:bg-custom-chocolat/90 hover:scale-105 active:scale-95 duration-200 cursor-pointer"
        :disabled="enTransition"
        @click="suivant"
      >
        Suivant
      </button>
      <button
        v-else
        type="button"
        class="rounded-lg bg-custom-green px-5 py-2 text-sm font-medium text-white transition-colors hover:bg-custom-green/90 hover:scale-105 active:scale-95 duration-200 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
        :disabled="chargementSoumission"
        @click="soumettre"
      >
        <font-awesome-icon v-if="chargementSoumission" :icon="['fas', 'spinner']" class="mr-2 animate-spin" />
        {{ props.mode === 'modification' ? 'Enregistrer' : 'Publier l\'avis' }}
      </button>
    </div>
  </div>
</template>

<style scoped>
@media (prefers-reduced-motion: reduce) {
  .hover\:scale-105:hover,
  .active\:scale-95:active {
    transform: none !important;
  }
}
</style>
