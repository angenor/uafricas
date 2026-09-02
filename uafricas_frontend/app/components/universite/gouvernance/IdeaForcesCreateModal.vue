<template>
  <AfricansModale
    :model-value="open"
    titre="Proposer une idée force"
    sous-titre="Partager une orientation pour le développement"
    icone="fa-solid fa-lightbulb"
    taille="large"
    @update:model-value="fermer()"
  >
    <AfricansEtapes :etapes="ETAPES" :courante="etapeCourante" class="mb-6" @aller="etapeCourante = $event" />

    <form id="form-idea-force" class="flex flex-col gap-5" @submit.prevent="soumettre">
      <p
        v-if="erreurMessage"
        class="flex items-center gap-2 rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreurMessage }}
      </p>

      <!-- ─── Étape 1 : l'idée ─── -->
      <template v-if="etapeCourante === 0">
        <AfricansChamp
          v-model="form.titre"
          libelle="Titre de la proposition"
          :maxlength="350"
          placeholder="Formulez votre idée en une phrase claire…"
          obligatoire
        />

        <div class="grid gap-5 md:grid-cols-2">
          <AfricansChamp
            v-model="form.categorie_proposition"
            libelle="Catégorie"
            type="select"
            obligatoire
          >
            <option v-for="c in categories" :key="c.value" :value="c.value">{{ c.label }}</option>
          </AfricansChamp>

          <fieldset>
            <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">Urgence</legend>
            <div class="flex gap-2">
              <button
                v-for="u in urgences"
                :key="u.value"
                type="button"
                class="flex-1 rounded-lg border-2 px-3 py-2.5 text-[12px] font-bold transition"
                :class="form.urgence === u.value ? u.activeClass : 'border-af-bordure bg-white text-af-corps hover:border-af-chocolat'"
                @click="form.urgence = u.value"
              >
                <font-awesome-icon :icon="u.icon" class="mr-1" />
                {{ u.label }}
              </button>
            </div>
          </fieldset>
        </div>

        <AfricansChamp
          v-if="form.categorie_proposition === 'autre'"
          v-model="form.categorie_proposition_detail"
          libelle="Précisez la catégorie"
          :maxlength="200"
        />

        <AfricansChamp
          v-model="form.description_generale"
          libelle="Description générale"
          type="textarea"
          :lignes="3"
          placeholder="Présentez brièvement votre proposition…"
          obligatoire
        />

        <AfricansChamp
          v-model="form.details_proposition"
          libelle="Détails de la proposition"
          type="textarea"
          :lignes="5"
          placeholder="Développez votre idée en détail : objectifs, modalités, bénéficiaires…"
          obligatoire
        />
      </template>

      <!-- ─── Étape 2 : mise en œuvre ─── -->
      <template v-else-if="etapeCourante === 1">
        <div class="grid gap-5 md:grid-cols-2">
          <AfricansChamp
            v-model="form.plan_implementation"
            libelle="Plan d'implémentation"
            type="textarea"
            :lignes="3"
            placeholder="Étapes concrètes pour mettre en œuvre l'idée…"
          />
          <AfricansChamp
            v-model="form.ressources_necessaires"
            libelle="Ressources nécessaires"
            type="textarea"
            :lignes="3"
            placeholder="Moyens humains, financiers, matériels…"
          />
        </div>

        <AfricansChamp
          v-model="form.impact_attendu"
          libelle="Impact attendu"
          type="textarea"
          :lignes="3"
          placeholder="Effets positifs attendus à court et long terme…"
        />

        <div>
          <div class="mb-2 flex items-center justify-between gap-4">
            <span class="text-[14px]/[1.4] text-af-atone italic">
              Modalités opérationnelles concrètes proposées
              <span class="text-af-atone-2">(10 étapes maximum)</span>
            </span>
            <button
              v-if="modalites.length < 10"
              type="button"
              class="shrink-0 text-[12px] font-bold text-af-chocolat transition hover:opacity-70"
              @click="ajouterModalite"
            >
              <font-awesome-icon icon="fa-solid fa-plus" class="mr-1" />
              Ajouter une étape
            </button>
          </div>
          <p v-if="modalites.length === 0" class="text-[12px] text-af-atone-2 italic">
            Aucune étape. Décrivez les étapes concrètes de mise en œuvre (facultatif).
          </p>
          <div v-for="(_, idx) in modalites" :key="idx" class="mb-2 flex items-center gap-2">
            <span class="grid size-7 shrink-0 place-items-center rounded-full bg-af-chocolat/10 text-[12px] font-bold text-af-chocolat">
              {{ idx + 1 }}
            </span>
            <input
              v-model="modalites[idx]"
              type="text"
              maxlength="500"
              :placeholder="`Étape ${idx + 1}…`"
              class="h-11 min-w-0 flex-1 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
            />
            <button
              type="button"
              class="grid size-10 shrink-0 place-items-center rounded-md border border-af-bordure text-af-atone transition hover:border-af-live hover:text-af-live"
              :aria-label="`Retirer l'étape ${idx + 1}`"
              @click="retirerModalite(idx)"
            >
              <font-awesome-icon icon="fa-solid fa-trash" class="text-[12px]" />
            </button>
          </div>
        </div>
      </template>

      <!-- ─── Étape 3 : localisation & médias ─── -->
      <template v-else>
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
              class="shrink-0 text-[12px] font-bold text-af-chocolat transition hover:opacity-70"
              @click="ajouterMedia"
            >
              <font-awesome-icon icon="fa-solid fa-plus" class="mr-1" />
              Ajouter une URL
            </button>
          </div>
          <p v-if="mediasUrls.length === 0" class="text-[12px] text-af-atone-2 italic">
            Aucun média. Ajoutez des URLs d'images, vidéos ou documents si nécessaire.
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

        <p class="flex items-start gap-2 rounded-lg border border-af-chocolat/20 bg-af-chocolat/5 px-4 py-3 text-[12px]/[1.6] text-af-corps">
          <font-awesome-icon icon="fa-solid fa-circle-info" class="mt-0.5 shrink-0 text-af-chocolat" />
          Votre proposition sera publiée immédiatement et visible par toute la communauté.
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
        @click="precedent"
      >
        Précédent
      </AfricansBouton>
      <AfricansBouton v-if="!derniereEtape" icone="fa-solid fa-arrow-right" @click="suivant">
        Suivant
      </AfricansBouton>
      <AfricansBouton
        v-else
        :desactive="enCours"
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
import type { CreerIdeaForcePayload, PaysPublic } from '~/composables/useGouvernance'

interface Props {
  open: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  created: [id: string]
}>()

const { creerIdeaForce, getPays } = useGouvernance()

const form = reactive<CreerIdeaForcePayload>({
  titre: '',
  description_generale: '',
  details_proposition: '',
  categorie_proposition: 'amelioration_gouvernance',
  categorie_proposition_detail: undefined,
  urgence: 'faible',
  plan_implementation: undefined,
  ressources_necessaires: undefined,
  impact_attendu: undefined,
  pays_id: '',
  region: undefined,
  ville_quartier_zone: undefined,
})

const mediasUrls = ref<string[]>([])
const modalites = ref<string[]>([])
const paysListe = ref<PaysPublic[]>([])
const enCours = ref(false)
const erreurMessage = ref<string | null>(null)

const categories = [
  { value: 'amelioration_gouvernance' as const, label: 'Amélioration de la gouvernance' },
  { value: 'education_formation' as const, label: 'Éducation et formation' },
  { value: 'sante_publique' as const, label: 'Santé publique' },
  { value: 'emploi_jeunes' as const, label: 'Emploi des jeunes' },
  { value: 'environnement' as const, label: 'Environnement' },
  { value: 'transport' as const, label: 'Transport' },
  { value: 'union_africains' as const, label: 'Union des africains' },
  { value: 'infrastructures' as const, label: 'Infrastructures' },
  { value: 'retour_cerveaux' as const, label: 'Retour des cerveaux' },
  { value: 'union_diaspora' as const, label: 'Union de la diaspora' },
  { value: 'lutte_corruption' as const, label: 'Lutte contre la corruption' },
  { value: 'urbanisation_durable' as const, label: 'Urbanisation durable' },
  { value: 'acces_energie' as const, label: "Accès à l'énergie" },
  { value: 'autre' as const, label: 'Autre' },
]

const urgences = [
  { value: 'faible' as const, label: 'Faible', icon: 'fa-solid fa-circle-info', activeClass: 'bg-af-vert/10 text-af-vert border-af-vert' },
  { value: 'elevee' as const, label: 'Élevée', icon: 'fa-solid fa-triangle-exclamation', activeClass: 'bg-af-chocolat/10 text-af-chocolat border-af-chocolat' },
  { value: 'critique' as const, label: 'Critique', icon: 'fa-solid fa-fire', activeClass: 'bg-af-live/10 text-af-live border-af-live' },
]

const ETAPES = [
  { titre: "L'idée" },
  { titre: 'Mise en œuvre' },
  { titre: 'Localisation & médias' },
] as const

const etapeCourante = ref(0)
const derniereEtape = computed(() => etapeCourante.value === ETAPES.length - 1)

/**
 * À quelle étape se corrige un message d'erreur donné. Les champs
 * obligatoires vivent tous à l'étape 1, sauf le territoire, à l'étape 3.
 */
function etapeDeLErreur(message: string): number {
  return message.includes('territoire') ? 2 : 0
}

function suivant() {
  const erreur = premiereErreurValidation()
  // Seule l'étape 1 est bloquante en avant : c'est la seule qui porte des
  // champs obligatoires en amont du territoire.
  if (etapeCourante.value === 0 && erreur && etapeDeLErreur(erreur) === 0) {
    erreurMessage.value = erreur
    return
  }
  erreurMessage.value = null
  etapeCourante.value = Math.min(etapeCourante.value + 1, ETAPES.length - 1)
}

function precedent() {
  erreurMessage.value = null
  etapeCourante.value = Math.max(etapeCourante.value - 1, 0)
}

/** Retourne un message d'erreur si le formulaire est invalide, sinon null. */
function premiereErreurValidation(): string | null {
  if (form.titre.trim().length < 5) return 'Le titre doit contenir au moins 5 caractères.'
  if (form.description_generale.trim().length < 10) return 'La description générale doit contenir au moins 10 caractères.'
  if (form.details_proposition.trim().length < 10) return 'Les détails de la proposition doivent contenir au moins 10 caractères.'
  if (!form.categorie_proposition) return 'Veuillez sélectionner une catégorie.'
  if (!form.pays_id) return 'Veuillez sélectionner un territoire.'
  return null
}

function ajouterMedia() {
  if (mediasUrls.value.length < 5) mediasUrls.value.push('')
}

function retirerMedia(idx: number) {
  mediasUrls.value.splice(idx, 1)
}

function ajouterModalite() {
  if (modalites.value.length < 10) modalites.value.push('')
}

function retirerModalite(idx: number) {
  modalites.value.splice(idx, 1)
}

function reinitialiser() {
  form.titre = ''
  form.description_generale = ''
  form.details_proposition = ''
  form.categorie_proposition = 'amelioration_gouvernance'
  form.categorie_proposition_detail = undefined
  form.urgence = 'faible'
  form.plan_implementation = undefined
  form.ressources_necessaires = undefined
  form.impact_attendu = undefined
  form.pays_id = ''
  form.region = undefined
  form.ville_quartier_zone = undefined
  mediasUrls.value = []
  modalites.value = []
  erreurMessage.value = null
  etapeCourante.value = 0
}

function fermer() {
  if (enCours.value) return
  emit('close')
}

async function soumettre() {
  if (enCours.value) return
  const erreur = premiereErreurValidation()
  if (erreur) {
    erreurMessage.value = erreur
    etapeCourante.value = etapeDeLErreur(erreur)
    return
  }
  enCours.value = true
  erreurMessage.value = null
  try {
    const payload: CreerIdeaForcePayload = {
      titre: form.titre.trim(),
      description_generale: form.description_generale.trim(),
      details_proposition: form.details_proposition.trim(),
      categorie_proposition: form.categorie_proposition,
      urgence: form.urgence,
      pays_id: form.pays_id,
    }
    if (form.categorie_proposition === 'autre' && form.categorie_proposition_detail?.trim()) {
      payload.categorie_proposition_detail = form.categorie_proposition_detail.trim()
    }
    if (form.plan_implementation?.trim()) payload.plan_implementation = form.plan_implementation.trim()
    if (form.ressources_necessaires?.trim()) payload.ressources_necessaires = form.ressources_necessaires.trim()
    if (form.impact_attendu?.trim()) payload.impact_attendu = form.impact_attendu.trim()
    if (form.region?.trim()) payload.region = form.region.trim()
    if (form.ville_quartier_zone?.trim()) payload.ville_quartier_zone = form.ville_quartier_zone.trim()

    const urlsValides = mediasUrls.value.map(u => u.trim()).filter(u => u.length > 0)
    if (urlsValides.length > 0) payload.medias_urls = urlsValides

    const etapesValides = modalites.value.map(e => e.trim()).filter(e => e.length > 0).slice(0, 10)
    if (etapesValides.length > 0) payload.modalites_operationnelles = etapesValides

    const id = await creerIdeaForce(payload)
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
  } else if (paysListe.value.length === 0) {
    try {
      paysListe.value = await getPays()
    } catch (err) {
      erreurMessage.value = err instanceof Error ? err.message : 'Erreur chargement territoires'
    }
  }
})
</script>
