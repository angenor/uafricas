<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Proposer un échange"
        :sous-titre="form.type === 'hors_afrique'
          ? 'Apportez votre expertise depuis l\'extérieur vers un pays d\'Afrique.'
          : 'Partagez votre expertise d\'un pays d\'Afrique à un autre et renforcez les compétences du continent.'"
        image="/images/alliance-afrique.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Sabbafrica', vers: '/echanges-sabbatiques' },
          { libelle: 'Proposer un échange' },
        ]"
      />
    </template>

    <div class="rounded-[10px] border border-af-bordure bg-white p-6 md:p-8">
      <h1 class="text-[20px]/[1.4] font-bold text-af-encre">
        Proposer un projet d'échange sabbatique
      </h1>
      <p class="mt-1 text-[14px]/[1.4] text-af-atone">
        Les champs marqués d'un <span class="text-af-live">*</span> sont obligatoires.
      </p>

      <AfricansEtapes :etapes="ETAPES" :courante="etapeCourante" class="my-6" @aller="etapeCourante = $event" />

      <p
        v-if="erreur"
        class="mb-5 flex items-center gap-2 rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreur }}
      </p>

      <!-- `v-show` et non `v-if` : l'éditeur de description est une instance
           EditorJS, et les deux champs de fichier gardent l'aperçu du fichier
           choisi. Les démonter à chaque changement d'étape détruirait l'un et
           viderait l'affichage des autres. -->
      <form class="flex flex-col gap-5" @submit.prevent="handleSubmit">
        <!-- ─── Étape 1 : le projet ─── -->
        <div v-show="etapeCourante === 0" class="flex flex-col gap-5">
          <fieldset>
            <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">
              Type de programme <span class="not-italic text-af-live">*</span>
            </legend>
            <div class="flex flex-col gap-3 sm:flex-row">
              <label v-for="type in TYPES_SELECTION" :key="type.value" class="flex-1 cursor-pointer">
                <input
                  type="radio"
                  name="type-programme"
                  :value="type.value"
                  :checked="form.type === type.value"
                  class="sr-only peer"
                  @change="changerType(type.value as TypeProgramme)"
                />
                <span
                  class="block rounded-lg border-2 px-4 py-3 text-center text-[14px]/[1.4] font-bold transition peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-af-chocolat"
                  :class="form.type === type.value
                    ? 'border-af-vert bg-af-vert/10 text-af-vert'
                    : 'border-af-bordure text-af-atone hover:border-af-chocolat'"
                >
                  <font-awesome-icon :icon="`fa-solid fa-${type.icon}`" class="mr-2" />
                  {{ type.label }}
                </span>
              </label>
            </div>
          </fieldset>

          <AfricansChamp
            v-model="form.titre"
            libelle="Titre du projet"
            placeholder="Ex: Programme d'échange en ingénierie agricole"
            obligatoire
          />

          <div class="flex flex-col gap-2">
            <p class="text-[14px]/[1.4] text-af-atone italic">
              Description du projet <span class="not-italic text-af-live">*</span>
            </p>
            <CommonEditorJs
              id="sabbatique-description-editor"
              ref="editorRef"
              v-model="form.descriptionData"
              placeholder="Décrivez votre projet d'échange : objectifs, activités prévues, profil recherché…"
              :tools="['header', 'list', 'paragraph', 'quote', 'delimiter', 'marker', 'underline']"
              min-height="200px"
            />
          </div>

          <AfricansChamp v-model="form.domaine" libelle="Domaine d'intervention" type="select" obligatoire>
            <option value="" disabled>Choisir un domaine</option>
            <option v-for="domaine in DOMAINES_FORM" :key="domaine.value" :value="domaine.value">
              {{ domaine.label }}
            </option>
          </AfricansChamp>

          <AfricansChamp
            v-if="form.domaine === 'autre'"
            v-model="form.domainePrecision"
            libelle="Précisez le domaine d'intervention"
            obligatoire
          />
        </div>

        <!-- ─── Étape 2 : lieu et calendrier ─── -->
        <div v-show="etapeCourante === 1" class="flex flex-col gap-5">
          <div class="grid gap-5 md:grid-cols-2">
            <AfricansChamp v-model="form.pays" libelle="Territoire" type="select" obligatoire>
              <option value="" disabled>Choisir un territoire</option>
              <option v-for="pays in PAYS_FORM" :key="pays.value" :value="pays.value">
                {{ pays.label }}
              </option>
            </AfricansChamp>
            <AfricansChamp v-model="form.ville" libelle="Ville" placeholder="Ex: Dakar" />
          </div>

          <AfricansChamp
            v-model="form.duree"
            libelle="Durée du programme"
            type="select"
            aide="Durée comprise entre 2 semaines et 12 mois."
            obligatoire
          >
            <option value="" disabled>Choisir une durée</option>
            <option v-for="duree in DUREES" :key="duree.value" :value="duree.value">
              {{ duree.label }}
            </option>
          </AfricansChamp>

          <!-- Champs natifs : `date` n'est pas un type d'AfricansChamp, qui ne
               rend que de la saisie libre. -->
          <div class="grid gap-5 md:grid-cols-2">
            <div class="flex flex-col gap-2">
              <label for="date-debut" class="text-[14px]/[1.4] text-af-atone italic">
                Date de début <span class="not-italic text-af-live">*</span>
              </label>
              <input
                id="date-debut"
                v-model="form.dateDebut"
                type="date"
                class="h-11 w-full rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] text-af-encre focus:border-af-chocolat focus:outline-none"
              />
            </div>
            <div class="flex flex-col gap-2">
              <label for="date-fin" class="text-[14px]/[1.4] text-af-atone italic">
                Date de fin <span class="not-italic text-af-live">*</span>
              </label>
              <input
                id="date-fin"
                v-model="form.dateFin"
                type="date"
                class="h-11 w-full rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] text-af-encre focus:border-af-chocolat focus:outline-none"
              />
            </div>
          </div>
        </div>

        <!-- ─── Étape 3 : conditions et pièces jointes ─── -->
        <div v-show="etapeCourante === 2" class="flex flex-col gap-5">
          <fieldset>
            <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">Prise en charge proposée</legend>
            <div class="flex flex-wrap gap-4">
              <label
                v-for="prise in PRISES_EN_CHARGE"
                :key="prise.value"
                class="flex cursor-pointer items-center gap-2 text-[14px]/[1.4] text-af-corps"
              >
                <input
                  v-model="form.prisesEnCharge"
                  type="checkbox"
                  :value="prise.value"
                  class="size-4 rounded border-af-bordure accent-af-vert"
                />
                {{ prise.label }}
              </label>
            </div>
          </fieldset>

          <div class="flex flex-col gap-2 rounded-lg border border-af-vert/30 bg-af-vert/5 p-4">
            <label for="couverture" class="text-[14px]/[1.4] font-bold text-af-vert">
              Image de couverture
            </label>
            <input
              id="couverture"
              type="file"
              accept="image/*"
              class="w-full text-[14px]/[1.4] text-af-corps file:mr-4 file:rounded-md file:border-0 file:bg-af-vert file:px-4 file:py-2 file:text-[14px] file:font-bold file:text-white hover:file:opacity-90"
              @change="handleCouvertureChange"
            />
          </div>

          <div class="flex flex-col gap-2 rounded-lg border border-af-chocolat/30 bg-af-chocolat/5 p-4">
            <label for="document" class="text-[14px]/[1.4] font-bold text-af-chocolat">
              Document du projet (PDF)
            </label>
            <input
              id="document"
              type="file"
              accept=".pdf"
              class="w-full text-[14px]/[1.4] text-af-corps file:mr-4 file:rounded-md file:border-0 file:bg-af-chocolat file:px-4 file:py-2 file:text-[14px] file:font-bold file:text-white hover:file:opacity-90"
              @change="handleDocumentChange"
            />
          </div>
        </div>

        <!-- ─── Étape 4 : l'organisation ─── -->
        <div v-show="etapeCourante === 3" class="flex flex-col gap-5">
          <fieldset>
            <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">
              Type d'organisation <span class="not-italic text-af-live">*</span>
            </legend>
            <div class="grid gap-3 sm:grid-cols-3">
              <label v-for="org in TYPES_ORGANISATION" :key="org.value" class="cursor-pointer">
                <input
                  v-model="form.typeOrganisation"
                  type="radio"
                  name="type-organisation"
                  :value="org.value"
                  class="sr-only peer"
                />
                <span
                  class="block rounded-lg border-2 px-2 py-3 text-center text-[14px]/[1.4] font-bold transition peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-af-chocolat"
                  :class="form.typeOrganisation === org.value
                    ? 'border-af-vert bg-af-vert/10 text-af-vert'
                    : 'border-af-bordure text-af-atone hover:border-af-chocolat'"
                >
                  <font-awesome-icon :icon="`fa-solid fa-${org.icon}`" class="mr-2" />
                  {{ org.label }}
                </span>
              </label>
            </div>
          </fieldset>

          <AfricansChamp
            v-model="form.statutLegal"
            libelle="Statut légal de l'organisation"
            placeholder="Ex: SARL, Association loi 1901, ONG, Établissement public…"
          />

          <div class="grid gap-5 md:grid-cols-2">
            <AfricansChamp
              v-model="form.organisateurNom"
              libelle="Nom / Organisation"
              placeholder="Ex: ONG Santé Pour Tous"
            />
            <AfricansChamp
              v-model="form.organisateurEmail"
              libelle="Email de contact"
              type="email"
              placeholder="contact@organisation.org"
            />
          </div>
        </div>

        <!-- Navigation -->
        <div class="mt-3 flex flex-wrap items-center gap-4 border-t border-af-bordure pt-6">
          <AfricansBouton vers="/echanges-sabbatiques" variante="secondaire">
            Annuler
          </AfricansBouton>

          <AfricansBouton
            v-if="etapeCourante > 0"
            variante="secondaire"
            icone="fa-solid fa-arrow-left"
            class="ml-auto"
            @click="etapeCourante -= 1"
          >
            Précédent
          </AfricansBouton>

          <AfricansBouton
            v-if="etapeCourante < ETAPES.length - 1"
            icone="fa-solid fa-arrow-right"
            :class="etapeCourante === 0 && 'ml-auto'"
            @click="suivant"
          >
            Suivant
          </AfricansBouton>
          <AfricansBouton
            v-else
            type="submit"
            variante="vert"
            :desactive="loading"
            :tourne="loading"
            :icone="loading ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
          >
            {{ loading ? 'Soumission en cours…' : 'Soumettre le projet' }}
          </AfricansBouton>
        </div>
      </form>
    </div>

    <AfricansModale
      :model-value="succes"
      titre="Projet soumis avec succès !"
      icone="fa-solid fa-circle-check"
      @update:model-value="fermerSucces"
    >
      <p class="text-[14px]/[1.6] text-af-corps">
        Votre projet d'échange sabbatique a bien été enregistré. Les candidats pourront
        désormais postuler.
      </p>

      <template #actions>
        <button
          type="button"
          class="text-base font-bold text-af-corps transition hover:opacity-70"
          @click="fermerSucces"
        >
          Proposer un autre projet
        </button>
        <AfricansBouton
          :vers="dernierProgrammeId ? `/echanges-sabbatiques/${dernierProgrammeId}` : '/echanges-sabbatiques'"
          icone="fa-solid fa-arrow-right"
        >
          {{ dernierProgrammeId ? 'Voir le projet' : 'Voir les programmes' }}
        </AfricansBouton>
      </template>
    </AfricansModale>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { reactive, ref, computed, onMounted } from 'vue'
import AOS from 'aos'
import {
  useSabbatiques,
  DOMAINES,
  DUREES,
  PAYS_AFRICAINS,
  PAYS_HORS_AFRIQUE,
  PRISES_EN_CHARGE,
  TYPES_ORGANISATION,
  type TypeProgramme,
} from '~/composables/useSabbatiques'
import { editorJsToHtml, type EditorJsData } from '~/composables/useEditorJs'

definePageMeta({ layout: false })

const route = useRoute()
const { creerProgramme, erreur: sabbatiqueErreur } = useSabbatiques()

const typesValides: TypeProgramme[] = ['interafricain', 'hors_afrique']
const typeParam = route.query.type as string | undefined
const typeInitial = typesValides.includes(typeParam as TypeProgramme) ? (typeParam as TypeProgramme) : ''

useHead({
  title: 'Proposer un projet d\'échange - AfricanS',
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

// Constantes sans l'option "Tous" pour le formulaire, + option "Autre" à préciser
const DOMAINES_FORM = [
  ...DOMAINES.filter(d => d.value !== ''),
  { value: 'autre', label: 'Autre' },
]

// Liste des territoires selon le type de programme :
// - interafricain → uniquement les pays d'Afrique
// - hors_afrique  → uniquement les pays hors d'Afrique
const PAYS_FORM = computed(() =>
  (form.type === 'hors_afrique' ? PAYS_HORS_AFRIQUE : PAYS_AFRICAINS)
    .filter(p => p.value !== '')
)

const editorRef = ref<{ save: () => Promise<EditorJsData | null>; clear: () => Promise<void> } | null>(null)

const form = reactive({
  type: typeInitial as string,
  typeOrganisation: '' as string,
  statutLegal: '',
  titre: '',
  descriptionData: undefined as EditorJsData | undefined,
  domaine: '' as string,
  domainePrecision: '',
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

// Basculer entre les deux pages « interafricain » / « hors_afrique »
// en mettant à jour l'URL, et réinitialiser le territoire (les listes diffèrent)
const changerType = (nouveauType: TypeProgramme) => {
  if (form.type === nouveauType) return
  form.type = nouveauType
  form.pays = ''
  navigateTo({ path: '/echanges-sabbatiques/proposer', query: { type: nouveauType } })
}

const loading = ref(false)
const succes = ref(false)
const erreur = ref<string | null>(null)
const dernierProgrammeId = ref<string | null>(null)

const hasDescription = computed(() => {
  return form.descriptionData && form.descriptionData.blocks && form.descriptionData.blocks.length > 0
})

const ETAPES = [
  { titre: 'Le projet' },
  { titre: 'Lieu & calendrier' },
  { titre: 'Conditions & pièces' },
  { titre: "L'organisation" },
] as const
const etapeCourante = ref(0)

/**
 * Ce qui manque à une étape, ou null. C'est la SOURCE de la validation :
 * `isFormValid` en est la conjonction, et l'envoi ramène à l'étape fautive.
 * Un second jeu de règles pour le bouton divergerait au premier champ ajouté.
 */
function manqueEtape(i: number): string | null {
  switch (i) {
    case 0:
      if (!form.type) return 'Choisissez le type de programme.'
      if (!form.titre.trim()) return 'Le titre du projet est requis.'
      if (!hasDescription.value) return 'La description du projet est requise.'
      if (!form.domaine) return "Choisissez un domaine d'intervention."
      if (form.domaine === 'autre' && !form.domainePrecision.trim()) {
        return "Précisez le domaine d'intervention choisi au titre de « Autre »."
      }
      return null
    case 1:
      if (!form.pays) return 'Choisissez un territoire.'
      if (!form.duree) return 'Choisissez une durée de programme.'
      if (!form.dateDebut || !form.dateFin) return 'Indiquez les dates de début et de fin.'
      return null
    case 2:
      // Pièces jointes et prises en charge : toutes facultatives.
      return null
    case 3:
      return form.typeOrganisation ? null : "Choisissez le type d'organisation."
    default:
      return null
  }
}

const isFormValid = computed(() => ETAPES.every((_, i) => manqueEtape(i) === null))

function suivant() {
  const manque = manqueEtape(etapeCourante.value)
  if (manque) {
    erreur.value = manque
    return
  }
  erreur.value = null
  etapeCourante.value = Math.min(etapeCourante.value + 1, ETAPES.length - 1)
}

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
  etapeCourante.value = 0
  form.type = ''
  form.typeOrganisation = ''
  form.statutLegal = ''
  form.titre = ''
  form.descriptionData = undefined
  form.domaine = ''
  form.domainePrecision = ''
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
  // L'envoi ramène à l'étape fautive : un message rendu sur une étape
  // invisible est un message perdu.
  for (let i = 0; i < ETAPES.length; i++) {
    const manque = manqueEtape(i)
    if (manque) {
      erreur.value = manque
      etapeCourante.value = i
      return
    }
  }

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
        typeOrganisation: form.typeOrganisation,
        statutLegal: form.statutLegal.trim() || undefined,
        titre: form.titre,
        description: descriptionHtml,
        domaine: form.domaine,
        domainePrecision: form.domaine === 'autre' ? form.domainePrecision.trim() : undefined,
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
      dernierProgrammeId.value = result.id
      succes.value = true
      await resetForm()
      window.scrollTo({ top: 0, behavior: 'smooth' })
    } else {
      erreur.value = sabbatiqueErreur.value || 'Une erreur est survenue lors de la soumission'
    }
  } catch (e: any) {
    erreur.value = e?.message || 'Une erreur est survenue lors de la soumission'
  } finally {
    loading.value = false
  }
}

const fermerSucces = () => {
  succes.value = false
}

onMounted(() => {
  AOS.init({
    duration: 800,
    easing: 'ease-out-cubic',
    once: true
  })
})
</script>
