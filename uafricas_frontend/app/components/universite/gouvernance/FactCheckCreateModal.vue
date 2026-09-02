<template>
  <AfricansModale
    :model-value="open"
    titre="Publier un FactCheck"
    sous-titre="Vérifier une idée reçue sur l'Afrique"
    icone="fa-solid fa-magnifying-glass-chart"
    taille="large"
    @update:model-value="fermer()"
  >
    <AfricansEtapes :etapes="ETAPES" :courante="etapeCourante" class="mb-6" @aller="etapeCourante = $event" />

    <form id="form-factcheck" class="flex flex-col gap-5" @submit.prevent="soumettre">
      <p
        v-if="erreurMessage"
        class="flex items-center gap-2 rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreurMessage }}
      </p>

      <!-- ─── Étape 1 : l'affirmation ─── -->
      <template v-if="etapeCourante === 0">
        <div>
          <AfricansChamp
            v-model="form.contenu"
            libelle="Contenu du factcheck"
            type="textarea"
            :lignes="5"
            placeholder="Décrivez l'idée reçue à vérifier et apportez votre analyse factuelle…"
            obligatoire
          />
          <p class="mt-1 text-[12px] text-af-atone">{{ form.contenu.length }} caractères (minimum 10)</p>
        </div>

        <fieldset>
          <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">
            Type <span class="not-italic text-af-live">*</span>
          </legend>
          <div class="grid gap-2 sm:grid-cols-3">
            <button
              v-for="t in typesPublication"
              :key="t.value"
              type="button"
              class="flex items-start gap-2 rounded-lg border-2 px-3 py-2.5 text-left text-[12px] font-bold transition"
              :class="form.type_publication === t.value
                ? 'border-af-chocolat bg-af-chocolat/10 text-af-chocolat'
                : 'border-af-bordure bg-white text-af-corps hover:border-af-chocolat'"
              @click="form.type_publication = t.value"
            >
              <font-awesome-icon :icon="t.icon" class="mt-0.5" />
              <span>{{ t.label }}</span>
            </button>
          </div>
        </fieldset>

        <!-- Un fait vécu se situe : sans territoire ni preuve, il n'est
             qu'une affirmation de plus. Le bloc n'apparaît que pour lui. -->
        <div
          v-if="form.type_publication === 'fait_vecu'"
          class="flex flex-col gap-4 rounded-lg border border-af-chocolat/20 bg-af-chocolat/5 p-4"
        >
          <p class="flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-location-dot" />
            Fait vécu ou observé
          </p>

          <AfricansChamp v-model="form.pays_id" libelle="Territoire" type="select">
            <option :value="undefined">Sélectionner un territoire</option>
            <option v-for="p in pays" :key="p.id" :value="p.id">{{ p.nom }}</option>
          </AfricansChamp>

          <p class="flex items-center gap-2 text-[12px] text-af-corps">
            <font-awesome-icon icon="fa-solid fa-calendar-day" />
            La date enregistrée est celle de la publication.
          </p>

          <div>
            <p class="mb-2 text-[14px]/[1.4] text-af-atone italic">Preuve (photo ou PDF)</p>
            <div v-if="!preuveUrl" class="flex flex-wrap items-center gap-3">
              <label
                class="inline-flex cursor-pointer items-center gap-2 rounded-lg border border-af-bordure bg-white px-4 py-2 text-[14px]/[1.4] font-bold text-af-corps transition hover:border-af-chocolat"
                :class="televersementEnCours && 'cursor-not-allowed opacity-50'"
              >
                <font-awesome-icon
                  :icon="televersementEnCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paperclip'"
                  :class="televersementEnCours && 'animate-spin'"
                />
                {{ televersementEnCours ? 'Téléversement…' : 'Joindre un fichier' }}
                <input
                  type="file"
                  accept="image/jpeg,image/png,image/webp,application/pdf"
                  class="sr-only"
                  :disabled="televersementEnCours"
                  @change="onPreuveSelectionnee"
                >
              </label>
              <span class="text-[12px] text-af-atone-2">Sinon, « Pas de preuve » sera affiché.</span>
            </div>
            <div
              v-else
              class="flex items-center justify-between gap-3 rounded-lg border border-af-vert/40 bg-af-vert/5 px-4 py-2.5"
            >
              <span class="flex min-w-0 items-center gap-2 text-[14px]/[1.4] text-af-vert">
                <font-awesome-icon :icon="preuveType === 'pdf' ? 'fa-solid fa-file-pdf' : 'fa-solid fa-image'" />
                <span class="truncate">{{ preuveType === 'pdf' ? 'Document PDF joint' : 'Photo jointe' }}</span>
              </span>
              <button type="button" class="shrink-0 text-af-live transition hover:opacity-70" aria-label="Retirer la preuve" @click="retirerPreuve">
                <font-awesome-icon icon="fa-solid fa-xmark" />
              </button>
            </div>
            <p v-if="erreurPreuve" class="mt-1 text-[12px] text-af-live">{{ erreurPreuve }}</p>
          </div>
        </div>
      </template>

      <!-- ─── Étape 2 : l'analyse ─── -->
      <template v-else>
        <fieldset>
          <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">Verdict</legend>
          <div class="grid grid-cols-2 gap-2 md:grid-cols-3">
            <button
              v-for="v in verdicts"
              :key="v.value"
              type="button"
              class="rounded-lg border-2 px-3 py-2 text-[12px] font-bold transition"
              :class="form.verdict === v.value ? v.activeClass : 'border-af-bordure bg-white text-af-corps hover:border-af-chocolat'"
              @click="form.verdict = form.verdict === v.value ? undefined : v.value"
            >
              <font-awesome-icon :icon="v.icon" class="mr-1" />
              {{ v.label }}
            </button>
          </div>
        </fieldset>

        <!-- Les deux volets s'opposent : le rouge et le vert sont ici
             porteurs de sens, pas décoratifs. -->
        <div class="grid gap-4 md:grid-cols-2">
          <div class="flex flex-col gap-3 rounded-lg border-l-4 border-af-live bg-af-live/5 p-3">
            <p class="text-[12px] font-bold text-af-live uppercase">
              <font-awesome-icon icon="fa-solid fa-xmark" class="mr-1" />Préjugé
            </p>
            <AfricansChamp v-model="form.prejuge_titre" libelle="Titre du préjugé" />
            <AfricansChamp
              v-model="form.prejuge_description"
              libelle="Description"
              type="textarea"
              :lignes="2"
              aide="Facultatif"
            />
          </div>
          <div class="flex flex-col gap-3 rounded-lg border-l-4 border-af-vert bg-af-vert/5 p-3">
            <p class="text-[12px] font-bold text-af-vert uppercase">
              <font-awesome-icon icon="fa-solid fa-check" class="mr-1" />Réalité
            </p>
            <AfricansChamp v-model="form.realite_titre" libelle="Titre de la réalité" />
            <AfricansChamp
              v-model="form.realite_description"
              libelle="Description"
              type="textarea"
              :lignes="2"
              aide="Facultatif"
            />
          </div>
        </div>

        <div class="grid gap-4 md:grid-cols-2">
          <AfricansChamp v-model="form.source_originale" libelle="Source" type="url" placeholder="https://…" />

          <div>
            <p class="mb-2 text-[14px]/[1.4] text-af-atone italic">Image illustrative</p>
            <div v-if="!imageApercu" class="flex flex-wrap items-center gap-3">
              <label
                class="inline-flex cursor-pointer items-center gap-2 rounded-lg border border-af-bordure bg-white px-4 py-2 text-[14px]/[1.4] font-bold text-af-corps transition hover:border-af-chocolat"
                :class="imageEnCours && 'cursor-not-allowed opacity-50'"
              >
                <font-awesome-icon
                  :icon="imageEnCours ? 'fa-solid fa-spinner' : 'fa-solid fa-image'"
                  :class="imageEnCours && 'animate-spin'"
                />
                {{ imageEnCours ? 'Téléversement…' : 'Choisir une image' }}
                <input
                  type="file"
                  accept="image/jpeg,image/png,image/webp"
                  class="sr-only"
                  :disabled="imageEnCours"
                  @change="onImageSelectionnee"
                >
              </label>
              <span class="text-[12px] text-af-atone-2">Facultatif</span>
            </div>
            <div v-else class="relative">
              <img :src="imageApercu" alt="" class="h-32 w-full rounded-lg border border-af-bordure object-cover">
              <button
                type="button"
                class="absolute top-2 right-2 grid size-7 place-items-center rounded-full bg-black/60 text-white transition hover:bg-black/80"
                aria-label="Retirer l'image"
                @click="retirerImage"
              >
                <font-awesome-icon icon="fa-solid fa-xmark" class="text-[12px]" />
              </button>
            </div>
            <p v-if="erreurImage" class="mt-1 text-[12px] text-af-live">{{ erreurImage }}</p>
          </div>
        </div>

        <p class="flex items-start gap-2 rounded-lg border border-af-chocolat/20 bg-af-chocolat/5 px-4 py-3 text-[12px]/[1.6] text-af-corps">
          <font-awesome-icon icon="fa-solid fa-circle-info" class="mt-0.5 shrink-0 text-af-chocolat" />
          Votre contribution sera publiée immédiatement et visible par tous les utilisateurs.
        </p>
      </template>
    </form>

    <template #actions>
      <button
        type="button"
        class="mr-auto text-base font-bold text-af-corps transition hover:opacity-70"
        @click="fermer"
      >
        Annuler
      </button>
      <AfricansBouton
        v-if="etapeCourante === 1"
        variante="secondaire"
        icone="fa-solid fa-arrow-left"
        @click="etapeCourante = 0"
      >
        Précédent
      </AfricansBouton>
      <AfricansBouton v-if="etapeCourante === 0" icone="fa-solid fa-arrow-right" @click="suivant">
        Suivant
      </AfricansBouton>
      <AfricansBouton
        v-else
        :desactive="!estValide || enCours"
        :tourne="enCours"
        :icone="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
        @click="soumettre"
      >
        {{ enCours ? 'Publication…' : 'Publier' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
import type { CreerFactcheckPayload, PaysPublic } from '~/composables/useGouvernance'
import type { TypePreuve, TypePublicationFactcheck } from '~/types/gouvernance'

interface Props {
  open: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  created: [id: string]
}>()

const { creerFactcheck, uploaderPreuve, getPays } = useGouvernance()

const form = reactive<CreerFactcheckPayload>({
  contenu: '',
  source_originale: undefined,
  verdict: undefined,
  image_couverture_url: undefined,
  prejuge_titre: undefined,
  prejuge_description: undefined,
  realite_titre: undefined,
  realite_description: undefined,
  type_publication: 'on_dit',
  pays_id: undefined,
})

const enCours = ref(false)
const erreurMessage = ref<string | null>(null)

// Types de publication
const typesPublication: { value: TypePublicationFactcheck; label: string; icon: string }[] = [
  { value: 'on_dit', label: 'On dit / entendu quelque part', icon: 'fa-solid fa-comments' },
  { value: 'adage_legende', label: 'Adage / Légende', icon: 'fa-solid fa-book-open' },
  { value: 'fait_vecu', label: 'Fait vécu ou observé', icon: 'fa-solid fa-location-dot' },
]

// Territoires (chargés à l'ouverture)
const pays = ref<PaysPublic[]>([])

// Preuve (fait vécu)
const preuveUrl = ref<string | undefined>(undefined)
const preuveType = ref<TypePreuve | undefined>(undefined)
const televersementEnCours = ref(false)
const erreurPreuve = ref<string | null>(null)

async function onPreuveSelectionnee(evt: Event) {
  const input = evt.target as HTMLInputElement
  const fichier = input.files?.[0]
  if (!fichier) return
  erreurPreuve.value = null
  televersementEnCours.value = true
  try {
    const { url, preuveType: type } = await uploaderPreuve(fichier)
    preuveUrl.value = url
    preuveType.value = type
  } catch (err) {
    erreurPreuve.value = err instanceof Error ? err.message : 'Téléversement impossible'
  } finally {
    televersementEnCours.value = false
    input.value = ''
  }
}

function retirerPreuve() {
  preuveUrl.value = undefined
  preuveType.value = undefined
  erreurPreuve.value = null
}

// Image illustrative (facultative, toutes les publications)
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const imageEnCours = ref(false)
const erreurImage = ref<string | null>(null)
const imageApercu = computed(() =>
  form.image_couverture_url
    ? (form.image_couverture_url.startsWith('http') ? form.image_couverture_url : `${apiBase}${form.image_couverture_url}`)
    : undefined,
)

async function onImageSelectionnee(evt: Event) {
  const input = evt.target as HTMLInputElement
  const fichier = input.files?.[0]
  if (!fichier) return
  erreurImage.value = null
  imageEnCours.value = true
  try {
    const { url, preuveType: type } = await uploaderPreuve(fichier)
    if (type !== 'image') {
      erreurImage.value = 'Veuillez choisir une image (JPEG, PNG ou WebP).'
      return
    }
    form.image_couverture_url = url
  } catch (err) {
    erreurImage.value = err instanceof Error ? err.message : 'Téléversement impossible'
  } finally {
    imageEnCours.value = false
    input.value = ''
  }
}

function retirerImage() {
  form.image_couverture_url = undefined
  erreurImage.value = null
}

const verdicts = [
  { value: 'vrai' as const, label: 'Vrai', icon: 'fa-solid fa-check', activeClass: 'border-af-vert bg-af-vert/10 text-af-vert' },
  { value: 'faux' as const, label: 'Faux', icon: 'fa-solid fa-xmark', activeClass: 'border-af-live bg-af-live/10 text-af-live' },
  { value: 'partiellement_vrai' as const, label: 'Partiellement vrai', icon: 'fa-solid fa-circle-half-stroke', activeClass: 'border-af-chocolat bg-af-chocolat/10 text-af-chocolat' },
  { value: 'trompeur' as const, label: 'Trompeur', icon: 'fa-solid fa-triangle-exclamation', activeClass: 'border-af-chocolat bg-af-chocolat/10 text-af-chocolat' },
  { value: 'non_verifie' as const, label: 'Non vérifié', icon: 'fa-solid fa-question', activeClass: 'border-af-atone bg-af-fond text-af-corps' },
]

const estValide = computed(() => form.contenu.trim().length >= 10)

const ETAPES = [
  { titre: "L'affirmation" },
  { titre: "L'analyse" },
] as const
const etapeCourante = ref(0)

function suivant() {
  // Le seul champ obligatoire vit à l'étape 1 : passer à la suite sans lui
  // n'aurait aucun sens, l'analyse porterait sur rien.
  if (!estValide.value) {
    erreurMessage.value = 'Le contenu du factcheck doit contenir au moins 10 caractères.'
    return
  }
  erreurMessage.value = null
  etapeCourante.value = 1
}

function reinitialiser() {
  etapeCourante.value = 0
  form.contenu = ''
  form.source_originale = undefined
  form.verdict = undefined
  form.image_couverture_url = undefined
  form.prejuge_titre = undefined
  form.prejuge_description = undefined
  form.realite_titre = undefined
  form.realite_description = undefined
  form.type_publication = 'on_dit'
  form.pays_id = undefined
  retirerPreuve()
  erreurImage.value = null
  erreurMessage.value = null
}

function fermer() {
  if (enCours.value) return
  emit('close')
}

async function soumettre() {
  if (!estValide.value || enCours.value) return
  enCours.value = true
  erreurMessage.value = null
  try {
    const payload: CreerFactcheckPayload = {
      contenu: form.contenu.trim(),
    }
    if (form.source_originale?.trim()) payload.source_originale = form.source_originale.trim()
    if (form.verdict) payload.verdict = form.verdict
    if (form.image_couverture_url?.trim()) payload.image_couverture_url = form.image_couverture_url.trim()
    if (form.prejuge_titre?.trim()) payload.prejuge_titre = form.prejuge_titre.trim()
    if (form.prejuge_description?.trim()) payload.prejuge_description = form.prejuge_description.trim()
    if (form.realite_titre?.trim()) payload.realite_titre = form.realite_titre.trim()
    if (form.realite_description?.trim()) payload.realite_description = form.realite_description.trim()
    payload.type_publication = form.type_publication
    if (form.type_publication === 'fait_vecu') {
      if (form.pays_id) payload.pays_id = form.pays_id
      if (preuveUrl.value && preuveType.value) {
        payload.preuve_url = preuveUrl.value
        payload.preuve_type = preuveType.value
      }
    }

    const id = await creerFactcheck(payload)
    emit('created', id)
    reinitialiser()
  } catch (err) {
    erreurMessage.value = err instanceof Error ? err.message : 'Erreur lors de la publication'
  } finally {
    enCours.value = false
  }
}

watch(() => props.open, async (v) => {
  if (!v) {
    reinitialiser()
    return
  }
  // Charger les territoires une seule fois à l'ouverture
  if (pays.value.length === 0) {
    try {
      pays.value = await getPays()
    } catch {
      // Sélecteur territoire simplement vide en cas d'échec
    }
  }
})
</script>
