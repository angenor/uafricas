<script setup lang="ts">
import { useUserStore } from '~/stores/user'

/**
 * Dépôt d'un document dans Librafrica.
 *
 * Le formulaire était écrit en clair dans la page, 200 lignes de template
 * mêlées à la grille de documents. Les champs sont les mêmes, dans le même
 * ordre, et le consentement porte toujours le nom réel du membre.
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
  { valeur: 'Téléchargeable', libelle: 'Téléchargeable' }]
const RAPPORTS = ['Auteur', 'Co-auteur', 'Aucun']

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

const surImage = (e: Event) => { image.value = (e.target as HTMLInputElement).files?.[0] ?? null }
const surFichier = (e: Event) => { fichier.value = (e.target as HTMLInputElement).files?.[0] ?? null }

/** Les deux conditions bloquantes sont dites AVANT l'envoi, pas par une alerte. */
const soumettre = () => {
  erreurLocale.value = null
  if (!fichier.value) {
    erreurLocale.value = 'Veuillez sélectionner le document PDF.'
    return
  }
  if (!formulaire.value.consent) {
    erreurLocale.value = 'Veuillez accepter la diffusion de cette publication.'
    return
  }
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
})
</script>

<template>
  <AfricansModale
    :model-value="modelValue"
    titre="Déposer un document"
    sous-titre="Mémoire, thèse, rapport, étude ou publication"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <form class="flex flex-col gap-5" @submit.prevent="soumettre">
      <AfricansChamp v-model="formulaire.titre" libelle="Titre du document" placeholder="Saisissez le titre" obligatoire />

      <AfricansChamp
        v-model="formulaire.description"
        libelle="Description"
        type="textarea"
        placeholder="Décrivez le contenu du document"
        obligatoire
      />

      <div class="grid gap-4 sm:grid-cols-2">
        <label class="flex cursor-pointer flex-col items-center gap-2 rounded-[10px] border-2 border-dashed border-af-bordure p-4 text-center transition hover:border-af-chocolat">
          <font-awesome-icon icon="fa-solid fa-image" class="text-2xl text-af-chocolat" />
          <span class="text-[14px]/[1.4] font-bold">Image de couverture</span>
          <span class="text-[12px]/[1.4] text-af-atone">{{ image ? image.name : 'Cliquez pour sélectionner' }}</span>
          <input type="file" class="sr-only" accept="image/*" @change="surImage" />
        </label>

        <label class="flex cursor-pointer flex-col items-center gap-2 rounded-[10px] border-2 border-dashed border-af-bordure p-4 text-center transition hover:border-af-chocolat">
          <font-awesome-icon icon="fa-solid fa-file-pdf" class="text-2xl text-af-chocolat" />
          <span class="text-[14px]/[1.4] font-bold">Document PDF</span>
          <span class="text-[12px]/[1.4] text-af-atone">{{ fichier ? fichier.name : 'Cliquez pour téléverser' }}</span>
          <input type="file" class="sr-only" accept=".pdf" @change="surFichier" />
        </label>
      </div>

      <div class="grid gap-4 sm:grid-cols-2">
        <label class="flex flex-col gap-2">
          <span class="text-[14px]/[1.4] font-bold">Type de document</span>
          <select
            v-model="formulaire.type"
            required
            class="h-11 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
          >
            <option value="">Sélectionnez un type</option>
            <option v-for="t in TYPES" :key="t" :value="t">{{ t }}</option>
          </select>
        </label>

        <label class="flex flex-col gap-2">
          <span class="text-[14px]/[1.4] font-bold">Accès</span>
          <select
            v-model="formulaire.acces"
            required
            class="h-11 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
          >
            <option value="">Sélectionnez un type d'accès</option>
            <option v-for="a in ACCES" :key="a.valeur" :value="a.valeur">{{ a.libelle }}</option>
          </select>
        </label>
      </div>

      <AfricansChamp
        v-model="formulaire.auteurBiblio"
        libelle="Informations sur l'auteur"
        type="textarea"
        placeholder="Biographie et informations sur l'auteur"
        obligatoire
      />

      <div class="grid gap-4 sm:grid-cols-2">
        <label class="flex flex-col gap-2">
          <span class="text-[14px]/[1.4] font-bold">Date de publication</span>
          <input
            v-model="formulaire.datePublication"
            type="date"
            required
            class="h-11 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
          />
        </label>

        <label class="flex flex-col gap-2">
          <span class="text-[14px]/[1.4] font-bold">Votre rapport avec le document</span>
          <select
            v-model="formulaire.rapport"
            class="h-11 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
          >
            <option value="">Sélectionnez votre rapport</option>
            <option v-for="r in RAPPORTS" :key="r" :value="r">{{ r }}</option>
          </select>
        </label>
      </div>

      <label class="flex cursor-pointer items-start gap-3 text-[14px]/[1.4] text-af-corps">
        <input v-model="formulaire.consent" type="checkbox" class="mt-1 size-4 accent-af-chocolat" />
        <span>
          Moi <strong class="font-bold text-af-encre">{{ utilisateurNom }}</strong>, j'accepte la
          diffusion de cette publication.
        </span>
      </label>

      <p v-if="erreurLocale || erreur" role="alert" class="flex items-start gap-2 text-[12px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5" />
        {{ erreurLocale || erreur }}
      </p>
    </form>

    <template #actions>
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="$emit('update:modelValue', false)"
      >
        Annuler
      </button>
      <AfricansBouton
        :desactive="enCours"
        :tourne="enCours"
        :icone="enCours ? 'fa-solid fa-spinner' : undefined"
        @click="soumettre"
      >
        {{ enCours ? 'Envoi en cours…' : 'Soumettre' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
