<script setup lang="ts">
import { useUserStore } from '~/stores/user'

/**
 * Dépôt d'un document dans Librafrica, en TROIS ÉTAPES.
 *
 * Le formulaire tenait d'un bloc : huit champs, deux téléversements et un
 * consentement, dans une modale qu'il fallait faire défiler pour atteindre le
 * bouton d'envoi. On n'y voyait ni où l'on en était, ni ce qui manquait.
 *
 * Le découpage a aussi permis de fermer un trou : les attributs `required` du
 * formulaire étaient DÉCORATIFS. Le bouton d'envoi n'est pas un `submit`, il
 * appelle `soumettre()` directement — la validation native du navigateur ne
 * s'exécutait donc jamais, et `soumettre()` ne vérifiait que le PDF et le
 * consentement. Un document sans titre, sans description, sans type ni date
 * partait au serveur. Chaque étape valide désormais ses propres champs.
 */
interface FormulaireDocument {
  titre: string
  description: string
  type: string
  acces: string
  auteurBiblio: string
  datePublication: string
  rapport: string
  consent: boolean
}

const props = defineProps<{ modelValue: boolean, enCours: boolean, erreur: string | null }>()

const emit = defineEmits<{
  'update:modelValue': [boolean]
  'soumettre': [{ formulaire: FormulaireDocument, image: File | null, fichier: File | null }]
}>()

const TYPES = ['Roman', 'Livre', 'Thèse', 'Mémoire', 'Rapport', 'Autre']
const ACCES = [
  { valeur: 'Lecture', libelle: 'Lecture seule' },
  { valeur: 'Téléchargeable', libelle: 'Téléchargeable' },
]
const RAPPORTS = ['Auteur', 'Co-auteur', 'Aucun']

const ETAPES = [
  { titre: 'Le document', icone: 'fa-solid fa-file-lines' },
  { titre: 'Le classement', icone: 'fa-solid fa-tags' },
  { titre: 'L\'auteur', icone: 'fa-solid fa-user-pen' },
] as const

const userStore = useUserStore()
const utilisateurNom = computed(() => userStore.fullName || userStore.displayName || 'Utilisateur')

const vide = (): FormulaireDocument => ({
  titre: '',
  description: '',
  type: '',
  acces: '',
  auteurBiblio: '',
  datePublication: '',
  rapport: '',
  consent: false,
})

const formulaire = ref<FormulaireDocument>(vide())
const image = ref<File | null>(null)
const fichier = ref<File | null>(null)
const erreurLocale = ref<string | null>(null)
const etape = ref(0)

const surImage = (e: Event) => { image.value = (e.target as HTMLInputElement).files?.[0] ?? null }
const surFichier = (e: Event) => { fichier.value = (e.target as HTMLInputElement).files?.[0] ?? null }

/**
 * Ce qui manque à l'étape courante, énoncé plutôt que signalé par un champ
 * rouge : la modale est étroite, et un liseré ne dit pas ce qu'il attend.
 */
const manqueEtape = (i: number): string | null => {
  const f = formulaire.value
  if (i === 0) {
    if (!f.titre.trim()) return 'Le titre du document est nécessaire.'
    if (!f.description.trim()) return 'La description est nécessaire.'
    if (!fichier.value) return 'Le document PDF est nécessaire.'
  }
  if (i === 1) {
    if (!f.type) return 'Choisissez un type de document.'
    if (!f.acces) return 'Choisissez un type d\'accès.'
    if (!f.datePublication) return 'La date de publication est nécessaire.'
  }
  if (i === 2) {
    if (!f.auteurBiblio.trim()) return 'Les informations sur l\'auteur sont nécessaires.'
    if (!f.consent) return 'Vous devez accepter la diffusion de cette publication.'
  }
  return null
}

/** Une étape franchie est une étape complète : on peut y revenir librement. */
const etapesFranchies = computed(() =>
  ETAPES.map((_, i) => manqueEtape(i) === null),
)

const suivant = () => {
  const manque = manqueEtape(etape.value)
  if (manque) {
    erreurLocale.value = manque
    return
  }
  erreurLocale.value = null
  etape.value += 1
}

const precedent = () => {
  erreurLocale.value = null
  etape.value -= 1
}

const allerA = (i: number) => {
  // On ne saute en avant que par des étapes déjà complètes.
  if (i > etape.value && ETAPES.slice(0, i).some((_, k) => !etapesFranchies.value[k])) return
  erreurLocale.value = null
  etape.value = i
}

const soumettre = () => {
  // Toutes les étapes sont revérifiées : sauter la dernière ne doit pas
  // permettre d'envoyer un dossier incomplet.
  for (let i = 0; i < ETAPES.length; i++) {
    const manque = manqueEtape(i)
    if (manque) {
      etape.value = i
      erreurLocale.value = manque
      return
    }
  }
  erreurLocale.value = null
  emit('soumettre', { formulaire: formulaire.value, image: image.value, fichier: fichier.value })
}

// Remise à zéro à la FERMETURE : sur un échec, la modale reste ouverte et la
// saisie doit survivre.
watch(() => props.modelValue, (ouvert) => {
  if (ouvert) return
  formulaire.value = vide()
  image.value = null
  fichier.value = null
  erreurLocale.value = null
  etape.value = 0
})
</script>

<template>
  <AfricansModale
    :model-value="modelValue"
    titre="Déposer un document"
    :sous-titre="`Étape ${etape + 1} sur ${ETAPES.length} : ${ETAPES[etape]!.titre}`"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <div class="flex flex-col gap-5">
      <!-- Fil des étapes. Les pastilles sont des boutons, mais on ne saute en
           avant que par des étapes déjà complètes : autrement le fil
           laisserait croire qu'on peut ignorer un champ nécessaire. -->
      <nav class="flex gap-2" aria-label="Étapes du dépôt">
        <button
          v-for="(e, i) in ETAPES"
          :key="e.titre"
          type="button"
          class="flex min-w-0 flex-1 items-center gap-2 rounded-lg border px-3 py-2 text-left transition"
          :class="i === etape
            ? 'border-af-chocolat bg-af-chocolat/[0.07]'
            : 'border-af-bordure hover:border-af-chocolat'"
          :aria-current="i === etape ? 'step' : undefined"
          @click="allerA(i)"
        >
          <span
            class="grid size-6 shrink-0 place-items-center rounded-full text-[11px] font-bold"
            :class="i < etape ? 'bg-af-vert text-white' : i === etape ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-atone'"
          >
            <font-awesome-icon v-if="i < etape" icon="fa-solid fa-check" />
            <template v-else>{{ i + 1 }}</template>
          </span>
          <span class="min-w-0 truncate text-[12px]/[1.3] font-bold" :class="i === etape ? 'text-af-chocolat' : 'text-af-corps'">
            {{ e.titre }}
          </span>
        </button>
      </nav>

      <!-- ─── 1. Le document ─── -->
      <template v-if="etape === 0">
        <AfricansChamp v-model="formulaire.titre" libelle="Titre du document *" placeholder="Saisissez le titre" />

        <AfricansChamp
          v-model="formulaire.description"
          libelle="Description *"
          type="textarea"
          placeholder="Décrivez le contenu du document"
        />

        <div class="grid gap-4 sm:grid-cols-2">
          <label
            class="flex cursor-pointer flex-col items-center gap-2 rounded-[10px] border-2 border-dashed p-4 text-center transition hover:border-af-chocolat"
            :class="fichier ? 'border-af-vert' : 'border-af-bordure'"
          >
            <font-awesome-icon icon="fa-solid fa-file-pdf" class="text-2xl" :class="fichier ? 'text-af-vert' : 'text-af-chocolat'" />
            <span class="text-[14px]/[1.4] font-bold">Document PDF *</span>
            <span class="w-full truncate text-[12px]/[1.4] text-af-atone">{{ fichier ? fichier.name : 'Cliquez pour téléverser' }}</span>
            <input type="file" class="sr-only" accept=".pdf" @change="surFichier" />
          </label>

          <label class="flex cursor-pointer flex-col items-center gap-2 rounded-[10px] border-2 border-dashed border-af-bordure p-4 text-center transition hover:border-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-image" class="text-2xl text-af-chocolat" />
            <span class="text-[14px]/[1.4] font-bold">Image de couverture</span>
            <span class="w-full truncate text-[12px]/[1.4] text-af-atone">{{ image ? image.name : 'Facultative' }}</span>
            <input type="file" class="sr-only" accept="image/*" @change="surImage" />
          </label>
        </div>
      </template>

      <!-- ─── 2. Le classement ─── -->
      <template v-else-if="etape === 1">
        <AfricansChamp v-model="formulaire.type" libelle="Type de document *" type="select">
          <option value="">Sélectionnez un type</option>
          <option v-for="t in TYPES" :key="t" :value="t">{{ t }}</option>
        </AfricansChamp>

        <AfricansChamp v-model="formulaire.acces" libelle="Accès *" type="select">
          <option value="">Sélectionnez un type d'accès</option>
          <option v-for="a in ACCES" :key="a.valeur" :value="a.valeur">{{ a.libelle }}</option>
        </AfricansChamp>

        <label class="flex flex-col gap-2">
          <span class="text-[14px]/[1.4] text-af-atone italic">Date de publication *</span>
          <input
            v-model="formulaire.datePublication"
            type="date"
            class="h-11 w-full rounded-md border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:border-af-chocolat focus:outline-none"
          />
        </label>
      </template>

      <!-- ─── 3. L'auteur et la diffusion ─── -->
      <template v-else>
        <AfricansChamp
          v-model="formulaire.auteurBiblio"
          libelle="Informations sur l'auteur *"
          type="textarea"
          placeholder="Biographie et informations sur l'auteur"
        />

        <AfricansChamp v-model="formulaire.rapport" libelle="Votre rapport avec le document" type="select">
          <option value="">Sélectionnez votre rapport</option>
          <option v-for="r in RAPPORTS" :key="r" :value="r">{{ r }}</option>
        </AfricansChamp>

        <label class="flex cursor-pointer items-start gap-3 text-[14px]/[1.4] text-af-corps">
          <input v-model="formulaire.consent" type="checkbox" class="mt-1 size-4 accent-af-chocolat" />
          <span>
            Moi <strong class="font-bold text-af-encre">{{ utilisateurNom }}</strong>, j'accepte la
            diffusion de cette publication.
          </span>
        </label>
      </template>

      <p v-if="erreurLocale || erreur" role="alert" class="flex items-start gap-2 text-[12px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5 shrink-0" />
        {{ erreurLocale || erreur }}
      </p>
    </div>

    <template #actions>
      <button
        v-if="etape === 0"
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="$emit('update:modelValue', false)"
      >
        Annuler
      </button>
      <AfricansBouton v-else variante="secondaire" icone="fa-solid fa-arrow-left" @click="precedent">
        Précédent
      </AfricansBouton>

      <AfricansBouton
        v-if="etape < ETAPES.length - 1"
        icone="fa-solid fa-arrow-right"
        @click="suivant"
      >
        Suivant
      </AfricansBouton>
      <AfricansBouton
        v-else
        :desactive="enCours"
        :tourne="enCours"
        :icone="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
        @click="soumettre"
      >
        {{ enCours ? 'Envoi en cours…' : 'Déposer' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
