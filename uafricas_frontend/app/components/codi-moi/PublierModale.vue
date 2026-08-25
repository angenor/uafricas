<template>
  <AfricansModale
    :model-value="modelValue"
    ton="vert"
    titre="Nouvelle publication"
    sous-titre="Partager une valeur africaine"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <form id="af-form-codimoi" class="flex flex-col gap-5" @submit.prevent="soumettre">
      <!-- La catégorie ouvre la marche alors que la maquette la met en second :
           c'est elle qui décide du libellé du champ suivant et de l'existence
           de la couleur de fond. La choisir après serait la choisir trop tard. -->
      <div class="grid gap-4 sm:grid-cols-3">
        <AfricansChamp v-model="formulaire.categorie" libelle="Catégorie" type="select">
          <option value="">Sélectionnez…</option>
          <option v-for="c in CATEGORIES" :key="c.value" :value="c.value">{{ c.label }}</option>
        </AfricansChamp>

        <AfricansChamp v-model="formulaire.pays" libelle="Territoire" type="select">
          <option value="">Sélectionnez…</option>
          <option v-for="p in PAYS_AFRICAINS" :key="p" :value="p">{{ p }}</option>
        </AfricansChamp>

        <AfricansChamp v-model="formulaire.groupeEthnique" libelle="Groupe ethnique" type="select">
          <option value="">Aucun</option>
          <option v-for="g in GROUPES_ETHNIQUES" :key="g" :value="g">{{ g }}</option>
        </AfricansChamp>
      </div>

      <AfricansChamp
        v-model="formulaire.contenu"
        :libelle="citationOuProverbe ? 'Proverbe / Citation' : 'Titre du contenu'"
        type="textarea"
        :placeholder="citationOuProverbe ? 'Saisissez le proverbe ou la citation…' : 'Titre de votre publication…'"
      />

      <AfricansChamp
        v-if="formulaire.categorie === 'citation'"
        v-model="formulaire.nomAuteur"
        libelle="Auteur de la citation"
        placeholder="Ex : Nelson Mandela"
      />

      <div v-if="citationOuProverbe" class="flex flex-col gap-2">
        <p class="text-[14px]/[1.4] text-af-atone italic">Couleur de fond</p>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="couleur in COULEURS_FOND"
            :key="couleur"
            type="button"
            class="size-9 rounded-full border-2 transition"
            :class="formulaire.couleurFond === couleur
              ? 'border-af-encre scale-110'
              : 'border-transparent hover:scale-105'"
            :style="{ backgroundColor: couleur }"
            :aria-label="`Couleur ${couleur}`"
            :aria-pressed="formulaire.couleurFond === couleur"
            @click="formulaire.couleurFond = couleur"
          />
        </div>
      </div>

      <AfricansChamp
        v-model="formulaire.explication"
        libelle="Explication / Contexte"
        type="textarea"
        placeholder="Expliquez le sens ou le contexte de cette valeur…"
      />

      <AfricansChamp
        v-model="formulaire.hashtagsRaw"
        libelle="Hashtags"
        placeholder="sagesse, proverbe, afrique"
        aide="Séparés par des virgules."
      />

      <p v-if="erreurLocale" class="text-[12px]/[1.4] text-af-live">{{ erreurLocale }}</p>
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
        type="submit"
        form="af-form-codimoi"
        :desactive="enCours"
        :tourne="enCours"
        :icone="enCours ? 'fa-solid fa-spinner' : undefined"
      >
        {{ enCours ? 'Publication…' : 'Publier' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
import {
  CATEGORIES_POST,
  COULEURS_FOND,
  PAYS_AFRICAINS,
  GROUPES_ETHNIQUES,
  type CategoriePost,
} from '~/composables/useCodiMoi'

export interface BrouillonCodimoi {
  categorie: CategoriePost
  contenu: string
  nomAuteur: string
  explication: string
  pays: string
  groupeEthnique: string
  couleurFond: string
}

const props = defineProps<{
  modelValue: boolean
  /** Publication en vol : le formulaire attend, sans se vider. */
  enCours?: boolean
  /**
   * Texte déjà saisi ailleurs : le composeur du fil d'actualité. Il amorce le
   * champ de contenu à l'ouverture : ce que l'utilisateur a tapé ne doit pas
   * être à retaper parce que la saisie a changé de fenêtre.
   */
  contenuInitial?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [boolean]
  publier: [BrouillonCodimoi & { hashtags: string[] }]
}>()

// « Tout » est une valeur de FILTRE, pas une catégorie publiable.
const CATEGORIES = CATEGORIES_POST.filter(c => c.value !== '')

const vide = () => ({
  categorie: '' as CategoriePost | '',
  contenu: '',
  nomAuteur: '',
  explication: '',
  pays: '',
  groupeEthnique: '',
  couleurFond: COULEURS_FOND[0] ?? '#2D5A27',
  hashtagsRaw: '',
})

const formulaire = ref(vide())
const erreurLocale = ref<string | null>(null)

const citationOuProverbe = computed(() =>
  formulaire.value.categorie === 'proverbe_adage' || formulaire.value.categorie === 'citation')

/**
 * La validation est ici et non sur `required` : les champs sont rendus par
 * `AfricansChamp`, qui ne porte pas l'attribut, et un `required` silencieux
 * sur un `<select>` masqué par une condition bloquerait la soumission sans
 * dire pourquoi.
 */
function soumettre() {
  const f = formulaire.value
  if (!f.categorie || !f.contenu.trim() || !f.pays) {
    erreurLocale.value = 'Catégorie, contenu et territoire sont obligatoires.'
    return
  }
  erreurLocale.value = null

  emit('publier', {
    categorie: f.categorie,
    contenu: f.contenu.trim(),
    nomAuteur: f.nomAuteur.trim(),
    explication: f.explication.trim(),
    pays: f.pays,
    groupeEthnique: f.groupeEthnique,
    couleurFond: citationOuProverbe.value ? f.couleurFond : '',
    hashtags: f.hashtagsRaw.split(',').map(h => h.trim()).filter(Boolean),
  })
}

// Le brouillon est remis à zéro à la FERMETURE, pas à l'ouverture : si la
// publication échoue, la modale reste ouverte et la saisie doit survivre.
watch(() => props.modelValue, (ouvert) => {
  if (ouvert) {
    if (props.contenuInitial) formulaire.value.contenu = props.contenuInitial
    return
  }
  formulaire.value = vide()
  erreurLocale.value = null
})
</script>
