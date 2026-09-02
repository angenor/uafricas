<template>
  <AfricansModale
    :model-value="open"
    :titre="estBonne ? 'Goodhabits' : 'Badhabits'"
    :sous-titre="estBonne
      ? 'Valoriser une action exemplaire de gouvernance'
      : 'Dénoncer un problème de gouvernance'"
    :icone="estBonne ? 'fa-solid fa-thumbs-up' : 'fa-solid fa-triangle-exclamation'"
    :ton="estBonne ? 'vert' : 'chocolat'"
    taille="large"
    @update:model-value="fermer()"
  >
    <AfricansEtapes :etapes="ETAPES" :courante="etapeCourante" class="mb-6" @aller="etapeCourante = $event" />

    <form id="form-habits" class="flex flex-col gap-5" @submit.prevent="soumettre">
      <p
        v-if="erreurMessage"
        class="flex items-center gap-2 rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreurMessage }}
      </p>

      <!-- ─── Étape 1 : la pratique ─── -->
      <template v-if="etapeCourante === 0">
        <!-- La bascule reste à l'étape 1 : changer de nature en cours de
             route redéfinirait les champs obligatoires des étapes suivantes. -->
        <div class="grid grid-cols-2 gap-2 rounded-lg bg-af-fond p-1">
          <button
            type="button"
            class="flex items-center justify-center gap-2 rounded-lg py-2.5 text-[14px]/[1.4] font-bold transition"
            :class="!estBonne ? 'bg-white text-af-chocolat shadow-sm' : 'text-af-atone hover:text-af-corps'"
            @click="changerMode('mauvaise')"
          >
            <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="text-[12px]" />
            Badhabits
          </button>
          <button
            type="button"
            class="flex items-center justify-center gap-2 rounded-lg py-2.5 text-[14px]/[1.4] font-bold transition"
            :class="estBonne ? 'bg-white text-af-vert shadow-sm' : 'text-af-atone hover:text-af-corps'"
            @click="changerMode('bonne')"
          >
            <font-awesome-icon icon="fa-solid fa-thumbs-up" class="text-[12px]" />
            Goodhabits
          </button>
        </div>

        <AfricansChamp
          v-model="form.titre"
          libelle="Titre"
          :maxlength="350"
          :placeholder="estBonne
            ? 'Résumez la bonne pratique en une phrase…'
            : 'Résumez la mauvaise pratique en une phrase…'"
          obligatoire
        />

        <div class="grid gap-5 md:grid-cols-2">
          <AfricansChamp v-model="form.categorie_probleme" libelle="Catégorie" type="select" obligatoire>
            <option v-for="c in categoriesActives" :key="c.value" :value="c.value">{{ c.label }}</option>
          </AfricansChamp>

          <fieldset>
            <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">
              {{ estBonne ? 'Impact' : 'Gravité' }}
            </legend>
            <div class="flex gap-2">
              <button
                v-for="n in niveauxActifs"
                :key="n.value"
                type="button"
                class="flex-1 rounded-lg border-2 px-3 py-2.5 text-[12px] font-bold transition"
                :class="niveauCourant === n.value ? n.activeClass : 'border-af-bordure bg-white text-af-corps hover:border-af-chocolat'"
                @click="changerNiveau(n.value)"
              >
                <font-awesome-icon :icon="n.icon" class="mr-1" />
                {{ n.label }}
              </button>
            </div>
          </fieldset>
        </div>

        <AfricansChamp
          v-if="form.categorie_probleme === 'autre'"
          v-model="form.categorie_probleme_detail"
          libelle="Précisez la catégorie"
          :maxlength="200"
        />

        <AfricansChamp
          v-model="form.description_generale"
          libelle="Description générale"
          type="textarea"
          :lignes="3"
          :placeholder="estBonne
            ? 'Décrivez brièvement cette bonne pratique…'
            : 'Décrivez brièvement la problématique…'"
          obligatoire
        />
      </template>

      <!-- ─── Étape 2 : le détail ─── -->
      <template v-else-if="etapeCourante === 1">
        <!-- Mauvaise pratique : détails en texte libre -->
        <AfricansChamp
          v-if="!estBonne"
          v-model="form.details_problematique"
          libelle="Détails de la problématique"
          type="textarea"
          :lignes="5"
          placeholder="Expliquez en détail le problème, son contexte, ses conséquences…"
          obligatoire
        />

        <!-- Bonne pratique : modalités de reproductibilité (10 maximum) -->
        <div v-else>
          <div class="mb-2 flex items-center justify-between gap-4">
            <span class="text-[14px]/[1.4] text-af-atone italic">
              Modalités pratiques de reproductibilité <span class="not-italic text-af-live">*</span>
              <span class="text-af-atone-2">(10 modalités maximum)</span>
            </span>
            <button
              v-if="modalitesReproductibilite.length < 10"
              type="button"
              class="shrink-0 text-[12px] font-bold text-af-vert transition hover:opacity-70"
              @click="ajouterModalite"
            >
              <font-awesome-icon icon="fa-solid fa-plus" class="mr-1" />
              Ajouter une modalité
            </button>
          </div>
          <p class="mb-2 text-[12px] text-af-atone">
            Décrivez les modalités pratiques pour reproduire cette bonne pratique : étapes,
            conditions, moyens…
          </p>
          <p v-if="modalitesReproductibilite.length === 0" class="text-[12px] text-af-atone-2 italic">
            Aucune modalité. Ajoutez au moins une modalité concrète.
          </p>
          <div v-for="(_, idx) in modalitesReproductibilite" :key="idx" class="mb-2 flex items-center gap-2">
            <span class="grid size-7 shrink-0 place-items-center rounded-full bg-af-vert/10 text-[12px] font-bold text-af-vert">
              {{ idx + 1 }}
            </span>
            <input
              v-model="modalitesReproductibilite[idx]"
              type="text"
              maxlength="500"
              :placeholder="`Modalité ${idx + 1}…`"
              class="h-11 min-w-0 flex-1 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
            >
            <button
              type="button"
              class="grid size-10 shrink-0 place-items-center rounded-md border border-af-bordure text-af-atone transition hover:border-af-live hover:text-af-live"
              :aria-label="`Retirer la modalité ${idx + 1}`"
              @click="retirerModalite(idx)"
            >
              <font-awesome-icon icon="fa-solid fa-trash" class="text-[12px]" />
            </button>
          </div>
        </div>

        <AfricansChamp
          v-model="form.preuves_temoignages"
          :libelle="estBonne ? 'Témoignages / Preuves' : 'Témoignages'"
          type="textarea"
          :lignes="3"
          :placeholder="estBonne
            ? 'Témoignages, chiffres, médias…'
            : 'Témoignages de personnes affectées ou ayant constaté les faits…'"
        />

        <AfricansChamp
          v-if="estBonne"
          v-model="form.solutions_proposees"
          libelle="Reproductibilité / Transposition"
          type="textarea"
          :lignes="3"
          placeholder="Comment d'autres communautés peuvent reproduire cette action ?"
        />

        <!-- Mauvaise pratique : solutions proposées (10 maximum) -->
        <div v-if="!estBonne">
          <div class="mb-2 flex items-center justify-between gap-4">
            <span class="text-[14px]/[1.4] text-af-atone italic">
              Solutions proposées <span class="text-af-atone-2">(10 propositions maximum)</span>
            </span>
            <button
              v-if="solutions.length < 10"
              type="button"
              class="shrink-0 text-[12px] font-bold text-af-chocolat transition hover:opacity-70"
              @click="ajouterSolution"
            >
              <font-awesome-icon icon="fa-solid fa-plus" class="mr-1" />
              Ajouter une proposition
            </button>
          </div>
          <p v-if="solutions.length === 0" class="text-[12px] text-af-atone-2 italic">
            Aucune proposition. Suggérez des solutions concrètes (facultatif).
          </p>
          <div v-for="(_, idx) in solutions" :key="idx" class="mb-2 flex items-center gap-2">
            <span class="grid size-7 shrink-0 place-items-center rounded-full bg-af-chocolat/10 text-[12px] font-bold text-af-chocolat">
              {{ idx + 1 }}
            </span>
            <input
              v-model="solutions[idx]"
              type="text"
              maxlength="500"
              :placeholder="`Proposition ${idx + 1}…`"
              class="h-11 min-w-0 flex-1 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
            >
            <button
              type="button"
              class="grid size-10 shrink-0 place-items-center rounded-md border border-af-bordure text-af-atone transition hover:border-af-live hover:text-af-live"
              :aria-label="`Retirer la proposition ${idx + 1}`"
              @click="retirerSolution(idx)"
            >
              <font-awesome-icon icon="fa-solid fa-trash" class="text-[12px]" />
            </button>
          </div>
        </div>
      </template>

      <!-- ─── Étape 3 : preuves et localisation ─── -->
      <template v-else-if="etapeCourante === 2">
        <div>
          <p class="mb-2 text-[14px]/[1.4] text-af-atone italic">Preuves (images ou PDF)</p>
          <div class="flex flex-wrap items-center gap-3">
            <div
              v-for="(preuve, idx) in preuvesFichiers"
              :key="preuve.url"
              class="relative size-20 overflow-hidden rounded-lg border border-af-bordure bg-af-fond"
            >
              <img
                v-if="preuve.type === 'image'"
                :src="urlAbsolue(preuve.url)"
                alt=""
                class="size-full object-cover"
              >
              <a
                v-else
                :href="urlAbsolue(preuve.url)"
                target="_blank"
                rel="noopener noreferrer"
                class="flex size-full flex-col items-center justify-center gap-1 text-af-live transition hover:opacity-70"
              >
                <font-awesome-icon icon="fa-solid fa-file-pdf" class="text-2xl" />
                <span class="text-[10px] font-bold">PDF</span>
              </a>
              <button
                type="button"
                class="absolute top-0.5 right-0.5 grid size-5 place-items-center rounded-full bg-black/60 text-white transition hover:bg-black/80"
                :aria-label="`Retirer la preuve ${idx + 1}`"
                @click="retirerPhoto(idx)"
              >
                <font-awesome-icon icon="fa-solid fa-xmark" class="text-[10px]" />
              </button>
            </div>
            <label
              v-if="preuvesFichiers.length < 5"
              class="flex size-20 cursor-pointer flex-col items-center justify-center gap-1 rounded-lg border-2 border-dashed border-af-bordure text-[12px] text-af-atone-2 transition hover:border-af-chocolat hover:text-af-chocolat"
              :class="photoEnCours && 'cursor-not-allowed opacity-50'"
            >
              <font-awesome-icon
                :icon="photoEnCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paperclip'"
                :class="photoEnCours && 'animate-spin'"
              />
              <span>Ajouter</span>
              <input
                type="file"
                accept="image/jpeg,image/png,image/webp,application/pdf"
                multiple
                class="sr-only"
                :disabled="photoEnCours"
                @change="onPhotosSelectionnees"
              >
            </label>
          </div>
          <p class="mt-1 text-[12px] text-af-atone-2">Jusqu'à 5 fichiers (images JPEG, PNG, WebP ou PDF).</p>
          <p v-if="erreurPhoto" class="mt-1 text-[12px] text-af-live">{{ erreurPhoto }}</p>
        </div>

        <div class="flex flex-col gap-4 rounded-lg bg-af-fond p-4">
          <p class="text-[14px]/[1.4] font-bold text-af-encre">Localisation</p>

          <AfricansChamp v-model="form.pays_id" libelle="Territoire" type="select" obligatoire>
            <option value="" disabled>Sélectionnez un territoire</option>
            <option v-for="p in paysListe" :key="p.id" :value="p.id">{{ p.nom }}</option>
          </AfricansChamp>

          <div class="grid gap-4 md:grid-cols-2">
            <AfricansChamp v-model="form.region" libelle="Région" :maxlength="250" aide="Facultatif" />
            <AfricansChamp
              v-model="form.ville_quartier_zone"
              libelle="Ville / Quartier / Zone"
              :maxlength="350"
              aide="Facultatif"
            />
          </div>
        </div>

        <div>
          <div class="mb-2 flex items-center justify-between gap-4">
            <span class="text-[14px]/[1.4] text-af-atone italic">Médias (URLs facultatives)</span>
            <button
              v-if="mediasUrls.length < 5"
              type="button"
              class="shrink-0 text-[12px] font-bold transition hover:opacity-70"
              :class="estBonne ? 'text-af-vert' : 'text-af-chocolat'"
              @click="ajouterMedia"
            >
              <font-awesome-icon icon="fa-solid fa-plus" class="mr-1" />
              Ajouter une URL
            </button>
          </div>
          <p v-if="mediasUrls.length === 0" class="text-[12px] text-af-atone-2 italic">
            Aucun média. Ajoutez des URLs d'images ou vidéos si nécessaire.
          </p>
          <div v-for="(_, idx) in mediasUrls" :key="idx" class="mb-2 flex gap-2">
            <input
              v-model="mediasUrls[idx]"
              type="url"
              placeholder="https://…"
              class="h-11 min-w-0 flex-1 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
            />
            <button
              type="button"
              class="grid size-10 shrink-0 place-items-center rounded-md border border-af-bordure text-af-atone transition hover:border-af-live hover:text-af-live"
              :aria-label="`Retirer l'URL ${idx + 1}`"
              @click="retirerMedia(idx)"
            >
              <font-awesome-icon icon="fa-solid fa-trash" class="text-[12px]" />
            </button>
          </div>
        </div>
      </template>

      <!-- ─── Étape 4 : vous et la publication ─── -->
      <template v-else>
        <!-- Mauvaise pratique : identité réelle, jamais anonyme -->
        <div
          v-if="!estBonne"
          class="flex flex-col gap-3 rounded-lg border border-af-live/30 bg-af-live/5 p-4"
        >
          <p class="flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-live">
            <font-awesome-icon icon="fa-solid fa-id-card" />
            Vos informations d'identité <span>*</span>
          </p>
          <p class="text-[12px]/[1.6] text-af-corps">
            Un signalement ne peut pas être anonyme : vous devez partager votre identité
            réelle et vos coordonnées.
          </p>
          <div class="grid gap-3 md:grid-cols-2">
            <AfricansChamp v-model="form.identite_nom" libelle="Nom (état civil)" :maxlength="150" obligatoire />
            <AfricansChamp v-model="form.identite_prenom" libelle="Prénom (état civil)" :maxlength="150" obligatoire />
            <AfricansChamp v-model="form.identite_courriel" libelle="Courriel" type="email" :maxlength="255" obligatoire />
            <AfricansChamp v-model="form.identite_contact" libelle="Contact (téléphone)" :maxlength="50" obligatoire />
          </div>
        </div>

        <label
          v-if="estBonne"
          class="flex cursor-pointer items-center gap-3 rounded-lg border-2 p-3 transition"
          :class="form.publication_anonyme
            ? 'border-af-vert bg-af-vert/5 text-af-vert'
            : 'border-af-bordure text-af-corps hover:border-af-chocolat'"
        >
          <input v-model="form.publication_anonyme" type="checkbox" class="sr-only peer" />
          <span
            class="grid size-5 shrink-0 place-items-center rounded-md border-2 peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-af-chocolat"
            :class="form.publication_anonyme ? 'border-af-vert bg-af-vert' : 'border-af-bordure'"
          >
            <font-awesome-icon v-if="form.publication_anonyme" icon="fa-solid fa-check" class="text-[12px] text-white" />
          </span>
          <span>
            <span class="block text-[14px]/[1.4] font-bold">Publier de manière anonyme</span>
            <span class="block text-[12px] opacity-75">Votre nom ne sera pas affiché publiquement</span>
          </span>
        </label>

        <p
          class="flex items-start gap-2 rounded-lg border px-4 py-3 text-[12px]/[1.6]"
          :class="estBonne
            ? 'border-af-vert/20 bg-af-vert/5 text-af-corps'
            : 'border-af-chocolat/20 bg-af-chocolat/5 text-af-corps'"
        >
          <font-awesome-icon
            icon="fa-solid fa-circle-info"
            class="mt-0.5 shrink-0"
            :class="estBonne ? 'text-af-vert' : 'text-af-chocolat'"
          />
          {{ estBonne
            ? 'Votre félicitation sera publiée immédiatement. Restez factuel et inspirant.'
            : 'Votre signalement sera publié immédiatement. Restez factuel et respectueux.' }}
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
        v-if="etapeCourante > 0"
        variante="secondaire"
        icone="fa-solid fa-arrow-left"
        @click="etapeCourante -= 1"
      >
        Précédent
      </AfricansBouton>
      <AfricansBouton
        v-if="etapeCourante < ETAPES.length - 1"
        icone="fa-solid fa-arrow-right"
        @click="suivant"
      >
        Suivant
      </AfricansBouton>
      <AfricansBouton
        v-else
        type="submit"
        form="form-habits"
        :desactive="enCours"
        :tourne="enCours"
        :icone="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
      >
        {{ enCours ? 'Publication…' : 'Publier' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
import type { CreerBadHabitPayload, PaysPublic, TypePratique } from '~/composables/useGouvernance'
import type { TypePreuve } from '~/types/gouvernance'

interface Props {
  open: boolean
  typePratiqueInitial?: TypePratique
}

const props = withDefaults(defineProps<Props>(), {
  typePratiqueInitial: 'mauvaise',
})
const emit = defineEmits<{
  close: []
  created: [id: string]
}>()

const { creerBadHabit, getPays, uploaderPreuve } = useGouvernance()
const userStore = useUserStore()

const form = reactive<CreerBadHabitPayload>({
  type_pratique: props.typePratiqueInitial,
  titre: '',
  description_generale: '',
  details_problematique: '',
  categorie_probleme: 'autre',
  categorie_probleme_detail: undefined,
  gravite: props.typePratiqueInitial === 'mauvaise' ? 'faible' : undefined,
  impact: props.typePratiqueInitial === 'bonne' ? 'fort' : undefined,
  preuves_temoignages: undefined,
  solutions_proposees: undefined,
  reproductibilite: undefined,
  publication_anonyme: false,
  pays_id: '',
  region: undefined,
  ville_quartier_zone: undefined,
  identite_nom: undefined,
  identite_prenom: undefined,
  identite_courriel: undefined,
  identite_contact: undefined,
})

const mediasUrls = ref<string[]>([])
const paysListe = ref<PaysPublic[]>([])
const enCours = ref(false)
const erreurMessage = ref<string | null>(null)

// Solutions proposées (mauvaise pratique), liste de 10 propositions max
const solutions = ref<string[]>([])

// Modalités pratiques de reproductibilité (bonne pratique), liste de 10 modalités max
const modalitesReproductibilite = ref<string[]>([])

// Preuves (Goodhabits & Badhabits) : fichiers uploadés (images ou PDF), max 5
const preuvesFichiers = ref<{ url: string; type: TypePreuve }[]>([])
const photoEnCours = ref(false)
const erreurPhoto = ref<string | null>(null)

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const urlAbsolue = (url: string) => (url.startsWith('http') ? url : `${apiBase}${url}`)

const estBonne = computed(() => form.type_pratique === 'bonne')

/** Préremplit l'identité réelle depuis le compte connecté (signalement non anonyme). */
function prefillIdentite() {
  const u = userStore.user
  if (!u) return
  if (!form.identite_nom) form.identite_nom = u.nom
  if (!form.identite_prenom) form.identite_prenom = u.prenom
  if (!form.identite_courriel) form.identite_courriel = u.email
}

function ajouterSolution() {
  if (solutions.value.length < 10) solutions.value.push('')
}
function retirerSolution(idx: number) {
  solutions.value.splice(idx, 1)
}

function ajouterModalite() {
  if (modalitesReproductibilite.value.length < 10) modalitesReproductibilite.value.push('')
}
function retirerModalite(idx: number) {
  modalitesReproductibilite.value.splice(idx, 1)
}

/** Modalités non vides (bonne pratique), limitées à 10. */
const modalitesValides = computed(() =>
  modalitesReproductibilite.value.map(m => m.trim()).filter(m => m.length > 0).slice(0, 10),
)

async function onPhotosSelectionnees(evt: Event) {
  const input = evt.target as HTMLInputElement
  const fichiers = Array.from(input.files ?? [])
  if (!fichiers.length) return
  erreurPhoto.value = null
  photoEnCours.value = true
  try {
    for (const fichier of fichiers) {
      if (preuvesFichiers.value.length >= 5) break
      const { url, preuveType } = await uploaderPreuve(fichier)
      preuvesFichiers.value.push({ url, type: preuveType })
    }
  } catch (err) {
    erreurPhoto.value = err instanceof Error ? err.message : 'Téléversement impossible'
  } finally {
    photoEnCours.value = false
    input.value = ''
  }
}

function retirerPhoto(idx: number) {
  preuvesFichiers.value.splice(idx, 1)
}

const categoriesMauvaise = [
  { value: 'corruption' as const, label: 'Corruption' },
  { value: 'service_public_defaillant' as const, label: 'Service public défaillant' },
  { value: 'infrastructure_degradee' as const, label: 'Infrastructure dégradée' },
  { value: 'acces_services_limite' as const, label: 'Accès aux services limité' },
  { value: 'insalubrite' as const, label: 'Insalubrité' },
  { value: 'probleme_securite' as const, label: 'Problème de sécurité' },
  { value: 'autre' as const, label: 'Autre' },
]

const categoriesBonne = [
  { value: 'civisme' as const, label: 'Civisme' },
  { value: 'service_public_exemplaire' as const, label: 'Service public exemplaire' },
  { value: 'solidarite' as const, label: 'Solidarité' },
  { value: 'innovation_sociale' as const, label: 'Innovation sociale' },
  { value: 'initiative_citoyenne' as const, label: 'Initiative citoyenne' },
  { value: 'leadership_exemplaire' as const, label: 'Leadership exemplaire' },
  { value: 'transparence' as const, label: 'Transparence' },
  { value: 'environnement' as const, label: 'Environnement' },
  { value: 'education' as const, label: 'Éducation' },
  { value: 'sante' as const, label: 'Santé' },
  { value: 'autre' as const, label: 'Autre' },
]

const gravites = [
  { value: 'faible' as const, label: 'Faible', icon: 'fa-solid fa-circle-info', activeClass: 'border-af-atone bg-af-fond text-af-corps' },
  { value: 'elevee' as const, label: 'Élevée', icon: 'fa-solid fa-triangle-exclamation', activeClass: 'border-af-chocolat bg-af-chocolat/10 text-af-chocolat' },
  { value: 'critique' as const, label: 'Critique', icon: 'fa-solid fa-skull', activeClass: 'border-af-live bg-af-live/10 text-af-live' },
]

const impacts = [
  { value: 'faible' as const, label: 'Modeste', icon: 'fa-solid fa-seedling', activeClass: 'border-af-vert/50 bg-af-vert/5 text-af-vert' },
  { value: 'fort' as const, label: 'Fort', icon: 'fa-solid fa-leaf', activeClass: 'border-af-vert bg-af-vert/10 text-af-vert' },
  { value: 'exemplaire' as const, label: 'Exemplaire', icon: 'fa-solid fa-star', activeClass: 'border-af-vert bg-af-vert text-white' },
]

const categoriesActives = computed(() => estBonne.value ? categoriesBonne : categoriesMauvaise)
const niveauxActifs = computed(() => estBonne.value ? impacts : gravites)
const niveauCourant = computed(() => estBonne.value ? form.impact : form.gravite)

function changerNiveau(valeur: string) {
  if (estBonne.value) {
    form.impact = valeur as 'faible' | 'fort' | 'exemplaire'
  } else {
    form.gravite = valeur as 'faible' | 'elevee' | 'critique'
  }
}

function changerMode(mode: TypePratique) {
  if (form.type_pratique === mode) return
  form.type_pratique = mode
  form.categorie_probleme = 'autre'
  if (mode === 'bonne') {
    form.gravite = undefined
    form.impact = 'fort'
    // Amorcer une première modalité de reproductibilité si la liste est vide
    if (modalitesReproductibilite.value.length === 0) modalitesReproductibilite.value = ['']
  } else {
    form.impact = undefined
    form.gravite = 'faible'
    // Signalement : jamais anonyme, identité réelle préremplie
    form.publication_anonyme = false
    prefillIdentite()
  }
}

const courrielValide = (c?: string) => !!c && c.includes('@') && c.includes('.')

const ETAPES = [
  { titre: 'La pratique' },
  { titre: 'Le détail' },
  { titre: 'Preuves & lieu' },
  { titre: 'Vous & publication' },
] as const
const etapeCourante = ref(0)

/**
 * Ce qui manque à une étape, ou null. C'est la SOURCE UNIQUE de validation,
 * partagée par le passage à l'étape suivante et par l'envoi : un second jeu
 * de règles pour le bouton divergerait au premier champ obligatoire ajouté.
 */
function manqueEtape(i: number): string | null {
  switch (i) {
    case 0:
      if (form.titre.trim().length < 5) return 'Le titre doit contenir au moins 5 caractères.'
      if (!form.categorie_probleme) return 'Choisissez une catégorie.'
      if (form.description_generale.trim().length < 10) {
        return 'La description générale doit contenir au moins 10 caractères.'
      }
      return null
    case 1:
      if (estBonne.value) {
        return modalitesValides.value.length > 0
          ? null
          : 'Ajoutez au moins une modalité de reproductibilité.'
      }
      return form.details_problematique.trim().length >= 10
        ? null
        : 'Les détails de la problématique doivent contenir au moins 10 caractères.'
    case 2:
      return form.pays_id ? null : 'Sélectionnez un territoire.'
    case 3:
      // Une bonne pratique peut être anonyme ; un signalement, jamais.
      if (estBonne.value) return null
      if (!form.identite_nom?.trim() || !form.identite_prenom?.trim()) {
        return 'Vos nom et prénom à l\'état civil sont requis pour un signalement.'
      }
      if (!courrielValide(form.identite_courriel?.trim())) return 'Indiquez un courriel valide.'
      if (!form.identite_contact?.trim()) return 'Indiquez un contact téléphonique.'
      return null
    default:
      return null
  }
}

function suivant() {
  const manque = manqueEtape(etapeCourante.value)
  if (manque) {
    erreurMessage.value = manque
    return
  }
  erreurMessage.value = null
  etapeCourante.value = Math.min(etapeCourante.value + 1, ETAPES.length - 1)
}

/** L'envoi ramène à l'étape fautive : un message invisible est un message perdu. */
function premiereEtapeIncomplete(): number | null {
  for (let i = 0; i < ETAPES.length; i++) if (manqueEtape(i)) return i
  return null
}

function ajouterMedia() {
  if (mediasUrls.value.length < 5) mediasUrls.value.push('')
}

function retirerMedia(idx: number) {
  mediasUrls.value.splice(idx, 1)
}

function reinitialiser() {
  etapeCourante.value = 0
  form.type_pratique = props.typePratiqueInitial
  form.titre = ''
  form.description_generale = ''
  form.details_problematique = ''
  form.categorie_probleme = 'autre'
  form.categorie_probleme_detail = undefined
  form.gravite = props.typePratiqueInitial === 'mauvaise' ? 'faible' : undefined
  form.impact = props.typePratiqueInitial === 'bonne' ? 'fort' : undefined
  form.preuves_temoignages = undefined
  form.solutions_proposees = undefined
  form.reproductibilite = undefined
  form.publication_anonyme = false
  form.pays_id = ''
  form.region = undefined
  form.ville_quartier_zone = undefined
  form.identite_nom = undefined
  form.identite_prenom = undefined
  form.identite_courriel = undefined
  form.identite_contact = undefined
  mediasUrls.value = []
  solutions.value = []
  modalitesReproductibilite.value = props.typePratiqueInitial === 'bonne' ? [''] : []
  preuvesFichiers.value = []
  erreurPhoto.value = null
  erreurMessage.value = null
}

function fermer() {
  if (enCours.value) return
  emit('close')
}

async function soumettre() {
  if (enCours.value) return
  const fautive = premiereEtapeIncomplete()
  if (fautive !== null) {
    erreurMessage.value = manqueEtape(fautive)
    etapeCourante.value = fautive
    return
  }
  enCours.value = true
  erreurMessage.value = null
  try {
    // Bonne pratique : les modalités (liste) sont enregistrées dans `details_problematique`
    const details = estBonne.value
      ? modalitesValides.value.join('\n')
      : form.details_problematique.trim()

    const payload: CreerBadHabitPayload = {
      type_pratique: form.type_pratique,
      titre: form.titre.trim(),
      description_generale: form.description_generale.trim(),
      details_problematique: details,
      categorie_probleme: form.categorie_probleme,
      publication_anonyme: form.publication_anonyme,
      pays_id: form.pays_id,
    }
    if (estBonne.value) {
      if (form.impact) payload.impact = form.impact
      if (form.solutions_proposees?.trim()) payload.reproductibilite = form.solutions_proposees.trim()
    } else {
      // Mauvaise pratique : jamais anonyme + identité réelle obligatoire
      payload.publication_anonyme = false
      payload.identite_nom = form.identite_nom?.trim()
      payload.identite_prenom = form.identite_prenom?.trim()
      payload.identite_courriel = form.identite_courriel?.trim()
      payload.identite_contact = form.identite_contact?.trim()
      if (form.gravite) payload.gravite = form.gravite
      const sols = solutions.value.map(s => s.trim()).filter(s => s.length > 0).slice(0, 10)
      if (sols.length > 0) payload.solutions_propositions = sols
    }
    // Preuves (images ou PDF) : partagées Goodhabits / Badhabits
    if (preuvesFichiers.value.length > 0) {
      payload.preuves_photos = preuvesFichiers.value.map(p => p.url).slice(0, 5)
    }
    if (form.categorie_probleme === 'autre' && form.categorie_probleme_detail?.trim()) {
      payload.categorie_probleme_detail = form.categorie_probleme_detail.trim()
    }
    if (form.preuves_temoignages?.trim()) payload.preuves_temoignages = form.preuves_temoignages.trim()
    if (form.region?.trim()) payload.region = form.region.trim()
    if (form.ville_quartier_zone?.trim()) payload.ville_quartier_zone = form.ville_quartier_zone.trim()

    const urlsValides = mediasUrls.value.map(u => u.trim()).filter(u => u.length > 0)
    if (urlsValides.length > 0) payload.medias_urls = urlsValides

    const id = await creerBadHabit(payload)
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
  if (!estBonne.value) prefillIdentite()
  else if (modalitesReproductibilite.value.length === 0) modalitesReproductibilite.value = ['']
  if (paysListe.value.length === 0) {
    try {
      paysListe.value = await getPays()
    } catch (err) {
      erreurMessage.value = err instanceof Error ? err.message : 'Erreur chargement territoires'
    }
  }
})

watch(() => props.typePratiqueInitial, (v) => {
  if (!props.open) form.type_pratique = v
})
</script>

